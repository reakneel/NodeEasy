use nodeeasy_core::{is_private_ip, CoreError};
use reqwest::{Client, Url};
use std::net::IpAddr;
use std::time::Duration;

#[derive(Clone)]
pub struct SourceFetcher { client: Client, allow_private: bool, max_bytes: usize }

impl SourceFetcher {
    pub fn new(allow_private: bool) -> Result<Self, reqwest::Error> {
        Ok(Self {
            client: Client::builder().timeout(Duration::from_secs(20)).redirect(reqwest::redirect::Policy::limited(5)).build()?,
            allow_private,
            max_bytes: 10 * 1024 * 1024,
        })
    }

    pub async fn fetch(&self, source_url: &str) -> Result<String, CoreError> {
        let url = Url::parse(source_url).map_err(|e| CoreError::InvalidSourceUrl(e.to_string()))?;
        if !matches!(url.scheme(), "http" | "https") { return Err(CoreError::InvalidSourceUrl("only http/https are allowed".into())); }
        if !self.allow_private { self.validate_host(url.host_str().ok_or_else(|| CoreError::InvalidSourceUrl("host missing".into()))?).await?; }
        let response = self.client.get(url).send().await.map_err(|e| CoreError::Decode(e.to_string()))?.error_for_status().map_err(|e| CoreError::Decode(e.to_string()))?;
        let mut body = Vec::new();
        let mut stream = response.bytes_stream();
        use futures_util::StreamExt;
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| CoreError::Decode(e.to_string()))?;
            if body.len() + chunk.len() > self.max_bytes { return Err(CoreError::Decode("source exceeds maximum size".into())); }
            body.extend_from_slice(&chunk);
        }
        String::from_utf8(body).map_err(|e| CoreError::Decode(e.to_string()))
    }

    async fn validate_host(&self, host: &str) -> Result<(), CoreError> {
        if is_private_ip(host) { return Err(CoreError::BlockedSource); }
        let addrs = tokio::net::lookup_host((host, 80)).await.map_err(|e| CoreError::InvalidSourceUrl(e.to_string()))?;
        if addrs.into_iter().map(|x| x.ip()).any(is_blocked_ip) { return Err(CoreError::BlockedSource); }
        Ok(())
    }
}

fn is_blocked_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => v4.is_private() || v4.is_loopback() || v4.is_link_local() || v4.is_unspecified() || v4.is_broadcast() || v4.is_documentation(),
        IpAddr::V6(v6) => v6.is_loopback() || v6.is_unspecified() || v6.is_unique_local() || v6.is_unicast_link_local(),
    }
}
