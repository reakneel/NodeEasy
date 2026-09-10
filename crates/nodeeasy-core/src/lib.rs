use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::net::IpAddr;
use uuid::Uuid;

pub mod source;
#[cfg(test)] mod source_tests;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Protocol { Shadowsocks, Shadowsocks2022, Vmess, Vless, Trojan, Hysteria, Hysteria2, Tuic, Wireguard, Socks5, Http, AnyTls }

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum NodeStatus { New, Healthy, Degraded, Unavailable, Stale }

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TestType { Tcp, Tls, Http, Latency, Download, Stability }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NodeEndpoint { pub host:String, pub port:u16 }
impl NodeEndpoint { pub fn validate(&self)->Result<(),CoreError>{if self.host.trim().is_empty()||self.port==0{Err(CoreError::InvalidEndpoint)}else{Ok(())}} }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node { pub id:Uuid,pub fingerprint:String,pub name:Option<String>,pub protocol:Protocol,pub endpoint:NodeEndpoint,pub country:Option<String>,pub city:Option<String>,pub asn:Option<String>,pub status:NodeStatus,pub score:f64,pub first_seen_at:DateTime<Utc>,pub last_seen_at:DateTime<Utc>,pub last_tested_at:Option<DateTime<Utc>> }
impl Node { pub fn fingerprint(protocol:Protocol,endpoint:&NodeEndpoint,identity:&[(&str,&str)])->String{let mut h=Sha256::new();h.update(format!("{:?}|{}|{}",protocol,endpoint.host.to_ascii_lowercase(),endpoint.port));for(k,v)in identity{h.update(b"|");h.update(k.as_bytes());h.update(b"=");h.update(v.as_bytes());}format!("{:x}",h.finalize())} }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source { pub id:Uuid,pub name:String,pub kind:String,pub url:String,pub enabled:bool,pub interval_seconds:u64,pub last_fetch_at:Option<DateTime<Utc>>,pub last_success_at:Option<DateTime<Utc>>,pub last_error:Option<String> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeTest { pub id:Uuid,pub node_id:Uuid,pub test_type:TestType,pub latency_ms:Option<f64>,pub download_bps:Option<f64>,pub upload_bps:Option<f64>,pub success:bool,pub error:Option<String>,pub tested_at:DateTime<Utc>,pub duration_ms:Option<u64> }

pub fn is_private_ip(host:&str)->bool{host.parse::<IpAddr>().map(|ip|match ip{IpAddr::V4(ip)=>ip.is_private()||ip.is_loopback()||ip.is_link_local()||ip.is_unspecified()||ip.is_broadcast()||ip.is_documentation(),IpAddr::V6(ip)=>ip.is_loopback()||ip.is_unspecified()||ip.is_unique_local()||ip.is_unicast_link_local()}).unwrap_or(false)}

#[derive(Debug,thiserror::Error)]
pub enum CoreError { #[error("invalid node endpoint")] InvalidEndpoint,#[error("unsupported protocol: {0}")] UnsupportedProtocol(String),#[error("invalid source URL: {0}")] InvalidSourceUrl(String),#[error("source blocked by SSRF policy")] BlockedSource,#[error("decode failed: {0}")] Decode(String),#[error("parse failed: {0}")] Parse(String) }
