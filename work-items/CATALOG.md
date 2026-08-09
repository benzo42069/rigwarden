# Work Item Catalog

Generated from 65 detailed packets.

## wave-00-research

| ID | Status | Priority | Title | Dependencies |
|---|---|---|---|---|
| `TOP-RSCH-001` | READY | critical | Verify Codex runtime, toolchain, and effective Luna subagent routing | — |
| `TOP-RSCH-002` | READY | high | Audit Axis, ForgeFX, fractal-midi, and related source licenses and provenance | — |
| `TOP-RSCH-003` | READY | high | Build the official Fractal device, firmware, and protocol source map | — |
| `TOP-RSCH-004` | READY | high | Establish the mobile transport feasibility and capability matrix | — |
| `TOP-RSCH-005` | READY | high | Perform a preliminary collision and package-identity screen for RigWarden | — |
| `TOP-RSCH-006` | READY | high | Define the blind-accessibility tooling and physical test plan | — |
| `TOP-RSCH-007` | READY | high | Map current Apple, Google Play, GitHub, and F-Droid distribution constraints | — |
| `TOP-RSCH-008` | READY | high | Finalize the lawful capture, sanitization, and fixture-contribution plan | — |

## wave-01-bootstrap

| ID | Status | Priority | Title | Dependencies |
|---|---|---|---|---|
| `TOP-BOOT-001` | BLOCKED_DEPENDENCY | critical | Establish the minimal repository, license, and instruction baseline | TOP-RSCH-001, TOP-RSCH-002, TOP-RSCH-003, TOP-RSCH-004, TOP-RSCH-005, TOP-RSCH-006, TOP-RSCH-007, TOP-RSCH-008 |
| `TOP-BOOT-002` | BLOCKED_DEPENDENCY | critical | Create the minimum Rust workspace and empty domain test harness | TOP-BOOT-001 |
| `TOP-BOOT-003` | BLOCKED_DEPENDENCY | critical | Create the minimum Flutter mobile app and widget-test harness | TOP-BOOT-001 |
| `TOP-BOOT-004` | BLOCKED_DEPENDENCY | critical | Establish fail-fast CI for the current minimal Rust and Flutter harnesses | TOP-BOOT-002, TOP-BOOT-003 |
| `TOP-BOOT-005` | BLOCKED_DEPENDENCY | high | Reject a work packet that omits its immutable work-item ID | TOP-BOOT-002, TOP-BOOT-004 |
| `TOP-BOOT-006` | BLOCKED_DEPENDENCY | high | Reject fixture provenance that lacks explicit redistribution permission | TOP-BOOT-005 |
| `TOP-BOOT-007` | BLOCKED_DEPENDENCY | high | Reject a completed evidence directory that is missing focused RED output | TOP-BOOT-005 |
| `TOP-BOOT-008` | BLOCKED_DEPENDENCY | high | Reject production asset manifests containing SVG files | TOP-BOOT-005 |
| `TOP-BOOT-009` | BLOCKED_DEPENDENCY | high | Expose fixture provenance validation as a deterministic command-line tool | TOP-BOOT-006, TOP-BOOT-007 |

## wave-02-core-vertical-slice

| ID | Status | Priority | Title | Dependencies |
|---|---|---|---|---|
| `TOP-A11Y-001` | BLOCKED_DEPENDENCY | high | Expose a serial signal path as an ordered nonvisual route | TOP-GRAPH-002, TOP-GRAPH-005, TOP-UI-001 |
| `TOP-CMD-001` | INTEGRATED | high | Validate one typed parameter mutation against an exact writable profile | TOP-REG-001, TOP-REG-003 |
| `TOP-CMD-002` | BLOCKED_DEPENDENCY | high | Reject every write mutation in an unknown-firmware read-only session | TOP-REG-002, TOP-CMD-001 |
| `TOP-CMD-003` | BLOCKED_DEPENDENCY | high | Order independent graph mutations deterministically | TOP-CMD-001, TOP-GRAPH-005 |
| `TOP-DOM-001` | BLOCKED_DEPENDENCY | high | Reject blank device-family identifiers | TOP-BOOT-002, TOP-BOOT-004 |
| `TOP-DOM-002` | BLOCKED_DEPENDENCY | high | Preserve a nonblank firmware identifier without semantic-version assumptions | TOP-DOM-001 |
| `TOP-DOM-003` | BLOCKED_DEPENDENCY | high | Represent an identified device with family, model, firmware, and transport endpoint | TOP-DOM-001, TOP-DOM-002 |
| `TOP-E2E-001` | BLOCKED_DEPENDENCY | high | Complete a simulated parameter edit, confirmation, and undo through the typed application path | TOP-SIM-001, TOP-UNDO-001, TOP-FFI-001, TOP-UI-001 |
| `TOP-FFI-000` | INTEGRATED | high | Bootstrap the generated Rust-Dart bridge harness without a product API | TOP-BOOT-003 |
| `TOP-FFI-001` | READY | high | Round-trip a typed device identity across the Rust-Dart boundary | TOP-DOM-003, TOP-BOOT-003, TOP-FFI-000 |
| `TOP-GRAPH-001` | BLOCKED_DEPENDENCY | high | Create graph nodes with stable typed identities | TOP-DOM-001 |
| `TOP-GRAPH-002` | BLOCKED_DEPENDENCY | high | Accept one valid serial connection between existing compatible ports | TOP-GRAPH-001 |
| `TOP-GRAPH-003` | BLOCKED_DEPENDENCY | high | Reject a connection whose source node or port does not exist | TOP-GRAPH-002 |
| `TOP-GRAPH-004` | BLOCKED_DEPENDENCY | high | Reject a prohibited routing cycle | TOP-GRAPH-002, TOP-GRAPH-003 |
| `TOP-GRAPH-005` | BLOCKED_DEPENDENCY | high | Produce deterministic topological traversal for the same graph | TOP-GRAPH-004 |
| `TOP-PRESET-001` | BLOCKED_DEPENDENCY | high | Preserve an opaque preset segment beside known normalized data | TOP-DOM-003, TOP-GRAPH-002 |
| `TOP-PRESET-002` | BLOCKED_DEPENDENCY | high | Round-trip the initial RigWarden offline container without losing opaque bytes | TOP-PRESET-001 |
| `TOP-REG-001` | BLOCKED_DEPENDENCY | high | Resolve an exact device and firmware profile as writable | TOP-DOM-003 |
| `TOP-REG-002` | BLOCKED_DEPENDENCY | high | Unknown firmware resolves to a non-writable session | TOP-REG-001 |
| `TOP-REG-003` | INTEGRATED | high | Expose one exact numeric parameter range from an in-memory device profile | TOP-REG-001 |
| `TOP-SIM-001` | BLOCKED_DEPENDENCY | high | Complete one scripted request-response exchange with deterministic correlation | TOP-CMD-003 |
| `TOP-SIM-002` | BLOCKED_DEPENDENCY | high | Reject a stale response from a previous connection generation | TOP-SIM-001 |
| `TOP-UI-001` | BLOCKED_DEPENDENCY | high | Adapt the session shell between narrow phone and wide tablet layouts | TOP-BOOT-003, TOP-FFI-001 |
| `TOP-UNDO-001` | BLOCKED_DEPENDENCY | high | Create an undo entry only after recording the confirmed previous value | TOP-PRESET-001, TOP-CMD-001 |
| `TOP-UNDO-002` | BLOCKED_DEPENDENCY | high | Create a new journal branch when preset context changes | TOP-UNDO-001, TOP-PRESET-002 |

## wave-03-am4-bootstrap

| ID | Status | Priority | Title | Dependencies |
|---|---|---|---|---|
| `TOP-AM4-E2E-001` | BLOCKED_HARDWARE | high | Read a physical AM4 preset name into visual and nonvisual UI | TOP-AM4-HIL-001, TOP-AM4-PROTO-003, TOP-A11Y-001 |
| `TOP-AM4-E2E-002` | BLOCKED_FIXTURE | high | Apply and undo one proven safe AM4 parameter change with read-back | TOP-AM4-E2E-001, TOP-UNDO-001 |
| `TOP-AM4-FIX-001` | BLOCKED_HARDWARE | critical | Acquire and approve one AM4 identity-query exchange fixture | TOP-RSCH-002, TOP-RSCH-003, TOP-RSCH-008, TOP-BOOT-006, TOP-BOOT-009 |
| `TOP-AM4-FIX-002` | BLOCKED_HARDWARE | critical | Acquire an AM4 live preset/read-buffer fixture with independently recorded metadata | TOP-AM4-HIL-001, TOP-BOOT-009 |
| `TOP-AM4-HIL-001` | BLOCKED_HARDWARE | critical | Discover a physical AM4 and select the exact profile | TOP-AM4-SIM-001, TOP-RSCH-004 |
| `TOP-AM4-PROTO-001` | BLOCKED_FIXTURE | high | Encode the provenance-approved AM4 identity query exactly | TOP-AM4-FIX-001, TOP-PRESET-002 |
| `TOP-AM4-PROTO-002` | BLOCKED_FIXTURE | high | Decode the provenance-approved AM4 identity response | TOP-AM4-FIX-001, TOP-DOM-003 |
| `TOP-AM4-PROTO-003` | BLOCKED_FIXTURE | high | Decode the AM4 preset fixture’s identity, name, and opaque remainder | TOP-AM4-FIX-002, TOP-PRESET-001 |
| `TOP-AM4-REG-001` | BLOCKED_FIXTURE | high | Resolve the captured AM4 firmware to one exact read/write profile | TOP-AM4-PROTO-002, TOP-REG-002 |
| `TOP-AM4-SIM-001` | BLOCKED_DEPENDENCY | high | Replay AM4 identity discovery through the simulator and registry | TOP-AM4-PROTO-001, TOP-AM4-PROTO-002, TOP-AM4-REG-001, TOP-SIM-001 |

## wave-04-fm3-bootstrap

| ID | Status | Priority | Title | Dependencies |
|---|---|---|---|---|
| `TOP-FM3-E2E-001` | BLOCKED_FIXTURE | critical | Read a complete physical FM3 preset into the normalized editor | TOP-FM3-HIL-001, TOP-FM3-HIL-002, TOP-PRESET-002, TOP-A11Y-001 |
| `TOP-FM3-E2E-002` | BLOCKED_FIXTURE | critical | Apply and undo one proven safe FM3 parameter change | TOP-FM3-E2E-001, TOP-UNDO-001 |
| `TOP-FM3-E2E-003` | BLOCKED_HARDWARE | critical | Recover truthfully from FM3 disconnect during a pending edit | TOP-FM3-E2E-001, TOP-SIM-002, TOP-UNDO-001 |
| `TOP-FM3-FIX-001` | BLOCKED_HARDWARE | critical | Acquire an FM3 identity exchange fixture over the selected control transport | TOP-FM3-RSCH-001, TOP-RSCH-008, TOP-BOOT-006, TOP-BOOT-009 |
| `TOP-FM3-HIL-001` | BLOCKED_HARDWARE | critical | Discover a physical FM3 over Android direct USB serial | TOP-FM3-NATIVE-002, TOP-FM3-SIM-001 |
| `TOP-FM3-HIL-002` | BLOCKED_HARDWARE | high | Discover a physical FM3 through the approved iOS-compatible interface path | TOP-FM3-IOS-001, TOP-FM3-SIM-001 |
| `TOP-FM3-IOS-001` | BLOCKED_DEPENDENCY | high | Enumerate the evidenced iOS FM3-compatible MIDI/interface path | TOP-FM3-RSCH-001, TOP-BOOT-003 |
| `TOP-FM3-NATIVE-001` | BLOCKED_DEPENDENCY | high | Enumerate a matching FM3 USB serial endpoint on Android without opening it | TOP-FM3-RSCH-001, TOP-BOOT-003 |
| `TOP-FM3-NATIVE-002` | BLOCKED_DEPENDENCY | high | Open, cancel, and close an Android FM3 USB serial session through a bounded adapter | TOP-FM3-NATIVE-001 |
| `TOP-FM3-PROTO-001` | BLOCKED_FIXTURE | high | Decode the approved FM3 identity response into typed identity | TOP-FM3-FIX-001, TOP-DOM-003 |
| `TOP-FM3-RSCH-001` | BLOCKED_DEPENDENCY | critical | Lock the FM3 transport and adapter matrix for iOS and Android | TOP-RSCH-004, TOP-RSCH-003, TOP-RSCH-002 |
| `TOP-FM3-SIM-001` | BLOCKED_DEPENDENCY | high | Replay FM3 discovery with serial fragmentation through simulator | TOP-FM3-PROTO-001, TOP-SIM-001, TOP-REG-002 |
