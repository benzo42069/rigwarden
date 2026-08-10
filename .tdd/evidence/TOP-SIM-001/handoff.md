status: INTEGRATED
work_item: TOP-SIM-001
requirements: SIM-001, TRANSPORT-003

Cycle completed:
- Preserved the original pre-amendment invalid package-selector evidence under `red-invalid-*` and `environment-initial-blocked.txt`.
- Applied the parent amendment allowing only minimal simulator workspace membership and its lockfile edge.
- Added a no-dependency `topology-simulator` crate and focused integration test harness.
- Fresh RED command: `cargo test -p topology-simulator scripted_exchange_correlates_expected_response -- --exact --nocapture`.
- Fresh RED status: exit 101, accepted; compiler reached the intended package and reported only missing simulator API symbols.
- Minimum GREEN: synthetic payload wrapper, request/response IDs, explicit synthetic identity, ordered transcript, structured mismatch, and deterministic in-memory exchange.
- Focused GREEN status: exit 0; final focused rerun after formatting also exit 0.
- Required final fail-fast sweep: simulator tests 0, command-engine tests 0, format check 0, clippy 0.
- First pre-format sweep failure (format exit 1) is preserved and explained; formatter-only cleanup was followed by focused GREEN rerun.

Files changed:
- `Cargo.toml` and `Cargo.lock` (amended membership/package edge only; broader pre-existing worktree changes not authored here).
- `crates/topology_simulator/Cargo.toml`
- `crates/topology_simulator/src/lib.rs`
- `crates/topology_simulator/src/scripted_transport.rs`
- `crates/topology_simulator/tests/request_response.rs`
- `.tdd/evidence/TOP-SIM-001/**`

Design boundaries:
- Payloads are typed `SyntheticPayload` and test values are explicitly non-vendor.
- No protocol-byte interpretation, external fixture, timing, retries, network, hardware, endpoint enumeration, or stateful device behavior.
- Simulator identity is `SimulatorIdentity::SyntheticScripted`; transcript records `Sent` then `Received` only for a matching request.

Dependency observations:
- `work-items/index.yaml` marks TOP-CMD-003 INTEGRATED.
- Starting HEAD `536d8901ac91ecdbc15e09356800d9f46be401dd` does not contain the dependency crate; current untracked command-engine files and broader manifest/lock edits are pre-existing worktree state. Parent/integration owner must verify the actual integration commit.
- Parent amended packet reference `a97936e37839fc9dca5d721eb1339ce87bb6964e` is not present in the local object database.

Claims earned by this worker: `GREEN_OBSERVED` candidate only; proposed `SIMULATOR_VERIFIED` after independent review and integration rerun.
Claims not earned: `BYTE_FIXTURE_VERIFIED`, `PLATFORM_DEVICE_VERIFIED`, `HARDWARE_VERIFIED`.
Independent review: `REVIEW_APPROVED` by `/root/sim001_review`.

Integration: parent reran the focused simulator test, simulator package test,
adjacent command-engine package test, workspace format check, and simulator
Clippy with `-D warnings`; every command exited 0. The bounded candidate was
published to public main with the packet/index status update. See
`integration-sweep.md`.
Next packet: TOP-SIM-002 after this simulator foundation is integrated; TOP-E2E-001 remains dependent on its own prerequisites.
