# Blind-accessibility test plan

**Packet:** `TOP-RSCH-006`  
**Requirements:** `A11Y-001`, `A11Y-011`, `QA-007`  
**Recorded:** 2026-08-08  
**Status:** `REVIEW_APPROVED` (bounded research only; physical validation remains unavailable)

## Decision

RigWarden must treat accessibility evidence as a ladder, not a single automated
check. Unit tests and Flutter semantics tests are necessary for deterministic
labels, state, value, actions, and focus contracts. They are not evidence that a
real VoiceOver or TalkBack user can complete a task. iOS VoiceOver evidence must
come from a physical iPhone/iPad: Apple explicitly says VoiceOver is unavailable
in Simulator [A1]. Android emulator evidence is useful for semantics and platform
smoke tests, and may run TalkBack when the image has the Android Accessibility
Suite, but it is not a substitute for a physical Android device [G7]. Every advertised
blind workflow therefore needs both a physical screen-reader run and a blind
tester run against the declared modeler/firmware/transport matrix.

No production behavior is created by this packet. The plan is a research input to
future implementation packets and ADRs.

## Truth model and verification labels

Use the following evidence labels from the project accessibility contract. A
higher label never retroactively upgrades a lower one.

| Layer | What it can prove | Label available | What it cannot prove |
| --- | --- | --- | --- |
| L1 / unit | Deterministic formatters, state reducers, debouncing, route descriptions, search, and journal transitions | `UNIT_VERIFIED` | Flutter tree, OS behavior, screen-reader speech, transport, or hardware |
| L4 / Flutter widget + semantics | Widget interaction, semantic name/role/value/unit/range/state/actions, traversal and focus behavior in the Flutter harness | `SEMANTICS_VERIFIED` | Native accessibility bridge, real VoiceOver/TalkBack, physical-device timing, or user success |
| L5 / simulator or emulator | App lifecycle, permissions, platform bridge, keyboard/switch paths, and accessibility-tree inspection on a virtual target | `PLATFORM_SIMULATOR_VERIFIED` | iOS VoiceOver (not available in Simulator), physical screen-reader behavior, modeler transport, or hardware |
| L7 / physical mobile | Real iOS VoiceOver or Android TalkBack navigation and announcements on the supported device build | `PLATFORM_DEVICE_VERIFIED` | A modeler write/read-back, broad population usability, or an untested device/OS/OEM |
| L8 / modeler hardware-in-loop | Blind tester completes the task while a declared modeler, firmware, transport, and adapter perform the safe workflow | `HARDWARE_VERIFIED` for that exact matrix, plus the physical-device evidence | Any other firmware, transport, device, or platform; population-wide claims |

The report must not use `VOICEOVER_VERIFIED` or `TALKBACK_VERIFIED` as a synonym
for semantics snapshots. A physical run may be described as `PLATFORM_DEVICE_VERIFIED`
with the screen reader and device named in the evidence record. A blind-user run
is recorded as task evidence, not as a claim about every blind user.

## Source-derived facts

The following are facts observed in the official documentation listed in the
source register. The access date for all web sources is 2026-08-08.

- **[FACT][F1]** Flutter's Accessibility Guideline API checks framework
  recommendations for target size, text contrast, and target labels. Flutter's
  documented example uses `ensureSemantics` and `meetsGuideline` in a widget
  test. This is a framework-level check, not a screen-reader task test.
- **[FACT][F2]** Flutter documents Accessibility Scanner on Android and
  Accessibility Inspector on iOS as inspection tools. Flutter also documents
  `debugDumpSemanticsTree()`; that tree is presented to system accessibility
  APIs. The dump is diagnostic evidence of the tree, not proof of what a user
  hears.
- **[FACT][F3]** Flutter integration tests can run with `flutter drive` on a
  physical device or emulator and can run on Firebase Test Lab. The test runner
  drives app behavior; it does not, by itself, enable or observe VoiceOver or
  TalkBack speech.
- **[FACT][A1]** Apple's accessibility-testing guidance says to identify main
  tasks, build a device/accessibility matrix, and work through tasks using one
  assistive technology at a time. It explicitly says to install the app on a
  physical device because VoiceOver is not available in Simulator. It recommends
  Screen Curtain for testing the eyes-free experience.
- **[FACT][A2]** Apple's Accessibility Inspector audits common issues such as
  element descriptions, hit regions, contrast, clipped text, traits, and
  Dynamic Type. Apple says passing those audits does not guarantee a fully
  accessible app and still requires testing assistive apps such as VoiceOver.
- **[FACT][A3]** Apple documents `XCUIApplication.performAccessibilityAudit` for
  automating the same class of audits in UI tests. An audit failure is useful
  regression evidence, but it is not VoiceOver task evidence.
- **[FACT][A4]** Apple describes VoiceOver as a gesture-based screen reader that
  provides an auditory description of the interface. The user guide documents
  linear swipe navigation, activation, and Screen Curtain gestures.
- **[FACT][G1]** Android's testing guidance describes TalkBack as the built-in
  screen reader and recommends both linear navigation and explore-by-touch. It
  asks the tester to check meaningful and succinct speech, reachability of every
  element, completion of main workflows, and whether temporary messages are read
  aloud. TalkBack developer settings can display speech output on screen and set
  verbose logging.
- **[FACT][G2]** Android documents a semantics tree used by accessibility services
  and by testing. Custom low-level controls may need explicit semantics; a tree
  that exposes only a whole canvas/control can leave individual content
  inaccessible.
- **[FACT][G3]** Android's Layout Inspector, TalkBack developer settings, and
  Accessibility Suite can inspect/debug semantics. Android recommends testing
  with its assistive technologies to understand the user experience.
- **[FACT][G4]** Android's `AccessibilityChecks` API automates a set of common
  checks in Espresso. The check set is not a complete TalkBack workflow.
- **[FACT][G5]** Google's Accessibility Scanner looks for content labels, touch
  target size, clickable items, and text/image contrast. Google explicitly says
  it is not a replacement for manual testing and does not guarantee app
  accessibility. Its recording contains screenshots, not video or audio.
- **[FACT][G6]** Android's official testing guidance includes user testing and
  suggests recruiting through disability organizations, colleges/universities,
  social networks, or a testing service. RigWarden should use this only as
  recruitment guidance and should not collect medical information.
- **[FACT][G7]** Android's official accessibility codelab runs TalkBack on a
  device or emulator with Android Accessibility Suite and warns that emulator
  TalkBack audio may be low quality; it also notes that setup options vary by
  emulator and Android version. This makes emulator TalkBack useful as a smoke
  check, not physical-device proof.

## Beta task evidence matrix

The task names are the fourteen minimum tasks in `docs/ACCESSIBILITY.md`. Every
row is a release-gated workflow. An empty or skipped layer is not a pass; it is
an unavailable claim or a blocker to the advertised scope.

| ID and beta task | L1 / unit evidence | L4 / Flutter semantics + widget evidence | L5 / simulator/emulator evidence | L7 / physical VoiceOver or TalkBack evidence | L8 / blind tester + real modeler evidence |
| --- | --- | --- | --- | --- | --- |
| A01 Connect or select a device | Endpoint/session state formatter distinguishes connected, ambiguous, offline, and disconnected; selection reducer is deterministic. | Device selector exposes name, model/firmware, transport, verification/read-only state, actions, and focus restoration after discovery. | Exercise permission denial, endpoint enumeration, lifecycle/background/foreground, and reconnect in iOS Simulator/Android emulator or a deterministic simulator; inspect the tree. iOS Simulator cannot supply VoiceOver. | On a physical device, navigate only with the screen reader; select a known endpoint, hear the exact target and connection state, and recover from a failed selection. | Connect to the declared AM4/FM3 fixture/hardware and select it without visual help; confirm the identity/firmware announcement before any write. |
| A02 Understand verification/read-only status | Capability formatter covers `hardware verified`, `community confirmed`, `simulator verified`, `experimental`, `read-only`, and `unsupported`; unknown firmware always reduces write capability. | Status has an accessible name, reason, and actions; read-only and unsupported states are not color-only and are announced on focus and mutation attempt. | Inject unknown firmware and permission/transport failures; assert no write control is exposed or enabled; inspect semantics with Accessibility Inspector/UI Automator. | User can locate the status, hear why it is read-only, and understand that no write will occur. | With a real unknown/unverified device or safe read-only profile, blind tester confirms the status and cannot accidentally transmit a write. |
| A03 Browse and load a preset | Deterministic list ordering, load state machine, and pending/confirmed/error formatter. | Preset list, search, current selection, loading progress, errors, and actions are semantic; focus returns to the loaded preset. | Replay list/read timeout and cancellation; test app lifecycle and file/device fixture on both virtual platforms. | Browse, load, cancel, and retry with VoiceOver/TalkBack; announcements distinguish local/offline from device-confirmed state. | Load a disposable preset from the real modeler, confirm the exact preset identity, and verify no adjacent preset was changed. |
| A04 Describe the complete signal path | Route-description generator emits stable node order, incoming/outgoing connections, split/merge text, bypass/channel state, and validation errors. | Structured route list exposes each node and connection as separate semantic elements with inspect/remove actions; no cable position or color dependency. | Render representative serial/split/merge/disconnected graphs in virtual builds; inspect focus traversal and semantic tree. | Traverse the entire route with linear navigation, locate a node, and hear connection context without seeing the canvas. | Blind tester describes the attached modeler's complete path and identifies a disconnected/invalid branch from announcements alone. |
| A05 Locate a block | Stable block index/search handles duplicate names, instance/channel context, and not-found errors. | Search field, results, direct-jump action, block context, and focus target expose name/type/instance/row/column. | Exercise large lists, keyboard input, orientation, and state restoration on simulator/emulator; inspect that each result is reachable. | Search and jump to a block by name; screen reader focus lands on the block and announces its context. | Locate a real block in a modeler preset and confirm the physical read state before editing. |
| A06 Change a parameter precisely | Formatter/parser enforces min/max/step/precision/unit and emits a mutation with pending/confirmed/failed state; debounced live updates are deterministic. | Parameter control exposes name, role, value, unit, range, step, precision mode, reset/edit actions, and read-only/pending/confirmed/error state. | Test keyboard/switch/semantics actions, large text, rotation, and rapid edits against a simulator; verify no focus jump on live refresh. | Set an exact value using VoiceOver/TalkBack controls, hear pending then confirmed/error feedback, and verify no announcement flood. | Change one bounded parameter on real hardware, hear the confirmation, independently read it back, then restore it. |
| A07 Confirm device acceptance | Reducer distinguishes optimistic, pending, acknowledgement/read-back confirmed, failed, partial, and disconnected states. | Confirmation/error live region is concise and focus-safe; optimistic state is never labeled confirmed. | Replay ack timeout, stale response, partial completion, and reconnect in simulator/emulator; assert announcement order. | Screen reader announces the actual confirmation or failure, including partial count and next action. | Perform the safe read → one bounded write → ack/read-back → independent read sequence on the real modeler; compare UI and hardware. |
| A08 Change scene and channel | Scene/channel value objects and transition reducer preserve valid ranges and current block context. | Scene/channel controls expose selected state, name/index, actions, pending/confirmed/error, and focus restoration. | Exercise orientation, lifecycle, and simulated device rejection; inspect selected state and announcement events. | Select a scene and channel without visual spatial cues; hear the selected value and device confirmation. | Change scene/channel on the real modeler, verify the state after read-back, and ensure no unrelated block changes. |
| A09 Add/remove a supported connection nonvisually | Graph mutation validator and command planner reject invalid routes and produce stable proposed destination/action lists. | Each connection has an accessible action (add/remove/inspect source/inspect destination); split/merge and errors are spoken. | Replay capability filtering, invalid target, cancellation, and partial mutation; inspect the route tree on virtual platforms. | Complete add/remove through lists/actions only; hear proposed destinations and confirmed/error result. | Apply one disposable connection change to the modeler, read back the graph, undo it, and confirm original topology. |
| A10 Undo the change | Undo journal records confirmed prior state, branch/preset identity, and partial failure; reducer is idempotent. | Undo/redo action exposes what will change, target context, pending/confirmed/error, and focus restoration. | Replay undo after disconnect, stale state, and app restart; verify journal recovery and semantic focus. | Undo the last change with the screen reader and hear exactly which parameter/route was restored and whether hardware accepted it. | Undo the real hardware mutation and independently verify the original value/topology after reconnect. |
| A11 Create/use a performance panel | Panel schema validates stable controls, ranges, actions, and concise announcement mode; high-frequency updates are throttled. | Panel controls have semantic names/roles/values, logical order, adjustable actions, and no visual-only meters/halos. | Exercise phone/tablet layout, keyboard/switch traversal, orientation, reduced motion, and update throttling in virtual builds. | Build and use a panel without sight; confirm focus order, adjustable values, and concise versus verbose announcements. | Use the panel while the modeler is attached; make a safe live change, confirm it, and ensure stage controls do not expose unsafe writes accidentally. |
| A12 Use tuner and tap tempo | Tuner formatter emits note/octave/cents/signal/in-tune state; cadence/throttle reducer and tap-tempo timing are deterministic. | Tuner has textual/semantic note, cents, signal, and in-tune state; cadence is user-controlled and updates are not unbounded. | Inject noisy/high-rate tuner data and test debouncing, reduced motion, and lifecycle in simulator/emulator. | Use tuner and tap tempo with VoiceOver/TalkBack; hear controlled updates and a clear no-signal/in-tune result. | Verify tuner/tempo against real audio/modeler behavior; no haptics/audio-only state and no screen-reader flood. |
| A13 Save/export an offline version | Serializer/export reducer preserves device identity, unknown data, history, and error states; path/permission handling is deterministic. | Save/export controls expose destination, format, progress, success/error, overwrite confirmation, and resulting file identity. | Exercise document picker/file permission, cancellation, rotation, and offline mode in both platform virtual targets. | Save/export with the screen reader and hear whether the file is local/offline and whether it completed. | Export the tested real preset after hardware reconciliation; import it offline and compare semantic diff without transmitting. |
| A14 Recover from a disconnect | Session state machine distinguishes disconnect, reconnecting, stale response, partial completion, cancellation, and safe recovery. | Banner/live region announces the transition without stealing focus; retry/reconnect/read-only actions are discoverable. | Inject cable/endpoint loss, app background/foreground, timeout, and reconnection in simulator/emulator; verify no stale optimistic state. | Disconnect during a task and recover using only the screen reader; hear which edits were confirmed, pending, or rolled back. | Unplug/replug the declared modeler/adapter, reconcile state, and confirm journal/undo safety; no blind retry or false success. |

### Matrix interpretation

- A01–A03 establish a safe session and read path. A04–A12 are the core editing
  and performance workflows. A13–A14 cover offline and failure recovery.
- A row is **not beta-pass** until L1/L4 evidence is green, the required virtual
  platform checks are recorded, and both physical screen-reader and blind-user
  evidence exist for each advertised platform/hardware combination.
- The hardware column is conditional for features not supported by a particular
  modeler. The compatibility report must name the profile and mark unsupported
  features rather than silently skipping the task.

## Automation and its limits

| Tool or method | Appropriate automation | Hard limit / claim it must not make |
| --- | --- | --- |
| Rust/Dart unit tests | Formatters, route text, state reducers, search, journal, debounce, capability decisions | No claim about a Flutter semantics tree, native bridge, speech, or hardware |
| Flutter widget + `Semantics` tests | Names, roles, values, actions, focus order, semantics snapshots, Guideline API checks [F1] | The test harness does not prove that iOS VoiceOver or Android TalkBack speaks the intended wording or that a blind user can finish the task |
| `debugDumpSemanticsTree()` / Flutter semantics debugger | Diagnose the tree exported toward system accessibility APIs [F3] | A tree dump is not an audio transcript or platform-device verification |
| Flutter `integration_test` / `flutter drive` / Firebase Test Lab | App-level interaction, lifecycle, and device/emulator coverage [F4] | It does not turn on, observe, or certify VoiceOver/TalkBack; a test can pass while speech, focus timing, or announcements are unusable |
| iOS Accessibility Inspector | Hierarchy inspection and common audits; export an audit report; `performAccessibilityAudit` in UI tests [A2][A3][A6] | Apple states that audits do not guarantee full accessibility [A2]. iOS Simulator cannot run VoiceOver [A1], so the audit cannot be labeled physical VoiceOver evidence |
| Android Layout Inspector / UI Automator / Compose semantics | Inspect native Android View/Compose properties and automate reachable native UI actions [G2][G3] | These are not Flutter-semantics-tree tools and cannot inspect RigWarden's Flutter semantics without an explicit Flutter bridge/test. They are not actual TalkBack speech or blind-user success. |
| Android Accessibility Scanner / Espresso `AccessibilityChecks` | Common native-View labels, target sizes, contrast, and other static checks [G4][G5] | These native Android checks do not establish Flutter semantics. Google says Accessibility Scanner is not a replacement for manual testing and does not guarantee accessibility [G5]; it records screenshots, not audio. |
| iOS Simulator | Flutter layout, semantics, Dynamic Type/reflow, keyboard, and deterministic native bridge tests | No VoiceOver [A1]. Do not claim `PLATFORM_DEVICE_VERIFIED` or VoiceOver completion |
| Android emulator | Virtual lifecycle/permission/semantics tests; TalkBack smoke only when the image includes Accessibility Suite and audio is usable [G7] | Emulator audio/feature fidelity and OEM behavior are not physical-device evidence; do not claim broad TalkBack or hardware compatibility |
| Physical iPhone/iPad + VoiceOver | Real rotor/focus/announcement and eyes-free task completion [A1][A4][A5] | Covers only the tested OS/device/build/settings; still no modeler claim without L8 |
| Physical Android + TalkBack | Real linear/explore-by-touch navigation, speech, and task completion [G1] | Covers only the tested OEM/OS/build/settings; still no modeler claim without L8 |
| Blind tester + physical modeler | User-centered task success plus hardware read/write/read-back/undo/reconnect | A small beta sample does not prove every blind user, language, OEM, firmware, or transport |

## Device and platform matrix

The product contract currently recommends iOS/iPadOS 16+ and Android 10+;
bootstrap must pin exact deployment targets before implementation. **[HYPOTHESIS]**
The device examples below are practical planning candidates, not a claim that
these models are the only supported devices.

| Role | Candidate matrix | Required accessibility configuration | Required transport/hardware coverage | Evidence |
| --- | --- | --- | --- | --- |
| iOS virtual layout target | One iPhone-sized Simulator at the minimum supported OS and one current-stable iPhone-sized Simulator; **one iPad-sized Simulator is mandatory** | Semantics enabled; Accessibility Inspector hierarchy/audit; Dynamic Type, contrast, reduced motion, orientation | Deterministic simulator/replay only | `PLATFORM_SIMULATOR_VERIFIED` for the checked virtual targets; never VoiceOver |
| iOS compact physical | iPhone SE (3rd generation) or the smallest currently supported physical iPhone, pinned by the bootstrap decision | VoiceOver, Screen Curtain for eyes-free pass, speaking rate recorded, rotor defaults recorded [A1][A5] | At least one supported USB/BLE/MIDI path if the phone is in the hardware matrix | `PLATFORM_DEVICE_VERIFIED`; L8 only after modeler run |
| iOS reference physical | Current 6.1-inch iPhone class (for example, the project-owned current iPhone) | Same VoiceOver settings; test portrait and landscape | Same adapter/transport as the compact device where possible | `PLATFORM_DEVICE_VERIFIED`; compare focus and reflow against compact |
| iPad physical | One current iPad class used by the project; include split-screen/landscape | VoiceOver, Dynamic Type, pointer/keyboard where supported | USB-C/adapter path where applicable | `PLATFORM_DEVICE_VERIFIED`; L8 only for declared iPad hardware path |
| Android virtual minimum | AVD pinned to the minimum supported Android API (currently the Android 10+ requirement; exact API is an open bootstrap decision) | UI Automator/Layout Inspector; Android accessibility checks; TalkBack only when Accessibility Suite is present | Deterministic simulator/replay; no modeler claim | `PLATFORM_SIMULATOR_VERIFIED` |
| Android virtual current | Current Google Play AVD pinned by API image and build ID, not an unbounded “latest” tag | Accessibility Suite/TalkBack smoke where available; capture whether speech output is usable | Deterministic simulator/replay; no modeler claim | `PLATFORM_SIMULATOR_VERIFIED`; optional emulator TalkBack notes |
| Android stock physical | One project-owned Google Pixel-class phone at minimum-supported OS and one current OS | TalkBack, Display speech output for sanitized logs, verbose developer logging only for test runs [G1] | USB-C OTG and class-compliant MIDI path; record adapter and cable | `PLATFORM_DEVICE_VERIFIED`; L8 for the exact hardware path |
| Android OEM physical | One current Samsung Galaxy-class phone on One UI, pinned model/OS | TalkBack and same task script; record OEM speech/settings differences [G1] | Repeat the most failure-prone USB/BLE/MIDI path | `PLATFORM_DEVICE_VERIFIED`; L8 only for the exact hardware path |
| Android tablet | One current Android tablet class; test large/reflow and split-screen | TalkBack, large display/font, keyboard/pointer where supported | USB-C/OTG where applicable | `PLATFORM_DEVICE_VERIFIED`; L8 only for declared hardware path |
| Modeler hardware fixture | AM4 and FM3 exact model/variant/firmware from the hardware packets | Run with iOS and Android physical devices above; blind tester operates the UI | Read current state; one bounded write; acknowledgement/read-back; independent read; undo; reconnect; adapter and transport recorded | `HARDWARE_VERIFIED` only for the exact declared matrix |

**Matrix rules**

1. Pin device model, OS build, app build, screen-reader version/settings,
   transport, adapter, and modeler firmware in every run. “iPhone” or
   “Android” alone is not a reproducible matrix entry.
2. Run all L1/L4 checks on every change. Run virtual platform inspection on
   every release candidate. Run physical VoiceOver/TalkBack smoke on every
   release candidate and the complete task suite before beta. Repeat L8 tasks
   after transport/profile changes.
3. A physical device may be shared only if the run records a clean state and
   the exact build/settings. A simulator/emulator pass must never satisfy
   `A11Y-011` or `QA-007` by substitution.
4. If a target device, screen reader, adapter, or modeler is unavailable, record
   the exact evidence gap and keep the advertised feature experimental/read-only;
   do not silently remove a phone or tablet row.

## Focus and announcement logging

### Run protocol

1. Create a disposable preset and safe output level. Close other editors. Read
   and journal the current state before any write. Assign a random run ID and
   redact device serials, account names, local paths, preset names containing
   personal data, and participant identity.
2. Record app commit/build, platform/device/OS, screen-reader version and
   settings, locale, text/display scale, transport/adapter, modeler
   model/variant/firmware, and whether the run is L1/L4/L5/L7/L8.
3. Give the tester the task script and acceptance outcome. The tester performs
   the task with the selected assistive technology. The observer may intervene
   for electrical/output safety, but may not provide navigation hints. On iOS,
   enable Screen Curtain for the eyes-free pass [A1][A5]. On Android, use
   TalkBack linear navigation and explore-by-touch; Display speech output is a
   test-only logging aid, not a substitute for listening [G1].
4. Log every focus move, action, announcement, state transition, and result.
   For live data (tuner/meter/connection state), record cadence and coalescing,
   not every sample. Repeat a failed run once only after preserving the failure;
   repeated passes do not erase a known race or usability defect.
5. For hardware writes, follow the hardware contract: read current value,
   record journal prior state, send one bounded change, receive acknowledgement
   or read-back, confirm in UI, independently read again, undo, and verify the
   original state after reconnect where relevant.

### Sanitized log schema

Store one row per event in `accessibility-results.md` or a machine-readable
sidecar. Keep raw audio/video out of the repository by default.

```text
run_id, task_id, step, timestamp_utc,
platform, device_model, os_build, app_build,
screen_reader, screen_reader_settings,
modeler_token, firmware, transport, adapter,
focus_target, action,
expected_semantics, observed_announcement,
state_before, state_after, hardware_readback,
result, evidence_path, notes
```

`observed_announcement` is a short, manually transcribed quotation or close
transcription, not a claim that a platform always uses identical wording. Keep
the expected semantic content separate from the spoken wording so wording
changes do not hide a missing value/unit/state. For a focus event with no speech,
record `NO_ANNOUNCEMENT` and whether that was expected.

- **iOS:** Accessibility Inspector output and optional audit HTML can support
  tree evidence [A2][A3]. Apple does not provide a general VoiceOver speech
  transcript in this plan; use an observer's timestamped transcription or an
  explicitly consented local audio capture. Screen Curtain is the eyes-free
  check [A1][A5].
- **Android:** TalkBack's developer “Display speech output” setting can expose
  the speech text for a sanitized local transcript, and verbose logs can help
  diagnose focus [G1]. Treat the display/log as diagnostic evidence; the physical
  listener still determines whether speech is understandable and timely.
- **Both:** Do not store participant names, diagnoses, medical history, raw
  voice, unrelated preset content, serials, API keys, or credentials. If an
  audio/video capture is necessary to investigate a defect, obtain explicit
  consent, store it outside the repository with access control, hash the file,
  transcribe only the relevant utterances, and delete it on the agreed date.

### Pass/fail rules for a task event

- `PASS`: the tester can reach the target, hears/reads the expected semantic
  content (name, role, value/unit/range/state/action as applicable), performs
  the action without visual help, and receives truthful pending/confirmed/error
  feedback.
- `FAIL`: any required target is unreachable; focus jumps or traps; state is
  only conveyed by color/position/halo; an optimistic value is announced as
  confirmed; an error/partial/disconnect is silent or misleading; a high-rate
  update floods speech; or a task requires visual canvas interpretation.
- `BLOCKED`: the required physical device, screen reader, adapter, firmware, or
  safe fixture is unavailable. Preserve the reason and do not upgrade the
  verification label.

## Contributor recruitment and feedback privacy

Recruit blind and low-vision musicians through accessible project channels and,
where useful, disability organizations, colleges, or universities [G6]. A direct
maintainer contact must accept accessible email/text feedback without requiring
screenshots or a visual reproduction. Offer a short task script, a plain-text
consent page, an accessible issue template, and a way to withdraw a contribution.

Collect only what is needed to run and interpret the task:

- random participant ID (not a name);
- preferred contact channel, kept in a separate opt-in ledger;
- VoiceOver/TalkBack familiarity (self-described);
- modeler/transport familiarity relevant to the task;
- device/OS/screen-reader details for reproducibility;
- task outcome and requested follow-up.

Do **not** ask for a diagnosis, visual-acuity measurement, medical history,
assistive-device serial, or unrelated demographic data. Participation, payment,
and recording consent must be separate choices. Store contact/consent records
outside the public repository with restricted access. Public issue reports use
the random ID and sanitized run/evidence tokens. A formal human-subjects or
privacy/legal determination is outside this engineering packet and requires
counsel if the project later enters a regulated research program.

## Release blockers

The following are P0 blockers for the affected advertised workflow; they cannot
be waived by a semantics snapshot or a simulator pass:

1. Any A01–A14 task cannot be completed on a supported physical iPhone with
   VoiceOver or supported physical Android device with TalkBack using only the
   nonvisual path.
2. A primary control lacks a name, role, value/unit/range, state, or available
   action; a route or connection is represented only by canvas position/color;
   or search cannot directly focus the requested block/parameter.
3. Focus is trapped, order is nondeterministic, focus is lost after a dialog,
   orientation change, reconnect, or live update, or a mutation steals focus
   without a user-controlled return path.
4. Pending, confirmed, partial, failed, read-only, unknown-firmware, or
   disconnected state is omitted, optimistic, silent, or misleading. Hardware
   acceptance must be based on the profile's acknowledgement/read-back policy.
5. A blind tester cannot complete **every applicable advertised A01–A14 task**
   against the declared AM4/FM3 modeler matrix: connect/select, understand
   status, browse/load, describe path, locate a block, precise change/confirm,
   scene/channel, nonvisual route mutation, undo, performance panel,
   tuner/tap-tempo, save/export, and disconnect recovery. A simulator or fake
   adapter cannot close this blocker.
6. Tuner/meters/live connection updates flood announcements, have no cadence
   control, or expose no equivalent textual/semantic state.
7. Large text/display scaling clips, overlaps, or hides an essential control;
   essential state is only color/halo/motion; reduced-motion or keyboard/switch
   paths regress a primary task.
8. Required physical VoiceOver/TalkBack or blind-user evidence is missing,
   skipped, unrepeatable, or has unsanitized personal data/secrets. Missing
   hardware is an honest `BLOCKED_HARDWARE`, not a pass.

## Facts, hypotheses, unknowns, and current environment

### Contradiction check

No material contradiction was found among the official sources. Flutter and
Android describe semantics/inspection and automated checks as useful lower-level
evidence, while Apple and Android separately require assistive-technology or
user-perspective testing. The only apparent tension is Android emulator
availability versus fidelity: the Android codelab permits a device or emulator
setup but warns about emulator audio/setup variation [G1][G7]. This plan resolves
that by allowing emulator TalkBack only as a smoke check and requiring physical
TalkBack for `PLATFORM_DEVICE_VERIFIED`.

### Verified facts

- The repository accessibility contract requires the fourteen tasks above,
  physical VoiceOver and TalkBack steps, and blind-user physical-device testing
  before beta (`docs/ACCESSIBILITY.md`).
- Product requirements mark `A11Y-001`, `A11Y-011`, and `QA-007` P0; AM4 and FM3
  are separately required to be hardware verified (`docs/PRODUCT_REQUIREMENTS.md`,
  `docs/RELEASE_PLAN_AND_DEFINITION_OF_DONE.md`).
- Official Flutter, Apple, and Android sources support the tool boundaries
  documented in this plan (see source register).
- At research time the starter kit is not a Git worktree. The environment has
  Rust/Cargo and Xcode, but Flutter/Dart, Java/JDK, and Gradle are missing. No
  platform or hardware run was attempted, so this packet earns no platform or
  hardware verification label.

### Hypotheses to test in implementation packets

- Flutter's Semantics output will expose the full structured route and parameter
  contract through the selected iOS and Android plugin/engine versions.
- The planned concise/verbose announcement model and live-update throttle will
  remain understandable at the selected VoiceOver/TalkBack speaking rates.
- The same focus order can be preserved across compact phones, tablets,
  orientation changes, external keyboard/switch input, and reconnects.
- The selected USB/BLE/MIDI adapters preserve enough lifecycle and timing
  information for the L8 blind hardware tasks.

### Unknowns / blockers

- Exact Flutter, iOS, Android, Java, and Gradle versions and deployment targets
  must be pinned by bootstrap; Flutter/Dart and JDK are absent in this extracted
  starter kit.
- Final project-owned device models, OEM/OS build combinations, screen-reader
  versions, locales, and transport adapters are not selected.
- No physical iPhone/iPad, Android phone/tablet, AM4, FM3, or lawful hardware
  fixture is present in this research environment.
- Announcement wording, speech latency, Braille output, and OEM behavior are
  empirical properties; official documentation does not let this plan predict
  them. They must be captured in physical runs.
- Any legal classification of participant recruitment, compensation, or
  recordings beyond ordinary product feedback requires counsel.

## Follow-up packets and decisions

These are proposals, not silently created work items:

1. **ADR-A11Y-001 (proposed):** Adopt the evidence ladder and verification-label
   vocabulary; require physical screen-reader and blind+hardware evidence before
   a beta accessibility claim. Resolve whether any legacy packet labels need
   renaming to the contract's `PLATFORM_DEVICE_VERIFIED` terminology.
2. **TOP-A11Y-002 (proposed):** Define the semantic vocabulary/state machine for
   controls, route nodes/connections, pending/confirmed/partial/error/read-only,
   focus restoration, and announcement cadence. Pair every behavior with L1/L4
   tests before UI implementation.
3. **TOP-A11Y-003 (proposed):** Build the sanitized focus/announcement log
   harness and deterministic fixture runner. Do not capture raw audio by
   default.
4. **TOP-A11Y-004 (proposed):** Execute the iOS physical VoiceOver matrix,
   including Screen Curtain, compact/reference phone, mandatory iPad, and
   Accessibility Inspector audit artifacts.
5. **TOP-A11Y-005 (proposed):** Execute the Android physical TalkBack matrix on
   stock Pixel-class and Samsung-class devices, with Display speech output used
   only as a diagnostic aid.
6. **TOP-A11Y-006 (proposed):** Run the blind-user beta task suite against the
   exact AM4/FM3 modeler/firmware/transport matrix and reconcile hardware logs,
   undo, disconnect, and privacy evidence.

`TOP-A11Y-001` (the existing serial-route implementation packet) correctly lists
physical VoiceOver/TalkBack as not-yet-available claims. Its eventual integration
should consume this plan rather than treating its Flutter semantics test as a
physical screen-reader result.

## Source register (official documentation)

All sources below were accessed 2026-08-08. URLs are recorded directly so a
reviewer can re-open the exact guidance; page publication dates may change.

- **[F1]** Flutter, “Accessibility testing,”
  <https://docs.flutter.dev/ui/accessibility/accessibility-testing>.
- **[F2]** Flutter, “Accessibility,”
  <https://docs.flutter.dev/ui/accessibility>.
- **[F3]** Flutter, “Debug Flutter apps from code” (semantics tree),
  <https://docs.flutter.dev/testing/code-debugging>.
- **[F4]** Flutter, “Check app functionality with an integration test,”
  <https://docs.flutter.dev/testing/integration-tests>.
- **[A1]** Apple Developer, “Performing accessibility testing for your app,”
  <https://developer.apple.com/documentation/accessibility/performing-accessibility-testing-for-your-app>.
- **[A2]** Apple Developer, “Performing accessibility audits for your app,”
  <https://developer.apple.com/documentation/accessibility/performing-accessibility-audits-for-your-app>.
- **[A3]** Apple Developer, “Accessibility Inspector,”
  <https://developer.apple.com/documentation/accessibility/accessibility-inspector>.
- **[A4]** Apple Developer, “VoiceOver,”
  <https://developer.apple.com/documentation/accessibility/voiceover>.
- **[A5]** Apple Developer, “Supporting VoiceOver in your app,”
  <https://developer.apple.com/documentation/uikit/supporting-voiceover-in-your-app>.
- **[A6]** Apple Developer, `XCUIApplication.performAccessibilityAudit(for:_:)`,
  <https://developer.apple.com/documentation/xcuiautomation/xcuiapplication/performaccessibilityaudit%28for%3A_%3A%29>.
- **[G1]** Android Developers, “Test your app's accessibility,”
  <https://developer.android.com/guide/topics/ui/accessibility/testing>.
- **[G2]** Android Developers, “Semantics” (Jetpack Compose accessibility),
  <https://developer.android.com/develop/ui/compose/accessibility/semantics>.
- **[G3]** Android Developers, “Inspect and debug” (Compose accessibility),
  <https://developer.android.com/develop/ui/compose/accessibility/inspect-debug>.
- **[G4]** AndroidX reference, `AccessibilityChecks`,
  <https://developer.android.com/reference/androidx/test/espresso/accessibility/AccessibilityChecks>.
- **[G5]** Google Accessibility Help, “Accessibility Scanner,”
  <https://support.google.com/accessibility/android/faq/6376582>.
- **[G6]** Android Developers, “Test your app's accessibility,” user-testing
  section and recruitment guidance, [G1].
- **[G7]** Android Developers, “Testing for Accessibility” codelab,
  <https://developer.android.com/codelabs/basic-android-kotlin-compose-test-accessibility>.

## Physical-testing gap list (open)

No physical accessibility or hardware evidence exists in this research
environment. Before public beta, the integration owner must close each item and
attach sanitized evidence to the relevant packet:

- iPhone/iPad physical VoiceOver runs for A01–A14, including Screen Curtain,
  exact iOS build, app build, speaking-rate/rotor settings, and announcement
  transcripts.
- Android physical TalkBack runs for A01–A14 on the selected stock and OEM
  devices, including exact OS/build, TalkBack settings, and diagnostic speech
  output where used.
- AM4 and FM3 blind-user runs for every advertised parameter, scene/channel,
  route, undo, save/export, tuner/tempo, and disconnect path; each must include
  safe read/write/read-back/reconnect evidence and the exact transport/adapter.
- Device-matrix coverage for the compact phone, reference phone, mandatory iPad,
  mandatory Android tablet, minimum OS, current OS, and OEM variation; untested
  combinations stay explicitly documented as evidence gaps or experimental.
- Independent accessibility review of focus/announcement logs, privacy
  redaction, and verification labels; no semantics-only claim may be promoted.
