# TOP-RSCH-004 handoff

Status: `REVIEW_APPROVED` (bounded research only)

Completed the bounded research deliverable at `docs/research/mobile-transport-feasibility.md` and recorded the source inventory, environment, packet copy, and machine-readable matrix in this directory.

Key conclusions:

* CoreMIDI and Android MIDI/USB-host APIs make USB MIDI and class-compliant interface paths plausible, but every mobile cell remains a physical-test hypothesis.
* FM3 is the important exception: Fractal documents COMM/USB-Serial, not USB MIDI. Android raw `UsbManager` is an experiment; generic iOS serial is blocked by the cited public API boundary.
* Standard/Ultra have no built-in USB path; five-pin requires an external interface.
* Axe-Fx III's official MIDI guide explicitly limits realtime `PUSH DATA` tuner/tempo to physical MIDI OUT, not MIDI-over-USB. FM9/newer/legacy realtime routes are not generalized.
* SysEx/editor, large transfers, write verification, BLE fragmentation, bridge security, background continuity, and exclusive ownership are separate unknown capabilities. Note-on/simulator evidence is explicitly insufficient.

No production code, protocol fixture, profile, manifest, or shared integration file was changed. No RED/GREEN cycle applies to this research-only packet.

Blockers for promotion to implementation:

1. No physical iOS/Android devices, Fractal hardware, adapters, powered hubs, or captures were available in this packet.
2. FM3 serial framing and all non-Axe-Fx-III editor protocols need provenance-approved vectors or user-owned captures.
3. Exact OS/firmware/adapter tuples and background/exclusive behavior need L8 hardware-in-loop evidence.

Next owner: independent `topology_reviewer`, followed by the parent integrator. Recommended follow-up packets and ADR boundaries are listed in the report.
