use async_trait::async_trait;
use nodeeasy_core::{Node, Protocol};
use serde::Serialize;
use serde_json::{json, Value};
use std::{path::PathBuf, time::Duration};
use tokio::{process::Command, time::timeout};

#[derive(Debug, Clone, Copy, Serialize)]
pub enum EngineKind { Mihomo, SingBox, Xray }
#[derive(Debug, Clone, Serialize)]
pub struct EngineConfig { pub kind: EngineKind, pub config: String }
#[derive(Debug, thiserror::Error)]
pub enum EngineError { #[error("unsupported protocol for engine")] UnsupportedProtocol, #[error("engine binary is unavailable")] BinaryUnavailable, #[error("engine exited with status {0}")] Exit(String), #[error("engine timed out")] Timeout, #[error("engine I/O: {0}")] Io(String), #[error("serialization: {0}")] Serialize(String) }

#[async_trait]
pub trait EngineAdapter: Send + Sync {
    fn kind(&self) -> EngineKind;
    fn build_config(&self, node: &Node, secret: Option<&Value>) -> Result<EngineConfig, EngineError>;
    async fn validate_binary(&self, binary: &str) -> Result<(), EngineError>;
}

pub struct MihomoAdapter;
pub struct SingBoxAdapter;
pub struct XrayAdapter;

fn protocol_type(p: Protocol) -> &'static str { match p { Protocol::Shadowsocks|Protocol::Shadowsocks2022=>"ss", Protocol::Vmess=>"vmess", Protocol::Vless=>"vless", Protocol::Trojan=>"trojan", Protocol::Hysteria=>"hysteria", Protocol::Hysteria2=>"hysteria2", Protocol::Tuic=>"tuic", Protocol::Wireguard=>"wireguard", Protocol::Socks5=>"socks5", Protocol::Http=>"http", Protocol::AnyTls=>"anytls" } }
fn tag(node:&Node)->String { node.name.clone().unwrap_or_else(||node.id.to_string()) }
fn merge(base:&mut serde_json::Map<String,Value>, secret:Option<&Value>){ if let Some(Value::Object(m))=secret { for (k,v) in m { base.insert(k.clone(),v.clone()); } } }
fn run_check(binary:&str)->impl std::future::Future<Output=Result<(),EngineError>> { let binary=binary.to_owned(); async move { let result=timeout(Duration::from_secs(5),Command::new(&binary).arg("version").output()).await.map_err(|_|EngineError::Timeout)?.map_err(|e|EngineError::Io(e.to_string()))?; if result.status.success(){Ok(())}else{Err(EngineError::Exit(String::from_utf8_lossy(&result.stderr).trim().to_owned()))} } }

#[async_trait]
impl EngineAdapter for MihomoAdapter {
    fn kind(&self)->EngineKind{EngineKind::Mihomo}
    fn build_config(&self,node:&Node,secret:Option<&Value>)->Result<EngineConfig,EngineError>{
        let mut p=serde_yaml::Mapping::new();p.insert("name".into(),tag(node).into());p.insert("type".into(),protocol_type(node.protocol).into());p.insert("server".into(),node.endpoint.host.clone().into());p.insert("port".into(),serde_yaml::Value::Number((node.endpoint.port as u64).into()));
        if let Some(Value::Object(m))=secret { for(k,v)in m{p.insert(k.clone().into(),serde_yaml::to_value(v).map_err(|e|EngineError::Serialize(e.to_string()))?);} }
        Ok(EngineConfig{kind:EngineKind::Mihomo,config:serde_yaml::to_string(&json!({"proxies":[p]})).map_err(|e|EngineError::Serialize(e.to_string()))?})
    }
    async fn validate_binary(&self,binary:&str)->Result<(),EngineError>{run_check(binary).await}
}
#[async_trait]
impl EngineAdapter for SingBoxAdapter {
    fn kind(&self)->EngineKind{EngineKind::SingBox}
    fn build_config(&self,node:&Node,secret:Option<&Value>)->Result<EngineConfig,EngineError>{let mut out=json!({"type":protocol_type(node.protocol),"tag":tag(node),"server":node.endpoint.host,"server_port":node.endpoint.port});if let Some(m)=out.as_object_mut(){merge(m,secret);}Ok(EngineConfig{kind:EngineKind::SingBox,config:serde_json::to_string_pretty(&json!({"outbounds":[out]})).map_err(|e|EngineError::Serialize(e.to_string()))?})}
    async fn validate_binary(&self,binary:&str)->Result<(),EngineError>{run_check(binary).await}
}
#[async_trait]
impl EngineAdapter for XrayAdapter {
    fn kind(&self)->EngineKind{EngineKind::Xray}
    fn build_config(&self,node:&Node,secret:Option<&Value>)->Result<EngineConfig,EngineError>{
        let mut vnext=json!({"address":node.endpoint.host,"port":node.endpoint.port});if let Some(Value::Object(m))=secret{merge(vnext.as_object_mut().ok_or(EngineError::Serialize("invalid vnext".into()))?,Some(&Value::Object(m.clone())));}
        let stream=secret.and_then(|v|v.get("streamSettings")).cloned().unwrap_or_else(||json!({}));
        let client=secret.and_then(|v|v.get("uuid")).map(|uuid|json!({"id":uuid})).unwrap_or_else(||json!({}));
        let protocol=match node.protocol{Protocol::Vmess=>"vmess",Protocol::Vless=>"vless",Protocol::Trojan=>"trojan",_=>return Err(EngineError::UnsupportedProtocol)};
        let outbound=json!({"protocol":protocol,"settings":{"vnext":[{"address":node.endpoint.host,"port":node.endpoint.port,"users":[client]}]},"streamSettings":stream});
        Ok(EngineConfig{kind:EngineKind::Xray,config:serde_json::to_string_pretty(&json!({"outbounds":[outbound]})).map_err(|e|EngineError::Serialize(e.to_string()))?})
    }
    async fn validate_binary(&self,binary:&str)->Result<(),EngineError>{run_check(binary).await}
}

pub fn adapter(kind:EngineKind)->Box<dyn EngineAdapter>{match kind{EngineKind::Mihomo=>Box::new(MihomoAdapter),EngineKind::SingBox=>Box::new(SingBoxAdapter),EngineKind::Xray=>Box::new(XrayAdapter)}}

pub async fn run_engine_binary(binary:&str,config_path:PathBuf)->Result<(),EngineError>{let status=timeout(Duration::from_secs(30),Command::new(binary).arg("run").arg("-c").arg(config_path).status()).await.map_err(|_|EngineError::Timeout)?.map_err(|e|EngineError::Io(e.to_string()))?;if status.success(){Ok(())}else{Err(EngineError::Exit(status.to_string()))}}
