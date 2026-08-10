# TOP-SIM-001 integration sweep

Integration date: 2026-08-10

Parent reran the reviewed candidate in the shared integration worktree before
publishing it to public `main`:

1. `cargo test -p topology-simulator scripted_exchange_correlates_expected_response -- --exact --nocapture` — exit 0.
2. `cargo test -p topology-simulator` — exit 0.
3. `cargo test -p topology-command-engine` — exit 0.
4. `cargo fmt --all -- --check` — exit 0.
5. `cargo clippy -p topology-simulator --all-targets -- -D warnings` — exit 0.

Independent review is recorded in `review.md`. The only promoted label is
`SIMULATOR_VERIFIED` for the synthetic in-memory script. This does not prove
protocol bytes, a physical transport, a mobile platform, or hardware.
