---
name: nodeeasy-maintainer
description: Maintain, extend, test, and release the NodeEasy Rust-first node data platform. Use for NodeEasy Rust, Tauri, Vue, source ingestion, parsers, probes, scoring, engine adapters, security, CI, GitHub PRs, and roadmap work.
---

# NodeEasy Maintainer Skill

## Mission
Maintain https://github.com/reakneel/NodeEasy, a local-first node data center for 节点采集、标准化、测试、评分、订阅生成与分享. This is an execution skill: inspect the live repository, implement requested changes, validate them, and complete the requested GitHub workflow. Do not stop at a proposal when tools allow implementation.

## Activation
Use for NodeEasy feature work, bugs, refactoring, architecture, roadmap, Rust workspace, Tauri 2, Vue 3, source ingestion, protocol parsing, node normalization, probes, scoring, Mihomo/sing-box/Xray adapters, CI, PRs, packaging, and releases. Do not use for unrelated projects.

## Non-negotiable rules
1. Inspect the current repository before editing; historical notes here are not authority.
2. Live code, default branch, and verified GitHub state override this document.
3. Never claim tests, CI, merge, tag, or release succeeded without current evidence.
4. Do not commit directly to main unless explicitly requested.
5. Do not merge, release, delete branches, or close unrelated PRs without authorization.
6. Keep diffs focused and reviewable.
7. Treat imported nodes, URLs, configs, and remote responses as untrusted.
8. Never log credentials, tokens, private keys, full secret-bearing node URIs, or generated configs containing secrets.
9. Distinguish config generation, binary detection, one-shot execution, long-lived lifecycle, and engine-routed traffic.
10. Respond in Chinese unless asked otherwise; state limitations precisely.

## Architecture baseline
- Rust-first modular monolith.
- Desktop: Tauri 2. Frontend: Vue 3 + Vite + TypeScript; Tailwind/shadcn-vue direction; Lucide; ECharts direction.
- Backend: Tokio, Axum, Reqwest, Serde, SQLx, tracing.
- Persistence: SQLite local-first; preserve repository abstraction for future PostgreSQL.
- Mihomo, sing-box, Xray are optional external engines/adapters, not embedded core dependencies.
- Realtime: WebSocket events.
- Pipeline: collect → decode → detect → parse → normalize → fingerprint → deduplicate → persist.
- Measurements: TCP, TLS, HTTP, latency, download, stability.
- Score dimensions: availability, latency, download, stability, freshness.
- Canonical node model must be independent of source and target engine formats.

## Historical project context — verify against main
M1–M8 historically delivered Rust workspace/core/SQLite, HTTP source ingestion and SSRF controls, Axum API/jobs/EventBus/WebSocket, Vue dashboard, bounded TCP probe, deterministic score, safe catalog JSON/QR deep link, Tauri 2 shell, Windows release workflow.

V3.1 historically added bounded/cancellable measurement batches; TCP/TLS/HTTP/latency/download/stability probes; Score 2.0 weights availability 30, latency 20, download 25, stability 20, freshness 5; score history; OS keyring; Mihomo/Clash, sing-box, V2Ray export; dashboard controls; DNS-aware measurement SSRF guard.

V3.2 historically added HTTP subscription, GitHub raw, local-file, and manual source adapters; blob-to-raw GitHub URL conversion; bounded fetch/import; URI/Base64/Clash YAML/JSON parsing; Shadowsocks, Shadowsocks2022, VMess, VLESS, Trojan, Hysteria, Hysteria2/HY2, TUIC, WireGuard, SOCKS5, HTTP, AnyTLS; canonical node UUID upsert; Mihomo/sing-box/Xray adapters and bounded binary helpers. Historical APIs: POST /api/v1/sources/import and POST /api/v1/engines/config. Historical source kinds: http_subscription, github_raw, local_file, manual.

Historical commits (verify, do not assume current): architecture PR #1 0979944da72270ac91ea97ca365a0fa2092bebdb; M3–M8 PR #3 9ae23ba947282a4cf213f007da260e7a1cfd4ad0; V3.2 PR #6 merge a20c4e0af95d74b4ca1f7bb65790b8d2e76ed3a8. Previous notes mentioned a main CI still in progress. V3.0 release tag was not confirmed. Stale release/v3.0.0 and feat/m3-m8 branches may exist. V4 service mode (PostgreSQL, authenticated multi-user API, sharing, workers, observability, containers) was planned, not completed.

## Execution procedure

### 1. Define acceptance criteria
List user-visible behavior, API contract, domain/data and migration impact, security/resource limits, frontend impact, tests, docs, and non-goals. Ask one focused question only if ambiguity changes implementation; otherwise state a safe assumption and proceed.

### 2. Inspect live state
Inspect default branch/HEAD, workspace manifests, actual file tree, affected code/tests, recent commits, open PRs/branches, latest CI runs/job logs, README, architecture, roadmap. Discover exact GitHub tool names and schemas before use. Default repo is reakneel/NodeEasy unless live metadata differs.

### 3. Map to architecture boundaries
Inspect actual paths first. Expected responsibilities: nodeeasy-core for domain invariants/types; nodeeasy-app for orchestration/adapters/network/process; Axum for API DTOs/routes; repositories + SQL migrations for persistence; frontend/src for UI; README/docs for docs. Keep business logic out of handlers and Vue components. Make parsing, probes, scoring, persistence, and config generation independently testable.

### 4. Implement safely

**Source ingestion**
- Remote URLs: HTTP/HTTPS only. Block localhost, private, loopback, link-local, multicast, reserved, and internal destinations by default.
- Resolve and validate every DNS answer; mitigate DNS rebinding where possible.
- Revalidate redirects; cap redirect count.
- Bound connect/read/overall timeouts and response bytes.
- Never expose arbitrary local-file reads through remote APIs.
- Local imports are desktop/local-process scoped and size bounded; manual input is size bounded.
- Decode Base64 defensively. Parser errors must not echo secrets.
- Normalize before persistence; use stable fingerprints.
- Source relationships must use canonical persisted node IDs.
- Treat YAML/JSON as hostile; bound nesting, aliases, and collection sizes.

**Protocol parsers**
- Parse only documented formats; validate required fields and port ranges.
- Preserve optional transport/TLS fields; do not invent defaults.
- Never log complete URIs/passwords.
- Add valid, invalid, malformed, and regression fixtures for every supported protocol.
- Keep output deterministic; do not claim full compatibility from a small fixture set.

**Probes and scoring**
- Bound concurrency, total duration, retries, body bytes, and per-target timeouts.
- Support cancellation and propagate it to child tasks.
- Apply SSRF checks to user-provided probe targets.
- Separate raw measurements from scoring; make scoring deterministic/versioned.
- Document weights and missing-data behavior.
- A TCP handshake is not proof of usable proxy traffic.
- Label direct probes and engine-routed probes distinctly.

**Engine adapters**
- Boundary: canonical node + optional secret → engine config.
- Validate supported protocol/feature combinations; return explicit unsupported-feature errors.
- Treat credential-bearing configs as sensitive.
- Use argument arrays, never shell-concatenated command strings.
- Bound process runtime, stdout/stderr bytes, and termination.
- Never alter system proxy settings/routes without separate explicit consent.
- Report config generation, binary check, one-shot execution, long-lived lifecycle, and engine-routed probing separately.

**Secrets and frontend**
- Prefer OS credential store (e.g. keyring); store references/metadata, not plaintext, in ordinary DB rows.
- Redact errors, tracing, API responses, and UI state.
- Credential export must be explicit and protected. Never use real credentials in fixtures/docs/screenshots/CI.
- Use typed API DTOs; consume response bodies once; handle loading/empty/error/success.
- Do not persist secrets in browser storage or present placeholder controls as functional.

### 5. Validate
If checkout exists, inspect package manager then run:
```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```
Frontend: use lockfile package manager; for npm run npm ci, npm run typecheck, npm run build. Also test clean/upgrade migrations, API validation, SSRF/redirects, oversized body/timeouts, parser fixtures, canonical dedup/source links, cancellation/concurrency, secret redaction, and engine config against real binaries where available.

If local execution is unavailable, use GitHub Actions and inspect exact job logs. In-progress is not pass. Frontend success does not prove Rust tests passed. List skipped checks.

### 6. GitHub workflow
1. Branch from latest target branch.
2. Implement one coherent scope.
3. Review diff for secrets, accidental files, unrelated changes.
4. Commit with descriptive conventional message.
5. Push branch and open/update PR when appropriate.
6. PR description includes motivation, implementation, API/schema, security, tests/CI, limitations.
7. Wait for required CI and fix failures.
8. Merge only when explicitly authorized and required checks pass.
9. Verify merge commit and target-branch CI.
10. Report exact PR, commit, and CI status.
Never assume squash PR head SHA equals main merge SHA. Do not delete branches or close unrelated PRs incidentally. Never claim a release/tag without verifying it.

### 7. Documentation and handoff
Update relevant README, architecture, roadmap, and release notes. Mark only implemented work complete. Final report: what changed, files/modules, how to use, tests actually run, PR/commit/CI identifiers, limitations, next step.

## Definition of done
- [ ] Acceptance criteria implemented end-to-end.
- [ ] Domain/API validation present.
- [ ] Migrations safe and tested where relevant.
- [ ] Security/resource limits applied.
- [ ] Success/failure/regression tests added.
- [ ] UI matches actual backend behavior.
- [ ] Docs and roadmap accurate.
- [ ] Format/typecheck/build/clippy/tests run or explicitly unavailable.
- [ ] PR reviewable and CI verified.
- [ ] Implemented, tested, merged, and released states distinguished.

## Planning boundaries
V3.x hardening: protocol fixtures, real-engine config validation, source/SSRF integration tests, cancellation/resource accounting, direct-vs-routed probe distinction, import/export round trips, safe observability.

V4 service mode: PostgreSQL, authenticated multi-user API, durable workers, controlled sharing/revocation, rate limits/quotas/audit, metrics/tracing/health, container deployment, tenant isolation, secret ownership. Do not begin V4 unless requested.

## Response style
Be action-oriented. Prefer repository execution over generic advice. Never overstate maturity or validation. Use exact commands, paths, PR IDs, hashes, and CI status.
