# NodeEasy Architecture Freeze v1.1

## 1. Product boundary

NodeEasy is a **node data platform**, not a proxy client. Its core responsibilities are:

`collect -> decode -> detect -> parse -> normalize -> fingerprint -> deduplicate -> persist -> test -> score -> export -> share`

The core does not own TUN, system proxy routing, traffic forwarding, or third-party proxy-engine authentication.

## 2. Runtime architecture

NodeEasy is a modular monolith. The desktop application contains independently bounded modules:

- `core`: domain models, errors, fingerprints and scoring primitives
- `sources`: public/authorized source collection and decoding
- `storage`: SQLite repositories and migrations
- `testing`: bounded network probes
- `scoring`: deterministic quality scoring
- `export`: safe catalog export and QR sharing in V3.0
- `api`: Axum HTTP + WebSocket API
- `app`: Tauri application lifecycle

Future services can be extracted by module boundary without changing the canonical domain model.

## 3. Technology

### Backend

- Rust stable
- Tokio
- Axum
- Reqwest
- Serde
- SQLx
- SQLite
- tracing

### Frontend

- Vue 3
- Vite
- TypeScript
- Tailwind CSS
- shadcn-vue direction
- Lucide direction
- ECharts direction

### Desktop

- Tauri 2

## 4. Canonical node model

Supported protocols initially include:

- Shadowsocks / Shadowsocks2022
- VMess
- VLESS
- Trojan
- Hysteria / Hysteria2
- TUIC
- WireGuard
- SOCKS5
- HTTP(S)
- AnyTLS

A node fingerprint is derived from canonical identity fields, not display names. Source relationships are stored separately so the same node can belong to multiple sources.

## 5. Data model

Core tables include:

- `sources`
- `source_runs`
- `nodes`
- `node_sources`
- `node_tests`
- `jobs`

SQLite is the first storage engine. SQLx migrations and repository boundaries keep a future PostgreSQL migration feasible.

## 6. Source engine

The source pipeline is intentionally staged:

`fetch -> size/SSRF validation -> decode -> detect -> parse -> normalize -> fingerprint -> deduplicate -> persist`

The initial remote source is HTTP(S). Only public or explicitly authorized sources are supported. Source fetching must protect against SSRF, enforce size limits, and avoid logging credentials, subscription tokens or private keys.

## 7. Testing model

V3.0 implements the first bounded TCP connectivity probe. The model is intentionally extensible toward:

1. TCP reachability
2. TLS handshake
3. HTTP request
4. latency
5. download throughput
6. repeated stability

A successful connectivity check is not equivalent to a stable or high-quality node.

## 8. API contract

V3.0 exposes:

- `GET /api/v1/health`
- `GET /api/v1/nodes`
- `POST /api/v1/nodes/{id}/test`
- `GET|POST /api/v1/sources`
- `POST /api/v1/sources/{id}/sync`
- `POST /api/v1/jobs`
- `GET /api/v1/export/nodes.json`
- `GET /api/v1/nodes/{id}/qr`
- `GET /api/v1/ws`

The frontend consumes this API rather than accessing storage directly.

## 9. Export and secret boundary

V3.0 deliberately avoids credential-bearing generated proxy configurations. Node catalog JSON and QR share targets do not expose private keys or subscription credentials.

Mihomo/Clash, sing-box and V2Ray/URI exporters belong to V3.1 and must be implemented behind a deliberate secret-management boundary. Secrets should not be stored in ordinary node records or emitted into logs.

Proxy engines remain optional adapters and must not become dependencies of the canonical core model.

## 10. UI direction

The dashboard replaces the old WinForms table as the primary experience:

- overview cards for node count, healthy count and quality metrics
- node table for dense operational data
- realtime WebSocket event updates
- source/test/export operations exposed through the API

More advanced gauges, time-series charts and operational pages are planned as the measurement system matures.

## 11. Security baseline

- HTTP/HTTPS only for remote source fetching by default
- block localhost, loopback, private, link-local and other internal destinations unless explicitly enabled
- enforce source size/rate limits
- do not expose engine controllers by default
- never persist or log secrets unnecessarily
- treat public/free nodes as untrusted and unsuitable for sensitive traffic
- isolate optional proxy engines from the core API

## 12. Delivery policy

V3.0 is the **architecture and local-first foundation release**. Later milestones may extend interfaces, but must not silently change the product boundary or canonical model.

The next major work should add measurement depth and secure export before introducing multi-user/service complexity.
