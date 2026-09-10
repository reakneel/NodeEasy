# NodeEasy

NodeEasy V3 is a local-first node data center: **collect → normalize → deduplicate → test → score → export/share**.

## Stack

- Rust: Tokio + Axum + SQLx + SQLite
- Desktop: Tauri 2
- Frontend: Vue 3 + Vite + TypeScript + Tailwind CSS
- Realtime: WebSocket event bus
- API-first modular monolith

## Milestones

- [x] M1 — domain model, SQLite schema, migrations, persistence
- [x] M2 — source engine, subscription decoding/parsing, deduplication, SSRF safeguards
- [x] M3 — API, jobs, event bus, WebSocket
- [x] M4 — dashboard UI foundation
- [x] M5 — bounded TCP connectivity testing
- [x] M6 — deterministic 0–100 scoring model
- [x] M7 — node catalog export and QR share target
- [x] M8 — Tauri 2 Windows packaging/release workflow

## Local development

```bash
cargo run -p nodeeasy-app
cd frontend && npm install && npm run dev
```

API defaults to `http://127.0.0.1:3000`; Vite proxies `/api` to it.

For the desktop build:

```bash
cd src-tauri
cargo tauri build
```

## API

- `GET /api/v1/health`
- `GET /api/v1/nodes`
- `POST /api/v1/nodes/{id}/test`
- `GET|POST /api/v1/sources`
- `POST /api/v1/sources/{id}/sync`
- `POST /api/v1/jobs`
- `GET /api/v1/export/nodes.json`
- `GET /api/v1/nodes/{id}/qr`
- `GET /api/v1/ws`

## Security baseline

Source fetching only permits HTTP(S), applies an explicit size limit, and blocks private/loopback/link-local destinations by default. Public/free nodes are untrusted and should never be used for sensitive traffic.

Proxy engines such as Mihomo, sing-box and Xray remain adapters for the next iteration; the core does not execute arbitrary proxy configurations.
