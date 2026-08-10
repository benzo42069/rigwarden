# Starter-Kit Implementation Milestone Progress

**Goal started:** 2026-08-09T09:35:00-05:00  
**Goal-start commit:** `536d8901ac91ecdbc15e09356800d9f46be401dd` (local worktree; public integration commits are recorded in packet evidence)  
**Phase:** Bootstrap validator chain, then core vertical-slice dependency chain

## Frozen in-scope work items

`TOP-RSCH-001`–`TOP-RSCH-008`; `TOP-BOOT-001`, `TOP-BOOT-002`, `TOP-BOOT-002R`, `TOP-BOOT-003`–`TOP-BOOT-009`; `TOP-A11Y-001`, `TOP-CMD-001`–`TOP-CMD-003`, `TOP-DOM-001`–`TOP-DOM-003`, `TOP-E2E-001`, `TOP-FFI-001`, `TOP-GRAPH-001`–`TOP-GRAPH-005`, `TOP-PRESET-001`–`TOP-PRESET-002`, `TOP-REG-001`–`TOP-REG-002`, `TOP-SIM-001`–`TOP-SIM-002`, `TOP-UI-001`, `TOP-UNDO-001`–`TOP-UNDO-002`; `TOP-AM4-E2E-001`–`TOP-AM4-E2E-002`, `TOP-AM4-FIX-001`–`TOP-AM4-FIX-002`, `TOP-AM4-HIL-001`, `TOP-AM4-PROTO-001`–`TOP-AM4-PROTO-003`, `TOP-AM4-REG-001`, `TOP-AM4-SIM-001`; `TOP-FM3-E2E-001`–`TOP-FM3-E2E-003`, `TOP-FM3-FIX-001`, `TOP-FM3-HIL-001`–`TOP-FM3-HIL-002`, `TOP-FM3-IOS-001`, `TOP-FM3-NATIVE-001`–`TOP-FM3-NATIVE-002`, `TOP-FM3-PROTO-001`, `TOP-FM3-RSCH-001`, `TOP-FM3-SIM-001`.

## Current status

- **Integrated:** `TOP-RSCH-001`–`TOP-RSCH-008`, `TOP-BOOT-001`, `TOP-BOOT-002R`–`TOP-BOOT-008`, `TOP-DOM-001`–`TOP-DOM-002`.
- **Historical review failure:** `TOP-BOOT-002`; superseded only for its missing audit evidence by `TOP-BOOT-002R`, whose source scope is product-empty and independently reviewed.
- **Integrated:** `TOP-BOOT-009` (truthful local fixture-validation CLI) and `TOP-DOM-003` (typed read-only device identity) after independent review and post-landing Rust sweeps.
- **Integrated:** `TOP-GRAPH-001` (stable routing node identity), `TOP-GRAPH-002` (typed output-to-input connection with source/destination guards), `TOP-GRAPH-003` (missing-source-port rejection without mutation), `TOP-GRAPH-004` (explicit policy-based cycle rejection), and `TOP-GRAPH-005` (deterministic topological traversal) after independent review and post-landing sweeps.
- **Integrated:** `TOP-REG-001` (exact in-memory device/firmware profile resolution with explicit profile-derived write capability), `TOP-REG-002` (known-family/model unknown firmware becomes read-only), and `TOP-REG-003` (profile-owned fixed-point numeric metadata) after independent review and post-landing sweeps.
- **Integrated:** `TOP-FFI-000` (pinned Flutter Rust Bridge 2.12.0 generated harness and Cargokit wiring; `BUILD_CODEGEN_VERIFIED` only) and `TOP-CMD-001` (profile-bound semantic numeric mutation validation; `UNIT_VERIFIED` only) after public candidate commit `a4cbb8d` matched reviewed blobs and passed immutable-tree sweeps.
- **Integrated:** `TOP-CMD-002` (unknown-firmware sessions and contradictory read-only profiles both reject write planning before parameter mapping; `UNIT_VERIFIED` and `READ_ONLY` only) after independent security review and public candidate commit `b9ac4b8` matched reviewed blobs and passed its focused and required integration sweeps.
- **Integrated:** `TOP-FFI-001` (a Rust-owned opaque `DeviceIdentity` handle round-trips across the generated Flutter bridge; `FFI_VERIFIED` only) after independent review and public candidate commit `4fc226b` matched reviewed blobs and passed the release-library build, real Flutter FFI test, analyzer, formatter, and Clippy sweep.
- **Integrated:** `TOP-CMD-003` (stable semantic graph-mutation planning respects dependencies and is insertion-order independent; `UNIT_VERIFIED` only) after independent review and public candidate commit `906816f` matched reviewed blobs and passed focused, command, routing, formatter, and Clippy integration sweeps.
- **Integrated:** `TOP-SIM-001` (one deterministic synthetic request/response exchange, correlation, transcript, and structured mismatch; `SIMULATOR_VERIFIED` only) after independent review and parent rerun of focused, simulator, command-engine, formatter, and Clippy checks. No byte, platform, or hardware claim is implied.
- **Integrated:** `TOP-UI-001` (adaptive phone/tablet session shell with a bounded Flutter L4 semantic contract) after three preserved TDD cycles, independent accessibility review, and a clean serial parent Flutter sweep. The claim excludes physical VoiceOver/TalkBack, switch-control, destination workflows, platform-device, and hardware proof.
- **Integrated:** `TOP-SIM-002` (a reused request ID from an older connection generation cannot confirm the newer request) after independent security review and parent Rust integration sweeps. This is `UNIT_VERIFIED` command-session correlation only, not simulator transport, protocol, platform, or hardware verification.
- **Integrated:** `TOP-UNDO-001` (an in-memory undo entry is finalized only after confirmation and stores the exact prior/new values) after independent review and parent Rust integration sweeps. This is `UNIT_VERIFIED` only; persistence, simulator, platform, and hardware behavior remain unavailable.
- **Integrated:** `TOP-UNDO-002` (preset contexts retain separate in-memory undo branches and the prior branch remains inspectable) after independent review and parent Rust integration sweeps. Pending-across-switch affinity is defensive source behavior, not a separately verified claim.
- **Externally gated:** AM4/FM3 fixture and hardware packets retain their packet-level `BLOCKED_FIXTURE` or `BLOCKED_HARDWARE` status until lawful fixtures or physical equipment are supplied.

## Verification truth

- `RUST_HARNESS_EXECUTABLE`: integrated via `TOP-BOOT-002R`; no domain/protocol/platform/hardware claim.
- `FLUTTER_HARNESS_EXECUTABLE`: integrated via `TOP-BOOT-003`; no platform-device/hardware claim.
- `SIMULATOR_VERIFIED`: `TOP-SIM-001` only, for its explicit synthetic scripted transport. No device, byte-fixture, platform, or hardware compatibility is implied.
- `SEMANTICS_VERIFIED`: `TOP-UI-001` only, for its Flutter L4 adaptive shell names, button semantics, selection, actions, focus recovery policy, live destination update, large-text branch, and reduced-motion branch. It is not a physical screen-reader or complete-editor claim.
- No native platform, accessibility-device, protocol-byte, fixture, or hardware verification label has been earned.

## Routing and next group

Project role configuration requests OpenAI `gpt-5.6-luna` at `max`; runtime routing research is integrated in `TOP-RSCH-001`. Sandbox isolation remains unverified, so write-capable work is serialized or narrowly scoped with independent review.

**Last completed integration sweep:** `TOP-UNDO-002` focused branch isolation, undo package, preset package, workspace format, and undo Clippy checks all exited 0 after independent review.

**Contract gate:** `TOP-E2E-001` is `BLOCKED_CONTRACT`: a real typed app→FFI→validator→synthetic simulator→journal composition path does not yet exist. It needs a narrow command/event bridge decision and composition packet; no fake Dart-only E2E test will be accepted. Fixture-derived AM4/FM3 leaves remain gated by lawful fixture or hardware inputs.
