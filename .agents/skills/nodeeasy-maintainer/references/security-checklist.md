# NodeEasy Security Checklist

Apply to every feature handling external input, network access, credentials, or child processes.

## Remote source URLs
- [ ] HTTP/HTTPS only.
- [ ] Reject localhost, loopback, private, link-local, multicast, reserved, and internal IP ranges by default.
- [ ] Resolve and validate every DNS answer.
- [ ] Mitigate DNS rebinding where possible.
- [ ] Revalidate redirects; cap redirect count.
- [ ] Bound connect/read/overall timeouts.
- [ ] Cap response and decompressed body bytes.
- [ ] Do not forward credentials across untrusted redirects.
- [ ] Never expose arbitrary local-file reads through remote APIs.

## Local/manual imports
- [ ] Local file access is desktop/local-process scoped.
- [ ] File/text sizes bounded.
- [ ] Errors do not echo credential-bearing content.
- [ ] Parser nesting, aliases, line count, and collection sizes bounded.
- [ ] Unsupported formats fail explicitly.

## Parsing and normalization
- [ ] Required fields and port ranges validated.
- [ ] Optional TLS/transport fields preserved accurately.
- [ ] No semantic-changing silent defaults.
- [ ] Stable fingerprint and deterministic normalization.
- [ ] Positive, negative, malformed, and regression fixtures.
- [ ] No full URI/password in logs.

## Probes
- [ ] Target passes SSRF validation.
- [ ] Concurrency, retries, timeout, total duration, and body size bounded.
- [ ] Cancellation reaches spawned tasks.
- [ ] Direct and engine-routed probes labeled distinctly.
- [ ] Raw measurements separated from scoring.

## Engine processes
- [ ] Executable path and args validated.
- [ ] Argument arrays used; no shell concatenation.
- [ ] Runtime/stdout/stderr bounded.
- [ ] Child process termination deterministic.
- [ ] Credential-bearing configs treated as secrets.
- [ ] No system proxy or route changes without explicit consent.

## Secrets
- [ ] OS credential store used where supported.
- [ ] DB stores references/metadata, not plaintext credentials.
- [ ] Tracing/errors/API/UI redact secrets.
- [ ] Credential export explicit and protected.
- [ ] No real credentials in fixtures/docs/screenshots/CI logs.
