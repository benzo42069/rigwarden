# ADR-0005: Permit typed, capability-gated semantic command and state events across the Flutter bridge

- Status: Accepted for the deterministic synthetic vertical slice
- Date: 2026-08-10
- Owner: parent orchestrator
- Requirements: `BLOCK-003`, `SIM-001`, `UNDO-001`, `QA-001`

## Context

`ADR-0004` correctly established the first generated Rust–Dart boundary with
one read-only identity value. `TOP-E2E-001` then exposed a real boundary gap:
the command validator, synthetic simulator, journal, generated bridge, and
Flutter shell exist as separate tested islands, but no approved application
path composes them. A Dart fake would not prove the Rust command or journal
path, while exposing raw frames or transport handles would violate the bridge
and threat-model boundaries.

## Decision

Extend the generated bridge only enough to expose **typed semantic commands**
and **typed application state events** for deterministic, capability-gated
workflows.

The first endpoint may accept a stable block ID, parameter ID, and stored
integer value; it must invoke Rust profile/capability/range validation before
any synthetic exchange. It may return typed pending, confirmed, read-only, and
structured rejection state. The UI presents values using the profile-supplied
precision and announces pending/confirmed/error truthfully.

The bridge must not expose:

- vendor or synthetic transport bytes;
- raw transport handles, endpoint-open operations, or arbitrary send methods;
- pointers or mutable Rust-owned domain construction from Dart;
- a writable mapping for unknown or read-only firmware;
- a hardware, native-platform, or protocol-compatibility claim.

The first UI is a test-owned deterministic harness, not a completed production
parameter editor. A separate work item must add the production screen and its
full accessibility workflow.

## Consequences

- Rust remains the sole owner of validation, synthetic execution, confirmation,
  and journal state.
- The composition path uses an explicitly labeled synthetic peer and sanitized
  typed transcript summary; it carries no vendor bytes or device claim.
- A focused Rust composition test precedes generated-binding and Flutter E2E
  work. `TOP-UNDO-003` is a prerequisite because the current journal can
  record an entry but cannot prepare and confirm one restoration operation.
- Generated binding files, bridge manifest dependency edges, and the lockfile
  remain integration-owned and must be regenerated and reviewed.
- The Flutter harness must expose semantic name, role, value/unit/range,
  pending/confirmed state, action, and deterministic focus behavior. Its
  evidence can earn only the declared simulator and Flutter-semantics labels.

## Alternatives considered

### Keep the bridge read-only and fake the edit in Dart

Rejected. It would leave the validator, simulator, and journal disconnected
from the claimed end-to-end path.

### Expose raw transport frames or a generic send API

Rejected. It would move an unsafe transport capability into presentation and
would be indistinguishable from a protocol/hardware claim.

### Build the complete production parameter editor first

Rejected for this slice. It broadens scope before the underlying command,
confirmation, and undo contract is proven.

## Verification plan

`TOP-UNDO-003` tests one prepared and confirmed semantic restoration. `TOP-E2E-000`
then tests Rust composition through validation, synthetic exchange, confirmed
journal entry, restoration exchange, and typed final state. `TOP-E2E-001`
then proves the generated Flutter bridge and test-owned semantic harness.
No result from those tests may be promoted to a byte, native-platform, or
hardware verification label.
