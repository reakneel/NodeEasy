use nodeeasy_core::TestType;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{net::IpAddr, time::Duration};
use tokio::net::TcpStream;
use tokio_util::sync::CancellationToken;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbeConfig {
    pub timeout_ms: u64,
    pub attempts: u32,
    pub http_path: String,
    pub download_url: Option<String>,
}
impl Default for ProbeConfig {
    fn default() -> Self { Self { timeout_ms: 5000, attempts: 3, http_path: "/".into(), download_url: None } }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub test_type: TestType,
    pub success: bool,
    pub latency_ms: Option<f64>,
    pub download_bps: Option<f64>,
    pub duration_ms: u64,
    pub bytes: u64,
    pub attempts: u32,
    pub error: Option<String>,
}

fn bounded_attempts(n: u32) -> u32 { n.clamp(1, 20) }
fn timeout(c: &ProbeConfig) -> Duration { Duration::from_millis(c.timeout_ms.clamp(250, 60_000)) }

pub async fn tcp(host: &str, port: u16, c: &ProbeConfig, cancel: &CancellationToken) -> Observation {
    let started = std::time::Instant::now();
    let fut = TcpStream::connect((host, port));
    let result = tokio::select! { _ = cancel.cancelled() => return Observation { test_type: TestType::Tcp, success:false, latency_ms:None, download_bps:None, duration_ms:started.elapsed().as_millis() as u64, bytes:0, attempts:1, error:Some("cancelled".into()) }, r = tokio::time::timeout(timeout(c), fut) => r };
    let success = matches!(result, Ok(Ok(_)));
    Observation { test_type: TestType::Tcp, success, latency_ms: success.then(|| started.elapsed().as_secs_f64()*1000.0), download_bps:None, duration_ms:started.elapsed().as_millis() as u64, bytes:0, attempts:1, error:(!success).then_some("tcp connect failed or timed out".into()) }
}

fn endpoint_url(scheme: &str, host: &str, port: u16, path: &str) -> Result<Url, String> {
    let url = Url::parse(&format!("{scheme}://{host}:{port}{path}")).map_err(|e| e.to_string())?;
    validate_public_target(&url)?;
    Ok(url)
}

pub fn validate_public_target(url: &Url) -> Result<(), String> {
    if url.scheme() != "http" && url.scheme() != "https" { return Err("only http/https targets are allowed".into()); }
    let host = url.host_str().ok_or("target has no host")?;
    if host.eq_ignore_ascii_case("localhost") || host.ends_with(".localhost") { return Err("private target blocked".into()); }
    if let Ok(ip) = host.parse::<IpAddr>() {
        if nodeeasy_core::is_private_ip(&ip.to_string()) { return Err("private target blocked".into()); }
    }
    Ok(())
}

pub async fn tls(host: &str, port: u16, c: &ProbeConfig, cancel: &CancellationToken) -> Observation {
    http_like(TestType::Tls, endpoint_url("https", host, port, "/"), c, cancel).await
}

pub async fn http(host: &str, port: u16, c: &ProbeConfig, cancel: &CancellationToken) -> Observation {
    let path = if c.http_path.starts_with('/') { c.http_path.as_str() } else { "/" };
    http_like(TestType::Http, endpoint_url("http", host, port, path), c, cancel).await
}

async fn http_like(kind: TestType, target: Result<Url, String>, c: &ProbeConfig, cancel: &CancellationToken) -> Observation {
    let started = std::time::Instant::now();
    let target = match target { Ok(v)=>v, Err(e)=>return Observation { test_type:kind, success:false, latency_ms:None, download_bps:None, duration_ms:0, bytes:0, attempts:1, error:Some(e) } };
    let client = match Client::builder().timeout(timeout(c)).redirect(reqwest::redirect::Policy::limited(3)).build() { Ok(v)=>v, Err(e)=>return Observation { test_type:kind, success:false, latency_ms:None, download_bps:None, duration_ms:0, bytes:0, attempts:1, error:Some(e.to_string()) } };
    let result = tokio::select! { _ = cancel.cancelled() => return Observation { test_type:kind, success:false, latency_ms:None, download_bps:None, duration_ms:started.elapsed().as_millis() as u64, bytes:0, attempts:1, error:Some("cancelled".into()) }, r = client.get(target).send() => r };
    match result {
        Ok(resp) => { let success = resp.status().is_success(); Observation { test_type:kind, success, latency_ms:Some(started.elapsed().as_secs_f64()*1000.0), download_bps:None, duration_ms:started.elapsed().as_millis() as u64, bytes:0, attempts:1, error:(!success).then(|| format!("http status {}", resp.status())) } }
        Err(e) => Observation { test_type:kind, success:false, latency_ms:None, download_bps:None, duration_ms:started.elapsed().as_millis() as u64, bytes:0, attempts:1, error:Some(e.to_string()) }
    }
}

pub async fn latency(host: &str, port: u16, c: &ProbeConfig, cancel: &CancellationToken) -> Observation {
    let attempts = bounded_attempts(c.attempts); let started=std::time::Instant::now(); let mut samples=Vec::new();
    for _ in 0..attempts { let o=tcp(host,port,c,cancel).await; if cancel.is_cancelled(){return Observation{test_type:TestType::Latency,success:false,latency_ms:None,download_bps:None,duration_ms:started.elapsed().as_millis() as u64,bytes:0,attempts:samples.len() as u32,error:Some("cancelled".into())};} if let Some(v)=o.latency_ms{samples.push(v);} }
    samples.sort_by(|a,b|a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)); let median=samples.get(samples.len()/2).copied();
    Observation { test_type:TestType::Latency, success:median.is_some(), latency_ms:median, download_bps:None, duration_ms:started.elapsed().as_millis() as u64, bytes:0, attempts, error:median.is_none().then_some("no successful latency samples".into()) }
}

pub async fn download(c: &ProbeConfig, cancel: &CancellationToken) -> Observation {
    let started=std::time::Instant::now(); let Some(raw)=&c.download_url else { return Observation{test_type:TestType::Download,success:false,latency_ms:None,download_bps:None,duration_ms:0,bytes:0,attempts:1,error:Some("download_url is required".into())}; };
    let Ok(url)=Url::parse(raw) else { return Observation{test_type:TestType::Download,success:false,latency_ms:None,download_bps:None,duration_ms:0,bytes:0,attempts:1,error:Some("invalid download URL".into())}; };
    if let Err(e)=validate_public_target(&url){return Observation{test_type:TestType::Download,success:false,latency_ms:None,download_bps:None,duration_ms:0,bytes:0,attempts:1,error:Some(e)};}
    let client=match Client::builder().timeout(timeout(c)).build(){Ok(v)=>v,Err(e)=>return Observation{test_type:TestType::Download,success:false,latency_ms:None,download_bps:None,duration_ms:0,bytes:0,attempts:1,error:Some(e.to_string())}};
    let response=tokio::select!{_ = cancel.cancelled()=>return Observation{test_type:TestType::Download,success:false,latency_ms:None,download_bps:None,duration_ms:started.elapsed().as_millis() as u64,bytes:0,attempts:1,error:Some("cancelled".into())},r=client.get(url).send()=>r};
    match response { Ok(mut r)=>{if !r.status().is_success(){return Observation{test_type:TestType::Download,success:false,latency_ms:None,download_bps:None,duration_ms:started.elapsed().as_millis() as u64,bytes:0,attempts:1,error:Some(format!("http status {}",r.status()))};} let mut total=0u64; loop{let chunk=tokio::select!{_ = cancel.cancelled()=>return Observation{test_type:TestType::Download,success:false,latency_ms:None,download_bps:None,duration_ms:started.elapsed().as_millis() as u64,bytes:total,attempts:1,error:Some("cancelled".into())},v=r.chunk()=>v}; match chunk{Ok(Some(b))=>{total+=b.len() as u64;if total>100*1024*1024{break;}},Ok(None)=>break,Err(e)=>return Observation{test_type:TestType::Download,success:false,latency_ms:None,download_bps:None,duration_ms:started.elapsed().as_millis() as u64,bytes:total,attempts:1,error:Some(e.to_string())}}} let secs=started.elapsed().as_secs_f64().max(0.001); Observation{test_type:TestType::Download,success:total>0,latency_ms:Some(started.elapsed().as_secs_f64()*1000.0),download_bps:Some(total as f64*8.0/secs),duration_ms:started.elapsed().as_millis() as u64,bytes:total,attempts:1,error:None}}, Err(e)=>Observation{test_type:TestType::Download,success:false,latency_ms:None,download_bps:None,duration_ms:started.elapsed().as_millis() as u64,bytes:0,attempts:1,error:Some(e.to_string())}}
}
