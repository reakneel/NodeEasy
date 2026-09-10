use nodeeasy_core::Node;
use serde::Serialize;

#[derive(Serialize)]
pub struct ExportNode<'a> { pub id:String, pub name:Option<&'a String>, pub protocol:String, pub host:&'a str, pub port:u16, pub score:f64, pub status:String }
pub fn json(nodes:&[Node])->String { serde_json::to_string_pretty(&nodes.iter().map(|n| ExportNode{id:n.id.to_string(),name:n.name.as_ref(),protocol:format!("{:?}",n.protocol).to_lowercase(),host:&n.endpoint.host,port:n.endpoint.port,score:n.score,status:format!("{:?}",n.status).to_lowercase()}).collect::<Vec<_>>()).unwrap_or_else(|_|"[]".into()) }
pub fn share_target(node:&Node)->String { format!("nodeeasy://node/{}", node.id) }
pub fn qr_ascii(node:&Node)->Result<String,Box<dyn std::error::Error+Send+Sync>> { let code=qrcode::QrCode::new(share_target(node).as_bytes())?; Ok(code.render::<qrcode::render::unicode::Dense1x2>().quiet_zone(false).build()) }
