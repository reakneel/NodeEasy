# NodeEasy Architecture Freeze v1.0

## 1. Product boundary

NodeEasy is a node data platform, not a proxy client. Its core responsibilities are:

`collect -> decode -> detect -> parse -> normalize -> fingerprint -> deduplicate -> persist -> test -> score -> export -> share`

The core does not own TUN, system proxy, traffic routing, or user authentication for third-party proxy engines.

## 2. Runtime architecture

NodeEasy is a modular monolith. One desktop application contains independently bounded modules:

- `core`: domain models, errors, configuration, fingerprints
- `sources`: public/authorized source collectors and decoders
- `storage`: SQLite repositories and migrations
- `testing`: TCP/TLS/HTTP/latency/download/stability probes
- `scoring`: availability, latency, speed, stability, freshness scoring
- `subscriptions`: generated subscription views
- `export`: Clash/Mihomo, sing-box, V2Ray/URI and share-link exporters
- `api`: Axum HTTP + WebSocket API
- `engines`: optional Mihomo/sing-box/Xray adapters
- `app`: Tauri application lifecycle

Future services can be extracted by module boundary without changing the domain model.

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
- shadcn-vue
- Lucide
- ECharts

### Desktop

- Tauri 2

## 4. Canonical node model

Supported protocols initially:

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

Core tables:

- `sources`
- `source_runs`
- `nodes`
- `node_sources`
- `node_tests`
- `node_status_history`
- `subscriptions`
- `subscription_items`
- `jobs`

SQLite is the first storage engine. SQLx migrations and repository boundaries keep a future PostgreSQL migration feasible.

## 6. Source engine

`NodeSource` is the extension point. Initial source types:

- HTTP subscription
- GitHub/raw public source
- local file
- manual input
- API source later

A source run records fetched, parsed, added, updated, removed and failed counts.

Only public or explicitly authorized sources are supported. Fetching must protect against SSRF and must not log credentials, subscription tokens or private keys.

## 7. Testing model

Testing is intentionally split into independent probes:

1. TCP reachability
2. TLS handshake
3. HTTP request
4. latency
5. download throughput
6. repeated stability

A single successful speed test does not equal a stable node. Stability is calculated from repeated observations and status history.

## 8. API contract

Initial API surface:

- `GET /api/v1/dashboard`
- `GET/POST/DELETE /api/v1/nodes`
- `GET/POST/PATCH/DELETE /api/v1/sources`
- `POST /api/v1/sources/{id}/sync`
- `POST /api/v1/tests/nodes/{id}`
- `POST /api/v1/tests/batch`
- `GET /api/v1/jobs`
- `GET /api/v1/jobs/{id}`
- `GET /api/v1/export/{format}`
- `GET/POST/PATCH /api/v1/subscriptions`
- `GET /api/v1/subscriptions/{id}/content`
- `GET /api/v1/ws`

The frontend consumes this API rather than accessing storage directly.

## 9. UI direction

Dashboard-first design replaces the old WinForms table as the primary experience.

- overview cards for node count, healthy count, average latency and throughput
- gauge/ring visualizations for health and quality
- time-series charts for availability, latency and speed
- node table remains for detailed operations
- source, test, subscription and settings pages remain accessible

The UI is optimized for at-a-glance monitoring while retaining dense operational data.

## 10. Security baseline

- HTTP/HTTPS only for remote source fetching by default
- block localhost, loopback, private, link-local and other internal destinations unless explicitly enabled
- do not expose engine controllers by default
- never persist or log secrets unnecessarily
- rate-limit source synchronization
- treat public/free nodes as untrusted and unsuitable for sensitive traffic
- isolate optional proxy engines from the core API

## 11. Delivery milestones

- M0 Architecture Freeze
- M1 Rust workspace + SQLite + domain models
- M2 Source Engine
- M3 Axum API + WebSocket
- M4 Tauri + dashboard
- M5 Test Engine
- M6 Scoring
- M7 Export + QR + subscriptions
- M8 Release

M0 is frozen by this document. Later milestones may extend interfaces, but must not silently change the product boundary or canonical model.
