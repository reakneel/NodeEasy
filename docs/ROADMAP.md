# NodeEasy Roadmap

## V3.0 — Foundation

**Status: complete.**

- [x] M0 architecture freeze
- [x] M1 Rust workspace, canonical domain, SQLite migrations/repositories
- [x] M2 HTTP(S) source ingestion, decoding, normalization, deduplication and SSRF safeguards
- [x] M3 Axum API, jobs, EventBus and WebSocket
- [x] M4 Vue/Vite dashboard foundation
- [x] M5 bounded TCP probe
- [x] M6 deterministic score foundation
- [x] M7 safe catalog JSON + QR deep links
- [x] M8 Tauri 2 Windows release workflow

## V3.1 — Measurement + Secure Export

**Status: complete.**

- [x] unified cancellable probes: TCP/TLS/HTTP/latency/download/stability
- [x] bounded batch execution and raw measurement/history persistence
- [x] Score 2.0 with explainable component breakdown/history
- [x] OS-backed secret store
- [x] Mihomo/Clash, sing-box and V2Ray/URI exporters
- [x] generated URI/base64 subscriptions
- [x] DNS-aware measurement SSRF guard

## V3.2 — Source and Engine Expansion

**Status: complete.**

### Sources

- [x] pluggable `SourceAdapter` contract
- [x] HTTP subscription adapter retained behind the common contract
- [x] GitHub blob → raw.githubusercontent.com adapter
- [x] raw GitHub public source ingestion with existing SSRF/size controls
- [x] local-file adapter with bounded file size
- [x] manual import API
- [x] source sync persistence and canonical node identity preservation

### Protocol parsing

- [x] Shadowsocks 2022
- [x] Hysteria / Hysteria2 / HY2
- [x] TUIC
- [x] SOCKS5
- [x] HTTP proxy
- [x] AnyTLS
- [x] expanded Clash YAML/JSON mappings
- [x] Base64 and URL-safe Base64 subscription decoding
- [x] protocol parser regression tests

### Engine adapters

- [x] engine-neutral `EngineAdapter` contract
- [x] Mihomo config adapter
- [x] sing-box config adapter
- [x] Xray config adapter for VMess/VLESS/Trojan
- [x] bounded binary availability checks
- [x] bounded engine process execution helper
- [x] engine-specific types isolated from `nodeeasy-core`

## V4 — Service Mode

Only after the local-first desktop workflow is stable:

- [ ] PostgreSQL deployment path
- [ ] authenticated multi-user API
- [ ] controlled subscription sharing
- [ ] background workers
- [ ] observability/metrics
- [ ] deployment/container profile

## Release gates

Every release must pass Rust check/clippy/test, frontend typecheck/build, source/parser regression tests, security regression tests and documentation/API consistency. Windows packaging must succeed for supported targets before a desktop release is called complete.
