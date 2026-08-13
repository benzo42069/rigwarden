# TOP-A11Y-001 independent accessibility review

reviewer: `/root/a11y001_review` (`topology_accessibility_reviewer`, OpenAI `gpt-5.6-luna` / `max`)
reviewed_at: `2026-08-13`
candidate: shared-worktree candidate at `536d8901ac91ecdbc15e09356800d9f46be401dd` plus the packet-scoped uncommitted route files
decision: `REVIEW_APPROVED` for the exact bounded Flutter L4 claim below

## Decisive scope

The candidate is approved only for a synthetic, Rust-authored serial snapshot
(`Input 1 -> Drive 1 -> Output 1`) rendered as a Flutter semantics route list.
The approved claim is that, at Flutter L4, the generated bridge snapshot is
consumed without Dart graph reconstruction; the three nodes are exposed in the
Rust traversal order; node labels include incoming/outgoing connection context;
and each snapshot connection is independently focusable with a button role,
connection name, and `Remove connection` custom action callback. The callback
receives the connection and does not mutate graph or hardware state. This is
the packet's fixture-only behavior, not a complete live route editor.

## Evidence and independent reruns

The strict-TDD record is valid. The accepted RED reaches the focused Flutter
test and fails only because `AccessibleRouteList` and its production file are
absent. The two earlier harness errors and the direct-init stall are preserved
and explicitly rejected in their own evidence; they are not used as RED proof.
The accepted GREEN and all packet sweeps are recorded with per-command status
0. I independently reran each required command from
`apps/mobile_flutter`:

| Command | Exit | Result |
|---|---:|---|
| `flutter test test/features/routing/accessible_route_list_test.dart --plain-name "serial route is completely navigable without canvas"` | 0 | Focused semantics test passed (`+1`). |
| `flutter test test/features/routing/` | 0 | Routing directory test passed (`+1`). |
| `flutter analyze` | 0 | `No issues found!`. |

The focused test calls `RustLib.init()` and
`readFixtureSerialRouteSnapshot()` in `WidgetTester.runAsync`; it does not use
`RustLib.initMock`, a mock bridge, or a hand-built Dart graph. The source only
formats the typed `SerialRouteSnapshot`; traversal and connection context come
from the integrated FRB/Rust route snapshot. Static inspection found no
`Color`, canvas, cable-position, gesture, graph traversal, graph mutation, or
hardware-write implementation in the candidate. The test invokes every
connection's custom action through the semantics owner and asserts the
callback receives exactly the snapshot connections.

## Prioritized findings and contract boundaries

### P0 — full nonvisual-editor requirements remain unproven

`docs/ACCESSIBILITY.md`, `A11Y-002`, `A11Y-003`, and `GRAPH-010` require a
complete structured route/editor equivalent. This candidate intentionally
covers only a serial fixture. It has no split/merge descriptions, row/column
or block metadata, bypass/channel/invalid/disconnected state, validation
errors, add/move/connect actions, actual remove mutation, search/direct jump,
or pending/confirmed/error mutation announcements. Those omissions match the
packet's explicit non-goals and are not a reason to reject this bounded packet,
but the parent must not advertise this candidate as the complete route editor
or as satisfying the full beta nonvisual task suite. The required follow-up
(`TOP-E2E-001`) and later graph/accessibility packets remain release gates.

### P1 — platform input, scaling, motion, and announcement proof is unavailable

The widget test establishes framework semantics only. It does not exercise a
physical VoiceOver/TalkBack service, keyboard traversal, switch control, or
external input; no large-text/reflow sweep or reduced-motion policy test is
present; and there is no live mutation/error event from which to verify a
screen-reader announcement. The static `ListView`/`Text` implementation has no
animation or color-only state, but code inspection is not proof of those
platform behaviors. Keep `PLATFORM_SIMULATOR_VERIFIED`,
`PLATFORM_DEVICE_VERIFIED`, VoiceOver/TalkBack, keyboard/switch, large-text,
reduced-motion, and announcement claims unavailable.

### P1 — only connection role is explicit

Connection semantics set `button: true` and expose the custom action, and the
test asserts that role/action. The route root and node containers are generic
labelled/focusable containers (no explicit heading/list role), which is
adequate for the packet's node-context description but not evidence of the
full contract's heading/landmark and control-role vocabulary. Do not infer
roles, values, units, ranges, selected/toggled/read-only state, or mutation
status for controls that this fixture does not contain; those are not
applicable here and must be covered by the workflows that introduce them.

### P2 — no post-action state transition by design

The `Remove connection` callback is an affordance boundary only. Actual graph
removal, confirmation/error/partial announcements, focus restoration, and
undo are explicitly out of scope. The callback test proves callback delivery
and no candidate-owned mutation, not a successful nonvisual edit.

## Verification-label audit

Supported after the candidate and its required integration rerun:

- `SEMANTICS_VERIFIED` — **bounded Flutter L4 only**, for the synthetic
  Rust-authored serial snapshot and the labels/order/context/button/custom
  action assertions listed above.

Unavailable and not claimed:

- complete `A11Y-002`/`GRAPH-010` nonvisual editor or beta task suite;
- `PLATFORM_SIMULATOR_VERIFIED`;
- physical VoiceOver/TalkBack or `PLATFORM_DEVICE_VERIFIED`;
- physical keyboard, switch-control, or external-input proof;
- `HARDWARE_VERIFIED`, live-device graph compatibility, protocol/transport
  compatibility, or actual connection-removal mutation.

A semantics tree/widget pass is not physical screen-reader evidence. Parent
integration must land only the packet-scoped files, rerun the exact focused
test and required sweeps in the integration worktree, preserve these claim
boundaries, and then update packet/index status. No production or test source
was edited by this reviewer; only this review evidence file was updated.
