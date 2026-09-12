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

V3.0 deliberately did not persist or export credentials and did not pretend TCP reachability was a complete proxy-quality measurement.

## V3.1 — Measurement + Secure Export

**Status: implementation target.**

### Measurement

- [ ] unified Probe interface
- [ ] TCP probe refactor
- [ ] TLS handshake probe
- [ ] HTTP probe
- [ ] repeated latency sampling
- [ ] controlled download throughput
- [ ] repeated stability/reconnect checks
- [ ] cancellation and bounded batch execution
- [ ] raw observation/history persistence
- [ ] status-history aggregation

### Scoring and dashboard

- [ ] Score 2.0 component model
- [ ] explainable score breakdown API
- [ ] measurement history API
- [ ] realtime batch progress
- [ ] dashboard gauges/time-series/ranking views

### Security and distribution

- [ ] OS-backed secret store
- [ ] secret references separate from Node
- [ ] Mihomo/Clash exporter
- [ ] sing-box exporter
- [ ] V2Ray/URI exporter
- [ ] generated subscription views
- [ ] export tests that prove secrets are not logged or persisted in node rows

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
