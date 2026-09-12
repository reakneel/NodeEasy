use async_trait::async_trait;
use nodeeasy_core::{source::{deduplicate, parse_subscription, ParsedNode}, CoreError};
use std::{path::PathBuf, sync::Arc};
use tokio::fs;

use crate::source_fetcher::SourceFetcher;

#[async_trait]
pub trait SourceAdapter: Send + Sync {
    fn kind(&self) -> &'static str;
    async fn collect(&self) -> Result<Vec<ParsedNode>, CoreError>;
}

pub type SharedSourceAdapter = Arc<dyn SourceAdapter>;

pub struct HttpSourceAdapter { pub url: String, pub fetcher: SourceFetcher }
#[async_trait]
impl SourceAdapter for HttpSourceAdapter {
    fn kind(&self) -> &'static str { "http_subscription" }
    async fn collect(&self) -> Result<Vec<ParsedNode>, CoreError> {
        Ok(deduplicate(parse_subscription(&self.fetcher.fetch(&self.url).await?)?))
    }
}

pub struct GithubRawSourceAdapter { pub url: String, pub fetcher: SourceFetcher }
#[async_trait]
impl SourceAdapter for GithubRawSourceAdapter {
    fn kind(&self) -> &'static str { "github_raw" }
    async fn collect(&self) -> Result<Vec<ParsedNode>, CoreError> {
        Ok(deduplicate(parse_subscription(&self.fetcher.fetch(&self.url).await?)?))
    }
}

pub struct LocalFileSourceAdapter { pub path: PathBuf }
#[async_trait]
impl SourceAdapter for LocalFileSourceAdapter {
    fn kind(&self) -> &'static str { "local_file" }
    async fn collect(&self) -> Result<Vec<ParsedNode>, CoreError> {
        let meta = fs::metadata(&self.path).await.map_err(|e| CoreError::Decode(e.to_string()))?;
        if meta.len() > 10 * 1024 * 1024 { return Err(CoreError::Decode("local source exceeds maximum size".into())); }
        let body = fs::read_to_string(&self.path).await.map_err(|e| CoreError::Decode(e.to_string()))?;
        Ok(deduplicate(parse_subscription(&body)?))
    }
}

pub struct ManualSourceAdapter { pub body: String }
#[async_trait]
impl SourceAdapter for ManualSourceAdapter {
    fn kind(&self) -> &'static str { "manual" }
    async fn collect(&self) -> Result<Vec<ParsedNode>, CoreError> {
        if self.body.len() > 10 * 1024 * 1024 { return Err(CoreError::Decode("manual source exceeds maximum size".into())); }
        Ok(deduplicate(parse_subscription(&self.body)?))
    }
}

pub fn github_raw_url(input: &str) -> Result<String, CoreError> {
    let trimmed = input.trim();
    if trimmed.starts_with("https://raw.githubusercontent.com/") { return Ok(trimmed.to_owned()); }
    let Some(rest) = trimmed.strip_prefix("https://github.com/") else {
        return Err(CoreError::InvalidSourceUrl("GitHub source must be a github.com repository URL or raw.githubusercontent.com URL".into()));
    };
    let parts: Vec<_> = rest.trim_end_matches('/').split('/').collect();
    if parts.len() >= 5 && parts[2] == "blob" {
        return Ok(format!("https://raw.githubusercontent.com/{}/{}/{}/{}", parts[0], parts[1], parts[3], parts[4..].join("/")));
    }
    Err(CoreError::InvalidSourceUrl("GitHub URL must point to a blob path".into()))
}
