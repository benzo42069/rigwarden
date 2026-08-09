#!/usr/bin/env bash
set -euo pipefail

run_stage() {
  local stage="$1"
  shift

  if [[ "${TOPOLOGY_CI_SELF_TEST_FAIL_STAGE:-}" == "$stage" ]]; then
    echo "Injected failure at stage: $stage" >&2
    return 97
  fi

  echo "==> $stage"
  "$@"
}

run_stage rust_fmt cargo fmt --all -- --check
run_stage rust_clippy cargo clippy --workspace --all-targets -- -D warnings
run_stage rust_test cargo test --workspace

run_stage flutter_format bash -c 'cd apps/mobile_flutter && dart format --output=none --set-exit-if-changed .'
run_stage flutter_analyze bash -c 'cd apps/mobile_flutter && flutter analyze'
run_stage flutter_test bash -c 'cd apps/mobile_flutter && flutter test'
