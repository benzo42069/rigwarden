# TOP-CMD-002 integration sweep

Integration commit: `b9ac4b8af81e86feedfa6992c095d7c436147c7b` on public `main`  
Executed: 2026-08-09 America/Chicago  
Integrator: `/root` (parent orchestrator)

Published blobs for `src/mutation.rs`, `tests/read_only.rs`, and `review.md`
were compared with the reviewed local candidates before the sweep; all matched.

All commands exited `0`:

```text
cargo test -p topology-command-engine read_only_session_cannot_plan_a_write -- --exact --nocapture
cargo test -p topology-command-engine read_only_status_cannot_plan_a_write_even_if_capability_is_true -- --exact --nocapture
cargo test -p topology-command-engine
cargo test -p topology-device-registry
cargo fmt --all -- --check
cargo clippy -p topology-command-engine --all-targets -- -D warnings
```

Claims established at L1 only: `UNIT_VERIFIED`, `READ_ONLY`. Protocol bytes,
simulator, platform, and hardware verification remain unavailable.
