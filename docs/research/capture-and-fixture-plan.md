# Lawful capture, sanitization, and fixture-contribution plan

**Packet:** `TOP-RSCH-008`  
**Requirements:** `CAPTURE-001`, `CAPTURE-002`, `CAPTURE-003`, `CAPTURE-004`, `QA-002`  
**Recorded:** 2026-08-08  
**Status:** `REVIEW_APPROVED` (research only; no fixture or hardware verification)  
**Scope:** research and documentation only; no production behavior or fixture bytes are created by this packet.

## Decision in one page

RigWarden should acquire evidence in this order:

1. published vendor facts and manuals;
2. a contributor's own, explicitly authorized read-only capture;
3. a permissively licensed implementation or vector, audited at the exact file and commit level;
4. a community capture with an explicit redistribution declaration; and
5. an independently generated simulator vector, labeled simulator-only.

The order is an engineering provenance priority, not a statement that a lower-ranked source is unlawful. A source is usable only when its exact device, variant, firmware, transport, direction, feature, provenance, sanitization, checksum, expected value, redistribution permission, and review state are recorded. This follows the repository provenance contract and schema (`docs/PROTOCOL_RESEARCH_AND_PROVENANCE.md`, `schemas/fixture-provenance.schema.json`, and `.codex/skills/enforce-topology-strict-tdd/references/protocol-fixture-provenance.md`).

The hard boundary for this plan is **read-only first**:

- Do not send an unknown opcode, frame, or write-shaped message to discover what it does.
- Do not capture or derive an unknown write operation, including by sniffing a vendor editor's write traffic.
- Do not infer a write from a nearby device, firmware, family member, round-trip symmetry, or a copied application implementation.
- Keep unknown write behavior `READ_ONLY`, `EXPERIMENTAL`, or `BLOCKED_FIXTURE` until a published specification or a separately approved, lawful fixture supplies the exact behavior.
- A bounded write verification is allowed only after the exact write is already evidence-backed. It is a hardware-verification procedure, not a reverse-engineering method.

Raw captures remain local and private by default. Only a sanitized derivative with a complete sidecar and an explicit contributor redistribution declaration may enter the repository. A missing declaration, ambiguous ownership, incomplete sanitization, or material legal uncertainty blocks merge; “the file was publicly downloadable” is not permission.

## Authority, evidence labels, and legal boundary

### Repository contracts (binding)

The following local contracts are the controlling requirements:

- `docs/PROTOCOL_RESEARCH_AND_PROVENANCE.md` requires a sidecar, independent expected values, a local Capture Lab, explicit export, sanitization, and no unknown-rights material.
- `schemas/fixture-provenance.schema.json` defines the required metadata fields and allowed source, direction, confidence, verification, and review values.
- `.codex/skills/enforce-topology-strict-tdd/references/protocol-fixture-provenance.md` requires immutable fixtures, checksums, explicit redistribution permission, and reviewer confirmation.
- `.codex/skills/enforce-topology-strict-tdd/references/hardware-verification.md` defines the minimum safe read/write and reconnection procedure; only the physical matrix can earn `HARDWARE_VERIFIED`.
- `docs/THREAT_MODEL.md` requires bounded parsing, secret redaction, exact profile matching, and no accidental capture upload.

These are project facts and do not need an external citation. They are restated here so an implementation packet cannot silently weaken them.

### External sources audited

The source list at the end of this report records access date and exact references. Key observations are:

- Fractal's current AM4 manual documents MIDI-over-USB, 3.5 mm Type-A MIDI in/out, a Type-A TRS-to-5-pin adapter, and an AM4-Edit SysEx surface; the current downloads page lists AM4 firmware 2.01 and the Windows USB driver. [S1][S2]
- Fractal's current FM3 manual explicitly says FM3 is **not** a USB MIDI device: editor communication uses “COMM over USB”/USB-serial channels, while ordinary MIDI uses the 5-pin MIDI In and Out/Thru ports; the downloads page lists current firmware 13.0 and separate Windows serial/audio drivers. [S3][S4]
- Fractal's published Axe-Fx III third-party MIDI PDF is a useful published source for that documented surface, but it is not a complete AM4 or FM3 editor protocol. It must not be stretched into a family-wide compatibility claim. [S5]
- Apple Core MIDI exposes device/entity/endpoint enumeration, SysEx I/O, BLE MIDI, and network MIDI; its BLE guidance says iOS 16+ can automatically reconnect paired peripherals. This supports an adapter plan, not proof that a particular Fractal unit is supported on iOS. [S6][S7][S8]
- Android's current USB-host and MIDI documentation requires device/feature enumeration and user permission before USB communication, and documents MIDI device/port discovery, hotplug callbacks, exclusive input access for sending, arbitrary-length SysEx, USB, and BLE transports. This is platform capability, not FM3 hardware evidence. [S9][S10]
- The audited `mcp-midi-control` repository provides useful *method* examples: tiered report/donate/probe/session/capture contributions, read-only probe gates, front-panel notes, single-action captures, local scratch captures, and distilled byte-exact goldens. Its exact commit and Apache-2.0 license were recorded, but its policy and artifacts are not RigWarden authority and must not override this packet's prohibition on unknown-write capture. [S11][S12][S13]
- Axis and ForgeFX were audited as reuse candidates at exact commits. Their root licenses are MIT, but the audit found no RigWarden-compatible fixture sidecar/redistribution record for direct fixture import. Their protocol claims and fixture bytes therefore remain research inputs, not approved RigWarden fixtures. [S14][S15]

This report is engineering policy, not jurisdiction-specific legal advice. Whether a vendor EULA, a particular capture, or a contributor's source material permits redistribution can require counsel. When that question is material, the status is `BLOCKED`; do not resolve it by confidence or by copying an open-source project's conclusion.

## Capture session types

Every session gets a unique private `session_id`, an explicit endpoint selection, a device/firmware record, and a human action log. A session may produce evidence without producing a distributable fixture.

| Type | Device interaction | What it can establish | Required output | Default status |
| --- | --- | --- | --- | --- |
| `REPORT` | None | A model/firmware/OS/port fact reported by a contributor | Written answer and source reference; no bytes | `COMMUNITY_CONFIRMED` only after review of the report; otherwise `PLANNED` |
| `DONATE` | None; reads an existing local file | A file's existence and declared origin | Original file stays private; sanitized derivative only if reviewed | `BLOCKED` until permission and sanitization are complete |
| `READ_PROBE` | Read-only query/dump/identity exchange | Device identity, read path, published catalog, or read response | Raw local capture, witness record, sanitized fixture if eligible | `CAPTURE_VERIFIED` after provenance review; otherwise `PLANNED` or `BLOCKED`; never a write claim |
| `PASSIVE_DEVICE_OUT` | Receives device output from a selected endpoint; sends nothing | Device-originated response/broadcast bytes | Raw local capture plus action/state notes | `CAPTURE_VERIFIED` only after provenance review |
| `DOCUMENTED_WRITE_VERIFY` | One already evidence-backed, bounded write followed by read-back and undo | Hardware confirmation of that known operation | Hardware matrix, sanitized logs, optional known-write fixture | `HARDWARE_VERIFIED` only for the tested matrix |
| `SIMULATOR_REPLAY` | No physical device | Deterministic parser/sequence behavior against a reviewed vector | Simulator fixture and transcript | `SIMULATOR_VERIFIED` / `simulator_only` confidence |
| `HARDWARE_MATRIX` | Repeats approved read/write procedure across declared OS/transport/adapter rows | Feature-specific physical compatibility | Matrix record, checksums, recovery notes | `HARDWARE_VERIFIED` only for rows actually run |

`DOCUMENTED_WRITE_VERIFY` is intentionally narrow. It may not be used to discover an unknown write. If the exact write frame, capability, profile, and expected acknowledgement are not already approved, stop at `READ_ONLY` and create a fixture/legal blocker.

**Status boundary:** `READ_ONLY`, `EXPERIMENTAL`, and `BLOCKED_FIXTURE` in this report are feature/session policy terms. They are not values for a fixture sidecar's `verification_status`. A sidecar must use only the enum accepted by `schemas/fixture-provenance.schema.json` (`PLANNED`, `BYTE_FIXTURE_VERIFIED`, `CAPTURE_VERIFIED`, `SIMULATOR_VERIFIED`, `COMMUNITY_CONFIRMED`, `HARDWARE_VERIFIED`, or `BLOCKED`).

### Universal preflight

Before any device session:

1. Record the exact model, hardware variant, firmware string, host/mobile device, OS, transport, adapter model/firmware, app commit, and session ID. Use a stable token for a serial; never put the serial in a fixture or report.
2. Confirm the endpoint manually. Do not pick the first matching endpoint. Record all candidate endpoint names when ambiguity matters.
3. Close other editors, firmware updaters, and MIDI/serial clients. Exclusive-port conflicts must be diagnosed, not fought.
4. Mute or turn down connected monitoring before connecting. Use a disposable working preset or a contributor-owned backup for any later documented write test. RigWarden itself must not perform an automatic backup before a first write.
5. Start the monitor before the query so the complete response is retained. Select one session only; do not collect unrelated MIDI traffic.
6. Confirm the selected session's outgoing allowlist. A `READ_PROBE` allowlist contains only approved identity/read requests. Anything else aborts before transport.
7. Annotate every human action and expected result, including “no action” intervals. For a panel-visible result, write the panel result down before consulting the editor UI.
8. Keep the raw capture local. Do not upload, sync, attach, or commit it automatically.

### `REPORT` and `DONATE`

These are the lowest-risk contributions and require no device connection. A contributor may report an OS port name, firmware screen, or manual section. For a donated cache or preset file, the contributor first identifies what the file contains and confirms it came from a device/editor they are authorized to use. The file is opened locally and scanned for serials, usernames, paths, keys, personal preset names, and unrelated data before any derivative is prepared. A public download location alone does not satisfy the declaration.

### `READ_PROBE`

The probe's wire gate must mechanically reject anything outside the read allowlist. The initial allowlist is deliberately small:

- identity/discovery query from a published specification or approved fixture;
- device-resident catalog/read operation whose message is documented or already approved;
- current working-buffer read or dump that does not switch, save, or mutate a preset;
- acknowledgement/read-back of a known, already approved operation, when required to verify a prior operation.

The probe must never save, overwrite, switch presets/scenes, change a parameter, enable a modifier, or emit arbitrary raw bytes. If a device's read protocol cannot be separated from a write or state-changing action, the operation is not a read probe and is blocked.

For every expected response, record the state shown on the device before the query, the query identity, and the result observed on the device or an independent manual. A timeout is a meaningful negative result when the method was otherwise valid; it is not permission to try an unapproved write.

### `PASSIVE_DEVICE_OUT`

Use this only when the transport exposes device-to-host bytes without **any** outbound message from RigWarden. Select one endpoint and observe one short device-originated broadcast or other unsolicited device output. An identity/read request is outbound and belongs only to `READ_PROBE` under its allowlist; it is never passive. For each capture, note whether the vendor editor was closed and whether any external controller could have generated traffic. Multi-megabyte sessions and mixed actions are not fixtures; they are private analysis material until reduced to an independently explained vector.

### `DOCUMENTED_WRITE_VERIFY`

This procedure is allowed only after an exact profile and write frame have been approved from a published spec or provenance-reviewed fixture:

1. Read the current value.
2. Record the confirmed prior state in the journal and on the hardware matrix.
3. Send one bounded, known-safe change within the documented capability/range.
4. Require the profile's acknowledgement or read-back.
5. Confirm the device's displayed value and the application's reconciled value.
6. Read independently again where supported.
7. Undo and confirm the original value is restored.
8. Disconnect/reconnect and confirm consistency where relevant.

If any step disagrees, stop, preserve the raw local evidence, mark the feature `BLOCKED`, and do not broaden the profile. No batch is called atomic unless the hardware guarantees atomicity.

## Sanitization and review

### Token vocabulary

Use stable, per-submission tokens so a reviewer can compare raw and sanitized records without publishing the underlying value. The tokens below are text labels; they are not permission to change protocol bytes:

| Token | Replace |
| --- | --- |
| `<REDACTED_SERIAL_001>` | Device serial or serial-like identifier |
| `<REDACTED_USB_UID_001>` | OS USB/MIDI unique ID not required for the protocol |
| `<REDACTED_ACCOUNT_001>` | Account, e-mail, or user name |
| `<REDACTED_HOST_001>` | Host name, IP, MAC, or network bridge identity |
| `<REDACTED_PATH_001>` | Local path, home directory, or cloud-provider path |
| `<REDACTED_KEY_001>` | API key, token, password, or credential |
| `<REDACTED_PRESET_001>` | Personal preset/scene name or note |
| `<REDACTED_TIMESTAMP_001>` | Timestamp unnecessary to the behavior |
| `<REDACTED_MIDI_001>` | Unrelated MIDI/serial traffic |
| `<REDACTED_VENDOR_ART_001>` | Vendor artwork or layout material that must not ship |

When two values must remain distinguishable, increment the suffix deterministically within that submission. Never use a real serial, user name, or path as a fixture ID.

### Byte-preserving rule

For text/JSON metadata, token substitution is normally safe. For a binary capture, do not alter bytes that determine framing, checksum, identity, value, or parser behavior. Instead:

- remove unrelated messages at message boundaries;
- keep a private raw copy and a reviewer-visible sanitization manifest;
- derive a new sanitized fixture only when the changed field is proven non-behavioral and the expected value is independently re-derived; or
- leave the artifact private and mark redistribution `BLOCKED`.

If a serial or user-specific value is embedded in a checksum-covered protocol field, replacing it changes the message. Create a new fixture ID/version, document the replacement and re-derived checksum, and do not call it a byte-identical capture of the original. If that re-derivation would require guessing, do not export it.

Never redact by blindly zeroing bytes, truncating frames, or rewriting a checksum. A sanitization that cannot preserve or independently re-derive the semantic contract is not a usable fixture.

### Review checklist

An independent reviewer (not the contributor who made the capture) checks:

- exact device, variant, firmware, transport, direction, and feature;
- source URL/commit/manual section and capture date;
- contributor ownership/authorization declaration;
- raw and sanitized SHA-256 values and file lengths;
- all token replacements and any removed message boundaries;
- absence of serials, paths, credentials, personal names, unrelated traffic, vendor binaries/artwork, or confidential material;
- expected parser result derived independently from the codec;
- checksum/framing and malformed/truncated behavior where applicable;
- distribution license and basis;
- verification label scope, including OS/adapter/firmware limits.

The reviewer sets `review.status` to `approved` only after every item is answered. A disagreement produces a new fixture version or `BLOCKED`; it never silently edits an immutable fixture.

## Contributor redistribution declaration

RigWarden should present this declaration at export time and store the resulting text or a signed reference in the private submission record. A checkbox with no text, a “publicly available” statement, or an issue attachment with no contributor declaration is insufficient.

> I am the owner of, or authorized by the owner to submit, the source material identified by this submission. I have described the exact device, firmware, transport, source, and feature. I grant RigWarden a non-exclusive, worldwide, royalty-free license to redistribute the **sanitized artifact and its provenance sidecar** under the SPDX license named below, and to make format/sanitization changes that do not change the documented protocol meaning. I understand that the raw capture stays private by default and is not redistributed by RigWarden unless I separately authorize it. I did not include vendor application binaries, vendor artwork, copied layouts, credentials, confidential information, or data belonging to another person without authorization. I understand that a merged fixture is public test data and that attribution may use the pseudonym I provide. I will identify any third-party material or legal restriction I discover before merge.

Required fields alongside the declaration:

```text
contributor_id: <stable pseudonym or private contact token>
authorization_basis: owner | written_permission | employer_permission | other-described
fixture_license_spdx: <exact SPDX ID or BLOCKED_PENDING_REVIEW>
credit_preference: named | pseudonymous | no_credit
raw_redistribution: false
sanitized_redistribution: true | false
declaration_recorded_at: <UTC timestamp>
```

The project may choose a default fixture-data license by ADR (for example, an explicit SPDX data license); this packet does not choose one silently. If the contributor cannot grant redistribution under a clear license, keep the artifact private and mark it `BLOCKED_FIXTURE`. A broad upstream software license does not automatically license embedded captures, preset content, or personal files.

## Raw versus sanitized storage

| Layer | Location and access | Contents | Repository/test use |
| --- | --- | --- | --- |
| Raw quarantine | Contributor-controlled local storage, e.g. `capture-lab/raw/<session-id>/`, access restricted to the contributor/reviewer | Original `.pcapng`, `.syx`, serial log, editor export, notes | Never committed; never used as a public golden |
| Sanitization work area | Local `capture-lab/work/<session-id>/` | Candidate derivative, token map, action log, independent witness record | Deleted or archived by contributor; not a source of truth |
| Public fixture | `fixtures/<family>/<model>/<firmware>/<transport>/<feature>/` after later implementation packet | Sanitized bytes, sidecar, expected value, checksum | Compatibility tests may consume only approved files |
| Private hardware evidence | `.tdd/evidence/<work-item-id>/` with secrets removed | Matrix, reviewer notes, sanitized logs/checksums | Supports the stated verification label; raw bytes remain external/private |

Raw storage is opt-in retention, local-first, and user-deletable. There is no automatic upload, cloud sync, telemetry, or background export. If raw material is deleted before review, the sanitized derivative cannot be approved unless its provenance and independent expected values remain reproducible. If raw material is retained privately, the public sidecar must not imply that anyone can obtain it.

The public fixture's checksum is the checksum of the distributed sanitized file. A private raw checksum may be recorded in a private contributor/reviewer record, but it is not a substitute for the public fixture checksum.

## Independent expected-value derivation

The expected value is a witness record, not an output generated by the codec under test.

### Derivation workflow

1. **State the question.** Example: “Does this approved identity response represent FM3, firmware 13.0, on the selected transport?” Do not ask “does encode(decode(x)) round trip?” as the only question.
2. **Create the witness before running the decoder.** Transcribe the expected model/firmware, field offsets, value/range, and checksum from the published source or from a human-observed device state. Record source page/section or panel observation.
3. **Capture/read once, then repeat.** Repeat the same read query when possible. Compare independent sessions and preserve discrepancies as separate fixture versions.
4. **Use a second independent check.** This may be a manual calculation, published table, a separate parser implementation, an independently reviewed open-source vector, or a front-panel/read-back observation. It must not call the production encoder/decoder.
5. **Compare literal values.** Decoder tests assert the witness object against the captured bytes. Encoder tests assert literal expected bytes transcribed from the source/witness. A production checksum helper must not be the only way expected checksum bytes are produced.
6. **Add round-trip and malformed cases.** Round-trip is a useful invariant after independent vectors exist; it is not compatibility evidence by itself. Add minimum/maximum lengths, truncated input, invalid checksum/framing, unknown fields, and opaque preservation where required.
7. **Resolve disagreement explicitly.** Keep both immutable inputs, record the contradictory source, and mark the feature `BLOCKED` or `EXPERIMENTAL`. Never update a golden in place to create green.

### What each evidence type can claim

- `published_spec` confidence can support a byte/file fixture only for the documented scope and exact version/transport.
- `direct_capture_single` is a single observation; it needs an independent expected value and reviewer before merge.
- `direct_capture_repeated` strengthens repeatability but still does not prove another firmware/adapter.
- `cross_implementation` is a cross-check, not permission to copy unknown data.
- `community_unreplicated` stays unverified until another reviewer or capture confirms it.
- `simulator_only` proves only the simulator behavior.

`BYTE_FIXTURE_VERIFIED` and `CAPTURE_VERIFIED` do not imply physical compatibility. `HARDWARE_VERIFIED` is earned only by the declared Layer 8/modeler matrix after integration.

## Fixture directory, naming, and versioning

The implementation packets should adopt this layout:

```text
fixtures/
  am4/
    fw-2.01/
      usb-midi/
        identity/
          am4-fw-2.01-usb-midi-device-to-host-identity-0001.v1.syx
          am4-fw-2.01-usb-midi-device-to-host-identity-0001.v1.provenance.yaml
  fm3/
    fw-13.0/
      usb-comm-serial/
        identity/
          fm3-fw-13.0-usb-comm-serial-device-to-host-identity-0001.v1.bin
          fm3-fw-13.0-usb-comm-serial-device-to-host-identity-0001.v1.provenance.yaml
```

Naming rules:

- Lowercase path tokens: `<family>/<model>/<firmware>/<transport>/<feature>/`.
- Preserve exact firmware text in the sidecar. If a firmware string is opaque, use `fw-opaque-<stable-token>` and explain the token; do not coerce it to a semantic version.
- Use transport tokens such as `usb-midi`, `usb-comm-serial`, `midi-5pin`, `ble-midi`, and `network-bridge` only when that path was actually observed. A proposed path is a matrix row, not a fixture.
- Use the direction enum from the schema (`host_to_device`, `device_to_host`, `bidirectional`, or `file`) in the sidecar and in the fixture ID where useful.
- Use a stable feature token (`identity`, `preset-read`, `parameter-read`, `status`, or `documented-write-verify`). Never call an unknown write feature “write” in a fixture path.
- Fixture IDs are immutable. A correction creates a new ID or `.v2` artifact and documents the superseded ID in reviewer notes; bytes and checksums are never silently replaced under an existing ID.
- Sidecar basename, fixture basename, and `fixture_id` must agree. `sha256` is the hash of the sanitized fixture file.
- A sidecar's `expected` object contains literal expected values, not values generated by invoking the codec. Large expected objects may be a separate reviewed file referenced by the sidecar only after the schema/validator packet defines that extension.

The initial AM4/FM3 fixture packets should create only identity/read fixtures. Any later write fixture must cite the exact published/approved write source and a completed `DOCUMENTED_WRITE_VERIFY` matrix.

## AM4 and FM3 hardware/adapter plan

This is a capability plan, not a claim that the current environment has either unit. The current environment has no attached AM4/FM3 and no approved RigWarden capture, so physical rows remain blocked until a contributor supplies lawful hardware evidence.

### AM4

**Documented facts.** The current AM4 manual describes MIDI-over-USB for AM4-Edit/Fractal-Bot/other MIDI applications, 3.5 mm Type-A MIDI In/Out, and a Type-A TRS-to-5-pin DIN adapter for traditional MIDI. It says AM4-Edit uses SysEx and that Scene MIDI is transmitted only through physical MIDI Out, not USB MIDI. The current support page lists AM4 firmware 2.01 and a Windows 11+ USB driver; macOS is described as driverless in the manual. [S1][S2]

**Capture rows.**

| Row | Path | Minimum hardware | Status and procedure |
| --- | --- | --- | --- |
| AM4-D1 | Desktop USB MIDI | AM4, USB cable, current official driver where required, host MIDI monitor | First identity/read probe. Enumerate and select the AM4 endpoint; capture only approved read traffic. |
| AM4-D2 | 5-pin MIDI | AM4 Type-A TRS adapter, bidirectional class-compliant MIDI interface, 5-pin cables | Read-only parity check against AM4-D1. Do not assume SysEx lengths/throughput match USB. |
| AM4-D3 | iOS/iPadOS via MIDI interface | Class-compliant bidirectional MIDI interface plus the device's approved USB-C/Lightning adapter | Feasibility row; Apple endpoint/SysEx support does not prove AM4 support. Start with identity/read only. |
| AM4-D4 | Android via USB host or BLE/bridge | Android host-capable device/OTG path or paired BLE/network bridge | Experimental feasibility row. Require platform permission and explicit endpoint selection; no direct AM4 claim without a capture. |

The AM4 manual's distinction between USB MIDI and physical Scene MIDI means a physical-MIDI capture cannot be silently substituted for a USB-MIDI fixture. Keep transport in the fixture ID and sidecar.

### FM3

**Documented facts.** The current FM3 manual says FM3 is not a USB MIDI device, uses “COMM over USB” channels for FM3-Edit/Fractal-Bot, uses a USB type-B port, and requires separate Windows USB-Serial and audio drivers; it documents MIDI In and Out/Thru as 5-pin ports. The current downloads page lists firmware 13.0 and Windows audio/serial driver versions. [S3][S4]

**Capture rows.**

| Row | Path | Minimum hardware | Status and procedure |
| --- | --- | --- | --- |
| FM3-D1 | Desktop USB COMM/serial | FM3 (exact Original/Mark II Turbo variant), USB type-B cable, official serial/audio drivers on Windows or driverless macOS path as applicable | Primary editor/read fixture path. Enumerate the serial/COMM endpoint; send only approved identity/read query. Do not label this `usb-midi`. |
| FM3-D2 | Android direct USB serial | FM3, OTG/USB host-capable Android device, USB permission flow, bounded serial adapter | Work-item target, not a vendor-confirmed compatibility claim. Enumerate without opening first; open only after permission and endpoint identity match; run read-only identity exchange. |
| FM3-D3 | iOS/iPadOS through 5-pin MIDI interface | FM3 5-pin MIDI In/Out, bidirectional class-compliant MIDI interface, approved iOS adapter | Primary iOS feasibility row because the FM3 manual does not describe direct USB MIDI. Use Core MIDI endpoint/SysEx APIs; verify a real FM3 capture before any claim. |
| FM3-D4 | BLE MIDI or local network bridge | A paired BLE MIDI interface or explicitly paired local bridge | Experimental. Apple/Android platform APIs support these transport families, but no FM3-specific path is implied. Identity/read only until hardware evidence exists. |

Android USB host documentation supports enumeration, permission, endpoint access, and attach/detach handling; it does not identify an FM3 endpoint or prove the FM3's serial framing. Apple Core MIDI documentation supports MIDI endpoint/SysEx/BLE/network APIs; it does not turn the FM3's COMM-over-USB channel into a Core MIDI endpoint. Those device-specific rows remain hypotheses until a physical capture closes them. [S6][S7][S8][S9][S10]

### Minimum hardware kit

The first hardware contributors should have:

- one AM4 with exact variant and firmware recorded (current support page: 2.01), plus USB cable and the official driver where required;
- one FM3 with exact Original/Mark II Turbo variant and firmware recorded (current support page: 13.0), plus USB type-B cable and official serial/audio drivers where required;
- an AM4 Type-A 3.5 mm TRS-to-5-pin DIN adapter;
- a bidirectional, class-compliant MIDI interface with 5-pin In and Out and cables;
- a USB-C/Lightning host adapter for iOS/iPadOS and an OTG/host-capable Android adapter/device;
- optional BLE MIDI adapter or local bridge only for experimental rows;
- safe monitoring/mute controls and a contributor-owned disposable preset/backup for any later documented write verification.

The kit list is a test prerequisite, not a recommendation to purchase a specific brand. Adapter model and firmware belong in every matrix row because a transport claim is four-dimensional: device, firmware, OS, and transport/adapter.

## Blockers, hypotheses, and follow-up packets

### Verified facts

- The repository requires explicit provenance sidecars, independent expected values, sanitization, and redistribution permission before fixture merge.
- AM4 currently documents USB MIDI and physical Type-A 3.5 mm MIDI; AM4 firmware/driver versions and macOS/Windows USB requirements are on the current official pages/manual.
- FM3 currently documents USB COMM/serial rather than USB MIDI and 5-pin MIDI In/Out; current firmware and Windows driver rows are on the official support page.
- Apple and Android document the platform transport primitives listed above; those documents do not prove a Fractal-specific endpoint.
- The audited open-source contribution guides distinguish hardware-confirmation tiers from wire-decoding and insist on front-panel notes, local raw capture, and single-action records. We reuse those methods only where they do not conflict with the unknown-write prohibition.

### Hypotheses (must not become compatibility claims)

- `H-AM4-MOBILE`: AM4 USB MIDI may be reachable from a particular iOS/iPadOS or Android adapter path; no current Fractal document reviewed here establishes that path.
- `H-FM3-ANDROID`: the FM3 COMM/serial endpoint may be reachable through Android USB host mode; the packet's native adapter work must verify it without sending writes.
- `H-FM3-IOS`: an iOS-compatible FM3 path is likely to require a 5-pin MIDI interface, BLE MIDI adapter, or local bridge rather than direct USB COMM; this is an engineering hypothesis based on the FM3 manual's “not USB MIDI” statement and Apple Core MIDI's documented endpoint model, not a hardware result.
- `H-RAW-FIELD`: a binary field that looks like a serial or user identifier may be removable without changing protocol meaning; this must be demonstrated per fixture or left private.

### Unknowns

- AM4 and FM3 full editor/read-buffer formats and firmware-specific field maps beyond published documentation.
- Exact current device endpoint names, serial framing, maximum safe chunk sizes, and adapter-specific behavior.
- Whether a particular mobile adapter exposes the required bidirectional SysEx/COMM path under lifecycle/background conditions.
- Whether a contributor's capture contains third-party material or personal data that can be licensed after sanitization.

### Current blockers

- No attached AM4 or FM3 hardware is available in this environment; `TOP-AM4-FIX-001`, `TOP-AM4-FIX-002`, `TOP-FM3-FIX-001`, `TOP-AM4-HIL-001`, `TOP-FM3-HIL-001`, and `TOP-FM3-HIL-002` remain hardware/fixture blocked.
- No lawful, provenance-complete RigWarden fixture corpus exists yet; do not manufacture bytes from a self-consistent codec or copy an external capture.
- Any EULA, copyright, privacy, or contributor-ownership question that cannot be resolved from the written declaration and source records requires counsel or a narrower fixture scope.

### Follow-up work

- `TOP-BOOT-006`: validator rejects missing/false redistribution permission (`CAPTURE-004`).
- `TOP-BOOT-007`: validator rejects incomplete evidence records.
- `TOP-BOOT-009`: deterministic fixture/provenance validator and sidecar checks.
- `TOP-AM4-FIX-001` / `TOP-AM4-FIX-002`: identity and read-buffer fixtures using the AM4-D1 row first.
- `TOP-FM3-FIX-001`: identity fixture using FM3-D1, then FM3-D2/D3 only when the transport rows are physically evidenced.
- `TOP-AM4-HIL-001`, `TOP-FM3-HIL-001`, `TOP-FM3-HIL-002`: physical rows and reconnect/recovery matrices.
- Proposed ADRs for the integration owner: fixture-data license default; raw-retention/deletion policy; contributor declaration UX and credit policy; AM4/FM3 adapter capability matrix; explicit unknown-write prohibition and escalation path.

## Source register (all accessed 2026-08-08)

### Official vendor/platform sources

- **[S1] Fractal Audio, AM4 Downloads.** Current manual/firmware/driver index, including firmware 2.01 and Windows USB-driver requirements.  
  <https://www.fractalaudio.com/am4-downloads/>
- **[S2] Fractal Audio, AM4 Owner's Manual v1.0.2 (2025-12-16).** USB MIDI, Type-A TRS MIDI, AM4-Edit/SysEx, physical MIDI versus USB MIDI, MIDI implementation, and OS/driver notes.  
  <https://www.fractalaudio.com/downloads/manuals/AM4/AM4-Owners-Manual.pdf>
- **[S3] Fractal Audio, FM3 Downloads.** Current manual/firmware/driver index, including firmware 13.0 and separate Windows serial/audio drivers.  
  <https://www.fractalaudio.com/fm3-downloads/>
- **[S4] Fractal Audio, FM3 Owner's Manual (current 7.x manual).** “Not a USB MIDI Device,” COMM-over-USB, USB-serial/audio driver requirements, and 5-pin MIDI In/Out/Thru.  
  <https://www.fractalaudio.com/downloads/manuals/FM3/FM3-Owners-Manual.pdf>
- **[S5] Fractal Audio, Axe-Fx III MIDI for Third-Party Devices, PDF revision 1.4.** Published third-party MIDI surface; use only for its documented scope.  
  <https://fractalaudio.com/downloads/misc/Axe-Fx%20III%20MIDI%20for%203rd%20Party%20Devices.pdf>
- **[S6] Apple Developer Documentation, Core MIDI.** MIDI devices/endpoints, SysEx, BLE, and networking API collections.  
  <https://developer.apple.com/documentation/coremidi>
- **[S7] Apple Developer Documentation, MIDI Services.** Device/entity/source/destination enumeration and SysEx I/O operations.  
  <https://developer.apple.com/documentation/coremidi/midi-services>
- **[S8] Apple Developer Documentation, MIDI Bluetooth.** BLE MIDI service/characteristic checks and iOS 16+ paired-peripheral reconnection behavior.  
  <https://developer.apple.com/documentation/coremidi/midi-bluetooth>
- **[S9] Android Developers, USB host overview.** USB host capability, enumeration, permissions, interfaces, and endpoints.  
  <https://developer.android.com/develop/connectivity/usb/host>
- **[S10] Android Developers, `android.media.midi` API reference.** MIDI device/port discovery, hotplug, exclusive input access, arbitrary-length SysEx, USB/BLE transports.  
  <https://developer.android.com/reference/android/media/midi/package-summary>

### Open-source method/license audits

- **[S11] TheAndrewStaker/mcp-midi-control, exact commit `59047175cfc4f23e092931b54a7c54f2bffde3ea`, `CONTRIBUTING.md`.** Tiered contribution flow, single-action captures, local scratch capture, and distilled goldens.  
  <https://github.com/TheAndrewStaker/mcp-midi-control/blob/59047175cfc4f23e092931b54a7c54f2bffde3ea/CONTRIBUTING.md>
- **[S12] TheAndrewStaker/mcp-midi-control, exact commit `59047175cfc4f23e092931b54a7c54f2bffde3ea`, `docs/contributing/SAFETY.md` and `docs/contributing/TIERS.md`.** Read-only probe gate, port exclusivity, safety boundaries, and contribution tiers.  
  <https://github.com/TheAndrewStaker/mcp-midi-control/tree/59047175cfc4f23e092931b54a7c54f2bffde3ea/docs/contributing>
- **[S13] TheAndrewStaker/mcp-midi-control, exact commit `59047175cfc4f23e092931b54a7c54f2bffde3ea`, `docs/contributing/EVIDENCE.md`.** Evidence labels and independent-check guidance; audited as method, not adopted as a legal conclusion.  
  <https://github.com/TheAndrewStaker/mcp-midi-control/blob/59047175cfc4f23e092931b54a7c54f2bffde3ea/docs/contributing/EVIDENCE.md>
- **[S14] sKuhLight/Axis, exact commit `6b87bd2472fd88854421fda0dd1d2d7a02d2dd19`, root `LICENSE` (MIT).** Reuse candidate audit; no RigWarden fixture sidecar found in the audited tree.  
  <https://github.com/sKuhLight/Axis/tree/6b87bd2472fd88854421fda0dd1d2d7a02d2dd19>
- **[S15] sKuhLight/ForgeFX, exact commit `c22862a5b2f2078f3cb92a2735e51f94c39a0062`, root `LICENSE` (MIT) and test fixture tree.** Reuse candidate audit; fixture files lack a RigWarden-compatible redistribution sidecar in the audited tree.  
  <https://github.com/sKuhLight/ForgeFX/tree/c22862a5b2f2078f3cb92a2735e51f94c39a0062>

### Licensing/provenance references

- **[S16] GitHub Docs, “Licensing a repository.”** A public repository without a license remains under default copyright; license information is guidance and not legal advice.  
  <https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/licensing-a-repository>
- **[S17] SPDX, “Handling License Info.”** Use exact SPDX identifiers/expressions and match the license text; identifiers make file-level obligations machine-readable.  
  <https://spdx.dev/learn/handling-license-info/>
