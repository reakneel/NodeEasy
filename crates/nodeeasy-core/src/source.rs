use crate::{CoreError, Node, NodeEndpoint, NodeStatus, Protocol};
use base64::{engine::general_purpose::STANDARD, Engine};
use chrono::Utc;
use serde::Deserialize;
use serde_json::Value;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputFormat { Uri, Base64Uri, ClashYaml, ClashJson }

#[derive(Debug, Clone)]
pub struct ParsedNode { pub node: Node, pub source_name: Option<String>, pub raw: String }

pub fn detect_format(body: &str) -> InputFormat {
    let trimmed = body.trim_start();
    if trimmed.starts_with('{') { return InputFormat::ClashJson; }
    if trimmed.starts_with("proxies:") || trimmed.contains("\nproxies:") { return InputFormat::ClashYaml; }
    if trimmed.lines().any(|line| line.trim_start().starts_with("ss://") || line.trim_start().starts_with("vmess://") || line.trim_start().starts_with("vless://")) { return InputFormat::Uri; }
    InputFormat::Base64Uri
}

pub fn parse_subscription(body: &str) -> Result<Vec<ParsedNode>, CoreError> {
    let input = body.trim();
    let decoded = match detect_format(input) {
        InputFormat::Base64Uri => STANDARD.decode(input.as_bytes()).ok().and_then(|b| String::from_utf8(b).ok()).unwrap_or_else(|| input.to_owned()),
        _ => input.to_owned(),
    };
    if matches!(detect_format(&decoded), InputFormat::ClashYaml | InputFormat::ClashJson) {
        return parse_clash(&decoded);
    }
    let mut result = Vec::new();
    for line in decoded.lines().map(str::trim).filter(|x| !x.is_empty() && !x.starts_with('#')) {
        if let Ok(parsed) = parse_uri(line) { result.push(parsed); }
    }
    if result.is_empty() { return Err(CoreError::Parse("no supported node entries found".into())); }
    Ok(result)
}

fn parse_uri(raw: &str) -> Result<ParsedNode, CoreError> {
    let url = Url::parse(raw).map_err(|e| CoreError::Parse(e.to_string()))?;
    let protocol = match url.scheme().to_ascii_lowercase().as_str() {
        "ss" => Protocol::Shadowsocks,
        "vmess" => Protocol::Vmess,
        "vless" => Protocol::Vless,
        "trojan" => Protocol::Trojan,
        "hy" | "hysteria" => Protocol::Hysteria,
        "hy2" | "hysteria2" => Protocol::Hysteria2,
        "tuic" => Protocol::Tuic,
        "socks" | "socks5" => Protocol::Socks5,
        "http" | "https" => Protocol::Http,
        "anytls" => Protocol::AnyTls,
        _ => return Err(CoreError::UnsupportedProtocol(url.scheme().into())),
    };
    if protocol == Protocol::Vmess { return parse_vmess(raw); }
    let host = url.host_str().ok_or(CoreError::InvalidEndpoint)?.to_string();
    let port = url.port().unwrap_or(match protocol { Protocol::Http => 80, _ => 443 });
    let endpoint = NodeEndpoint { host, port };
    endpoint.validate()?;
    let name = url.fragment().map(|x| x.to_string()).filter(|x| !x.is_empty());
    let mut identity = vec![("scheme", url.scheme())];
    if let Some(user) = url.username().strip_prefix("") { identity.push(("user", user)); }
    let fingerprint = Node::fingerprint(protocol, &endpoint, &identity);
    let now = Utc::now();
    Ok(ParsedNode { node: Node { id:Uuid::new_v4(), fingerprint, name, protocol, endpoint, country:None, city:None, asn:None, status:NodeStatus::New, score:0.0, first_seen_at:now, last_seen_at:now, last_tested_at:None }, source_name:None, raw:raw.to_string() })
}

fn parse_vmess(raw: &str) -> Result<ParsedNode, CoreError> {
    let encoded = raw.strip_prefix("vmess://").ok_or_else(|| CoreError::Parse("invalid vmess URI".into()))?;
    let bytes = STANDARD.decode(encoded).or_else(|_| base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(encoded)).map_err(|e| CoreError::Decode(e.to_string()))?;
    let config: Vmess = serde_json::from_slice(&bytes).map_err(|e| CoreError::Parse(e.to_string()))?;
    let port = config.port.parse::<u16>().map_err(|_| CoreError::InvalidEndpoint)?;
    let endpoint = NodeEndpoint { host:config.add.clone(), port };
    endpoint.validate()?;
    let fingerprint = Node::fingerprint(Protocol::Vmess, &endpoint, &[("id", &config.id), ("net", &config.net)]);
    let now=Utc::now();
    Ok(ParsedNode { node:Node{id:Uuid::new_v4(),fingerprint,name:(!config.ps.is_empty()).then_some(config.ps),protocol:Protocol::Vmess,endpoint,country:None,city:None,asn:None,status:NodeStatus::New,score:0.0,first_seen_at:now,last_seen_at:now,last_tested_at:None},source_name:None,raw:raw.to_string() })
}

#[derive(Debug, Deserialize)]
struct Vmess { #[serde(default)] ps:String, add:String, port:String, id:String, #[serde(default)] net:String }

fn parse_clash(body: &str) -> Result<Vec<ParsedNode>, CoreError> {
    let root: Value = if body.trim_start().starts_with('{') { serde_json::from_str(body).map_err(|e| CoreError::Parse(e.to_string()))? } else { serde_yaml::from_str(body).map_err(|e| CoreError::Parse(e.to_string()))? };
    let proxies = root.get("proxies").and_then(Value::as_array).ok_or_else(|| CoreError::Parse("missing proxies array".into()))?;
    let mut out=Vec::new();
    for proxy in proxies {
        let name=proxy.get("name").and_then(Value::as_str).map(str::to_owned);
        let p=proxy.get("type").and_then(Value::as_str).unwrap_or_default().to_ascii_lowercase();
        let protocol=match p.as_str(){"ss"=>Protocol::Shadowsocks,"ssr"=>Protocol::Shadowsocks,"vmess"=>Protocol::Vmess,"vless"=>Protocol::Vless,"trojan"=>Protocol::Trojan,"hysteria"=>Protocol::Hysteria,"hysteria2"=>Protocol::Hysteria2,"hy2"=>Protocol::Hysteria2,"tuic"=>Protocol::Tuic,"wireguard"=>Protocol::Wireguard,"socks5"=>Protocol::Socks5,"http"=>Protocol::Http,"anytls"=>Protocol::AnyTls,_=>continue};
        let host=proxy.get("server").and_then(Value::as_str).ok_or_else(|| CoreError::Parse("proxy server missing".into()))?.to_owned();
        let port=proxy.get("port").and_then(Value::as_u64).and_then(|x|u16::try_from(x).ok()).ok_or(CoreError::InvalidEndpoint)?;
        let endpoint=NodeEndpoint{host,port}; endpoint.validate()?;
        let fingerprint=Node::fingerprint(protocol,&endpoint,&[]); let now=Utc::now();
        out.push(ParsedNode{node:Node{id:Uuid::new_v4(),fingerprint,name,protocol,endpoint,country:None,city:None,asn:None,status:NodeStatus::New,score:0.0,first_seen_at:now,last_seen_at:now,last_tested_at:None},source_name:None,raw:serde_json::to_string(proxy).unwrap_or_default()});
    }
    if out.is_empty(){return Err(CoreError::Parse("no supported proxies found".into()));} Ok(out)
}

pub fn deduplicate(nodes: impl IntoIterator<Item=ParsedNode>) -> Vec<ParsedNode> {
    let mut seen=std::collections::HashSet::new();
    nodes.into_iter().filter(|n|seen.insert(n.node.fingerprint.clone())).collect()
}
