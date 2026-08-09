# Product Requirements

## 1. Product contract

RigWarden is a free, local-first editor and preset workstation for modern modelers. The product must be useful without an account, cloud service, AI provider, desktop companion, or internet connection after installation and profile acquisition.

The requirements below are target-product requirements. They do not imply that every device profile has been physically verified. Verification status is tracked separately.

Priority terms:

- **P0** — required for the first public beta.
- **P1** — required before the first stable release.
- **P2** — committed follow-on capability.
- **Research** — architecture must permit it, but implementation waits for evidence.

## 2. Device sessions and discovery

### Session requirements

- **DEV-001 P0:** Discover compatible endpoints automatically.
- **DEV-002 P0:** Always provide a manual endpoint/device selector.
- **DEV-003 P0:** Identify device family, model, hardware variant when available, and firmware.
- **DEV-004 P0:** Select an exact device/firmware profile before enabling writes.
- **DEV-005 P0:** Unknown firmware must default to read-only or unsupported—not nearest-version writable fallback.
- **DEV-006 P0:** Display the transport and per-feature capability matrix for the active session.
- **DEV-007 P0:** Explain likely exclusive-port conflicts and recovery steps.
- **DEV-008 P0:** Handle disconnect, reconnect, endpoint disappearance, and app lifecycle transitions truthfully.
- **DEV-009 P0:** Permit one active editor target.
- **DEV-010 P1:** Preserve architecture for multi-device rigs and combined performance panels.
- **DEV-011 P0:** Never identify a simulator as physical hardware.
- **DEV-012 P0:** Expose a read-only session option even when writes are supported.

## 3. Preset browsing and state

- **PRESET-001 P0:** List, select, and search presets supported by the active device.
- **PRESET-002 P0:** Read the live edit buffer.
- **PRESET-003 P0:** Distinguish live edited state, last state read, last explicitly stored device state, and offline library state.
- **PRESET-004 P0:** Rename and explicitly save/store a preset.
- **PRESET-005 P0:** Import and export individual presets.
- **PRESET-006 P0:** Backup and restore supported preset/bank/system data when the profile permits it.
- **PRESET-007 P0:** Preserve opaque/unknown bytes required for lossless round trips.
- **PRESET-008 P0:** Refuse destructive serialization when unknown required data cannot be preserved safely.
- **PRESET-009 P0:** Offer a manual Snapshot action.
- **PRESET-010 P0:** Offer an optional reminder to snapshot without silently creating user-visible preset backups.
- **PRESET-011 P0:** Support offline creation and editing.
- **PRESET-012 P0:** Show compatibility warnings before importing or sending a preset to a different device/firmware.
- **PRESET-013 P1:** Cross-device preset conversion with semantic loss reporting.

## 4. Routing graph

- **GRAPH-001 P0:** Display the device-appropriate routing grid.
- **GRAPH-002 P0:** Add, remove, move, and replace blocks.
- **GRAPH-003 P0:** Create and remove valid connections.
- **GRAPH-004 P0:** Represent serial paths, splits, merges, parallel paths, inputs, and outputs.
- **GRAPH-005 P0:** Validate all device-specific placement and routing constraints before writing.
- **GRAPH-006 P0:** Reject prohibited cycles and illegal endpoint combinations.
- **GRAPH-007 P0:** Preserve stable block identity across movement.
- **GRAPH-008 P0:** Produce deterministic traversal and deterministic command ordering.
- **GRAPH-009 P0:** Report graph errors in human-readable and machine-readable forms.
- **GRAPH-010 P0:** Provide a complete nonvisual list/tree representation with equivalent editing actions.
- **GRAPH-011 P1:** Provide semantic graph diff between versions or device/live state.
- **GRAPH-012 P1:** Provide intent-preserving cross-device graph translation.

## 5. Blocks and parameters

- **BLOCK-001 P0:** List every block type supported by the selected device/firmware pack.
- **BLOCK-002 P0:** List models/types available to each block.
- **BLOCK-003 P0:** Render every supported parameter with correct type, range, unit, enum, taper, display conversion, default, and availability.
- **BLOCK-004 P0:** Handle channels and channel-specific state.
- **BLOCK-005 P0:** Handle bypass state.
- **BLOCK-006 P0:** Handle modifiers/controllers and their curves where supported.
- **BLOCK-007 P0:** Search parameters by name, alias, block, and semantic category.
- **BLOCK-008 P0:** Show advanced and expert parameters without hiding them permanently.
- **BLOCK-009 P0:** Support precise numeric entry in addition to touch adjustment.
- **BLOCK-010 P0:** Avoid sending redundant writes when the confirmed device state already matches.
- **BLOCK-011 P0:** Show unsupported or firmware-specific parameters explicitly rather than silently dropping them.
- **BLOCK-012 P1:** Offer block library/favorites and reusable block-channel templates when device formats permit.

## 6. Scenes, channels, controllers, and performance

- **SCENE-001 P0:** List, rename, select, and edit scenes where supported.
- **SCENE-002 P0:** Edit scene-specific bypass/channel state and other supported scene data.
- **SCENE-003 P0:** Change block channels.
- **SCENE-004 P0:** Edit modifiers and controller assignments.
- **SCENE-005 P0:** Clearly distinguish preset-global, block-channel, and scene-specific state.
- **PERF-001 P0:** Create custom performance panels from arbitrary supported parameters/actions.
- **PERF-002 P0:** Export, import, share, and bundle performance panels with presets.
- **PERF-003 P0:** Provide Stage Mode with large targets, screen-awake control, navigation lock, and reduced accidental edits.
- **PERF-004 P0:** Provide tuner, tap tempo, scene, channel, bypass, and user-selected controls.
- **PERF-005 P0:** Make every performance control screen-reader operable.
- **PERF-006 P1:** Support multi-device performance panels after multi-device sessions are implemented.

## 7. Tuner, tempo, and looper

- **UTILITY-001 P0:** Display tuner state when supported by the device/transport.
- **UTILITY-002 P0:** Show when tuner streaming is unavailable on the selected transport.
- **UTILITY-003 P0:** Display and edit tempo.
- **UTILITY-004 P0:** Send tap-tempo actions safely.
- **UTILITY-005 P0:** Control and display supported looper state/actions.
- **UTILITY-006 P0:** Never claim realtime streaming support based only on request/response simulation.
- **UTILITY-007 P1:** Offer accessible spoken/haptic tuner feedback without making audio/haptics the only state indication.

## 8. Cab, IR, and DynaCab

- **CAB-001 P0:** Browse/select factory and user cabs where supported.
- **CAB-002 P0:** Import, export, rename, organize, and transfer user cabs with device-specific validation.
- **CAB-003 P0:** Edit complete DynaCab parameters supported by the selected profile.
- **CAB-004 P0:** Report slot limits, format incompatibilities, and destructive overwrite operations before execution.
- **CAB-005 P0:** Preserve cab metadata and provenance where available.
- **CAB-006 P1:** Audition and batch-organize user cabs.
- **CAB-007 P2:** Dedicated offline multi-IR mixing environment after the first beta.

## 9. FC-6 and FC-12

- **FC-001 P0:** Edit complete supported layout structures.
- **FC-002 P0:** Edit switch tap and hold functions.
- **FC-003 P0:** Edit labels, colors, layout links, per-preset overrides, and supported display behavior.
- **FC-004 P0:** Validate assignments against device/firmware capabilities.
- **FC-005 P0:** Provide a nonvisual list-based editor.
- **FC-006 P0:** Preview changes semantically before large layout writes.
- **FC-007 P1:** Export/import reusable layout templates.

## 10. Offline library and files

- **LIB-001 P0:** Store presets and metadata locally.
- **LIB-002 P0:** Organize with folders, tags, favorites, and setlists.
- **LIB-003 P0:** Search by preset name, device, firmware, blocks, models, tags, and notes.
- **LIB-004 P0:** Maintain version history and semantic diff.
- **LIB-005 P0:** Detect exact and semantic duplicates.
- **LIB-006 P0:** Filter by compatibility and expected conversion loss.
- **LIB-007 P0:** Use transactional storage migrations.
- **LIB-008 P0:** Access user-selected iCloud Drive, Google Drive, Dropbox, network shares, and similar providers only through OS file pickers/document providers.
- **LIB-009 P0:** Never require a RigWarden cloud account.
- **LIB-010 P0:** Make backup/export formats documented and recoverable without the app where practical.
- **LIB-011 P1:** Optional encrypted local library backups.

## 11. Undo, redo, recovery, and transactions

- **UNDO-001 P0:** Record confirmed previous state before completing a mutation.
- **UNDO-002 P0:** Persist the command journal across app crashes and disconnects.
- **UNDO-003 P0:** Support undo and redo of parameter, routing, block, scene, and supported structural changes.
- **UNDO-004 P0:** Create named branches when the user changes preset/session context.
- **UNDO-005 P0:** Represent partial batch completion truthfully.
- **UNDO-006 P0:** Never pretend a hardware batch is atomic unless the protocol provides an atomic transaction.
- **UNDO-007 P0:** Recover safely from stale acknowledgements, timeouts, and reconnects.
- **UNDO-008 P0:** Allow the user to inspect what an undo will change.
- **UNDO-009 P1:** Compare/restore arbitrary journal checkpoints.

## 12. Device/firmware packs

- **PACK-001 P0:** Packs are declarative data, not downloaded executable code.
- **PACK-002 P0:** Packs include identity match rules, firmware applicability, capabilities, block/parameter catalogs, transport restrictions, and provenance.
- **PACK-003 P0:** Normal remote packs are signed and verified.
- **PACK-004 P0:** Developer mode may install unsigned local packs with an explicit warning.
- **PACK-005 P0:** Pack updates are independent of app-store releases.
- **PACK-006 P0:** Unknown or ambiguous matching never enables writes.
- **PACK-007 P0:** Public compatibility status is generated from evidence, not hand-edited marketing.
- **PACK-008 P0:** Keep current plus two previous major firmware profiles where possible.
- **PACK-009 P0:** Retain valid community profiles beyond that window.
- **PACK-010 P1:** Community contribution tooling validates schema, fixtures, provenance, and compatibility claims before review.

## 13. Connectivity

Targeted transport families:

- direct USB MIDI;
- direct Android USB host/serial where required;
- class-compliant USB MIDI interfaces;
- five-pin MIDI through compatible interfaces;
- BLE MIDI;
- desktop/Raspberry Pi network bridge;
- later desktop-native MIDI/serial paths.

Requirements:

- **TRANSPORT-001 P0:** Separate transport bytes from protocol meaning.
- **TRANSPORT-002 P0:** Endpoint enumeration and manual selection.
- **TRANSPORT-003 P0:** Bounded timeouts, cancellation, retry policy, fragmentation, and backpressure.
- **TRANSPORT-004 P0:** Hotplug/unplug handling.
- **TRANSPORT-005 P0:** Per-feature capability reporting by device, OS, transport, and adapter path.
- **TRANSPORT-006 P0:** No silent fallback to a different endpoint or device.
- **TRANSPORT-007 P0:** Sanitize logs.
- **TRANSPORT-008 P0:** Detect or explain exclusive-port conflicts.
- **TRANSPORT-009 P0:** BLE and network bridge may launch as experimental but must be real, testable implementations—not placeholders.
- **TRANSPORT-010 P0:** The app remains useful offline without a bridge.
- **TRANSPORT-011 P1:** Desktop transports on macOS, Windows, and Linux.
- **TRANSPORT-012 P1:** Multi-device session multiplexing.

## 14. Simulator and Capture Lab

- **SIM-001 P0:** Deterministic simulator supports discovery, preset reads, parameter writes, routing changes, acknowledgements, timeouts, disconnects, and partial failure.
- **SIM-002 P0:** Replay provenance-approved captures.
- **SIM-003 P0:** Inject latency, fragmentation, dropped frames, stale responses, invalid checksums, and device resets.
- **SIM-004 P0:** Simulator identifies itself unmistakably.
- **SIM-005 P0:** Contributors can build most UI and domain behavior without hardware.
- **CAPTURE-001 P0:** Capture Lab records only user-selected endpoints and sessions.
- **CAPTURE-002 P0:** Sanitize serials, personal paths, credentials, and unrelated traffic.
- **CAPTURE-003 P0:** Generate fixture sidecars and checksums.
- **CAPTURE-004 P0:** Require explicit redistribution permission before a contribution is accepted.
- **CAPTURE-005 P0:** Offer read-only research modes where feasible.
- **CAPTURE-006 P1:** Guided unsupported-device/firmware contribution workflow.

## 15. AI

- **AI-001 P0:** AI is optional and disabled until configured.
- **AI-002 P0:** Support BYOK OpenRouter and a generic OpenAI-compatible provider interface.
- **AI-003 P0:** Store provider secrets in Keychain/Keystore or an equivalent platform secret store.
- **AI-004 P0:** Show exactly what preset data will be sent.
- **AI-005 P0:** Exclude serial numbers, credentials, unrelated library data, and raw captures.
- **AI-006 P0:** Model output is a strict provider-independent mutation-plan schema.
- **AI-007 P0:** AI has no raw SysEx or transport-send tool.
- **AI-008 P0:** Local deterministic code validates device, firmware, graph, type, range, capability, and write safety.
- **AI-009 P0:** Multi-parameter or structural changes require semantic preview and approval.
- **AI-010 P0:** Exact validated single changes may use a configurable fast path.
- **AI-011 P0:** Provider/model allowlists, cost estimate, and daily/monthly caps.
- **AI-012 P0:** AI content is excluded from telemetry by default.
- **AI-013 P0:** Provider timeout/cancellation produces no hardware write.
- **AI-014 P0:** Preset Doctor.
- **AI-015 P0:** Tone Architect.
- **AI-016 P0:** Scene Composer.
- **AI-017 P1:** Preset Explainer, Troubleshooter, Performance Panel Generator, and Cross-Device Translator.
- **AI-018 P2:** Reference-audio matching only after a separate DSP/measurement design and validation phase.
- **AI-019 P1:** LAN/local OpenAI-compatible provider support without a project backend.

## 16. Accessibility

- **A11Y-001 P0:** Full VoiceOver and TalkBack semantics for all primary workflows.
- **A11Y-002 P0:** Routing has a complete nonvisual representation and editor.
- **A11Y-003 P0:** Every control exposes name, role, value, unit, range, state, and available actions.
- **A11Y-004 P0:** Predictable focus order.
- **A11Y-005 P0:** Complete keyboard, switch-control, and external-input operation where supported.
- **A11Y-006 P0:** Scalable text and layout reflow.
- **A11Y-007 P0:** High contrast and no state communicated solely by color.
- **A11Y-008 P0:** Reduced motion.
- **A11Y-009 P0:** Errors and successful mutations are announced.
- **A11Y-010 P0:** Parameter and block search provide direct navigation.
- **A11Y-011 P0:** Blind-user physical-device testing is part of beta acceptance.
- **A11Y-012 P1:** Braille-display-friendly labels and concise/verbose announcement modes.
- **A11Y-013 P1:** Optional tuner haptic/audio feedback with equivalent textual state.

## 17. Visual and interaction design

- **UI-001 P0:** Studio Carbon is default.
- **UI-002 P0:** Stage Amber, Console Ivory, and Electric Slate ship.
- **UI-003 P0:** Every major view supports phone/tablet and portrait/landscape.
- **UI-004 P0:** Complete editor on phones.
- **UI-005 P0:** Tablet pointer, keyboard, precision input, drag/drop, and split-screen support.
- **UI-006 P0:** Approved PNG assets for all icons, controls, panels, switches, sockets, tabs, halos, and decorative surfaces.
- **UI-007 P0:** Knobs may be procedurally rendered.
- **UI-008 P0:** Dynamic functional graphics may render routing cables, meters, curves, waveforms, focus regions, and other live data.
- **UI-009 P0:** No production SVG assets.
- **UI-010 P0:** No copied vendor/competitor artwork or layouts.
- **UI-011 P0:** No placeholder art or dead control may count as feature completion.
- **UI-012 P0:** All touch targets meet platform accessibility guidance and stage-use needs.
- **UI-013 P0:** UI communicates connected, pending, confirmed, failed, partial, offline, and read-only states explicitly.

## 18. Privacy, security, and telemetry

- **PRIV-001 P0:** No account.
- **PRIV-002 P0:** No mandatory telemetry.
- **PRIV-003 P0:** Crash reports are explicit opt-in.
- **PRIV-004 P0:** Logs are local and sanitized by default.
- **PRIV-005 P0:** User can inspect and export/delete telemetry-related local data.
- **SEC-001 P0:** Untrusted SysEx/file input is bounded, parser-safe, and fuzzed.
- **SEC-002 P0:** Pack signatures are verified before normal installation.
- **SEC-003 P0:** Secrets never enter logs, fixtures, crash reports, or AI prompts.
- **SEC-004 P0:** Raw transmission requires Developer Mode plus a separate danger control.
- **SEC-005 P0:** Write rate and retries are bounded.
- **SEC-006 P0:** No downloaded executable extension mechanism.
- **SEC-007 P0:** Threat model and security-contact process are public.
- **SEC-008 P1:** Reproducible-build documentation and supply-chain attestations.

## 19. Distribution and platform

- **PLAT-001 P0:** iOS/iPadOS lead implementation.
- **PLAT-002 P0:** Android first-class implementation.
- **PLAT-003 P0:** Baseline target recommendation: iOS/iPadOS 16+ and Android 10+, subject to the bootstrap audit.
- **PLAT-004 P0:** TestFlight.
- **PLAT-005 P0:** Apple App Store.
- **PLAT-006 P0:** Google Play internal/open test and production.
- **PLAT-007 P0:** Signed APKs through GitHub Releases.
- **PLAT-008 P0:** F-Droid-compatible pure-open-source flavor.
- **PLAT-009 P0:** F-Droid flavor starts with telemetry disabled and no network permission until user enables profile updates or AI.
- **PLAT-010 P1:** macOS, Windows, and Linux desktop clients from the same monorepo.
- **PLAT-011 P0:** Store builds remain free.

## 20. Quality and release

- **QA-001 P0:** Strict observed RED–GREEN–REFACTOR for production behavior.
- **QA-002 P0:** Byte-exact independent fixtures for protocol compatibility.
- **QA-003 P0:** Fuzz/property tests for untrusted parsers and graph invariants.
- **QA-004 P0:** Deterministic simulator E2E.
- **QA-005 P0:** Native-platform tests.
- **QA-006 P0:** Physical hardware matrix.
- **QA-007 P0:** Real VoiceOver/TalkBack workflow tests.
- **QA-008 P0:** Soak, reconnect, and cancellation testing.
- **QA-009 P0:** No unexplained warnings or skipped required tests.
- **QA-010 P0:** No placeholder buttons/screens in beta.
- **QA-011 P0:** AM4 and FM3 hardware-verified end-to-end before public beta.
- **QA-012 P0:** Compatibility page generated from evidence records.
- **QA-013 P0:** Clean-clone build and release verification.
- **QA-014 P0:** Independent review before integration.
