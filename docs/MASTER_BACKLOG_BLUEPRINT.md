# Master Backlog Blueprint

This is the product-wide decomposition map. It is not permission to assign an epic to one worker. Terra must convert leaf behaviors into the work-packet schema before implementation.

## TOP-000 — Research, repository contracts, and evidence system

- Runtime/model routing verification.
- Current Codex/toolchain audit.
- External source/license inventory.
- Official device/manual/spec inventory.
- Transport feasibility matrix.
- Working-name collision search.
- Accessibility test-tool survey.
- Store/distribution requirements.
- Protocol capture and fixture policy.
- Minimal repository bootstrap.
- Requirement/packet/evidence schema validators.
- CI fail-fast baseline.
- License/notice/security/contribution files.
- Generated compatibility-report skeleton.

## TOP-100 — Core identity and capability model

- Device family/model/variant identifiers.
- Firmware semantic/opaque version representation.
- Transport identifiers.
- Device identity response.
- Exact profile resolution.
- Verified compatible ranges.
- Unknown firmware read-only result.
- Ambiguous identity result.
- Capability set and feature restrictions.
- Verification-label types.
- Simulator identity isolation.
- Session generation IDs.
- Stable block/parameter IDs.

## TOP-200 — Normalized preset document

- Document metadata and schema version.
- Original raw payload reference.
- Known typed sections.
- Opaque segments.
- Stable block instances.
- Routing graph reference.
- channels/scenes/controllers.
- cab/FC extensions.
- unknown-data preservation.
- lossless round-trip contract.
- destructive-conversion refusal.
- migrations.
- semantic hash.
- exact hash.
- compatibility report.
- cross-device loss report.

## TOP-300 — Routing graph

- Node/port types.
- Device grid geometry.
- valid serial connection.
- split.
- merge.
- parallel path.
- endpoint validation.
- cycle rules.
- placement rules.
- stable identity on move.
- deterministic traversal.
- disconnected/dead-path detection.
- graph diff.
- graph mutation plans.
- accessible path description.
- nonvisual add/remove/move/connect.
- undo/redo.
- serialization.
- large-graph performance/property tests.

## TOP-400 — Command engine and session reconciliation

- typed mutation.
- capability validation.
- deterministic command plan.
- request correlation.
- write queue.
- rate limiting.
- bounded retry.
- timeout.
- cancellation.
- stale response rejection.
- acknowledgement.
- read-back.
- optimistic/pending/confirmed state.
- partial completion.
- reconnect reconciliation.
- read-only enforcement.
- sanitized logs.
- transactional UI preview.
- no false atomicity.

## TOP-500 — Persistent undo and recovery

- journal schema.
- prior confirmed state capture.
- append.
- crash recovery.
- parameter undo/redo.
- routing undo/redo.
- block mutation undo/redo.
- scene/channel undo/redo.
- partial-operation record.
- branch on preset/session change.
- manual snapshot.
- snapshot reminder.
- checkpoint diff.
- journal compaction.
- corruption handling.
- migration.

## TOP-600 — Protocol common and family codecs

- bounded byte cursor.
- 7-bit/value helpers based on evidence.
- framing.
- checksums.
- escaping/fragmentation.
- message identity.
- parser errors.
- encoder errors.
- unknown message preservation.
- fuzz harness.
- provenance fixture loader.
- Gen 1/2/3 and current-device family modules only as evidence requires.
- exact firmware variants.
- no guessing.

## TOP-700 — Simulator, replay, and Capture Lab

- scripted transport.
- stateful fake device.
- identity/discovery.
- preset read.
- parameter write.
- graph write.
- acknowledgement/read-back.
- latency.
- fragmentation.
- dropped frame.
- stale response.
- disconnect/reconnect.
- reset.
- partial failure.
- capture import.
- sanitizer.
- provenance sidecar generator.
- fixture checksum.
- contributor export bundle.
- simulator UI.

## TOP-800 — Flutter shell and design system

- app shell.
- adaptive breakpoints.
- phone portrait/landscape.
- tablet portrait/landscape.
- navigation.
- session status.
- Studio Carbon.
- Stage Amber.
- Console Ivory.
- Electric Slate.
- PNG asset manifest/loader.
- procedural knob.
- dynamic functional renderer.
- touch/keyboard/pointer.
- reduced motion.
- high contrast.
- localization-ready strings.
- error surfaces.
- loading/pending/partial states.

## TOP-900 — Accessibility

- semantic vocabulary.
- structured route view.
- route actions.
- parameter browser.
- scene/channel editor.
- performance panel semantics.
- focus restoration.
- direct search/jump.
- concise/verbose announcements.
- live-update throttling.
- keyboard/switch control.
- tuner semantics.
- large text.
- non-color state.
- iOS VoiceOver matrix.
- Android TalkBack matrix.
- blind-user beta tasks.

## TOP-1000 — iOS/iPadOS

- CoreMIDI endpoint enumeration.
- USB MIDI.
- class-compliant interface.
- BLE MIDI.
- permissions where applicable.
- endpoint hotplug.
- lifecycle.
- file picker.
- Keychain.
- background/stage behavior.
- screen awake.
- TestFlight.
- App Store privacy/entitlements.
- VoiceOver physical tests.
- iPhone/iPad/device matrix.

## TOP-1100 — Android

- MIDI API.
- USB host.
- FM3 serial path.
- class-compliant interface.
- BLE MIDI.
- permissions.
- endpoint hotplug.
- lifecycle.
- document provider.
- Keystore.
- stage behavior.
- signed APK.
- Play testing/production.
- F-Droid flavor.
- TalkBack physical tests.
- phone/tablet matrix.

## TOP-1200 — AM4 vertical slice

- identity fixtures/query/response.
- exact profile.
- discovery.
- preset list/read.
- minimal preset decode.
- parameter catalog.
- one parameter read/write/read-back.
- channels/scenes as supported.
- routing behavior as supported.
- block/model inventory.
- preset save.
- offline round trip.
- UI.
- accessibility.
- undo.
- reconnect.
- hardware matrix.
- fixture contribution.

## TOP-1300 — FM3 vertical slice

- transport verification.
- Android direct USB serial.
- iOS feasible path(s).
- identity.
- profile.
- complete grid.
- preset read/write.
- blocks/models/parameters.
- scenes/channels/modifiers.
- tuner/tempo/looper.
- cabs.
- FC integration.
- offline round trip.
- UI/accessibility.
- undo/reconnect/soak.
- hardware matrix.

## TOP-1400 — FM9, Axe-Fx III, VP4

For each:

- exact transport matrix;
- identity/profile;
- fixture corpus;
- preset codec;
- routing constraints;
- blocks/parameters/models;
- scenes/channels/modifiers;
- utilities;
- cabs/FC as applicable;
- UI/accessibility;
- simulator;
- community/hardware evidence.

## TOP-1500 — Legacy

Axe-Fx II family, AX8, FX8, then Standard/Ultra:

- source/license audit;
- transport requirements;
- exact hardware variants;
- firmware profiles;
- preset/file formats;
- safe read-only first;
- community capture guide;
- simulator;
- experimental labeling;
- hardware contributors;
- progressive write enablement.

## TOP-1600 — Complete block and parameter experience

- metadata catalog.
- widget mapping.
- precise numeric entry.
- enum/search.
- aliases.
- units/display conversions.
- taper.
- min/max/default.
- expert pages.
- conditional visibility.
- channel/global/scene scope.
- modifier editing.
- favorites/templates.
- screen-reader semantics.
- large catalog performance.

## TOP-1700 — Utilities and performance

- tuner.
- tempo/tap.
- looper.
- performance panel model.
- panel editor.
- import/export.
- stage lock.
- screen awake.
- user actions.
- accessible stage mode.
- later multi-device panels.

## TOP-1800 — Cab and DynaCab

- inventory.
- metadata.
- import/export.
- transfer.
- overwrite preview.
- organization.
- DynaCab parameter editor.
- batch operations.
- compatibility.
- accessible flows.
- later offline IR mixer.

## TOP-1900 — FC-6/FC-12

- layouts.
- switch tap/hold.
- labels/colors.
- links.
- functions.
- per-preset overrides.
- preview.
- validation.
- write/partial recovery.
- import/export templates.
- nonvisual editor.
- hardware matrix.

## TOP-2000 — Offline library

- SQLite selection/ADR.
- schema/migrations.
- folders/tags/favorites/setlists.
- search.
- exact and semantic hash.
- version history.
- diff.
- duplicate detection.
- compatibility filtering.
- import/export.
- OS file picker.
- backups.
- corruption recovery.
- large-library performance.

## TOP-2100 — AI foundation

- provider interface.
- OpenRouter.
- generic OpenAI-compatible.
- secret storage.
- prompt data model.
- data preview.
- mutation-plan schema.
- validator.
- semantic diff.
- approval policy.
- cancellation/no-write.
- cost limits.
- provider restrictions.
- redaction.
- fake-provider tests.
- gated live-provider tests.

## TOP-2200 — Preset Doctor

- deterministic diagnostic rules.
- route reachability.
- silent-scene analysis.
- unsupported parameter detection.
- redundant/no-op detection.
- level-risk rules where justified.
- explanation layer.
- prioritization.
- no-write default.
- fix-plan preview.
- accessible output.

## TOP-2300 — Tone Architect

- intent schema.
- supported catalog grounding.
- assumption reporting.
- full plan validation.
- CPU/resource preflight where available.
- semantic preview.
- staged execution.
- rollback.
- device/firmware constraints.
- test corpus.

## TOP-2400 — Scene Composer

- scene-intent schema.
- preserve graph.
- distinguish global/channel/scene data.
- generate variations.
- preview.
- conflict resolution.
- partial write handling.
- test corpus.

## TOP-2500 — Pack distribution and community workflow

- schema.
- pack builder.
- signature.
- trust store.
- update manifest.
- rollback.
- unsigned developer mode.
- contributor validator.
- provenance lint.
- compatibility evidence bundle.
- generated docs.
- moderation/review.

## TOP-2600 — Privacy/security/telemetry

- opt-in crash reporting.
- allowlisted fields.
- local logs.
- export/delete.
- F-Droid network policy.
- secret redaction.
- profile signatures.
- network bridge pairing.
- threat-model tests.
- security process.

## TOP-2700 — Performance and resilience

- parser fuzz.
- graph property tests.
- large preset benchmarks.
- large catalog UI.
- long-session soak.
- repeated reconnect.
- BLE large transfer.
- USB hotplug.
- journal growth/compaction.
- storage migration.
- battery/memory profiling.
- rate-limit validation.

## TOP-2800 — Distribution

- clean-clone build.
- iOS signing/TestFlight.
- App Store.
- Android signing.
- Play testing/production.
- GitHub release.
- F-Droid metadata/build.
- SBOM.
- third-party notices.
- privacy manifests.
- reproducible build.
- update/migration.

## TOP-2900 — Public documentation and beta operations

- user manual.
- connection matrix.
- troubleshooting.
- accessibility guide.
- contributor guide.
- capture guide.
- profile guide.
- AI privacy guide.
- compatibility report.
- release notes.
- issue templates.
- security policy.
- governance.
- beta feedback/triage.
