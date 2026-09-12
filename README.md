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
- [x] V3.1 backend — unified measurement probes, bounded batch execution, history, Score 2.0 breakdown, secret store, Mihomo/Clash, sing-box and V2Ray/URI export
- [ ] V3.1 frontend polish — gauges, history charts and ranking interactions
- [ ] V3.2 — pluggable source/engine adapters
- [ ] V4 — service mode / multi-user deployment

## Local development

```bash
cargo run -p nodeeasy-app
cd frontend && npm install && npm run dev
```

API defaults to `http://127.0.0.1:3000`; Vite proxies `/api` to it.

## Measurement API

- `POST /api/v1/nodes/{id}/test` — bounded TCP probe
- `POST /api/v1/nodes/test-batch` — bounded concurrent multi-probe run
- `POST /api/v1/nodes/{id}/score` — recompute and persist Score 2.0
- `GET /api/v1/nodes/{id}/history` — raw test history
- `GET /api/v1/nodes/{id}/score-history` — score history

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
- `POST /api/v1/jobs`
- `GET /api/v1/export/nodes.json`
- `GET /api/v1/nodes/{id}/qr`
- `GET /api/v1/ws`

## Security baseline

Source fetching and measurement targets are restricted to HTTP(S), private/loopback/link-local targets are blocked by default, execution is bounded, download bodies are capped, redirects are constrained, and credentials are kept outside normal node rows and logs. Public/free nodes remain untrusted and should never be used for sensitive traffic.

Protocol-aware proxy engines such as Mihomo, sing-box and Xray remain adapters. The canonical core model does not depend on an engine implementation.
