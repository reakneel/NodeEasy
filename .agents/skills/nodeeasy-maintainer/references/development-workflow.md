# NodeEasy Development Workflow

1. **Scope:** define behavior, API, domain/schema, migration, security, UI, tests, docs, and non-goals.
2. **Inspect:** read live tree, manifests, affected code/tests, commits, open PRs, CI, README, architecture, roadmap. Do not assume historical paths or commits are current.
3. **Design:** map domain to nodeeasy-core; orchestration/adapters/network/process to nodeeasy-app; DTO/routes to Axum; persistence to repositories/migrations; UI to frontend/src; docs to README/docs.
4. **Implement:** smallest complete vertical slice. Validate at boundaries; bound work; support cancellation; redact secrets; fail explicitly for unsupported features.
5. **Test:** run validation script or equivalent. Cover success, failure, malformed input, limits, regression, and migrations.
6. **Review:** inspect diff for secrets, accidental files, unbounded work, silent fallbacks, untested branches, misleading UI, and unrelated formatting.
7. **Deliver:** create task branch and PR. Include summary, API/schema changes, security, exact tests, CI state, limitations, and follow-ups. Merge only when authorized and required checks pass.

## Status vocabulary
- **Implemented:** code exists.
- **Tested:** specified checks ran successfully.
- **CI green:** all required workflows completed successfully.
- **Merged:** PR merge confirmed and merge SHA known.
- **Released:** tag/release record exists and is verified.
