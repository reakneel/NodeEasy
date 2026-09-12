use crate::{CoreError, Node, NodeEndpoint, NodeStatus, Protocol};
use base64::{engine::general_purpose::{STANDARD, URL_SAFE, URL_SAFE_NO_PAD}, Engine};
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
    let t = body.trim_start();
    if t.starts_with('{') { InputFormat::ClashJson } else if t.starts_with("proxies:") || t.contains("\nproxies:") { InputFormat::ClashYaml }
    else if t.lines().any(|l| { let x=l.trim_start(); x.starts_with("ss://") || x.starts_with("ss2022://") || x.starts_with("vmess://") || x.starts_with("vless://") || x.starts_with("trojan://") || x.starts_with("hysteria://") || x.starts_with("hysteria2://") || x.starts_with("hy2://") || x.starts_with("tuic://") || x.starts_with("socks5://") || x.starts_with("http://") }) { InputFormat::Uri } else { InputFormat::Base64Uri }
}

pub fn parse_subscription(body: &str) -> Result<Vec<ParsedNode>, CoreError> {
    let input=body.trim();
    let decoded=if matches!(detect_format(input),InputFormat::Base64Uri){decode_base64_text(input).unwrap_or_else(||input.to_owned())}else{input.to_owned()};
    if matches!(detect_format(&decoded),InputFormat::ClashYaml|InputFormat::ClashJson){return parse_clash(&decoded);}
    let mut out=Vec::new();
    for line in decoded.lines().map(str::trim).filter(|x|!x.is_empty()&&!x.starts_with('#')) { if let Ok(n)=parse_uri(line){out.push(n);} }
    if out.is_empty(){Err(CoreError::Parse("no supported node entries found".into()))}else{Ok(out)}
}

fn decode_base64_text(input:&str)->Option<String>{
    [STANDARD.decode(input),URL_SAFE_NO_PAD.decode(input),URL_SAFE.decode(input)].into_iter().flatten().find_map(|b|String::from_utf8(b).ok())
}
fn node(protocol:Protocol,host:String,port:u16,name:Option<String>,identity:Vec<(String,String)>,raw:&str)->Result<ParsedNode,CoreError>{
    let endpoint=NodeEndpoint{host,port}; endpoint.validate()?;
    let refs=identity.iter().map(|(k,v)|(k.as_str(),v.as_str())).collect::<Vec<_>>();
    let fingerprint=Node::fingerprint(protocol,&endpoint,&refs); let now=Utc::now();
    Ok(ParsedNode{node:Node{id:Uuid::new_v4(),fingerprint,name,protocol,endpoint,country:None,city:None,asn:None,status:NodeStatus::New,score:0.0,first_seen_at:now,last_seen_at:now,last_tested_at:None},source_name:None,raw:raw.to_owned()})
}
fn parse_uri(raw:&str)->Result<ParsedNode,CoreError>{
    let u=Url::parse(raw).map_err(|e|CoreError::Parse(e.to_string()))?; let scheme=u.scheme().to_ascii_lowercase();
    if scheme=="vmess" {return parse_vmess(raw)}
    let protocol=match scheme.as_str(){"ss"=>Protocol::Shadowsocks,"ss2022"=>Protocol::Shadowsocks2022,"vless"=>Protocol::Vless,"trojan"=>Protocol::Trojan,"hysteria"=>Protocol::Hysteria,"hysteria2"|"hy2"=>Protocol::Hysteria2,"tuic"=>Protocol::Tuic,"socks"|"socks5"=>Protocol::Socks5,"http"|"https"=>Protocol::Http,"anytls"=>Protocol::AnyTls,_=>return Err(CoreError::UnsupportedProtocol(scheme))};
    let host=u.host_str().ok_or(CoreError::InvalidEndpoint)?.to_owned(); let port=u.port().unwrap_or(if protocol==Protocol::Http{80}else{443});
    let name=u.fragment().map(str::to_owned).filter(|x|!x.is_empty()); let mut identity=vec![("scheme".into(),scheme.clone())];
    if !u.username().is_empty(){identity.push(("user".into(),u.username().to_owned()));}
    if let Some(p)=u.password(){identity.push(("password".into(),p.to_owned()));}
    node(protocol,host,port,name,identity,raw)
}
fn parse_vmess(raw:&str)->Result<ParsedNode,CoreError>{
    let encoded=raw.strip_prefix("vmess://").ok_or_else(||CoreError::Parse("invalid vmess URI".into()))?;
    let bytes=STANDARD.decode(encoded).or_else(|_|URL_SAFE_NO_PAD.decode(encoded)).map_err(|e|CoreError::Decode(e.to_string()))?;
    let c:Vmess=serde_json::from_slice(&bytes).map_err(|e|CoreError::Parse(e.to_string()))?; let port=c.port.parse().map_err(|_|CoreError::InvalidEndpoint);
    node(Protocol::Vmess,c.add,port,c.ps.is_empty().then_some(c.add.clone()).or_else(||Some(c.ps)),vec![("id".into(),c.id),("net".into(),c.net)],raw)
}
#[derive(Debug,Deserialize)] struct Vmess{#[serde(default)]ps:String,add:String,port:String,id:String,#[serde(default)]net:String}
fn parse_clash(body:&str)->Result<Vec<ParsedNode>,CoreError>{
    let root:Value=if body.trim_start().starts_with('{'){serde_json::from_str(body)}else{serde_yaml::from_str(body)}.map_err(|e|CoreError::Parse(e.to_string()))?;
    let proxies=root.get("proxies").and_then(Value::as_array).ok_or_else(||CoreError::Parse("missing proxies array".into()))?; let mut out=Vec::new();
    for p in proxies{let Some(host)=p.get("server").and_then(Value::as_str)else{continue};let Some(port)=p.get("port").and_then(Value::as_u64).and_then(|x|u16::try_from(x).ok())else{continue};let typ=p.get("type").and_then(Value::as_str).unwrap_or("").to_ascii_lowercase();let protocol=match typ.as_str(){"ss"=>Protocol::Shadowsocks,"ssr"=>Protocol::Shadowsocks,"ss2022"|"shadowsocks2022"=>Protocol::Shadowsocks2022,"vmess"=>Protocol::Vmess,"vless"=>Protocol::Vless,"trojan"=>Protocol::Trojan,"hysteria"=>Protocol::Hysteria,"hysteria2"|"hy2"=>Protocol::Hysteria2,"tuic"=>Protocol::Tuic,"wireguard"=>Protocol::Wireguard,"socks5"=>Protocol::Socks5,"http"=>Protocol::Http,"anytls"=>Protocol::AnyTls,_=>continue};let name=p.get("name").and_then(Value::as_str).map(str::to_owned);let mut identity=Vec::new();for k in ["uuid","password","username","cipher","network","sni","tls","client-fingerprint","flow"]{if let Some(v)=p.get(k){if let Some(s)=v.as_str(){identity.push((k.to_owned(),s.to_owned()));}}}if let Ok(n)=node(protocol,host.to_owned(),port,name,identity,&serde_json::to_string(p).unwrap_or_default()){out.push(n);}}
    if out.is_empty(){Err(CoreError::Parse("no supported proxies found".into()))}else{Ok(out)}
}
pub fn deduplicate(nodes:impl IntoIterator<Item=ParsedNode>)->Vec<ParsedNode>{let mut seen=std::collections::HashSet::new();nodes.into_iter().filter(|n|seen.insert(n.node.fingerprint.clone())).collect()}
