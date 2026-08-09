# Topology Test Ladder

Use the cheapest level that can prove the behavior, and never let it impersonate a higher level.

## L0 — Static/schema validation

Proves:

- JSON/YAML/TOML schema;
- generated registry consistency;
- link/manifest constraints;
- forbidden dependency/asset checks.

Does not prove runtime behavior.

## L1 — Pure unit/property

Proves:

- values;
- parsing of local non-protocol formats;
- state transitions;
- graph invariants;
- range/capability validation;
- command planning;
- journal logic;
- deterministic presentation models.

Does not prove wire bytes, FFI, OS, or hardware.

## L2 — Byte/file compatibility

Proves:

- exact framing;
- exact encoding/decoding;
- checksum;
- malformed input;
- opaque preservation;
- fixture round trip;
- firmware-specific mapping.

Needs independent, provenance-approved expected data.

Does not prove transport or hardware.

## L3 — Simulator/replay

Proves:

- sequencing;
- correlation;
- timeout;
- retry;
- disconnect;
- partial completion;
- deterministic E2E against simulator.

Does not prove physical device.

## L4 — Flutter widget/layout/semantics

Proves:

- layout behavior;
- interaction in Flutter test harness;
- semantic tree;
- focus logic;
- theme/asset state;
- accessibility contract at framework level.

Does not prove real screen-reader or native transport.

## L5 — FFI/native platform

Proves:

- generated bridge;
- lifecycle;
- platform API integration;
- permissions;
- endpoint enumeration;
- native cancellation/hotplug;
- emulator/simulator behavior.

Fake adapters remain contract-only.

## L6 — Full app deterministic E2E

Proves:

- UI to Rust to protocol to simulator and back;
- journal/state reconciliation;
- nonvisual and visual path integration.

Does not prove real hardware.

## L7 — Physical mobile/desktop device

Proves:

- app behavior on real iOS/Android/desktop hardware;
- real VoiceOver/TalkBack;
- real USB/BLE/file/secret-store behavior.

Still does not prove modeler compatibility unless attached.

## L8 — Modeler hardware-in-loop

Proves the declared device/firmware/OS/transport feature matrix.

Only L8 grants `HARDWARE_VERIFIED`.

## L9 — Distribution

Proves signed/store/F-Droid/release behavior, upgrades, entitlements, permissions, and reproducibility.

## Common claim mapping

| Claim | Minimum evidence |
|---|---|
| Graph rejects a cycle | L1 |
| Known SysEx response decodes correctly | L2 |
| Discovery state machine handles timeout | L3 |
| Routing list has semantic actions | L4 |
| CoreMIDI endpoint enumerates on iOS simulator/fixture | L5 as applicable |
| VoiceOver user can edit a parameter | L7 |
| AM4 parameter write works | L8 |
| TestFlight build installs/upgrades | L9 |

## Matrices

Cross-boundary claims need multiple levels. Example “blind user changes AM4 parameter over USB and can undo” requires:

- L1 domain/undo;
- L2 protocol;
- L3 simulator;
- L4 semantics;
- L5 native bridge;
- L7 VoiceOver;
- L8 AM4;
- potentially L9 for distributed build.
