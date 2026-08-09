# TOP-BOOT-004 handoff

Status: INTEGRATED

Reviewed source and evidence were published at `066501ef8f249da5bbd5a2b4b937db3c04d4dada`; the integration owner reran `bash scripts/ci-local.sh` successfully from the same integration worktree before status promotion.

Added a local fail-fast CI script and a GitHub Actions wrapper. The test-only `TOPOLOGY_CI_SELF_TEST_FAIL_STAGE` injection proves an intermediate Rust clippy failure exits 97 before subsequent stages; normal local Rust and Flutter checks pass. No release/platform/hardware claim exists.
