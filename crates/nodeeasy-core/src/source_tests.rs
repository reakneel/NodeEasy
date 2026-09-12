use super::source::{detect_format,parse_subscription,InputFormat};
use crate::Protocol;

#[test]
fn detects_uri_subscription(){assert_eq!(detect_format("vless://uuid@example.com:443#demo"),InputFormat::Uri);}
#[test]
fn parses_additional_uri_protocols(){let body="ss2022://user:secret@example.com:443#ss2022\nhysteria2://token@example.com:443#hy2\nsocks5://user:pass@example.com:1080#socks\nhttp://user:pass@example.com:8080#http";let nodes=parse_subscription(body).unwrap();assert_eq!(nodes.len(),4);assert_eq!(nodes[0].node.protocol,Protocol::Shadowsocks2022);assert_eq!(nodes[1].node.protocol,Protocol::Hysteria2);assert_eq!(nodes[2].node.protocol,Protocol::Socks5);assert_eq!(nodes[3].node.protocol,Protocol::Http);}
#[test]
fn parses_clash_expanded_protocols(){let body="proxies:\n  - name: hy2\n    type: hysteria2\n    server: example.com\n    port: 443\n  - name: ss2022\n    type: ss2022\n    server: example.net\n    port: 8443\n  - name: anytls\n    type: anytls\n    server: example.org\n    port: 443\n";let nodes=parse_subscription(body).unwrap();assert_eq!(nodes.len(),3);assert_eq!(nodes[0].node.protocol,Protocol::Hysteria2);assert_eq!(nodes[1].node.protocol,Protocol::Shadowsocks2022);assert_eq!(nodes[2].node.protocol,Protocol::AnyTls);}
#[test]
fn decodes_base64_subscription(){use base64::{engine::general_purpose::STANDARD,Engine};let body=STANDARD.encode("trojan://pass@example.com:443#demo");let nodes=parse_subscription(&body).unwrap();assert_eq!(nodes[0].node.protocol,Protocol::Trojan);}
