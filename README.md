# NodeEasy

NodeEasy V3 is a local-first node data center: **collect → normalize → deduplicate → measure → score → export/share**.

## Stack

- Rust: Tokio + Axum + SQLx + SQLite
- Desktop: Tauri 2
- Frontend: Vue 3 + Vite + TypeScript + Tailwind CSS
- Realtime: WebSocket event bus
- API-first modular monolith
- OS-backed secret storage for credential-bearing exports

## Release path

- [x] V3.0 — foundation, source engine, API, dashboard, TCP probe, scoring, safe catalog export, Tauri packaging
- [x] V3.1 — measurement framework, Score 2.0, secure secret boundary, Mihomo/Clash, sing-box, V2Ray/URI export, subscriptions and dashboard inspection
- [x] V3.2 — pluggable sources, GitHub/raw, local/manual import, expanded parsers and Mihomo/sing-box/Xray adapters
- [ ] V4 — service mode / multi-user deployment

## Source API

Create a source with `kind` set to one of:

- `http_subscription` — public HTTP(S) subscription
- `github_raw` — GitHub blob URL or `raw.githubusercontent.com` URL
- `local_file` — local path available to the desktop process

Then call `POST /api/v1/sources/{id}/sync`.

Manual import is available at `POST /api/v1/sources/import` with JSON `{ "name": "manual", "body": "...", "kind": "manual" }`.

All remote source fetching retains the V3.1 SSRF/DNS/body-size safeguards.

## Supported input formats

- URI lines: Shadowsocks, Shadowsocks 2022, VMess, VLESS, Trojan, Hysteria, Hysteria2/HY2, TUIC, SOCKS5, HTTP(S), AnyTLS
- Base64 / URL-safe Base64 subscriptions
- Clash/Mihomo YAML
- Clash/Mihomo JSON

Unsupported entries are skipped while a source is parsed; a source fails only when no supported node remains.

## Measurement API

- `POST /api/v1/nodes/{id}/test` — bounded TCP probe
- `POST /api/v1/nodes/test-batch` — bounded concurrent multi-probe run
- `POST /api/v1/nodes/{id}/score` — recompute and persist Score 2.0
- `GET /api/v1/nodes/{id}/history` — raw test history
- `GET /api/v1/nodes/{id}/score-history` — score history

## Engine adapter API

`POST /api/v1/engines/config` accepts `{ "engine": "mihomo|sing-box|xray", "node_id": "..." }` and returns an engine-specific config envelope without changing the canonical node model.

The Rust `EngineAdapter` contract also provides bounded binary availability checks. Actual Mihomo, sing-box or Xray binaries remain optional runtime dependencies; NodeEasy does not silently install or launch a system-wide proxy.

## Secure export API

- `PUT|DELETE /api/v1/nodes/{id}/secret` — OS-backed credential storage
- `GET /api/v1/nodes/{id}/export/mihomo`
- `GET /api/v1/nodes/{id}/export/clash`
- `GET /api/v1/nodes/{id}/export/sing-box`
- `GET /api/v1/nodes/{id}/export/v2ray`
- `GET /api/v1/subscriptions/base64`
- `GET /api/v1/subscriptions/uri`

Secret material is generic JSON options and is never returned by the API. Credentials are kept outside ordinary node rows.

## Core API

- `GET /api/v1/health`
- `GET /api/v1/nodes`
- `GET|POST /api/v1/sources`
- `POST /api/v1/sources/{id}/sync`
- `POST /api/v1/sources/import`
- `POST /api/v1/engines/config`
- `POST /api/v1/jobs`
- `GET /api/v1/export/nodes.json`
- `GET /api/v1/nodes/{id}/qr`
- `GET /api/v1/ws`

## Security baseline

Source fetching and measurement targets are restricted to HTTP(S), private/loopback/link-local targets are blocked by default, execution is bounded, download bodies are capped, redirects are constrained, and credentials are kept outside normal node rows and logs. Public/free nodes remain untrusted and should never be used for sensitive traffic.

Protocol-aware proxy engines such as Mihomo, sing-box and Xray remain adapters. The canonical core model does not depend on an engine implementation.

## Release gate

Before merging V3.2, CI must pass Rust format/check/clippy/test plus frontend typecheck/build. Parser regression tests cover expanded protocols and Base64 decoding. Windows packaging remains the desktop release gate.
