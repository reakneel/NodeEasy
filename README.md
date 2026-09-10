# NodeEasy

NodeEasy is the next-generation node collection, normalization, testing, scoring, subscription export, and sharing platform.

## Architecture

- Rust backend: Tokio + Axum + SQLx + SQLite
- Desktop shell: Tauri 2
- Frontend: Vue 3 + Vite + TypeScript + Tailwind CSS
- UI: shadcn-vue + Lucide + ECharts
- API-first modular monolith
- Proxy engines are optional adapters; they are not part of the core

See `docs/ARCHITECTURE.md` for the architecture freeze.
