# NodeEasy Roadmap

## V3.0 — Foundation Release

Status: **complete**.

NodeEasy V3.0 is the local-first node data center foundation:

`collect -> decode -> parse -> normalize -> fingerprint -> deduplicate -> persist -> test -> score -> export -> share`

### Completed milestones

- [x] M0 — Architecture freeze and product boundary
- [x] M1 — Rust workspace, domain model, SQLite schema, migrations and repositories
- [x] M2 — HTTP source ingestion, decoding, parsing, normalization, fingerprinting, deduplication and SSRF safeguards
- [x] M3 — Axum API, jobs, EventBus and WebSocket events
- [x] M4 — Vue 3 + Vite dashboard foundation
- [x] M5 — bounded TCP connectivity probe with timeout and persistence
- [x] M6 — deterministic 0–100 score model
- [x] M7 — safe node catalog JSON export and QR deep-link sharing
- [x] M8 — Tauri 2 desktop shell and Windows release workflow

### V3.0 scope notes

M5 currently provides the first TCP probe. TLS, HTTP, download and repeated stability probes remain separate capabilities for the next iteration.

M7 intentionally does **not** export credential-bearing Mihomo/Clash/sing-box configurations yet. The current export surface is a safe node catalog plus `nodeeasy://` QR targets. A protected secret store should exist before credentials, private keys or generated subscriptions are persisted or exposed.

## V3.1 — Measurement and Export

Priority: **high**.

- [ ] TLS handshake probe
- [ ] HTTP probe
- [ ] latency measurement with repeated observations
- [ ] download throughput probe
- [ ] stability/reconnect probe
- [ ] cancellation and bounded batch testing
- [ ] richer test history and status history APIs
- [ ] explainable score breakdown in the dashboard
- [ ] secure credential/secret store
- [ ] Mihomo/Clash exporter
- [ ] sing-box exporter
- [ ] V2Ray/URI exporters
- [ ] generated subscription views

## V3.2 — Platform and Source Expansion

- [ ] pluggable source adapters
- [ ] GitHub/raw public source adapter
- [ ] local-file source adapter
- [ ] manual import API
- [ ] additional protocol parsers
- [ ] optional Mihomo adapter
- [ ] optional sing-box adapter
- [ ] optional Xray adapter

The core domain remains proxy-engine agnostic. Platform/engine adapters must not leak engine-specific types into the canonical node model.

## V4 — Service Mode

Only after the local-first desktop workflow is stable:

- [ ] PostgreSQL-compatible persistence path
- [ ] authenticated multi-user API
- [ ] subscription sharing/access controls
- [ ] background worker mode
- [ ] observability and metrics
- [ ] deployment/container profile

## Release acceptance

A release is considered complete only when:

1. Rust workspace CI passes.
2. Frontend typecheck/build passes.
3. Tauri Windows packaging succeeds for supported targets.
4. Security baseline remains enabled.
5. Documentation matches the implemented API and feature set.
6. No credential-bearing export is enabled without a deliberate secret-management design.
