use chrono::{DateTime, Utc};
use nodeeasy_core::{Node, NodeEndpoint, NodeStatus, Protocol, Source};
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn upsert_node(pool: &SqlitePool, node: &Node) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO nodes (id,fingerprint,name,protocol,host,port,country,city,asn,status,score,first_seen_at,last_seen_at,last_tested_at,created_at,updated_at) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?) ON CONFLICT(fingerprint) DO UPDATE SET name=excluded.name,protocol=excluded.protocol,host=excluded.host,port=excluded.port,country=excluded.country,city=excluded.city,asn=excluded.asn,last_seen_at=excluded.last_seen_at,updated_at=excluded.updated_at")
        .bind(node.id.to_string()).bind(&node.fingerprint).bind(&node.name).bind(format_protocol(node.protocol)).bind(&node.endpoint.host).bind(node.endpoint.port as i64).bind(&node.country).bind(&node.city).bind(&node.asn).bind(format_status(node.status)).bind(node.score).bind(node.first_seen_at).bind(node.last_seen_at).bind(node.last_tested_at).bind(node.first_seen_at).bind(node.last_seen_at).execute(pool).await?;
    Ok(())
}

pub async fn list_nodes(pool: &SqlitePool, limit: i64) -> Result<Vec<Node>, sqlx::Error> {
    let rows=sqlx::query_as::<_,NodeRow>("SELECT id,fingerprint,name,protocol,host,port,country,city,asn,status,score,first_seen_at,last_seen_at,last_tested_at FROM nodes ORDER BY score DESC,last_seen_at DESC LIMIT ?").bind(limit.clamp(1,1000)).fetch_all(pool).await?;
    rows.into_iter().map(TryInto::try_into).collect()
}

pub async fn upsert_source(pool: &SqlitePool, source: &Source) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO sources (id,name,kind,url,enabled,interval_seconds,last_fetch_at,last_success_at,last_error,created_at,updated_at) VALUES (?,?,?,?,?,?,?,?,?,?,?) ON CONFLICT(id) DO UPDATE SET name=excluded.name,kind=excluded.kind,url=excluded.url,enabled=excluded.enabled,interval_seconds=excluded.interval_seconds,updated_at=excluded.updated_at")
        .bind(source.id.to_string()).bind(&source.name).bind(&source.kind).bind(&source.url).bind(source.enabled).bind(source.interval_seconds as i64).bind(source.last_fetch_at).bind(source.last_success_at).bind(&source.last_error).bind(Utc::now()).bind(Utc::now()).execute(pool).await?;
    Ok(())
}

#[derive(sqlx::FromRow)]
struct NodeRow{id:String,fingerprint:String,name:Option<String>,protocol:String,host:String,port:i64,country:Option<String>,city:Option<String>,asn:Option<String>,status:String,score:f64,first_seen_at:DateTime<Utc>,last_seen_at:DateTime<Utc>,last_tested_at:Option<DateTime<Utc>>}
impl TryFrom<NodeRow> for Node { type Error=sqlx::Error; fn try_from(r:NodeRow)->Result<Self,Self::Error>{ let id=Uuid::parse_str(&r.id).map_err(|e|sqlx::Error::Decode(Box::new(e)))?; let protocol=parse_protocol(&r.protocol).ok_or_else(||sqlx::Error::Protocol("invalid protocol".into()))?; let status=parse_status(&r.status).ok_or_else(||sqlx::Error::Protocol("invalid status".into()))?; Ok(Node{id,fingerprint:r.fingerprint,name:r.name,protocol,endpoint:NodeEndpoint{host:r.host,port:r.port as u16},country:r.country,city:r.city,asn:r.asn,status,score:r.score,first_seen_at:r.first_seen_at,last_seen_at:r.last_seen_at,last_tested_at:r.last_tested_at}) }}
fn format_protocol(p:Protocol)->&'static str{match p{Protocol::Shadowsocks=>"shadowsocks",Protocol::Shadowsocks2022=>"shadowsocks2022",Protocol::Vmess=>"vmess",Protocol::Vless=>"vless",Protocol::Trojan=>"trojan",Protocol::Hysteria=>"hysteria",Protocol::Hysteria2=>"hysteria2",Protocol::Tuic=>"tuic",Protocol::Wireguard=>"wireguard",Protocol::Socks5=>"socks5",Protocol::Http=>"http",Protocol::AnyTls=>"anytls"}}
fn parse_protocol(s:&str)->Option<Protocol>{Some(match s{"shadowsocks"=>Protocol::Shadowsocks,"shadowsocks2022"=>Protocol::Shadowsocks2022,"vmess"=>Protocol::Vmess,"vless"=>Protocol::Vless,"trojan"=>Protocol::Trojan,"hysteria"=>Protocol::Hysteria,"hysteria2"=>Protocol::Hysteria2,"tuic"=>Protocol::Tuic,"wireguard"=>Protocol::Wireguard,"socks5"=>Protocol::Socks5,"http"=>Protocol::Http,"anytls"=>Protocol::AnyTls,_=>return None})}
fn format_status(s:NodeStatus)->&'static str{match s{NodeStatus::New=>"new",NodeStatus::Healthy=>"healthy",NodeStatus::Degraded=>"degraded",NodeStatus::Unavailable=>"unavailable",NodeStatus::Stale=>"stale"}}
fn parse_status(s:&str)->Option<NodeStatus>{Some(match s{"new"=>NodeStatus::New,"healthy"=>NodeStatus::Healthy,"degraded"=>NodeStatus::Degraded,"unavailable"=>NodeStatus::Unavailable,"stale"=>NodeStatus::Stale,_=>return None})}
