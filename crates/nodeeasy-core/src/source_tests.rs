#[cfg(test)]
mod tests {
    use super::super::source::*;
    use base64::{engine::general_purpose::STANDARD, Engine};

    #[test]
    fn parses_vless_uri() {
        let nodes=parse_subscription("vless://user@example.com:443?security=tls#demo").unwrap();
        assert_eq!(nodes.len(),1);
        assert_eq!(nodes[0].node.protocol,crate::Protocol::Vless);
        assert_eq!(nodes[0].node.endpoint.host,"example.com");
    }

    #[test]
    fn parses_base64_subscription() {
        let encoded=STANDARD.encode("trojan://secret@example.com:443#edge\n");
        let nodes=parse_subscription(&encoded).unwrap();
        assert_eq!(nodes.len(),1);
        assert_eq!(nodes[0].node.protocol,crate::Protocol::Trojan);
    }

    #[test]
    fn parses_clash_yaml() {
        let yaml="proxies:\n  - name: edge\n    type: trojan\n    server: example.com\n    port: 443\n";
        let nodes=parse_subscription(yaml).unwrap();
        assert_eq!(nodes.len(),1);
        assert_eq!(nodes[0].node.name.as_deref(),Some("edge"));
    }

    #[test]
    fn deduplicates_by_fingerprint() {
        let input="trojan://secret@example.com:443#a\ntrojan://secret@example.com:443#b";
        let nodes=parse_subscription(input).unwrap();
        assert_eq!(deduplicate(nodes).len(),1);
    }
}
