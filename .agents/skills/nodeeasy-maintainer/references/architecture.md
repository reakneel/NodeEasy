# NodeEasy Architecture Reference

## Product
NodeEasy is a local-first node data center for collecting, normalizing, testing, scoring, exporting, and sharing proxy-node data.

## Layers
| Layer | Responsibility | Rule |
|---|---|---|
| Tauri 2 | Desktop lifecycle/local integration | Keep platform details out of domain |
| Vue 3 frontend | Dashboard and user interaction | Typed APIs; no secret persistence |
| Axum API | HTTP/WebSocket boundary | Validate DTOs; delegate business logic |
| Application services | Source sync, jobs, probes, engine adapters | Bounded work and cancellation |
| Core domain | Canonical nodes, fingerprints, scoring | Independent of source/engine formats |
| Repositories | Persistence | SQLite first; preserve PostgreSQL path |
| Adapters | Source and engine formats | Isolate external formats/binaries |

## Source pipeline
`collect → decode → detect → parse → normalize → fingerprint → deduplicate → persist`

Each stage needs explicit errors and resource limits. Normalize to canonical domain types before persistence. Source-to-node relations must use the canonical persisted node ID returned by upsert.

## Measurement and score
Measurements: TCP, TLS, HTTP, latency, download, stability. Keep raw measurements separate from scoring. Scoring must be deterministic, versioned, and explicit about missing data. A direct probe must not be represented as proxy-routed.

Historical Score 2.0 weights: availability 30, latency 20, download 25, stability 20, freshness 5. Verify live code before relying on these.

## Engine boundary
`Canonical Node + optional secret → Engine-specific config`

Mihomo, sing-box, and Xray are optional external engines. Config generation does not imply binary management, long-running process supervision, system proxy changes, or probe traffic routing. Report each capability separately.

## Persistence and security
SQLite is the local-first store; SQL stays behind repositories. Prefer OS credential storage. Ordinary DB rows should contain references/metadata rather than plaintext secrets. Treat public node data and source content as untrusted.
