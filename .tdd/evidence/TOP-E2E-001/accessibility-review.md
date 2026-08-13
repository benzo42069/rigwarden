# TOP-E2E-001 final accessibility re-review (final correction cycle)

Reviewer: `/root/e2e001_a11y_review` (`topology_accessibility_reviewer`, OpenAI `gpt-5.6-luna` / `max`)
Reviewed: 2026-08-10
Candidate: shared worktree at `536d8901ac91ecdbc15e09356800d9f46be401dd` (final accessibility correction; no source edits by this reviewer)
Decision: `REVIEW_APPROVED` for the declared bounded Flutter `SEMANTICS_VERIFIED` claim only

## Decisive verdict

The final correction closes the previously recorded emitted-semantics gaps at
the test-owned Flutter L4 boundary. The focused tests now inspect emitted
`SemanticsNode` data and actual `FocusNode` identity, not only source intent.
This review approves the packet's bounded semantic claim for the deterministic
synthetic harness. It does not promote the result to a native-platform,
physical screen-reader, switch-control, complete-editor, protocol, or hardware
claim.

## Independent verification

I independently reran these commands from `apps/mobile_flutter`; each exited
`0`:

- `flutter test test/core/bridge/simulated_parameter_edit_test.dart --plain-name "simulated edit confirmation and undo complete end to end"`
- `flutter test test/core/bridge/simulated_parameter_edit_test.dart --plain-name "emitted semantics retain focus order, keyboard activation, and recovery"`
- `flutter test test/core/bridge/simulated_parameter_edit_test.dart --plain-name "read-only synthetic rejection crosses FFI without exchange"`
- `flutter test test/core/bridge/simulated_parameter_edit_test.dart --plain-name "out-of-range typed error renders through FFI semantics"`
- `flutter test test/core/bridge/simulated_parameter_edit_test.dart` (all four tests, exit `0`)

The final-cycle evidence also records release bridge build, FRB codegen,
workspace Cargo tests/format/Clippy, full Flutter tests/analyze/format, and
`bash scripts/ci-local.sh` all exiting `0` (`a11y-final-*`). The observed
strict-TDD mutation records are negative as required: removing focused flags,
the live-region flag, or error rendering makes the focused test fail with exit
`1` (`a11y-mutation-red-focus.log`, `a11y-mutation-red-live.log`,
`a11y-mutation-red-error.log`, `a11y-mutation-red-error-guard.log`, and the
read-only mutants); the candidate was restored before GREEN.

## What is verified at Flutter L4

- **Real FFI, no mock.** Every focused scenario calls `_ensureRustInitialized`,
  which invokes `RustLib.init()`, then uses generated opaque-session factories
  and methods. No `RustLib.initMock` or Dart fake is used. The generated module
  exposes typed state/error DTOs only; no transport bytes, handles, or arbitrary
  send surface crosses the bridge.
- **Emitted name, role, action, and availability.** `getSemantics` observes
  `button == true`, the edit tap action when enabled, and the undo tap action
  absent when disabled. Labels/hints name the synthetic target (`Amp 1 gain`)
  and action. Pending edit disables both controls; read-only disables edit and
  removes its tap action; the final no-journal undo is disabled.
- **Target/context and numeric metadata.** The Rust DTO independently asserts
  target `amp-1/gain`; the emitted state node is checked for the full synthetic
  context `synthetic preset / Amp 1 / gain`, current value, literal unit
  `synthetic stored units`, range `0.0 to 10.0`, step `0.1`, and precision `1`.
  `_expectFullStateSemantics` checks that complete label at initial, pending
  edit, confirmed edit, pending undo, confirmed undo, read-only, and error
  states.
- **Truthful pending/confirmed state.** The real FFI flow checks pending edit
  and pending undo before confirmation (zero/new exchange and retained journal
  invariants), then confirmed edit/undo and exact journal/transcript state. The
  semantic label and visible text carry the matching phase/value; no optimistic
  confirmed state is accepted.
- **Read-only and typed failure state.** The read-only session rejects a write
  with typed `readOnly` and zero exchanges, and the widget emits disabled
  semantics plus a `Read-only` error label/text. A real writable request for
  stored value `101` crosses FFI, returns typed `outOfRange` with zero
  exchanges, and is fed through the harness error-state callback; the emitted
  live label and visible error text contain the exact rejection message.
- **Live status and announcements at the framework boundary.** The state node
  emits `isLiveRegion == true` and the full phase/value/context/unit/range/step/
  precision/error label on every tested state. This verifies Flutter live-region
  semantics intent and mutation sensitivity, not a captured spoken announcement.
- **Focus order, keyboard activation, and recovery.** The final test observes
  actual `FocusNode` identity and emitted `isFocused` flags, requests edit focus,
  traverses with Tab to undo in declared order, activates undo with Enter, and
  verifies focus returns to edit after confirmation. Enter activation of edit is
  also verified. Focus flags and live-region mutations have independent RED
  evidence.
- **Non-color communication.** Phase, value, error, action availability, and
  simulator status are conveyed with text and semantics labels/flags; no tested
  state relies on color, halo, cable, image, or spatial position.

## Findings and claim boundaries

No blocking finding remains for the bounded Flutter semantics claim. The
following are intentionally unavailable and must not be carried into a broader
release statement:

- **Native/physical accessibility:** no iOS/Android platform integration,
  VoiceOver, TalkBack, or physical screen-reader announcement task was run. A
  semantics-tree result is not physical proof.
- **Switch and external input:** Tab/Enter keyboard behavior is verified only
  in the Flutter framework harness. Switch-control, external-input, and native
  keyboard behavior remain unverified.
- **Visual variants:** large-text scaling/reflow, clipping behavior, visible
  focus styling, and reduced-motion behavior have no test in this static
  harness. They are explicitly not claimed, not silently inferred from the
  semantic pass.
- **Complete nonvisual editor:** this is a test-owned one-parameter synthetic
  harness, not the production editor. Search/direct jump, routing and scenes,
  disconnect recovery, device verification, and the full blind-user beta task
  suite remain deferred to the production accessibility work (for example
  `TOP-A11Y-001`).
- **Transport/device claims:** no protocol-byte, native transport, modeler, or
  hardware verification is implied; the packet's unavailable
  `BYTE_FIXTURE_VERIFIED`, `PLATFORM_DEVICE_VERIFIED`, and
  `HARDWARE_VERIFIED` labels remain unavailable.

## Verification-label decision

`SEMANTICS_VERIFIED` is approved only at Flutter L4 for the emitted semantics
and focus behavior listed above, with the synthetic/non-production boundary
shown explicitly. `PLATFORM_SIMULATOR_VERIFIED`,
`PLATFORM_DEVICE_VERIFIED`, VoiceOver/TalkBack, switch-control, large-text,
reduced-motion, complete nonvisual-editor, byte-fixture, and hardware labels
are not approved by this review.

Prior `REVIEW_FAILED` records below are preserved as historical findings for
the pre-correction candidates; they do not describe the current emitted
semantics after the final correction.

---

# TOP-E2E-001 correction-cycle accessibility re-review (12:15 amendment)

Reviewer: `/root/e2e001_a11y_review` (`topology_accessibility_reviewer`, OpenAI `gpt-5.6-luna` / `max`)
Reviewed: 2026-08-10
Candidate: `536d8901ac91ecdbc15e09356800d9f46be401dd` (shared worktree; correction source/test frozen)
Decision: `REVIEW_FAILED` for the packet's broad `SEMANTICS_VERIFIED` claim

## Correction-cycle scope and verdict

The 12:15 amendment added profile-derived semantic metadata, typed read-only/
error state, action availability, staged exchange/journal assertions, and
Flutter semantics-node assertions. Those corrections are present and the
focused real-FFI tests pass. They close the prior P0 gaps for basic role/action,
synthetic target metadata, disabled controls, and the read-only fixture.

The broad claim still cannot be approved. The corrected test does not actually
assert emitted focus/recovery behavior, a live-region flag/announcement, or the
complete value/context/state labels at every pending/confirmed transition. A
source `FocusTraversalGroup`, a `liveRegion: true` declaration, or a visible
`Text` assertion cannot substitute for those emitted semantics checks. This is
an L4 evidence boundary, not physical VoiceOver/TalkBack proof.

Large-text/reflow and reduced-motion variants are explicitly out of scope for
this static, test-owned harness and are not claimed. The complete production
nonvisual editor remains deferred by ADR-0005.

## Independent verification

- Exact focused happy-path command, run from `apps/mobile_flutter`, passed:
  `flutter test test/core/bridge/simulated_parameter_edit_test.dart
  --plain-name "simulated edit confirmation and undo complete end to end"`
  (exit 0).
- Exact focused read-only/error command also passed:
  `flutter test test/core/bridge/simulated_parameter_edit_test.dart
  --plain-name "read-only synthetic rejection crosses FFI without exchange"`
  (exit 0).
- The correction evidence records release-library rebuild and FRB generation
  exit 0 (`release-build-correction-exit-status.txt`, `codegen-exit-status.txt`),
  plus all required correction sweeps exit 0. No mock initializer is used;
  `_ensureRustInitialized` calls `RustLib.init()` (`:314-327`).
- No physical iOS/Android platform, VoiceOver/TalkBack service, switch-control
  task, or modeler hardware was used. No screenshot, golden, or semantics tree
  is treated as physical proof.

## What the correction now proves at Flutter L4

- **Real FFI and typed boundary:** The generated API creates writable and
  explicitly read-only opaque Rust sessions (`api/simulated_parameter_edit.dart:12-39`)
  and exposes typed state/error DTOs only (`:42-141`). The focused tests invoke
  these through `RustLib.init()` and `tester.runAsync`, not `RustLib.initMock`.
- **Names, roles, and actions:** `getSemantics` observes `button == true`, tap
  action, labels, and enabled state for the edit control; the final undo node
  is also a button with no tap action when disabled (`simulated_parameter_edit_test.dart:80-88,
  :186-206`). Labels include `Amp 1 gain`; the enabled edit hint includes the
  action text (`:195-197`).
- **Target/context and profile metadata:** Rust supplies `target`, synthetic
  `context`, `unit`, converted minimum/maximum, stored/display step, and
  precision (`api/simulated_parameter_edit.rs:87-109,309-335`). The emitted
  state label includes unit, range, step, and precision, and the test observes
  those strings (`simulated_parameter_edit_test.dart:72-79`). This is synthetic
  metadata only; it is not a physical parameter-unit claim.
- **Pending/confirmed and journal truth:** The test checks pending edit with
  zero exchanges/transcript, confirmed edit with one exchange and exact journal
  prior/new values, pending undo with the journal retained, then confirmed undo
  with two exchanges and `[45,45,30,30]` transcript values
  (`simulated_parameter_edit_test.dart:91-178`).
- **Read-only/error and disabled availability:** The read-only fixture crosses
  FFI, returns typed `readOnly` rejection with zero exchanges, disables the edit
  node and removes its tap action, exposes a `Read-only` state label, and shows
  the error text (`simulated_parameter_edit_test.dart:220-310`). Pending edit
  disables both controls (`:99-118`).
- **Non-color boundary:** State and availability are communicated with text,
  semantic labels, hints, and enabled/action flags; no status depends on color,
  halo, cable, or image. The explicit simulator label and no-byte/no-vendor
  checks remain (`:179-184`).

## Remaining findings

### P0 — Broad `SEMANTICS_VERIFIED` remains unsupported by emitted focus/live-state evidence

The amendment required focus order or recovery and live status at the
test-owned harness boundary. The harness has an `OrderedTraversalPolicy` and
two `FocusTraversalOrder`s (`simulated_parameter_edit_test.dart:448-481`), but
the only keyboard check sends one Tab and then merely finds the edit label
(`:215-217`). It does not inspect `SemanticsData.focused`, identify the focused
node, walk the declared order, activate via keyboard, or verify focus survives
the pending/confirmed `setState` updates. There is no focus-restoration or
recovery assertion. Likewise, the source sets `liveRegion: true` (`:483-497`),
but no test asserts the emitted live-region flag or a status announcement; it
only checks selected label text (`:99-105,207-212`). Source intent and a
semantics label are not a screen-reader announcement proof.

This leaves deterministic nonvisual navigation, keyboard activation/recovery,
and mutation announcement claims unavailable at the required evidence level.

### P1 — Emitted semantic coverage is partial across values/context/phases/error

The emitted state node is checked for unit/range/step/precision, but not for
the current numeric value at initial/pending/confirmed states. Context is
asserted on the Rust DTO and only the shorter `Amp 1 gain`/`Action: edit`
fragments are asserted on the semantic control; the full synthetic preset/block
context is not asserted in the emitted label/hint. Pending edit and confirmed
undo are inspected semantically, while confirmed edit and pending undo are
checked only as ordinary `Text.data`; the read-only test checks `Read-only` but
not the full error phrase in the semantic label. A regression could therefore
drop part of the spoken value/context/state while these assertions remain
green. The source currently composes the intended full string, but this review
does not promote source-only intent to a complete semantics claim.

### P1 — Keyboard evidence is framework-only; switch/external-input is absent

The single `sendKeyEvent(Tab)` call is not an activation/traversal task and no
switch-control or external-input path is exercised. Even after a stronger
Flutter focus test, native keyboard/switch behavior would remain L5/L7 work.
Keep those labels unavailable.

### P1 — Failed-action announcement is not exercised in the harness

The read-only fixture renders an initial error state and verifies a disabled
edit control. The out-of-range call is tested as a typed FFI rejection, but its
error is not fed through `_edit` into a widget state update or a semantic error
announcement. The harness callbacks do not catch a failed writable action and
render a “Change failed” status. This is enough for the bounded read-only
fixture assertion, not for general mutation failure announcements.

### P2 — Large text/reflow and reduced motion are intentionally unclaimed

No `MediaQuery` scale/reflow or `disableAnimations` test exists, by explicit
correction scope. This is not a defect in the static harness's bounded claim,
but it means no large-text or reduced-motion verification label may be carried
forward to a production editor.

### P2 — Complete nonvisual equivalent remains intentionally deferred

The test-owned harness is not the production parameter editor. Search/direct
jump, editable numeric entry, device/session verification, routing, scenes,
disconnect recovery, and the full beta task suite remain out of scope under
ADR-0005 and must be delivered/reviewed by later packets (`TOP-A11Y-001`, etc.).

## Correction-cycle verification-label audit

- Supported: real host Rust–Dart FFI execution; bounded Flutter L4 emitted
  button role/action/name, synthetic target metadata, textual value metadata,
  pending/confirmed text path, read-only/error fixture, disabled availability,
  and non-color state.
- Not supported: packet-wide `SEMANTICS_VERIFIED` because emitted focus/order,
  keyboard recovery, live-region/announcement, and full phase/value/context
  coverage remain incomplete. The parent may narrow the packet claim to the
  bounded items above or run another focused accessibility cycle.
- Explicitly not claimed: large-text/reflow, reduced motion, physical native
  keyboard/switch behavior, iOS/Android simulator accessibility,
  VoiceOver/TalkBack (`PLATFORM_DEVICE_VERIFIED`), blind-user completion,
  protocol-byte, and modeler/hardware (`HARDWARE_VERIFIED`) claims.

## Disposition

`REVIEW_FAILED` for the current broad accessibility gate. Preserve the valid
correction RED/GREEN, release/codegen, and sweep evidence. Before granting the
packet's `SEMANTICS_VERIFIED` label, either narrow the packet claim to the
bounded emitted semantics listed above or add a final strict-TDD cycle that
asserts focused-node/order/recovery behavior, live-region state, complete
value/context/phase/error labels, and keyboard activation. Native screen-reader
and switch-control verification remains a separate platform-device gate.

---

# Prior candidate accessibility review

Reviewer: `/root/e2e001_a11y_review` (`topology_accessibility_reviewer`, OpenAI `gpt-5.6-luna` / `max`)
Reviewed: 2026-08-10
Candidate: `536d8901ac91ecdbc15e09356800d9f46be401dd` (shared worktree; packet source is frozen)
Decision: `REVIEW_FAILED`

## Scope and verdict

This review covers only the test-owned Flutter harness in
`apps/mobile_flutter/test/core/bridge/simulated_parameter_edit_test.dart`, its
generated typed bridge, and the Rust-owned semantic session permitted by
ADR-0005. No production editor screen is being required from this packet.

The harness does cross the intended Rust path and has a useful bounded L4
semantics slice, but it cannot earn the packet's broad `SEMANTICS_VERIFIED`
claim. ADR-0005 requires the harness to expose semantic name, role,
value/unit/range, pending/confirmed state, action, and deterministic focus
behavior. The current source/test proves only some names/actions and visible
pending/confirmed text. Unit, range, context, focus, large-text, reduced-motion,
error/read-only, keyboard/switch, and native screen-reader behavior are not
proved (several are absent from the harness).

## Verification performed

- The canonical release-library precondition is present and its recorded build
  passed: `CARGO_TARGET_DIR=crates/topology_bridge/target cargo build -p
  topology-bridge --release` (`.tdd/evidence/TOP-E2E-001/release-build-exit-status.txt`, exit 0).
- Recorded FRB code generation passed:
  `flutter_rust_bridge_codegen generate --config-file flutter_rust_bridge.yaml`
  (`codegen-exit-status.txt`, exit 0).
- The focused test calls `RustLib.init()` and the generated
  `createSimulatedParameterEditSession()` (`simulated_parameter_edit_test.dart:14-19`);
  it does not call `RustLib.initMock`. An independent rerun against the release
  dylib with `flutter test --no-pub test/core/bridge/simulated_parameter_edit_test.dart
  --plain-name "simulated edit confirmation and undo complete end to end"`
  passed (exit 0). The exact command without `--no-pub` hit Flutter's generated
  iOS ephemeral cleanup error in this worktree; the packet's recorded canonical
  GREEN remains exit 0.
- The harness drives typed Rust methods through generated FRB and checks the
  pending/confirmed edit and undo values (`:56-104`), exact journal summary,
  explicit synthetic simulator label, and absence of `vendor`/`bytes` text
  (`:105-110`).
- No physical iOS/Android device, VoiceOver/TalkBack service, switch-control
  task, or modeler hardware was used. No screenshot, golden, or semantics tree
  is treated as physical proof.

## Positive findings (bounded L4 support)

- **Real FFI, no mock:** `RustLib.init()` loads the generated bridge; the test
  then invokes the Rust-owned opaque session and its `beginEdit`, `confirmEdit`,
  `beginUndo`, and `confirmUndo` methods. The generated API exposes typed state
  only; it contains no endpoint handle, arbitrary send, or transport bytes.
- **Names and declared roles:** The edit and undo controls are wrapped with
  `Semantics(button: true, label: ...)` (`:207-221`) and the test locates both
  semantic labels (`:52-54`). The labels include the target and requested
  operation (`Edit Amp 1 gain to 4.5`, `Undo Amp 1 gain edit`). The `button`
  flag is source-level evidence only; this test does not inspect the emitted
  role/flags with a semantics-node assertion.
- **Pending/confirmed text path:** The test releases each confirmation gate
  only after observing `Pending edit: 4.5` and `Pending undo: 3.0`, then checks
  `Confirmed edit: 4.5` and `Confirmed undo: 3.0` (`:56-104`). The state region
  is marked `liveRegion: true` and its source label includes phase and value
  (`:223-232`), which is framework-level intent, not a captured announcement.
- **Non-color and simulator boundary:** State is communicated as text/semantic
  phase labels, not color, halo, cable, or image. The explicit simulator label
  (`:235-237`, tested at `:105-110`) keeps this slice from implying a vendor or
  hardware workflow.

## Prioritized findings

### P0 — Required value/unit/range/context semantics are not exposed

ADR-0005 consequence lines 53-55 and `docs/ACCESSIBILITY.md` lines 54-68
require value, unit, minimum/maximum, step/precision, and block/scene context
for the interactive workflow. Rust state contains `display_value`,
`decimal_places`, `min_stored`, and `max_stored` (`crates/topology_bridge/src/api/
simulated_parameter_edit.rs:59-74`), but the Flutter harness never renders or
includes those bounds, a unit, a step/precision description, or session/preset
context. Its live label exposes only the machine target `amp-1/gain` and the
formatted value (`simulated_parameter_edit_test.dart:226-231`). A blind user
cannot discover the valid range or unit from this UI. The profile itself has no
unit metadata, so this is not merely an unasserted label; the required unit is
not available at this boundary.

### P0 — Focus order and keyboard/switch operation are unimplemented/unproved

The harness is a plain `Scaffold`/`ListView` with no
`FocusTraversalGroup`, explicit traversal order, focus restoration, or
focusable state region (`simulated_parameter_edit_test.dart:200-250`). The
focused test taps semantic controls but never traverses with keyboard, checks
focus after a state update, or invokes a switch/external-input action. This
fails the deterministic focus and supported-input requirements in
`docs/ACCESSIBILITY.md:84-96` and the explicit ADR-0005 focus requirement. The
default widget-tree order is not sufficient evidence for a stable nonvisual
workflow.

### P1 — Mutation announcement evidence is only source intent, not a semantics assertion

`liveRegion: true` is present, but the test asserts ordinary `Text.data`, not
the emitted semantic label/flags or an announcement event. No
`SemanticsTester`, semantics-node assertion, platform accessibility inspection,
or announcement cadence test verifies that a screen reader hears pending then
confirmed state. Error announcements are not covered. Therefore the source
supports a bounded L4 intent (`Current parameter amp-1/gain: <phase> <value>`),
not verified mutation announcements or physical VoiceOver/TalkBack behavior.

### P1 — Action availability and failure boundaries are not truthful

Both buttons stay enabled in every phase (`:207-221`). `Undo` is exposed and
operable before any journal entry exists, and both controls remain operable
while an edit/undo is pending. Rust correctly returns typed `Err(String)` for
those cases (`crates/topology_bridge/src/api/simulated_parameter_edit.rs:120-132,
:179-206`), but `_edit`/`_undo` do not catch or render the error (`:158-198`).
An invalid activation therefore has no disabled/read-only/error semantics and
no “Change failed” announcement. The synthetic profile is intentionally
writable (`SessionCapabilities::new(true)`, `VerificationStatus::Experimental`;
Rust `:260-272`), so no read-only profile path is required for the happy-path
scenario; it is nevertheless unverified and must not be claimed.

### P1 — Large text/reflow and reduced-motion behavior are not covered

There is no `MediaQuery` text-scaling/reflow branch, no large-text test, and no
reduced-motion policy or test in the harness. The vertical `ListView` may scroll,
but that does not prove labels/buttons remain usable or unclipped at large text;
default Material interaction animation is also not shown to honor disabled
animations. `docs/ACCESSIBILITY.md:98-108` remains unavailable for this slice.

### P1 — Complete nonvisual equivalent is intentionally deferred, not earned

This test-owned harness provides only two fixed semantic actions and a linear
state transcript. It has no parameter search/direct jump, editable numeric
control, unit/range exposure, device verification/read-only status, routing
representation, error/disconnect recovery, or other beta tasks. ADR-0005
explicitly says the first UI is not a production parameter editor and defers
the full accessibility workflow to a separate work item. Keep the complete
nonvisual-equivalent and blind-user claims unavailable; route them through
`TOP-A11Y-001`/the production editor packets.

## Verification-label audit

- Supported: a real generated Rust–Dart bridge path and the exact synthetic
  pending/edit/undo/confirmation state flow at the recorded Flutter test layer;
  bounded L4 control labels/basic button semantics and textual non-color state.
- Not supported: packet-wide `SEMANTICS_VERIFIED`; value/unit/range/context
  control semantics, deterministic focus, keyboard/switch behavior, truthful
  error/read-only behavior, large-text/reflow, reduced-motion, or verified
  mutation announcements.
- Not run/not available: `PLATFORM_SIMULATOR_VERIFIED` for native accessibility,
  physical VoiceOver/TalkBack (`PLATFORM_DEVICE_VERIFIED`), blind-user task
  completion, protocol-byte, and modeler/hardware (`HARDWARE_VERIFIED`) claims.
  The Flutter semantics tree and a passing widget test cannot upgrade any of
  those labels.

## Required disposition

`REVIEW_FAILED` for the broad accessibility gate. Preserve the existing
strict-TDD RED/GREEN and release/codegen evidence. Before claiming
`SEMANTICS_VERIFIED`, add a bounded accessibility cycle (or amend the claim)
that exposes profile-derived unit/range/precision/context, explicit focus and
supported input behavior, truthful disabled/error/read-only states, and
semantics-level pending/confirmed announcement assertions. A separate L5/L7
device review remains mandatory for native VoiceOver/TalkBack and switch
control; a complete nonvisual editor workflow remains a later packet.
