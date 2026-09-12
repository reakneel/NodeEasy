use base64::{engine::general_purpose::STANDARD, Engine};
use nodeeasy_core::{Node, Protocol};
use serde_json::{json, Value};
use crate::secrets::SecretMaterial;

fn s(v:&Value,k:&str)->Option<String>{v.get(k).and_then(Value::as_str).map(ToOwned::to_owned)}
fn required(v:&Value,k:&str)->Result<String,String>{s(v,k).ok_or_else(||format!("missing secret option: {k}"))}

pub fn mihomo(node:&Node, secret:Option<&SecretMaterial>)->Result<String,String>{
    let o=secret.map(|s|&s.options).ok_or("credentials are required for config export")?;
    let mut m=serde_yaml::Mapping::new();
    m.insert("name".into(), Value::String(node.name.clone().unwrap_or_else(||node.id.to_string())));
    m.insert("type".into(), Value::String(format!("{:?}",node.protocol).to_lowercase()));
    m.insert("server".into(), Value::String(node.endpoint.host.clone()));
    m.insert("port".into(), Value::Number(node.endpoint.port.into()));
    for (k,v) in o.as_object().ok_or("secret options must be an object")? { m.insert(Value::String(k.clone()),v.clone()); }
    serde_yaml::to_string(&vec![Value::Mapping(m)]).map_err(|e|e.to_string()).map(|body|format!("proxies:\n{body}"))
}

pub fn sing_box(node:&Node, secret:Option<&SecretMaterial>)->Result<String,String>{
    let o=secret.map(|s|&s.options).ok_or("credentials are required for config export")?;
    let mut out=json!({"type":format!("{:?}",node.protocol).to_lowercase(),"tag":node.name.clone().unwrap_or_else(||node.id.to_string()),"server":node.endpoint.host,"server_port":node.endpoint.port});
    let map=out.as_object_mut().unwrap();
    if let Some(obj)=o.as_object(){for(k,v) in obj{map.insert(k.clone(),v.clone());}}
    serde_json::to_string_pretty(&json!({"outbounds":[out]})).map_err(|e|e.to_string())
}

pub fn v2ray_uri(node:&Node, secret:Option<&SecretMaterial>)->Result<String,String>{
    let o=secret.map(|s|&s.options).ok_or("credentials are required for URI export")?;
    let name=node.name.clone().unwrap_or_else(||node.id.to_string());
    let name_enc=urlencoding(&name);
    let uri=match node.protocol {
        Protocol::Vless=>{let id=required(o,"uuid")?;format!("vless://{id}@{}:{}?{}#{name_enc}",node.endpoint.host,node.endpoint.port,query(o))}
        Protocol::Trojan=>{let password=required(o,"password")?;format!("trojan://{password}@{}:{}?{}#{name_enc}",node.endpoint.host,node.endpoint.port,query(o))}
        Protocol::Vmess=>{let id=required(o,"uuid")?;let obj=json!({"v":"2","ps":name,"add":node.endpoint.host,"port":node.endpoint.port.to_string(),"id":id,"aid":s(o,"aid").unwrap_or_else(||"0".into()),"net":s(o,"network").unwrap_or_else(||"tcp".into()),"type":s(o,"type").unwrap_or_else(||"none".into()),"tls":s(o,"tls").unwrap_or_default()});format!("vmess://{}",STANDARD.encode(serde_json::to_vec(&obj).map_err(|e|e.to_string())?))}
        _=>return Err("V2Ray URI exporter currently supports VMess, VLESS and Trojan".into()),
    };Ok(uri)
}
fn query(o:&Value)->String{let Some(m)=o.as_object() else{return String::new()};m.iter().filter_map(|(k,v)|v.as_str().map(|x|format!("{}={}",urlencoding(k),urlencoding(x)))).collect::<Vec<_>>().join("&")}
fn urlencoding(s:&str)->String{s.bytes().map(|b|match b{b'A'..=b'Z'|b'a'..=b'z'|b'0'..=b'9'|b'-'|b'_'|b'.'|b'~'=>char::from(b).to_string(),_=>format!("%{b:02X}")}).collect()}
