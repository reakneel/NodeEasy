#!/usr/bin/env bash
set -euo pipefail

echo "== Rust formatting =="
cargo fmt --all -- --check

echo "== Rust workspace check =="
cargo check --workspace --all-targets

echo "== Rust clippy =="
cargo clippy --workspace --all-targets -- -D warnings

echo "== Rust tests =="
cargo test --workspace

if [ -d frontend ]; then
  cd frontend
  if [ -f package-lock.json ]; then
    npm ci
    npm run typecheck --if-present
    npm run build
  elif [ -f pnpm-lock.yaml ]; then
    corepack pnpm install --frozen-lockfile
    corepack pnpm run typecheck --if-present
    corepack pnpm run build
  elif [ -f yarn.lock ]; then
    corepack yarn install --immutable
    corepack yarn typecheck --if-present
    corepack yarn build
  else
    echo "No recognized frontend lockfile; inspect package manager manually." >&2
    exit 2
  fi
fi

echo "All configured validation checks passed."
