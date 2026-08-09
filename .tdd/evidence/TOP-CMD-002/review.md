# TOP-CMD-002 independent security review

Reviewer: `/root/cmd002_review` (`topology_security_reviewer`)
Review timestamp: `2026-08-09T16:10:01-05:00`
Review basis: shared candidate at starting commit
`536d8901ac91ecdbc15e09356800d9f46be401dd`; correction source/test changes and
their additive evidence are present in the shared worktree.
Decision: `REVIEW_APPROVED` (candidate; parent integration rerun remains required)

## Findings and resolution

The initial review found one blocking fail-open path:

**CMD-002-F1 — `ReadOnly` status could be bypassed by a contradictory write
capability (HIGH hardware-write authorization risk).** The original
`mutation.rs` nested the status check under `!capabilities().can_write()`,
while public `DeviceProfile::new` permits a `ReadOnly` profile with the write
bit set. A valid declared parameter then produced an accepted mutation.

The implementation corrected this before approval. The new
`read_only_status_cannot_plan_a_write_even_if_capability_is_true` test creates
that contradictory profile with an in-range, declared `amp-1/gain` request.
Its observed RED (`red-correction.log`, exit 101) shows the validator returned
`ValidatedParameterMutation`, proving the intended bypass. The GREEN
(`green-correction.log`, exit 0) follows the minimal fix: the
`VerificationStatus::ReadOnly` guard now runs unconditionally at
`crates/topology_command_engine/src/mutation.rs:82-86`, before the capability
check and profile numeric lookup. The test asserts the structured
`ReadOnly { firmware: "1.1" }` error. This closes the path for malformed or
future pack-derived profile combinations as well as the registry-generated
unknown-firmware session.

## Behavior and security audit

- `read_only_session_cannot_plan_a_write` resolves known family/model firmware
  `1.1` against only writable firmware `1.0`, submits syntactically valid
  stored value `45`, and asserts the firmware-bearing structured rejection.
  The exact writable parameter test remains green. No validated mutation,
  protocol bytes, queue item, transport call, or hardware plan is created on
  either read-only rejection path.
- The guard is before capability and `numeric_parameter` lookup. Registry
  resolution remains exact over family/model/opaque firmware, with unknown
  firmware represented as a non-writable `ReadOnly` profile; no nearest or
  lower-version fallback is present.
- Stored values and profile bounds are `i32` and are compared inclusively
  without arithmetic; precision is a bounded `u8`. No integer overflow or
  float/NaN path exists in this packet.
- `ParameterMutationRequest` owns arbitrary `String` IDs and profile metadata
  is a `Vec`; this crate has no parser, file/network boundary, or size limit.
  Any future SysEx/file/AI boundary must bound and fuzz before constructing
  these values under SEC-001. That remains a future input-budget obligation,
  not an exploit in this typed unit slice.
- There is no request correlation, retry, partial-completion, transport,
  network, telemetry/logging, secret, pack-install/signature, protocol-byte,
  or AI/raw-transport implementation in the assigned scope. Those claims are
  explicitly unavailable and are not inferred from these unit tests.

## TDD and evidence audit

- The original focused RED/GREEN for the registry-resolved unknown-firmware
  behavior is preserved (`red.*`, `green.*`).
- The correction has its own observed, focused RED/GREEN pair with exact
  commands and exit statuses in `red-correction-*` and `green-correction-*`.
  The RED reaches the intended test and fails only because the contradictory
  profile is accepted; it is not a selector, fixture, or unrelated compile
  failure.
- The final fail-fast candidate sweep is complete and independently recorded
  in `sweep-final-*`: command-engine tests, registry tests, rustfmt check, and
  command-engine Clippy all exit 0. The command-engine package run exercises
  both read-only tests and the pre-existing exact writable test.
- I independently reran from `/Users/benzo/Desktop/Topology_Codex_Starter_Kit_v1`:

  1. `cargo test -p topology-command-engine read_only_status_cannot_plan_a_write_even_if_capability_is_true -- --exact --nocapture` — exit 0.
  2. `cargo test -p topology-command-engine` — exit 0.
  3. `cargo test -p topology-device-registry` — exit 0.
  4. `cargo fmt --all -- --check` — exit 0.
  5. `cargo clippy -p topology-command-engine --all-targets -- -D warnings` — exit 0.

## Verification-label audit and integration conditions

After the parent lands this bounded candidate and reruns the focused command,
the correction test, and all required sweeps from the immutable integration
commit, it may claim `UNIT_VERIFIED` and `READ_ONLY` for the exercised
known-family/model unknown-firmware and fail-closed read-only capability
behavior. `BYTE_FIXTURE_VERIFIED`, `SIMULATOR_VERIFIED`,
`PLATFORM_SIMULATOR_VERIFIED`, `PLATFORM_DEVICE_VERIFIED`,
`HARDWARE_VERIFIED`, pack-signature, transport, retry, telemetry, secret, and
AI-isolation claims remain unavailable.
