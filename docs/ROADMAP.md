# NodeEasy Roadmap

## M0 — Architecture Freeze

Status: complete.

Deliverables:
- product boundary
- Rust/Tauri/Vue technology baseline
- canonical node model
- source/test/export boundaries
- API contract
- SQLite schema direction
- security baseline
- release milestone sequence

## M1 — Foundation

Create a Cargo workspace and Tauri/Vue application skeleton. Add domain types, error model, configuration, SQLx migrations, repositories, tracing, and CI.

Acceptance:
- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all`
- frontend typecheck/build

## M2 — Source Engine

Implement source adapters, decoding, format detection, protocol parsers, canonicalization, fingerprinting and deduplication.

## M3 — API

Expose Axum REST APIs and WebSocket events. Long-running work uses jobs and bounded concurrency rather than blocking HTTP handlers.

## M4 — Dashboard

Implement the dashboard-first Tauri UI with Tailwind, shadcn-vue, Lucide and ECharts.

## M5 — Testing

Implement TCP/TLS/HTTP/latency/download/stability probes with cancellation, timeout and concurrency controls.

## M6 — Scoring

Add deterministic scoring and explainable score components. Persist raw observations so scoring can evolve without losing test history.

## M7 — Export and sharing

Add Mihomo/Clash, sing-box, V2Ray/URI exporters, generated subscriptions and QR/share-link views.

## M8 — Release

GitHub Actions builds the Tauri application on version tags and publishes Windows x64/ARM64 release artifacts.
