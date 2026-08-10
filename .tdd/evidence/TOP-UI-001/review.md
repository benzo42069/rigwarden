# Current cycle-3 decision — bounded L4 approval

Reviewer: `/root/ui001_tablet_role_review` (`topology_accessibility_reviewer`, OpenAI `gpt-5.6-luna` / `max`)
Reviewed: 2026-08-10
Candidate commit: `536d8901ac91ecdbc15e09356800d9f46be401dd` (cycle-3 source/test remain uncommitted in this shared worktree)
Decision: `REVIEW_APPROVED` for the exact bounded Flutter L4 shell claim recorded below.

The earlier cycle-1 and cycle-2 `REVIEW_FAILED` records remain below unchanged. This approval does not upgrade any platform-device, VoiceOver/TalkBack, hardware, visual-asset, or complete-editor claim.

# TOP-UI-001 independent accessibility review

Reviewer: `/root/ui001_accessibility_review` (`topology_accessibility_reviewer`, OpenAI `gpt-5.6-luna` / `max`)
Reviewed: 2026-08-09
Candidate commit: `536d8901ac91ecdbc15e09356800d9f46be401dd` (packet source is still untracked/frozen in this worktree)
Decision: `REVIEW_FAILED`

## Scope and decision

The adaptive structure is present and the focused Flutter test is a valid L4
widget/semantics test for the narrow claim it actually asserts: the same
declared labels are exposed in a `BottomNavigationBar` at 390x844 and a
`NavigationRail` at 1024x768. The source uses SDK controls, gives the current
destination a labelled semantics container, and does not add production art.

The packet cannot be approved as submitted because the claimed
`SEMANTICS_VERIFIED` accessibility gate is broader than the evidence and the
strict accessibility contract. The test is a label/structure presence check,
not proof of control actions, focus, announcements, scaling, reduced motion,
keyboard/switch operation, or a complete nonvisual workflow. Integration must
either narrow the claim to adaptive labelled navigation and preserve the
required follow-up gate, or add the missing accessibility tests/behavior in a
new bounded cycle before approval.

## Prioritized findings

### P0 — required accessibility assertions are absent; broad semantics claim is unsupported

`session_shell_layout_test.dart` checks only widget type and a regex prefix for
each destination label. It does not exercise activating a destination, assert
the role/selected state/action, verify deterministic traversal or focus
restoration across the width/orientation change, or assert an announcement when
the current destination changes. `SessionShell` has no explicit focus order,
focus restoration, or live-region/announcement behavior. This fails the
accessibility-contract assertions for names/roles/actions, focus order,
announcements, and keyboard/switch access as evidence requirements. The shell
is structurally usable in the Flutter tree, but the full `SEMANTICS_VERIFIED`
label must not be granted from this test alone.

### P0 — large text/reflow and reduced-motion behavior are unverified (with a concrete SDK risk)

There is no large-text or reduced-motion test. Flutter's `BottomNavigationBar`
clamps item-label text scaling to a maximum factor of `1.0`, and both SDK
navigation widgets animate selection with theme-duration animations; this shell
does not provide an accessible-navigation/reduced-motion policy or a reflow
strategy. Six phone destinations at 390 logical pixels therefore have no
evidence of scalable, unclipped labels, and reduced motion is not established.

### P1 — non-color and nonvisual completeness are only partial

The SDK supplies selected semantics for the navigation controls and the
current destination has a text label, so the candidate is not relying on a
PNG, halo, cable, or color for screen-reader names. However, the phone branch
has no explicit visual selected marker beyond the SDK's label styling, and the
content node is a generic labelled container (no heading/landmark trait in the
frozen source). No test proves a sight-free user can traverse and activate all
destinations. Destination screens and the complete route/parameter equivalent
are explicitly deferred by the packet and remain release-blocking follow-ups;
this review does not treat the placeholder content as a completed editor.

### P1 — required full-suite rerun is not reproducible in this shared worktree

Independent reruns performed:

- Focused command `flutter test test/features/session/session_shell_layout_test.dart --plain-name "session shell adapts without hiding editor destinations"`: **exit 0**.
- `flutter analyze`: **exit 0**, no issues.
- `dart format --output=none --set-exit-if-changed .`: **exit 0**, 29 files checked, 0 changed.
- Required `flutter test`: **exit 1** twice because Flutter 3.44.9 crashed while deleting the generated `build/native_assets/macos/native_assets.json` (`PathNotFoundException`). This is a tool/build-environment failure, not an observed widget assertion failure.
- Supporting `flutter test --no-test-assets`: **exit 0**, all 3 tests passed. The implementer's evidence also contains a prior direct required-sweep pass, but integration must rerun the exact required command serially from the integrated worktree and preserve its status.

## Verification-label audit

- Supported now: **L4 / Flutter widget semantics — adaptive navigation labels and intentional phone/tablet structure only**.
- Not supported: full `SEMANTICS_VERIFIED` for the accessibility contract until the P0 assertions above are covered and the claim is reviewed again.
- Not run/not available: `PLATFORM_SIMULATOR_VERIFIED`, physical VoiceOver/TalkBack (`PLATFORM_DEVICE_VERIFIED`), blind-user task evidence, and all modeler/hardware claims (`HARDWARE_VERIFIED`). A semantics tree, screenshot, golden, or `--no-test-assets` pass cannot upgrade any of these.

## Required disposition

Do not mark TOP-UI-001 `INTEGRATED` under the broad semantics claim from this
review. The parent may amend the packet/claim to the narrow adaptive-labelled
navigation behavior and keep TOP-A11Y-001/TOP-E2E-001 as blocking follow-ups,
or run a new strict-TDD cycle for focus/actions/announcements, large text,
reduced motion, and keyboard/switch paths. After integration, rerun the exact
focused test and every required sweep in a serialized clean worktree.

---

## Cycle-2 final independent accessibility review — `REVIEW_FAILED`

Reviewer: `/root/ui001_accessibility_final` (`topology_accessibility_reviewer`,
OpenAI `gpt-5.6-luna` / `max`)

Reviewed: 2026-08-10

Amendment reviewed: `7859a40632ca18a86ad629b68d621f397609dcdf` (the amended
packet is recorded in `cycle-2-work-item.yaml`). The production source and test
were treated as frozen; no production files were edited.

### Independent verification run

All required Flutter commands were run independently from
`apps/mobile_flutter` against the current frozen worktree:

| Command | Exit | Result |
|---|---:|---|
| `flutter test test/features/session/session_shell_layout_test.dart --plain-name "session shell adapts without hiding editor destinations"` | 0 | Focused amended widget/semantics test passed (`+1`). |
| `flutter test` | 0 | All three Flutter tests passed (`+3`). |
| `flutter analyze` | 0 | `No issues found!`. |
| `dart format --output=none --set-exit-if-changed .` | 0 | 29 files checked, 0 changed. |

I also ran a read-only semantics probe at 1024x768 (the normal tablet branch)
using Flutter 3.44.9. It returned exit 0, but printed the relevant semantics
for every `NavigationRail` destination as `button=false`,
`role=SemanticsRole.none`, `action=true`, with the expected selected state.
The probe was temporary and was deleted; it did not modify production or
checked-in evidence.

### What the amended cycle fixes at L4

- Phone `BottomNavigationBar` destinations expose labels, button role, tap
  actions, and selected state (source `session_shell.dart:224-240`; assertions
  in `session_shell_layout_test.dart:43-60`).
- Static large-text/reduced-motion destinations expose explicit button,
  selected, focus, sort-order, and tap semantics (source
  `session_shell.dart:377-405`).
- The shell adds an ordered focus group and a stable content focus node; the
  test checks policy and content focus restoration across the width change
  (`session_shell.dart:76-98,136-159`; test `:35-42,82-93`).
- The current destination is a live, focusable semantic region and destination
  changes are asserted for Routing and Library (`session_shell.dart:192-207`;
  test `:62-79,163-178`).
- Large text uses a wrapping static layout with soft-wrapped labels and ordinal
  semantic sort keys (`session_shell.dart:309-347`; test `:111-149`).
- Reduced motion selects the static, non-animated branch and the test confirms
  an immediate update with no scheduled frame (`session_shell.dart:71-74,100-134`;
  test `:151-178`).

### Prioritized findings

#### P0 — normal tablet navigation does not expose an explicit role

The packet behavior requires navigation to expose a role in both phone and
tablet layouts. The phone and static fallback branches set `button: true`, but
the normal tablet branch delegates directly to Flutter `NavigationRail`
(`session_shell.dart:245-275`). Flutter 3.44.9's destination semantics in
this branch have a tap action and selected flag but neither the button flag nor
an explicit semantic role. The amended test checks tablet action/selection only
(`session_shell_layout_test.dart:94-108`), so the gap was not caught by the
focused GREEN.

This violates the strict accessibility assertions for control role and leaves
the broad packet claim unsupported. Do not grant `SEMANTICS_VERIFIED` for both
layouts until the normal tablet destination nodes have an explicit role (or the
packet is narrowed to the actual SDK semantics and its claim is amended through
a new RED/GREEN cycle).

#### P1 — focus/keyboard evidence is still framework-level and partial

The source has an ordered traversal group, static destination key handling for
Enter/Space, and semantic tap actions. The test verifies the policy object and
focus restoration, but it does not perform a traversal sequence, send keyboard
events, or exercise switch-control actions for the normal `NavigationRail` /
`BottomNavigationBar` branches. This is a remaining L4 evidence gap, not
physical input proof.

#### P1 — complete nonvisual editor workflow remains out of scope

The current content is a labelled destination placeholder; destination screens,
routing/parameter values and units, mutation confirmation/error states, undo,
tuner, performance controls, and disconnect recovery are explicitly deferred
to follow-up packets. This packet can only earn a narrow shell-navigation L4
claim, not the contract's complete nonvisual equivalent or beta task suite.

### Verification-label audit

- Supported after these reruns: L4 Flutter layout/semantics evidence for the
  adaptive shell, phone/static navigation semantics, live destination label,
  large-text wrapping, and reduced-motion branch.
- Not supported: packet-wide `SEMANTICS_VERIFIED` because the normal tablet
  navigation role is absent; keyboard/switch behavioral proof is partial.
- Not run and unavailable: `PLATFORM_SIMULATOR_VERIFIED`, physical VoiceOver or
  TalkBack (`PLATFORM_DEVICE_VERIFIED`), blind-user task completion, and all
  modeler/hardware claims (`HARDWARE_VERIFIED`). A semantics tree, screenshot,
  golden, or passing Flutter test cannot upgrade those labels.

### Disposition

`REVIEW_FAILED`. Preserve the amended RED/GREEN and sweep logs, add a bounded
cycle that gives normal tablet destinations an explicit semantic role and
exercises keyboard/switch traversal where applicable, then obtain a fresh
independent accessibility review before integration. No production changes
were made by this reviewer.

---

## Cycle-3 final independent accessibility review — `REVIEW_APPROVED` (bounded L4)

Reviewer: `/root/ui001_tablet_role_review` (`topology_accessibility_reviewer`,
OpenAI `gpt-5.6-luna` / `max`)

Reviewed: 2026-08-10

Candidate commit: `536d8901ac91ecdbc15e09356800d9f46be401dd` (cycle-3 packet
files are still uncommitted in this shared worktree)

### Exact claim supported

`SEMANTICS_VERIFIED` at Flutter L4 for the bounded adaptive session shell only:
the six synthetic destinations are reachable in the narrow phone
`BottomNavigationBar` and wide tablet `NavigationRail`; each destination has a
stable accessible name, Flutter button/control semantics, selected state, tap
action, and the same destination callback; the current destination is exposed
as a live semantic label with focus; the declared ordered focus group and
content focus node survive the tested phone-to-tablet width change; large text
uses the wrapping static navigation branch; reduced motion uses the static
non-animated branch. This is not a claim about the destination screens or the
complete editor workflow.

### Independent verification

The exact focused test and every packet-required sweep were rerun serially
from `apps/mobile_flutter`:

| Command | Exit | Result |
|---|---:|---|
| `flutter test test/features/session/session_shell_layout_test.dart --plain-name "session shell adapts without hiding editor destinations"` | 0 | Focused amended L4 test passed. |
| `flutter test` | 0 | All three Flutter tests passed. |
| `flutter analyze` | 0 | No issues found. |
| `dart format --output=none --set-exit-if-changed .` | 0 | 29 files checked, 0 changed. |

Cycle-3 evidence independently contains a valid RED (tablet `isButton=false`)
before the minimum wrapper, GREEN after the wrapper, and final all-zero sweeps;
the earlier format-detection failure remains preserved in its original
evidence.

### Actual semantics probe

A temporary read-only Flutter probe (deleted after execution; no production or
checked-in test file changed) inspected the emitted semantics at 390x844 and
1024x768. It observed:

- Phone nodes: all six labels include the stable destination text (Flutter adds
  `Tab N of 6`), `button=true`, explicit selected true/false, and a tap action.
- Tablet nodes: all six labels have `button=true`, explicit selected true/false,
  and a tap action after cycle 3. Flutter 3.44 reports `SemanticsRole.none` for
  these nodes because the SDK has no separate button enum; Flutter's `button`
  flag is the framework/native control-role hint documented for VoiceOver and
  TalkBack. This is framework evidence, not native-device proof. The prior
  cycle-2 tablet gap was the missing button flag, which the focused RED/GREEN
  corrected.
- Semantic activation was exercised directly on phone and tablet nodes and
  changed the current destination label. After Routing activation, the same
  content focus node remained focused through the 390-to-1024 width change.
- The current-content node emitted `liveRegion=true`, the exact
  `Current editor destination: <label>` text, and focused=true after a
  destination action.
- Selection and current-destination state are exposed through text and
  semantics (`selected`, labels, and live content), not a color, halo, cable,
  or production image; the shell continues to use SDK/zero-size icon tokens.
- Large text (`TextScaler.linear(2)`) selected `_StaticNavigation`, rendered a
  `Wrap` with soft-wrapped, non-clipped text configuration, ordinal semantic
  sort keys, button/tap semantics, and all six labels. No layout exception was
  emitted by the probe or focused test.
- Reduced motion (`disableAnimations: true`) selected `_StaticNavigation`; a
  Library activation updated the live node immediately and left
  `hasScheduledFrame == false`.
- The static branch's Enter-key handler was exercised with a focused Library
  destination and changed the live destination. This is framework keyboard
  evidence only, not physical keyboard or switch-control proof.

Source/test anchors: `session_shell.dart:76-182,192-207,211-283,286-419` and
`session_shell_layout_test.dart:30-183`.

### Prioritized findings and boundaries

#### P1 — physical screen-reader and switch-control behavior remains unverified

No iOS/Android simulator accessibility service, physical VoiceOver/TalkBack,
or switch-control task was run. Semantic actions and one framework keyboard
path were exercised, but they cannot establish native focus announcements,
rotor/local-context actions, or switch-control navigation. Keep those claims
unavailable and require L5/L7 follow-up.

#### P2 — focus-order evidence is policy/recovery-level, not a full traversal task

The candidate exposes `OrderedTraversalPolicy`, explicit static ordinal keys,
and a stable content focus node; the required test and probe verify policy
presence and content-focus restoration across the tested width change. They do
not walk every normal SDK navigation item with real Tab/arrow traversal or
prove order under native switch focus. The bounded claim therefore covers the
declared Flutter policy and tested recovery path only.

#### P2 — current content has a live label but no explicit heading/landmark role

`Current editor destination: <label>` is a focusable live semantic container,
but the shell does not assign a heading or landmark role to the major content
region. That is outside the packet's bounded destination-navigation claim and
should be covered when real destination screens are added.

#### P1 — complete nonvisual editor equivalent remains deferred

Destination screens, routing nodes/connections, parameter values/units/ranges,
pending/confirmed/error hardware states, undo, tuner, performance controls,
and disconnect recovery are outside this packet and remain the required
`TOP-A11Y-001`/`TOP-E2E-001` follow-up scope. This review approves only the
session-shell navigation contract above.

No value, unit, minimum/maximum, precision, or device-state control is present
in this shell, so those control-semantics requirements are not applicable to
the bounded claim and are intentionally unavailable rather than inferred.
Likewise, pending/confirmed/error/read-only and block/scene context states are
not represented until the deferred editor destinations and session-status
packets land.

### Verification-label audit

- Supported: `SEMANTICS_VERIFIED` / L4 for the exact bounded shell claim above;
  cycle-3 RED/GREEN and independent final Flutter sweeps are reproducible.
- Not supported: `PLATFORM_SIMULATOR_VERIFIED`, physical VoiceOver/TalkBack
  (`PLATFORM_DEVICE_VERIFIED`), blind-user task completion, visual asset
  completeness, or any modeler/hardware claim (`HARDWARE_VERIFIED`). A
  semantics tree, screenshot, visual golden, temporary probe, or `--no-test-assets`
  run cannot upgrade those labels.

### Disposition

`REVIEW_APPROVED` for the bounded L4 shell claim only. Parent integration must
land the cycle-3 packet-owned source/test delta, rerun the exact focused test
and all four required sweeps in the integration worktree, and preserve this
review alongside the prior failed records. No production files were edited by
this reviewer.
