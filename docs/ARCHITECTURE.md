# NodeEasy Architecture Freeze v1.3

## 1. Product boundary

NodeEasy is a **node data center**, not a proxy client. The product pipeline is:

`collect -> decode -> detect -> parse -> normalize -> fingerprint -> deduplicate -> persist -> test -> score -> export -> share`

NodeEasy owns node intelligence, measurement, ranking and distribution. It does not own TUN, system proxy routing or long-lived traffic forwarding.

## 2. Runtime architecture

NodeEasy remains a modular monolith:

- `core`: canonical domain, validation, fingerprinting, parsing primitives and scoring
- `sources`: pluggable public/authorized source adapters
- `storage`: SQLite repositories/migrations
- `testing`: bounded cancellable probes
- `scoring`: deterministic quality scoring
- `secrets`: OS-backed credential storage
- `export`: protocol configuration/subscription adapters
- `engine`: optional Mihomo/sing-box/Xray adapters
- `api`: Axum HTTP + WebSocket
- `app`: Tauri lifecycle

The boundaries are intentionally extraction-friendly for later service mode.

## 3. Source adapter boundary

Every source implements:

`SourceAdapter::collect() -> ParsedNode[]`

Current adapters:

- HTTP(S) subscription
- GitHub blob URL → raw GitHub
- raw public GitHub
- local file
- manual API import

Remote sources remain HTTP(S)-only and use the existing SSRF, DNS and body-size controls. Local/manual inputs are size bounded. Source-specific authentication is not part of the canonical source contract.

## 4. Parser boundary

The parser converts external representations into `ParsedNode` and never exposes source/engine-specific structures to storage or scheduling code.

Supported representations include:

- URI lines
- standard / URL-safe Base64 subscriptions
- Clash/Mihomo YAML
- Clash/Mihomo JSON

Canonical protocols include Shadowsocks, Shadowsocks 2022, VMess, VLESS, Trojan, Hysteria, Hysteria2, TUIC, WireGuard, SOCKS5, HTTP(S) and AnyTLS.

## 5. Identity

Fingerprinting uses canonical protocol/endpoint/identity fields. Source membership is separate from node identity. During refresh, repository upsert returns the canonical database UUID so a new parser instance cannot create orphaned source relationships.

Secrets are never part of the canonical `Node` record.

## 6. Measurement

All probes follow:

`prepare -> bounded attempt(s) -> observation -> persist -> aggregate -> score`

Requirements: explicit timeout, cancellation, bounded concurrency, controlled targets, capped bodies, structured errors and SSRF protection.

Probe types are TCP, TLS, HTTP, latency, download and stability. Protocol-aware proxy traffic is an engine concern, not a core probe concern.

## 7. Engine adapter boundary

Engines implement an engine-neutral `EngineAdapter`:

`canonical Node + optional secret -> EngineConfig`

Current adapters:

- Mihomo config generation
- sing-box config generation
- Xray config generation for VMess/VLESS/Trojan

The adapter layer also exposes bounded binary availability checks and a bounded process runner. Actual engine binaries are optional runtime dependencies; they are never embedded into `nodeeasy-core`.

## 8. Security boundary

- public/free nodes are untrusted
- source fetches block private/loopback/link-local destinations by default
- source and download bodies are size bounded
- engine processes have explicit timeouts
- secrets use the OS credential provider and are excluded from normal node rows/logs
- credential-bearing exports require secret material
- no engine is allowed to silently turn NodeEasy into a system-wide proxy

## 9. API

The frontend consumes the API rather than storage directly. Long-running work publishes lifecycle events through EventBus/WebSocket.

V3.2 adds:

- `POST /api/v1/sources/import`
- `POST /api/v1/engines/config`

Existing source sync and export APIs continue to use canonical contracts.

## 10. Extensibility policy

Adding a source, protocol parser or engine must not require changes to Scheduler, Worker, dashboard state or storage semantics. New integrations belong behind `SourceAdapter`, parser normalization, or `EngineAdapter` boundaries.

## 11. Delivery policy

V3.0 foundation, V3.1 measurement/security/export and V3.2 source/parser/engine expansion are complete. V4 is reserved for authenticated service mode, PostgreSQL deployment and multi-user operations after the desktop workflow proves stable.
