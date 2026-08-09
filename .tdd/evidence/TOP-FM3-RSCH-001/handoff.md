# TOP-FM3-RSCH-001 handoff

Status: `REVIEW_PENDING` (bounded research complete; independent review not
self-approved)

## Behavior delivered

`docs/research/fm3-transport-matrix.md` answers the bounded FM3 Android/iOS
transport question with source-backed `FACT_DEVICE`/`FACT_PLATFORM`,
`HYPOTHESIS`, `UNKNOWN`, `BLOCKED`, `CONDITIONAL`, and
`SECONDARY/UNVERIFIED` labels. It explicitly separates:

* FM3's official USB audio + `COMM over USB` editor path from generic USB MIDI;
* Android `UsbManager` mechanics from an unobserved FM3 raw USB endpoint;
* ordinary iPhone direct serial (`BLOCKED`) from class-compliant 5-pin MIDI
  (`HYPOTHESIS`);
* M-series iPadOS 16+ USBDriverKit (`CONDITIONAL`) from ordinary CoreMIDI;
* BLE-MIDI and network bridge hypotheses from Fractal-native transport facts;
* basic PC/CC/clock possibilities from unknown full-editor/SysEx,
  backup/dump, realtime, throughput, and write/read-back behavior.

## Files changed

* `docs/research/fm3-transport-matrix.md`
* `.tdd/evidence/TOP-FM3-RSCH-001/work-item.yaml`
* `.tdd/evidence/TOP-FM3-RSCH-001/environment.txt`
* `.tdd/evidence/TOP-FM3-RSCH-001/source-index.md`
* `.tdd/evidence/TOP-FM3-RSCH-001/source-link-check.txt`
* `.tdd/evidence/TOP-FM3-RSCH-001/sweep-commands.txt`
* `.tdd/evidence/TOP-FM3-RSCH-001/sweep.log`
* `.tdd/evidence/TOP-FM3-RSCH-001/sweep-exit-statuses.txt`
* `.tdd/evidence/TOP-FM3-RSCH-001/files-changed.txt`
* `.tdd/evidence/TOP-FM3-RSCH-001/review.md`
* `.tdd/evidence/TOP-FM3-RSCH-001/handoff.md`

No product code, shared manifest, profile, fixture, capture, protocol bytes,
vendor binary/artwork, credential, or generated build output was created.

## Source and provenance decisions

* Official FM3 manual/download sources are the device authority for the USB
  boundary, 5-pin MIDI, backup/dump limitation, SysEx/editor statement, and
  firmware/driver dates.
* Official Android and Apple API sources establish platform mechanics only.
  They do not identify an FM3 endpoint, expose COMM framing, or prove editor
  compatibility.
* Community `mcp-midi-control` and ForgeFX identifiers are retained only as
  pinned, secondary capture leads (`2466`, `8011`, `if03`/`MI_03`). No CDC,
  nominal baud, framing, or codec behavior is inferred or copied.
* Every external URL in the report/source index was checked on 2026-08-09;
  25 URLs returned HTTP 200, including the cited Apple EASession and
  community LICENSE/NOTICE pages.
* Local links were checked after correction. The report links the source audit
  from its same directory; the evidence index uses the repository-root-relative
  `../../../docs/research/...` path.

## TDD/evidence boundary

This is a research packet. Under the strict-TDD research exception, no RED or
GREEN command applies because no executable production behavior was written.
The validator command exited 2 with missing PyYAML/jsonschema and is recorded
as `BLOCKED_ENVIRONMENT`; no schema-validation claim is made. The corrected
fail-fast evidence sweeps passed packet-copy, source-label, relative-link,
no-raw-byte, whitespace, and scoped-write checks.

## Claims earned

* Source-backed FM3 USB/editor versus 5-pin distinction.
* Android raw USB host and Android/Apple MIDI API boundaries.
* Ordinary iPhone direct FM3 serial is blocked in the reviewed public API set;
  M-series iPadOS DriverKit is a separate conditional route.
* Adapter, permission, lifecycle, fragmentation, security, and power risks
  are named for Android/iOS 5-pin, BLE, and bridge paths.
* Follow-up packets and `TOP-ADR-004` boundary decision are named without
  creating production work.

## Claims unavailable

No mobile transport, editor, SysEx, identity, backup, realtime, throughput,
write, firmware, platform-device, capture, or hardware compatibility claim.
No `BYTE_FIXTURE_VERIFIED`, `CAPTURE_VERIFIED`, `PLATFORM_DEVICE_VERIFIED`, or
`HARDWARE_VERIFIED` label.

## Follow-up work

Existing packets: `TOP-FM3-NATIVE-001`, `TOP-FM3-NATIVE-002`,
`TOP-FM3-IOS-001`, `TOP-FM3-FIX-001`, `TOP-FM3-PROTO-001`,
`TOP-FM3-HIL-001`, and `TOP-FM3-HIL-002`.

Proposed bounded packets: `TOP-FM3-IOS-DRIVERKIT-001`, `TOP-FM3-BLE-001`, and
`TOP-FM3-BRIDGE-001`. Extend/schedule `TOP-ADR-004` to preserve the iPhone
blocked versus M-series iPadOS conditional distinction. Do not amend the
existing wave-00 report from this worker lane.

## Blockers

* No FM3, Android/iOS device, MIDI interface, BLE adapter, powered hub, bridge,
  descriptor capture, or approved identity/read fixture was available.
* FM3 COMM framing, identity exchange, endpoint ownership, and full editor
  behavior remain unknown.
* M-series iPadOS DriverKit requires separate entitlement/distribution and
  hardware feasibility work.
* Community identifiers remain secondary/unverified and cannot authorize
  endpoint matching or writes.

Independent reviewer: `/root/fm3_rsch001_review`; review record remains
`REVIEW_PENDING` in `review.md`.
