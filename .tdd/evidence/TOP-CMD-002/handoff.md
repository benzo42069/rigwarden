# TOP-CMD-002 handoff

Status: `INTEGRATED`
Work-item: `TOP-CMD-002`

## Behavior delivered

`validate_parameter_mutation` now checks the selected profile's write
verification status and write capability before looking up numeric parameter metadata. A registry-resolved
known device with unknown firmware is represented by a non-writable
`VerificationStatus::ReadOnly` profile, and a mutation request against it
returns `MutationValidationError::ReadOnly { firmware }` with the observed
firmware string. The status guard is unconditional, so even an inconsistent
ReadOnly profile with a true write capability cannot plan a write. No
validated mutation, protocol mapping, wire bytes, queue, or transport object
is created.

The focused test constructs a writable profile for firmware `1.0`, resolves a
known family/model with unknown firmware `1.1`, submits a syntactically valid
`amp-1/gain` stored value `45` request, and asserts the structured rejection.
The pre-existing exact writable test remains green in the package sweep.

## TDD and sweeps

- RED: `cargo test -p topology-command-engine read_only_session_cannot_plan_a_write -- --exact --nocapture`, exit 101. The only diagnostic is the deliberately absent `ReadOnly` error variant.
- GREEN: same focused command, exit 0.
- Post-format focused rerun: exit 0.
- Correction RED: `cargo test -p topology-command-engine read_only_status_cannot_plan_a_write_even_if_capability_is_true -- --exact --nocapture`, exit 101 because the old nested guard returned `ValidatedParameterMutation`.
- Correction GREEN: same command, exit 0 after the unconditional status guard.
- Required final fail-fast sweeps: command-engine tests, registry tests,
  formatter, and command-engine Clippy all exit 0; see `sweep-final.log` and
  `sweep-final-exit-statuses.txt`.
- The first formatter check reported only mechanical line wrapping in the new
  tests, and a correction-sweep formatter run was also preserved before the
  mechanical fix; those outputs remain under `sweep-initial-*` and
  `sweep-correction-*` and were fixed before the final sweep.

## Scope and design

Changed only `crates/topology_command_engine/src/mutation.rs`,
`crates/topology_command_engine/tests/read_only.rs`, and this evidence
directory. No manifest, lockfile, registry, protocol, transport, UI,
device-pack, index, or traceability changes.

## Claims

Available after independent security review and parent integration rerun:

- `UNIT_VERIFIED`
- `READ_ONLY`

Unavailable:

- `BYTE_FIXTURE_VERIFIED`
- `SIMULATOR_VERIFIED`
- `PLATFORM_SIMULATOR_VERIFIED`
- `PLATFORM_DEVICE_VERIFIED`
- `HARDWARE_VERIFIED`

## Review and integration

The first independent security review recorded a valid fail-open finding; the
tightly coupled test-first correction was independently re-reviewed and
approved. Parent landed the bounded source/test/evidence paths in public
candidate commit `b9ac4b8af81e86feedfa6992c095d7c436147c7b`, confirmed the
published source and review blobs match this evidence, and reran both focused
tests plus all four required sweeps successfully. Packet and index status are
now `INTEGRATED`; see `integration-sweep.md`.

Suggested next packet: none required by TOP-CMD-002; dependent work remains
parent-scheduled after integration.
