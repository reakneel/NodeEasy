# NodeEasy Maintainer Skill

Agent skill for maintaining the NodeEasy repository.

## Contents
- SKILL.md — activation metadata, project context, implementation rules, security, testing, GitHub workflow, definition of done.
- references/architecture.md — system boundaries and data flow.
- references/security-checklist.md — source, parser, probe, process, and secret checks.
- references/development-workflow.md — repeatable feature workflow.
- scripts/validate.sh — Rust and frontend validation helper.

## Install
Copy `.agents/skills/nodeeasy-maintainer/` into the skill directory supported by your agent runtime, or point the runtime to this directory. Entry point: `SKILL.md`.

## Use
Ask the agent to work on NodeEasy features, bugs, parsers, probes, scoring, engine adapters, security, tests, CI, PRs, or releases. The skill requires inspection of the live repository before action.

Historical commits and milestones are context only; verify live code, PRs, branches, and CI.
