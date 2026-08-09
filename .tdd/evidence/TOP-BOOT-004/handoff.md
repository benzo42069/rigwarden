# TOP-BOOT-004 handoff

Status: REVIEW_APPROVED; pending integration.

Added a local fail-fast CI script and a GitHub Actions wrapper. The test-only `TOPOLOGY_CI_SELF_TEST_FAIL_STAGE` injection proves an intermediate Rust clippy failure exits 97 before subsequent stages; normal local Rust and Flutter checks pass. No release/platform/hardware claim exists.
