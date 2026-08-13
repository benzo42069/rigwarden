# TOP-E2E-001 final independent correctness review (post-accessibility approval)

review_status: `REVIEW_APPROVED`

work_item: `TOP-E2E-001`

reviewer: `/root/e2e001_final_correctness` (`topology_reviewer`, OpenAI `gpt-5.6-luna` / `max`)

reviewed_at: `2026-08-13T00:32:59-05:00`

reviewed_baseline: `536d8901ac91ecdbc15e09356800d9f46be401dd` (shared worktree; candidate source remains uncommitted)

## Decisive final verdict

`REVIEW_APPROVED` for the exact bounded synthetic Flutter/Rust candidate. The
final accessibility section in `accessibility-review.md` is independently
`REVIEW_APPROVED` for the emitted Flutter L4 semantics and focus claim. The
core safety proof remains intact after those accessibility changes: the real
release FFI path validates before private synthetic exchange, exposes only
typed state/errors, retains the confirmed journal entry while undo is pending,
and consumes it only after restoration confirmation. No blocking correctness,
security, realtime, or release finding remains for this bounded packet.

This is not approval of a physical-device, native screen-reader/switch,
large-text/reduced-motion, protocol-byte, complete-editor, or hardware claim.
Those labels remain unavailable exactly as the packet and final accessibility
review state.

## Independent required reruns

Every required final command below exited `0` in this shared worktree:

| Layer | Command | Working directory | Exit |
|---|---|---|---:|
| Release bridge | `CARGO_TARGET_DIR=crates/topology_bridge/target cargo build -p topology-bridge --release` | repository root | 0 |
| Focused happy path | `flutter test test/core/bridge/simulated_parameter_edit_test.dart --plain-name "simulated edit confirmation and undo complete end to end"` | `apps/mobile_flutter` | 0 |
| Focused focus/keyboard/recovery | `flutter test test/core/bridge/simulated_parameter_edit_test.dart --plain-name "emitted semantics retain focus order, keyboard activation, and recovery"` | `apps/mobile_flutter` | 0 |
| Focused read-only/error | `flutter test test/core/bridge/simulated_parameter_edit_test.dart --plain-name "read-only synthetic rejection crosses FFI without exchange"` | `apps/mobile_flutter` | 0 |
| Focused out-of-range/error semantics | `flutter test test/core/bridge/simulated_parameter_edit_test.dart --plain-name "out-of-range typed error renders through FFI semantics"` | `apps/mobile_flutter` | 0 |
| Rust workspace | `cargo test --workspace` | repository root | 0 |
| Flutter package | `flutter test` | `apps/mobile_flutter` | 0 |
| Required local gate | `bash scripts/ci-local.sh` | repository root | 0 |

The first focused Flutter attempt and first `ci-local` attempt were preserved as
environment-only failures: the host pub cache lacked Flutter and nested
Cargokit dependencies. After resolving the existing locked packages with
`flutter pub get` and `dart pub get` (no product/source/evidence edits), the
exact commands above were rerun and passed. The initial failures are not
accepted as RED or hidden as behavior failures.

## Core safety and typed-boundary audit

- `begin_edit` validates the current and requested values before journal
  staging or any exchange (`crates/topology_bridge/src/api/simulated_parameter_edit.rs:168-192`).
  `confirm_edit` exchanges only the already-validated value and confirms its
  journal entry afterward (`:199-235`).
- `begin_undo` stages `PendingUndo` without consuming the completed entry
  (`:238-263`). `confirm_undo` prepares and validates the restoration, performs
  the private exchange, confirms the journal proposal, and only then clears
  pending state (`:266-306`).
- `exchange` and `synthetic_payload` are private Rust implementation details
  (`:437-474`). The generated session surface contains only the two typed
  factories, typed opaque-session actions, typed state/error DTOs, and typed
  transcript entries (`apps/mobile_flutter/lib/core/bridge/generated/api/simulated_parameter_edit.dart:12-40,42-120,192-220`).
  No raw payload bytes, endpoint-opening operation, transport handle, pointer,
  or arbitrary send method is exposed. The pre-existing identity fixture's
  copied endpoint string is read-only metadata, not a transport capability.
- The private simulator exchange is explicitly labeled
  `rigwarden.synthetic-scripted-simulator`; no vendor/protocol/hardware claim
  is emitted. The test asserts no `vendor` or `bytes` text in the harness
  (`apps/mobile_flutter/test/core/bridge/simulated_parameter_edit_test.dart:186-195`).

## Direct staged transcript/journal/error proof

The real generated FFI happy-path test observes the typed facts rather than
only rendered text (`apps/mobile_flutter/test/core/bridge/simulated_parameter_edit_test.dart:24-35,96-184`):

- Initial and pending-edit states have `exchangeCount == 0` and an empty
  transcript. After edit confirmation, the count is `1`, transcript values are
  exactly `[45, 45]`, and the journal records prior/new `30/45` with one entry.
- Pending undo remains at count `1` with transcript `[45,45]` and the journal
  entry still present (`journalEntryCount == 1`, prior/new `30/45`).
- Undo confirmation reaches count `2`, typed transcript values exactly
  `[45,45,30,30]`, final stored/display value `30`/`3.0`, and zero completed
  journal entries. The generated transcript DTO is typed Request/Confirmed,
  not a byte log.
- The same bridge session rejects both an explicitly read-only write and
  out-of-range `101` with typed error codes and `exchangeCount == 0`
  (`simulated_parameter_edit_test.dart:341-391`). The out-of-range error is
  also fed through the harness and asserted in emitted semantics/text
  (`:440-518`).

## Exact bounded Flutter L4 claim

The final accessibility review approves `SEMANTICS_VERIFIED` only for this
test-owned deterministic synthetic harness. The final focused tests now
observe emitted button role, tap action, name, enabled/disabled state, target
context, current value/unit/range/step/precision, pending/confirmed/read-only/
error labels, live-region flag, focus-node identity/order, keyboard activation,
and focus recovery (`simulated_parameter_edit_test.dart:287-337,562-577,777-845`).
State is communicated textually and semantically, not by color or imagery.

The following remain explicitly unavailable: native/platform semantics,
VoiceOver/TalkBack, switch control, physical keyboard, large-text/reflow,
reduced-motion, complete nonvisual editor workflow, protocol-byte or native
transport compatibility, and modeler/hardware verification. A simulator and
Flutter semantics pass cannot promote any of those labels.

## Non-blocking architecture note

The bridge session still independently orchestrates the validator, private
scripted simulator, and journal instead of calling E2E-000's all-at-once
helper. This is a maintenance/drift risk, not a proof gap now that this exact
session has direct staged FFI assertions. Factor shared composition only in a
separately scoped follow-up; do not describe this candidate as reusing
E2E-000's proof.

## Scope and disposition

No production, test, generated, or other evidence file was edited by this
reviewer; this final decision updates only
`.tdd/evidence/TOP-E2E-001/review.md`. The historical `REVIEW_FAILED` review
and findings remain below unchanged. Parent integration must still land the
bounded candidate in an immutable worktree and rerun its required sweep before
changing packet/index status; this candidate-level approval does not itself
mark `INTEGRATED`.

---

# TOP-E2E-001 independent correctness review (correction re-review)

review_status: `REVIEW_FAILED`

work_item: `TOP-E2E-001`

reviewer: `/root/e2e001_review` (`topology_reviewer`, OpenAI `gpt-5.6-luna` / `max`)

reviewed_at: `2026-08-10`

reviewed_baseline: `536d8901ac91ecdbc15e09356800d9f46be401dd` (shared worktree; the candidate has no worker commit)

## Decision

`REVIEW_FAILED` remains the decisive packet verdict after the 12:15 correction
cycle. The corrected direct Flutter route is real generated FFI (not a Dart
mock), and its typed staged assertions now prove zero exchanges/transcript
before confirmation, one exchange after edit confirmation, journal retention
while undo is pending, the typed transcript values `45/45/30/30`, and final
journal consumption. The direct read-only and out-of-range fixtures return
typed error codes with `exchange_count == 0`. Rust source tracing confirms
validation before the private scripted exchange and journal confirmation only
after each exchange; the generated surface exposes no payload bytes, endpoint
handles, or arbitrary send operation.

The session still independently duplicates the all-at-once E2E-000 composition.
That is an architecture/maintenance risk, but it does not make this corrected
proof circular: this session's own source and direct FFI assertions cross the
validator, private scripted simulator, Journal, and generated typed boundary.
No claim is made that E2E-000's helper test covers this session.

The packet nevertheless requires an independent accessibility approval for its
unchanged broad `SEMANTICS_VERIFIED` claim. The correction-cycle accessibility
review remains `REVIEW_FAILED`: the harness does not assert emitted focus/order
or recovery, the live-region flag/announcement, and complete phase/value/
context/error semantics. The 12:15 amendment records `claim_changed: false`,
so those unavailable proofs remain a blocking packet gate. This is an evidence
boundary, not a failed build or a native-device claim.

## Historical initial verification (superseded by correction re-review)

All commands below were run from the frozen shared worktree after rebuilding
the configured release library. Every command exited `0`.

| Layer | Command | Exit |
|---|---|---:|
| Release FFI library | `CARGO_TARGET_DIR=crates/topology_bridge/target cargo build -p topology-bridge --release` | 0 |
| Focused Flutter FFI test | `cd apps/mobile_flutter && flutter test test/core/bridge/simulated_parameter_edit_test.dart --plain-name "simulated edit confirmation and undo complete end to end"` | 0 |
| Rust workspace | `cargo test --workspace` | 0 |
| Flutter package | `cd apps/mobile_flutter && flutter test` | 0 |
| Required local gate | `bash scripts/ci-local.sh` | 0 |

The recorded FRB generation command also exited `0` (`codegen.log` and
`codegen-exit-status.txt`). I did not rerun the generator because this review
is forbidden from rewriting generated files; the generated output was audited
against the amended packet scope and the rebuilt release library loaded it.

## Historical initial scope/boundary audit (superseded by correction re-review)

- The Rust implementation paths are within the amended write scope. The
  generated module split `apps/mobile_flutter/lib/core/bridge/generated/api/simulated_parameter_edit.dart`
  is present and is the exact extra path granted by the 11:45 amendment.
- `RustLib.init()` is used by the focused test (`simulated_parameter_edit_test.dart:14-19`),
  not `RustLib.initMock`. The test calls the generated opaque-session methods
  (`:28-35`), so it is not a Dart fake or a disconnected widget-only test.
- The generated DTOs contain only typed phase/state/transcript/value fields
  (`api/simulated_parameter_edit.dart:12-151`). `SyntheticPayload`,
  `ScriptedTransport`, and the private payload-construction function occur
  only inside Rust (`api/simulated_parameter_edit.rs:286-315`) and do not
  appear in the generated session API. No new raw transport handle,
  endpoint-open method, arbitrary send, or vendor-protocol claim is exposed.
- The direct RED is valid: the amended test reached the Flutter compiler and
  failed on the intended missing typed session API symbols in `red.log`, with
  the configured release-library precondition satisfied. The earlier
  `integration_test` device failures are preserved as environment attempts,
  not accepted as RED evidence.

## Historical initial behavior trace (superseded by correction re-review)

The current staged Rust source does truthfully cross the requested contracts at
this commit, even though the proof is incomplete:

1. `begin_edit` validates both the current and requested stored values before
   creating a pending journal entry or changing phase (`api/simulated_parameter_edit.rs:120-143`).
2. `confirm_edit` revalidates, calls the private scripted exchange, and only
   then confirms the pending journal ID (`:146-176`).
3. `begin_undo` reads the confirmed entry and stages `PendingUndo` without
   removing history (`:179-195`).
4. `confirm_undo` validates, prepares the journal proposal, exchanges the
   restoration, and calls `confirm_undo` before clearing the pending flag and
   reporting `ConfirmedUndo` (`:197-223`).

The focused test observes the pending and confirmed text states and final
journal summary (`simulated_parameter_edit_test.dart:56-104`), and the rerun
proved that these calls execute through the real release FFI library.

## Historical initial findings (superseded by correction re-review)

### E2E001-CORE-001 — HIGH — Flutter GREEN does not prove simulator exchange or staged journal ordering

`simulated_parameter_edit_test.dart:56-110` checks rendered phase/value text,
the final journal count, a simulator label, and absence of two strings. It
never reads the typed `exchangeCount` or `transcript` fields that the bridge
returns (`api/simulated_parameter_edit.rs:59-75`), and it does not assert that
the journal still has one entry while `PendingUndo` is shown. A mutant that
removes either `exchange` call, moves journal consumption before the
restoration, or fabricates the final state after the UI gate can therefore
still satisfy this Flutter test. The test then does not prove the packet's
acceptance bullets “deterministic command reaches simulator,” “undo sends
restoration,” or staged journal retention.

Required correction: assert the independent typed facts at every stage (zero
exchanges initially/pending edit, one after edit confirmation, still one
journal entry during pending undo, two exchanges and the exact request/confirm
sequence `45,45,30,30` after undo). Keep the assertions on typed fields, not
payload bytes.

### E2E001-CORE-002 — MEDIUM — The bridge session duplicates E2E-000 instead of using its reviewed composition API

The new module independently imports and orchestrates the validator,
`ScriptedTransport`, and `Journal` (`api/simulated_parameter_edit.rs:8-18,
:120-223,
:286-315`). It never calls the integrated `compose_synthetic_parameter_edit`
composition (`crates/topology_bridge/src/simulated_edit.rs:241-359`), despite
the packet precondition that TOP-E2E-000 supplies the reviewed Rust-owned
composition API. Thus E2E-000's independently reviewed no-exchange guards,
transcript assertions, and restoration checks do not cover the path exercised
by Flutter. At this commit the duplicate source trace has the right order, so
the path is not a fake bypass; the risk is contract drift and a false sense
that the reviewed composition proof was reused.

Required correction: either factor a shared Rust-owned staged composition
primitive (with a packet amendment granting the shared path) and have both
E2E-000 and the FRB session use it, or add a narrowly scoped bridge-session
test/fixture that independently proves the same validator-before-exchange,
exchange correlation, transcript, and journal invariants. Do not claim that
TOP-E2E-000's proof covers this duplicate implementation without one of those
changes.

### E2E001-SEC-003 — MEDIUM — No negative bridge test demonstrates validation before exchange

The only Flutter scenario requests the valid value `45`. TOP-E2E-000 proves
zero exchanges for an invalid/read-only profile in its own all-at-once helper,
but that test does not exercise `SimulatedParameterEditSession`. A regression
that calls the private exchange before rejecting an out-of-range request in
`begin_edit` would leave the focused Flutter test green. The source currently
orders validation before exchange, but ADR-0005 makes that ordering a bridge
security contract, not just an implementation detail.

Required correction: add a bridge-session negative case for stored `101` (and,
if the session becomes profile-selectable, a contradictory read-only profile)
that observes an error and zero exchange attempts. The public observation may
be a typed exchange counter/error summary; it must not expose payload bytes or
a transport handle.

### E2E001-A11Y-004 — BLOCKING — Broad `SEMANTICS_VERIFIED` claim has already failed independent review

The independent accessibility review in
`.tdd/evidence/TOP-E2E-001/accessibility-review.md:6,72-127,155-164` is
`REVIEW_FAILED`. It identifies missing value/unit/range/context semantics,
deterministic focus and keyboard/switch behavior, semantics-level announcement
assertions, truthful disabled/error states, and large-text/reduced-motion
coverage. The harness does provide a bounded real-FFI label/pending-text slice,
but that does not satisfy the packet/ADR's broad `SEMANTICS_VERIFIED` claim.
Keep only a narrowed bounded L4 claim (if the packet is amended) until a new
accessibility cycle is reviewed.

## Historical initial verification-label audit (superseded by correction re-review)

- `SIMULATOR_VERIFIED`: TOP-E2E-000 remains independently integrated at its
  Rust composition layer. This candidate's direct staged FRB path is only
  source-trace/green-test evidence until E2E001-CORE-001 and CORE-002 are
  corrected; do not promote the candidate's claim as if it reused E2E-000.
- `SEMANTICS_VERIFIED`: unavailable for the packet's current broad claim;
  independent accessibility review is `REVIEW_FAILED`. A bounded L4
  labels/pending-text claim is supportable only after packet claim amendment.
- `BYTE_FIXTURE_VERIFIED`: unavailable; no vendor/provenance bytes are used.
- `FFI_VERIFIED`: the real host FFI route was exercised by the focused rerun,
  but the packet does not earn a native-platform claim; keep the packet's
  declared unavailable platform labels.
- `PLATFORM_DEVICE_VERIFIED`: unavailable.
- `HARDWARE_VERIFIED`: unavailable.

## Historical initial required disposition (superseded by correction re-review)

Do not integrate or mark TOP-E2E-001 complete from the initial candidate. Preserve the
valid amended RED, codegen, release-build, and all-zero sweep evidence. Amend
the packet or implementation to close CORE-001/CORE-002 (and SEC-003), obtain
the required accessibility decision for the claimed semantics label, then
rerun the focused test and all required sweeps in the integration worktree.

## Correction re-review (final)

Reviewed the 12:15 amendment, correction RED/GREEN/read-only evidence, current
Rust and generated FRB source, current Flutter test, ADR-0005, E2E-000's
composition/test/evidence, and the correction-cycle accessibility review. No
production, test, or generated source was edited by this reviewer; this review
record is the only file changed.

### Independent command reruns

Each command below was rerun by this reviewer from the stated working
directory and exited `0` unless noted. The accidental first read-only command
attempt from the repository root was a shell working-directory error and is
not used as evidence; the exact command was rerun from `apps/mobile_flutter`
and passed.

| Layer | Command | Exit |
|---|---|---:|
| FRB code generation | `cd apps/mobile_flutter && flutter_rust_bridge_codegen generate --config-file flutter_rust_bridge.yaml` | 0 |
| Release bridge | `CARGO_TARGET_DIR=crates/topology_bridge/target cargo build -p topology-bridge --release` | 0 |
| Focused staged FFI | `cd apps/mobile_flutter && flutter test test/core/bridge/simulated_parameter_edit_test.dart --plain-name "simulated edit confirmation and undo complete end to end"` | 0 |
| Focused negative FFI | `cd apps/mobile_flutter && flutter test test/core/bridge/simulated_parameter_edit_test.dart --plain-name "read-only synthetic rejection crosses FFI without exchange"` | 0 |
| Rust workspace | `cargo test --workspace` | 0 |
| Flutter package | `cd apps/mobile_flutter && flutter test` | 0 |
| Required local gate | `bash scripts/ci-local.sh` | 0 |

The correction evidence independently records RED `1`, both focused GREEN
commands `0`, codegen/release completion, and the required cargo/flutter/CI
sweeps all `0` (`red-correction-exit-status.txt`,
`green-correction-exit-status.txt`,
`green-readonly-correction-exit-status.txt`,
`sweep-*-correction*`).

### Current behavior and boundary audit

- The amended Flutter test calls `RustLib.init()` through `_ensureRustInitialized`
  (`apps/mobile_flutter/test/core/bridge/simulated_parameter_edit_test.dart:314-328`),
  never `RustLib.initMock`, and invokes generated opaque-session methods. Its
  `_runReal` wrapper uses `tester.runAsync` and rejects a null result
  (`:342-350`), so this is a real host Rust–Dart FFI route rather than a Dart
  fake or a widget-only simulation.
- The generated session API exposes only the two typed factories, opaque
  `beginEdit`/`confirmEdit`/`beginUndo`/`confirmUndo`/`initialState` methods,
  typed error DTOs/enums, and typed state/transcript DTOs
  (`apps/mobile_flutter/lib/core/bridge/generated/api/simulated_parameter_edit.dart:12-221`).
  `SyntheticPayload`, `ScriptedTransport`, and `synthetic_payload` are private
  Rust implementation details (`crates/topology_bridge/src/api/simulated_parameter_edit.rs:437-474`).
  The generated Rust codec's opaque `Vec<u8>` machinery is FRB internals, not a
  public session field or send/endpoint operation. No raw bytes, transport
  handles, endpoint opener, or arbitrary send surface is introduced.
- Rust owns the ordering: `begin_edit` validates both confirmed and requested
  values before journal staging and before any exchange
  (`api/simulated_parameter_edit.rs:163-193`); `confirm_edit` validates,
  exchanges, then confirms the Journal entry (`:195-235`); `begin_undo` stages
  `PendingUndo` without consuming the completed entry (`:238-264`); and
  `confirm_undo` prepares, validates, exchanges, confirms undo, then clears
  pending state (`:266-307`). The private exchange increments a counter and
  appends typed Request/Confirmed entries only after the scripted response
  correlates (`:437-470`). This is a truthful validator → private synthetic
  simulator → Journal path, with no protocol-byte or hardware assertion.

### Direct staged assertions (observed)

The corrected happy-path test now directly observes typed state at every
boundary (`apps/mobile_flutter/test/core/bridge/simulated_parameter_edit_test.dart`):

- Initial state has `exchangeCount == 0` and an empty transcript (`:24-35`).
- Releasing the edit action yields `PendingEdit` with zero exchanges, an empty
  transcript, and no completed journal entry; controls are disabled before the
  confirmation gate is released (`:91-118`). Thus a mutant that exchanges or
  confirms before the gate would fail this direct FFI observation.
- Edit confirmation yields one exchange, typed transcript values `[45, 45]`,
  and journal prior/new `30/45` with one completed entry (`:120-144`).
- `PendingUndo` reports exchange count still `1`, transcript `[45,45]`, and the
  confirmed journal entry still present (`:146-161`). The entry is not consumed
  merely by staging undo.
- Undo confirmation yields exactly two exchanges and typed transcript values
  `[45,45,30,30]`, then reports zero completed entries and final `3.0`
  (`:162-184`). The transcript DTO carries a typed `Request`/`Confirmed` kind
  (`api/simulated_parameter_edit.dart:192-220`); this direct assertion checks
  its ordered semantic values, while E2E-000 separately checks the exact kinds.

The negative test crosses the same generated boundary, not E2E-000's helper:

- The explicit read-only factory returns typed initial read-only state with
  zero exchanges/empty transcript, and `beginEdit(45)` returns
  `SimulatedParameterEditErrorCode.readOnly` with `exchangeCount == 0`
  (`simulated_parameter_edit_test.dart:220-250`).
- A separate writable session's invalid `beginEdit(101)` returns
  `SimulatedParameterEditErrorCode.outOfRange` with `exchangeCount == 0`
  (`:252-270`). This closes the original bridge negative-path proof gap.

### Session duplication and non-circularity decision

`SimulatedParameterEditSession` imports and privately orchestrates the validator,
`ScriptedTransport`, and `Journal` itself (`api/simulated_parameter_edit.rs:8-19`)
instead of calling E2E-000's all-at-once
`compose_synthetic_parameter_edit` (`crates/topology_bridge/src/simulated_edit.rs:241-359`).
That duplication is a maintenance/drift risk and should not be described as
reuse of E2E-000's proof. It is not a current correctness blocker after the
amendment: the session exposes staged boundaries that the all-at-once helper
cannot provide, and the direct generated-FFI assertions above independently
observe the session's own exchange count, typed transcript values, journal
retention, validation errors, and final ordering. The corrected path therefore
truthfully crosses all requested contracts, while E2E-000 remains an adjacent
Rust composition proof rather than a substitute for this path.

### Findings after correction

#### E2E001-CORE-001 — CLOSED_BY_CORRECTION — former missing staged exchange/journal proof

The direct typed assertions at `simulated_parameter_edit_test.dart:91-184`
close the former high-severity gap: no exchange/transcript before confirms,
one edit exchange, retained journal during pending undo, exact two-exchange
`45/45/30/30` transcript, and post-confirm journal consumption are all
observed through real FFI. The test remains intentionally byte-free.

#### E2E001-SEC-003 — CLOSED_BY_CORRECTION — former missing bridge negative proof

The read-only and out-of-range fixtures at `:220-270` exercise this exact
session and assert typed rejection codes with zero exchange attempts. The
source validator returns before journal/exchange work (`api/simulated_parameter_edit.rs:388-435`).

#### E2E001-CORE-002 — NON-BLOCKING ARCHITECTURE RISK — independent session composition

The implementation still duplicates E2E-000's composition instead of calling
its all-at-once helper. This would have remained a proof gap without direct
session assertions; the correction now supplies those assertions and the
source trace. Preserve the distinction in future evidence and refactor only in
a separately scoped packet if shared composition is desired.

#### E2E001-A11Y-004 — BLOCKING — broad semantics claim still lacks independent approval

The correction-cycle accessibility review remains `REVIEW_FAILED` for the
packet's unchanged broad `SEMANTICS_VERIFIED` label
(`.tdd/evidence/TOP-E2E-001/accessibility-review.md:1-16,79-164`). The
corrected harness now emits/observes bounded role/action/name, synthetic
target metadata, unit/range/step/precision text, pending/confirmed text,
disabled/read-only/error state, and non-color state
(`simulated_parameter_edit_test.dart:68-118,186-217,290-310`). However, its
single `sendKeyEvent(Tab)` does not inspect focused semantics nodes, traversal
order, activation, or focus recovery; `liveRegion: true` is source intent but
no emitted flag/announcement is asserted; and phase/value/context/error
coverage is partial across transitions. The amendment explicitly says
`claim_changed: false` (`work-item.yaml:165-175`), and large-text/reduced-motion
are intentionally unclaimed, so the independent accessibility rejection still
blocks the packet's declared claim.

### Verification-label audit after correction

- `SIMULATOR_VERIFIED`: bounded candidate evidence is now present for this
  direct synthetic path (explicit simulator identity plus two typed exchanges),
  but packet completion still requires the independent accessibility gate and
  parent integration rerun.
- `SEMANTICS_VERIFIED`: unavailable for the packet's broad claim; only the
  bounded emitted Flutter L4 facts listed above are supported.
- `BYTE_FIXTURE_VERIFIED`: unavailable; no vendor/provenance bytes are used.
- `PLATFORM_DEVICE_VERIFIED` and `HARDWARE_VERIFIED`: unavailable; no native
  accessibility service, platform device, or modeler hardware was exercised.
- No protocol, physical-unit, native-transport, or arbitrary-send claim is
  earned. The session's `synthetic stored units` label remains explicitly
  synthetic.

### Final required disposition

`REVIEW_FAILED`. Do not mark TOP-E2E-001 `INTEGRATED` or promote its broad
`SEMANTICS_VERIFIED` claim from this candidate. Preserve the valid correction
RED/GREEN, codegen, release, direct FFI, and all-zero sweep evidence. To clear
this review, either (a) make a parent-owned packet amendment that narrows the
claim to the bounded emitted semantics actually proven and obtains a fresh
accessibility approval, or (b) run another strict-TDD accessibility correction
that asserts focused-node/order/recovery behavior, emitted live-region state,
complete phase/value/context/error labels, and keyboard activation, then rerun
the focused test and all required sweeps in the integration worktree.
