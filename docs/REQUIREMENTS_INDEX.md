# Requirement Namespace Index

This index defines stable prefixes used by work packets, evidence, code comments, tests, compatibility reports, and ADRs.

| Prefix | Area |
|---|---|
| DEV | Device identity, session, discovery, and active-target behavior |
| PRESET | Preset state, file behavior, import/export, and save semantics |
| GRAPH | Routing graph and graph mutation |
| BLOCK | Block inventory, models, parameters, channels, and modifiers |
| SCENE | Scenes and scene-specific behavior |
| PERF | Performance panels and stage mode |
| UTILITY | Tuner, tempo, looper, and related utilities |
| CAB | Cab, IR, and DynaCab |
| FC | FC-6/FC-12 editing |
| LIB | Offline library and storage |
| UNDO | Undo, redo, branches, recovery, and mutation history |
| PACK | Device/firmware definition packs |
| TRANSPORT | MIDI, USB, serial, BLE, and network transport |
| SIM | Simulator and replay |
| CAPTURE | Capture Lab |
| AI | AI providers and mutation planning |
| A11Y | Accessibility |
| UI | Visual/interaction system |
| PRIV | Privacy and telemetry |
| SEC | Security |
| PLAT | Platforms and distribution |
| QA | Verification and release quality |

## Traceability rule

Every production behavior test must cite at least one requirement ID. Every requirement must eventually map to:

- one or more work-item IDs;
- one or more test IDs or test selectors;
- an evidence directory;
- an implementation state;
- a verification label;
- any device/firmware/platform applicability.

The traceability matrix is integration-owned. Workers propose updates in handoff files rather than racing on the global matrix.
