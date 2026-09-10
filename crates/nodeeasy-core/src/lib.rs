use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Shadowsocks,
    Shadowsocks2022,
    Vmess,
    Vless,
    Trojan,
    Hysteria,
    Hysteria2,
    Tuic,
    Wireguard,
    Socks5,
    Http,
    AnyTls,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NodeEndpoint {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub fingerprint: String,
    pub name: Option<String>,
    pub protocol: Protocol,
    pub endpoint: NodeEndpoint,
}

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("invalid node endpoint")]
    InvalidEndpoint,
    #[error("unsupported protocol: {0}")]
    UnsupportedProtocol(String),
}
