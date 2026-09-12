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

### Measurement

- [x] unified cancellable Probe interface
- [x] TCP probe refactor
- [x] TLS HTTPS handshake probe
- [x] HTTP probe
- [x] repeated latency sampling
- [x] controlled download throughput with HTTPS target validation and body cap
- [x] repeated stability observations
- [x] cancellation and bounded batch execution
- [x] raw test/history persistence
- [x] score history persistence

### Scoring and dashboard

- [x] Score 2.0 component model
- [x] explainable score breakdown API
- [x] measurement history API
- [x] realtime batch progress events
- [x] dashboard score breakdown and history panel

### Security and distribution

- [x] OS-backed secret store
- [x] secrets separated from Node records
- [x] Mihomo/Clash exporter
- [x] sing-box exporter
- [x] V2Ray/URI exporter
- [x] generated URI/base64 subscription views
- [x] DNS-aware measurement SSRF guard

## V3.2 — Source and Engine Expansion

- [ ] pluggable source adapters
- [ ] GitHub/raw public source adapter
- [ ] local-file source adapter
- [ ] manual import API
- [ ] additional protocol parsers
- [ ] optional Mihomo adapter
- [ ] optional sing-box adapter
- [ ] optional Xray adapter

Adapters must not leak engine-specific types into the canonical core model.

## V4 — Service Mode

Only after the local-first desktop workflow is stable:

- [ ] PostgreSQL deployment path
- [ ] authenticated multi-user API
- [ ] controlled subscription sharing
- [ ] background workers
- [ ] observability/metrics
- [ ] deployment/container profile

## Release gates

Every release must pass Rust check/clippy/test, frontend typecheck/build, security regression tests and documentation/API consistency. Windows packaging must succeed for supported targets before a desktop release is called complete.
