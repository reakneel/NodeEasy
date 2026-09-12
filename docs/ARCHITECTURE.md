# NodeEasy Architecture Freeze v1.2

## 1. Product boundary

NodeEasy is a **node data center**, not a proxy client. The product pipeline is:

`collect -> decode -> detect -> parse -> normalize -> fingerprint -> deduplicate -> persist -> test -> score -> export -> share`

NodeEasy owns node intelligence, measurement, ranking and distribution. It does not own TUN, system proxy routing, traffic forwarding or third-party engine authentication.

## 2. Runtime architecture

NodeEasy is a modular monolith with stable internal boundaries:

- `core`: canonical domain model, validation, fingerprinting and scoring primitives
- `sources`: public/authorized source collection and decoding
- `storage`: SQLite repositories and migrations
- `testing`: bounded, cancellable network probes
- `scoring`: deterministic quality scoring and explanations
- `secrets`: OS-backed credential storage
- `export`: protocol-specific configuration/subscription adapters
- `api`: Axum HTTP + WebSocket API
- `app`: Tauri lifecycle and desktop integration

The module boundaries are intentionally extraction-friendly for future service mode.

## 3. Technology

- Rust stable / Tokio / Axum / Reqwest / Serde / SQLx / tracing
- SQLite first; schema remains PostgreSQL-compatible in principle
- Tauri 2 desktop shell
- Vue 3 + Vite + TypeScript + Tailwind CSS

## 4. Canonical node model

The canonical protocol enum supports Shadowsocks, Shadowsocks2022, VMess, VLESS, Trojan, Hysteria, Hysteria2, TUIC, WireGuard, SOCKS5, HTTP(S) and AnyTLS.

A fingerprint is derived from canonical identity fields rather than display names. Source membership is separate from node identity so the same node may arrive from multiple sources.

Secrets are never part of the canonical `Node` record. Credential material is referenced through the secrets boundary.

## 5. Measurement architecture

All probes implement the same lifecycle concept:

`prepare -> bounded attempt(s) -> observation -> persist -> aggregate -> score`

Probe requirements:

- explicit timeout/deadline
- cancellation support
- bounded concurrency for batches
- no arbitrary redirect into localhost/private networks
- capped response/body size
- controlled measurement targets
- structured errors without secrets

Probe types:

1. TCP reachability
2. TLS handshake
3. HTTP request
4. latency samples
5. download throughput
6. repeated stability

An endpoint probe measures reachability/transport quality; protocol-aware proxy traffic remains an adapter concern.

## 6. Scoring architecture

Score 2.0 consumes raw observations rather than a single test result. Components are independently explainable:

- availability
- latency
- download throughput
- stability
- freshness

The API exposes both the aggregate score and component breakdown so the dashboard can explain rankings.

## 7. Secret boundary and exporters

Credential-bearing configuration is generated only through the secret store. Secrets use an OS-backed credential provider where available and are never written into ordinary node rows or logs.

Exporters are adapters:

- Mihomo/Clash
- sing-box
- V2Ray/URI

They consume the canonical node plus secret material and return generated configuration. Engine-specific structures must not leak back into `core`.

## 8. Source security

Remote source fetching is HTTP(S)-only by default and applies SSRF filtering, size limits and bounded execution. Public/free nodes are untrusted. Source content, node metadata and credentials are treated as hostile input.

## 9. API and realtime model

The frontend consumes the API rather than storage directly. Long-running source sync and batch testing publish lifecycle events through EventBus/WebSocket.

Core API groups are:

- health and nodes
- source ingestion/sync
- jobs and realtime events
- measurement/test history
- score explanations
- secret references
- exports/subscriptions

## 10. Desktop boundary

Tauri starts the local API on loopback and serves the Vite frontend. The desktop shell does not become the domain layer. This keeps the backend independently testable and allows a later service deployment.

## 11. Extensibility policy

New platforms, protocols or proxy engines must be adapters. Scheduler, Worker, storage and dashboard logic consume canonical Node/Test/Score/Event contracts and must not branch on engine-specific types.

## 12. Delivery policy

V3.0 is the local-first foundation. V3.1 completes measurement depth, explainable scoring, secret storage and protocol exporters. V3.2 expands sources/protocols/engine adapters. Service mode is deferred until the desktop workflow is stable.
