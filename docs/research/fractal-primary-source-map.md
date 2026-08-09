# Fractal primary source map

**Packet:** `TOP-RSCH-003`  
**Requirements:** `DEV-003`, `PACK-002`, `QA-002`  
**Recorded/accessed:** 2026-08-08 (America/Chicago)  
**Status:** `REVIEW_APPROVED` — bounded source/provenance research only; no protocol or hardware compatibility claim.  
**Research route:** OpenAI `gpt-5.6-luna`, `max` reasoning, using the official Fractal support site and linked manuals/release archives.  

## Decision

Topology may claim only the standard MIDI controls and file/editor capabilities
that the named device manual or a Fractal-published MIDI document actually
states. The official material does **not** publish one cross-device, full-editor
wire protocol. In particular, it does not establish a portable protocol for
device discovery, parameter catalogs, grid placement/cabling, save-to-location,
large preset/cab transfers, acknowledgements, or firmware-update framing.

The one detailed published third-party control specification located in the
official support corpus is **Axe-Fx III MIDI for 3rd Party Devices**, revision
1.4, for Axe-Fx III firmware 1.13 and later. It names supported control/query
operations and documents optional tuner/tempo realtime pushes. It is not a
specification for Axe-Edit III's complete editor, and it is not automatically a
specification for FM9, FM3, or any legacy model.

This report therefore separates:

* **FACT** — stated in a current Fractal support page, device manual, Fractal
  release note, or the Fractal-published third-party MIDI PDF.
* **SECONDARY** — a published archive, community wiki, or open-source project;
  useful corroboration or a capture lead, never a vendor compatibility claim.
* **HYPOTHESIS** — a bounded inference that must not enable a write.
* **UNKNOWN/GAP** — not specified by the sources reviewed.
* **BLOCKER** — evidence or authority required before Topology can make the
  proposed claim.

No SysEx bytes are reproduced or derived here. Command names and feature
categories are retained only to describe the documented boundary.

## Per-device inventory

The firmware value is the current value shown by the official downloads page on
the access date. A manual's “current as of” value is recorded separately; the
two are intentionally not conflated.

| Target | Official firmware/release source and manual coverage | Identity/discovery and transport | Documented control messages | Preset/file and realtime facts | Cab / FC facts | Full-editor boundary |
| --- | --- | --- | --- | --- | --- | --- |
| **AM4** | **FACT:** [AM4 downloads](https://www.fractalaudio.com/am4-downloads/) lists firmware **2.01 (2026-06-04)**, factory presets for 1.01+, and a shared AM4/VP4 Windows USB driver. The linked [AM4 Owner's Manual](https://www.fractalaudio.com/downloads/manuals/AM4/AM4-Owners-Manual.pdf) is PDF revision **v1.0.2 (2025-12-16)**; the support listing is dated 2025-11-24. | **FACT:** Type-A 3.5 mm TRS MIDI IN/OUT, optional MIDI Thru, USB audio and USB MIDI/editor connection. `SETUP > System Info` exposes firmware, DynaCab version, release date, and CPU level. **UNKNOWN:** no published identity query, device-ID handshake, endpoint enumeration, or USB editor framing. | **FACT:** MIDI implementation documents basic channel (default 1, selectable 1–16), receive CCs for input/output volume, tap tempo, tuner, four external controllers, preset/scene functions, and block bypass/channel switches; PC transmit/receive and Scene MIDI/Send MIDI PC are documented. The manual says Fractal SysEx is used extensively by AM4-Edit, but does not publish a third-party command envelope or parameter write map. | **FACT:** 104 preset slots, four scenes per preset, four block channels; AM4-Edit and Fractal-Bot handle firmware, presets, cabs, and backups. The manual documents standard MIDI PC mapping. **UNKNOWN:** editor dump framing, checksums, acknowledgements, read-back semantics, and tuner/tempo telemetry format/routing. | **FACT:** amp/cab block includes 45+ DynaCab models and user cabs; the manual describes **256** onboard user-cab slots and Fractal-Bot/AM4-Edit transfer. **UNKNOWN:** current 2.01 DynaCab/user-cab wire format. No FC-6/FC-12 reference was found; **do not claim FC compatibility**. | **UNKNOWN/BLOCKER:** no official full editor protocol, discovery, or safe write framing. AM4-specific lawful capture and current-editor corroboration are required before any write claim. |
| **VP4** | **FACT:** [VP4 downloads](https://www.fractalaudio.com/vp4-downloads/) lists firmware **4.02 (2026-03-20)**, factory presets for 2.00+, and the shared AM4/VP4 Windows USB driver. [VP4 Owner's Manual](https://www.fractalaudio.com/downloads/manuals/VP4/VP4-Owners-Manual.pdf) is **v1.06 (2024-10-23)**. **SECONDARY/CONTRADICTION:** the community release-notes page reports 4.03 (2026-06-04), while the official page still lists 4.02; the official 4.03 package/release note was not located. | **FACT:** Type-A 3.5 mm TRS MIDI IN/OUT and optional Thru; USB 2.0/editor connection. System Info reports firmware/date/CPU. **UNKNOWN:** identity query, device-ID handshake, and editor transport framing. | **FACT:** receive CCs for input/output volume, tap tempo, tuner, four external controllers, preset/scene controls, and block bypass/channel; PC transmit/receive and MIDI clock receive (not transmit) are documented. SysEx is described as used extensively by VP4-Edit without a public command map. | **FACT:** 104 presets, four scenes and four channels; VP4-Edit supports editing/library operations and Fractal-Bot handles backups/firmware. **UNKNOWN:** full preset transfer, acknowledgements, and realtime tuner/tempo data. | **FACT:** the v1.06 manual describes VP4 as an effects unit without amp/cab modeling. **UNKNOWN/CONTRADICTORY:** current firmware/wiki material suggests later IR Player/user-cab management, but that behavior is not covered by the reviewed manual and the official 4.02/4.03 release-note boundary is unresolved. No official FC-6/FC-12 reference. **SECONDARY:** the community [Fractal forum VP4 FAQ](https://forum.fractalaudio.com/threads/vp4-faq.208790/) says VP4 is not FC-6/FC-12 compatible. | **BLOCKER:** resolve the official current firmware/release-note contradiction and obtain lawful v4.x editor captures before claiming cab transfer or any full-editor write. |
| **Axe-Fx III (Original, Mark II, Standard, TURBO)** | **FACT:** [Axe-Fx III downloads](https://www.fractalaudio.com/axe-fx-iii-downloads/) lists firmware **32.06 (2026-06-25)**, Windows USB driver 6.16 (2026-06-10), and USB firmware 1.18 (2026-01-05). [Owner's Manual](https://www.fractalaudio.com/downloads/manuals/axe-fx-3/Axe-Fx-III-Owners-Manual.pdf) is **July 2022**, current as of firmware **20.x**, covering all listed hardware variants. | **FACT:** high-speed USB MIDI/audio and 5-pin MIDI; USB is used for editing, backup, and updates; Windows driver required, Mac class-compliant. System/MIDI pages expose configuration, not a published identity handshake. **UNKNOWN:** discovery query, model-ID response, endpoint selection, and editor session framing. | **FACT:** [Axe-Fx III MIDI for 3rd Party Devices](https://www.fractalaudio.com/downloads/misc/Axe-Fx%20III%20MIDI%20for%203rd%20Party%20Devices.pdf), revision **1.4**, documents named Set/Get Bypass, Set/Get Channel, Set/Get Scene, patch/scene-name queries, looper state, tempo tap, tuner on/off, status dump, tempo, and optional realtime tempo/tuner push; it states support from firmware **1.13+** and that realtime pushes are MIDI OUT, not MIDI-over-USB. The owner manual separately documents standard CC/PC and editor SysEx use. | **FACT:** 1024 or 512 presets depending on model; eight scenes and up to four channels per block; Axe-Edit III and Fractal-Bot handle presets, system backups, firmware, and user-cab files. **FACT:** manual documents banks and system/user-cab backup distinctions. **UNKNOWN:** full dump chunking, checksums, acknowledgement/error grammar, grid/cable writes, and firmware update framing. | **FACT:** up to **2048** user cabs; user IR `.syx` transfer through Fractal-Bot/Axe-Edit; FC-6/FC-12 attach over FASLINK 2 and are documented in the FC setup/FC manual. **UNKNOWN:** current 32.06 DynaCab additions and their wire format; the 2022 manual predates later firmware. | **FACT/BOUNDARY:** the third-party PDF is a documented control surface, not a full Axe-Edit protocol. Full editor writes and discovery remain undocumented; use read-only until lawful capture and corroboration exist. |
| **FM9 (Original, TURBO)** | **FACT:** [FM9 downloads](https://www.fractalaudio.com/fm9-downloads/) lists firmware **12.0 (2026-07-07)**, factory presets v9.02, Windows USB driver 6.16 (2026-06-10), and USB firmware 1.04. [FM9 Owner's Manual](https://www.fractalaudio.com/downloads/manuals/FM9/FM9-Owners-Manual.pdf) is **2022-08-17**, current as of firmware **3.x**, covering Original/TURBO. | **FACT:** high-speed USB MIDI/audio for editor, backup, and updates; Windows driver required, Mac class-compliant; 5-pin MIDI also documented. System Info is local UI data only. **UNKNOWN:** host discovery/device identity and editor framing. | **FACT:** standard CC/PC assignment covers input/output volume, tap, tuner, 16 external controllers, scenes, looper, and block bypass/channel; PC transmit and Fractal SysEx editor use are documented. MIDI clock is received, not transmitted. **HYPOTHESIS (unverified):** some Axe-Fx III third-party operations may be shared by the gen-3 family; the official PDF does not state FM9 coverage. | **FACT:** FM9-Edit and Fractal-Bot handle presets, system backups, firmware, and user cabs; backups include FC Controllers/Global/I/O/MIDI/Tuner. Banks A–D are documented. **UNKNOWN:** preset/file frames, write acknowledgement, and read-back contract. | **FACT:** FC-6/FC-12 host/control workflows are documented; the manual documents user-cab loading. **GAP:** the 2022 manual predates current DynaCab/firmware 12.0 details, so current cab catalog, chunking, and compatibility need current release-note/editor evidence. | **BLOCKER:** do not port Axe-Fx III bytes or full-editor behavior to FM9 without a device-specific official statement or lawful capture. |
| **FM3 (Standard, Mark II Turbo)** | **FACT:** [FM3 downloads](https://www.fractalaudio.com/fm3-downloads/) lists firmware **13.0 (2026-07-16)**, factory presets for 10.00+, Windows USB audio 6.16 and serial driver 1.3 (2026-06-10). [FM3 Owner's Manual](https://www.fractalaudio.com/downloads/manuals/FM3/FM3-Owners-Manual.pdf) is **2023-07-12**, current as of firmware **7.x**, for Standard/Mark II Turbo. | **FACT:** manual documents USB editor/Fractal-Bot use and separate Windows drivers; it says normal backup/dump workflows are not supported over 5-pin MIDI, while noting a slow/advised-against transmit-only fallback. **SECONDARY:** the community MIDI page describes an FM3 USB serial editor channel rather than generic DAW MIDI. **UNKNOWN:** discovery identity and USB serial framing. | **FACT:** standard CC/PC assignments cover input/output volume, tap, tuner, 16 external controllers, scenes, looper, and block bypass/channel; FM3-Edit SysEx use and no MIDI-clock transmit are documented. `Send Realtime Sysex` is a setting for tuner/tempo messages. **UNKNOWN:** message bytes, USB-vs-DIN routing, and full editor writes. | **FACT:** FM3-Edit/Fractal-Bot handle presets, backups, firmware, and `.syx` user cabs; user cabs are separate from SYSTEM backup. **UNKNOWN:** dump framing, ack/read-back, grid/cable writes, and current v13 format. | **FACT:** Cab block includes UltraRes/DynaCab; FM3 supports FC-6/FC-12 and documents OMG9/layout/per-preset switch workflows. **GAP:** current v13 DynaCab/user-cab catalog and transfer format require an updated source or capture. | **BLOCKER:** the generic-USB/editor distinction and device-specific write protocol must be corroborated before any FM3 write support. |
| **Axe-Fx II family (Original, Mark II, XL, XL+)** | **FACT:** [Axe-Fx II downloads](https://www.fractalaudio.com/axe-fx-ii-downloads/) lists final Ares **2.0 (2019-09-18)** for XL/XL+/Original/Mark II and Quantum **10.01 (2018-05-11)**; factory banks are tied to Quantum 8.02+. [Axe-Fx II Owner's Manual](https://www.fractalaudio.com/downloads/manuals/axe-fx-2/Axe-Fx-II-Owners-Manual.pdf) is **Doc Q7.0 (2017-02-12)** and covers all variants. | **FACT:** two-way high-speed USB MIDI is documented for editing, updates, program changes, and tempo; 5-pin MIDI is also covered. **UNKNOWN:** identity/discovery command and exact per-variant endpoint behavior. | **FACT:** manual documents CC receive assignments (I/O volume, scene, tap/tuner, 12 external controllers, looper, every block bypass and X/Y), PC/bank mapping, and Fractal real/non-real SysEx; tempo/tuner realtime SysEx is stated. **SECONDARY:** [archived public Fractal MIDI spec](https://archive.axefx.fr/AxeFX%20II/Docs%20%26%20Manuals/Fractal%20MIDI%20Spec%20Public%2022%2001%202014%20resaved.htm) names additional query/dump/grid/parameter categories. It is old, archived, and not a current or variant-wide guarantee. | **FACT:** Axe-Edit and Fractal-Bot handle presets/banks/system, firmware, and user-cab transfer; the manual notes older manual dump/receive paths were removed in favor of Fractal-Bot. **UNKNOWN:** exact current framing, checksums, ack/error, and differences across Original/Mark II/XL/XL+. | **FACT:** user-cab capacity differs by model (manual documents 100 for Mark II and up to 1024 for XL/XL+); IRs transfer as MIDI SysEx via software. **FACT:** MFC-101 is the documented legacy foot controller; no FC-6/FC-12 reference was found. | **BLOCKER:** the archived spec is a research lead only. Firmware- and variant-specific capture/corroboration is required before implementing any command beyond documented standard MIDI. |
| **AX8** | **FACT:** [AX8 downloads](https://www.fractalaudio.com/ax8-downloads/) lists firmware **Quantum 10.01 (2018-09-11)** and factory presets (2017-06-05). [AX8 Owner's Manual](https://www.fractalaudio.com/downloads/manuals/AX8/AX8-Owners-Manual.pdf) is **Manual Version 6/6.0.1 (2016-12-19)**. | **FACT:** two-way MIDI-over-USB, no USB audio and no driver are documented; 5-pin MIDI is also documented. **UNKNOWN:** host identity/discovery and editor-session framing. | **FACT:** standard CC covers master volume, tap/tuner, 12 external controllers, looper, every block bypass and X/Y; PC/CC transmit is documented; AX8-Edit SysEx use is described. No detailed public third-party command map was found in the official manual. | **FACT:** Fractal-Bot sends/receives presets, banks, user cabs, and system; AX8-Edit manages presets/banks/Cab Packs. **UNKNOWN:** full dump framing, ack/read-back, and editor write grammar. | **FACT:** manual documents **512** user cabs and `.syx` transfer via Fractal-Bot/AX8-Edit; no FC-6/FC-12 reference was found. **Do not claim FC compatibility.** | **BLOCKER:** require lawful AX8-Edit captures and a firmware-specific source before claiming full editor support. |
| **FX8 (Original, Mark II)** | **FACT:** [FX8 downloads](https://www.fractalaudio.com/fx8-downloads/) lists firmware **5.03 (2017-06-29)** for Original/Mark II; 5.04 is explicitly marked unsupported beta (2017-11-14). [FX8 Owner's Manual](https://www.fractalaudio.com/downloads/manuals/FX8/FX8-Owners-Manual.pdf) is **2016-11-14**, for firmware **3.0+**, Original/Mark II. | **FACT:** two-way MIDI-over-USB and Fractal-Bot file/update transport are documented; no driver is required. **UNKNOWN:** identity/discovery and editor session framing. | **FACT:** CC receive covers master volume, true bypass, tap/tuner, 12 external controllers, looper, block bypass and X/Y; MIDI block transmits CC/PC; preset selection emits PC; MIDI clock is received, not transmitted. FX8-Edit SysEx use is described, not specified. | **FACT:** 128 presets and no bank select; Fractal-Bot handles firmware/backups/files, with warnings for ALL PRESETS/SYSTEM overwrite. **UNKNOWN:** dump chunks, ack/error, and full editor writes. | **FACT:** FX8 is effects-only; no amp/cab/user-cab workflow is documented. No FC-6/FC-12 reference. | **BLOCKER:** full editor and file protocol remain undocumented; cab support is not applicable to the effects-only design. |

### Cross-device category map

| Category | Verified official boundary | Not verified / unsafe to infer |
| --- | --- | --- |
| **Identity/discovery** | Manuals expose local System Info/firmware pages. Some support pages name model variants and drivers. | No reviewed official manual publishes a host `WHO_AM_I`/device-ID handshake, descriptor contract, discovery timeout, or a portable model-ID map. A community model-ID list is secondary and is not enough to authorize writes. |
| **Transport** | Current gen-3 devices (Axe-Fx III/FM9) document high-speed USB MIDI/audio; FM3 documents USB editor/serial drivers and explicitly limits 5-pin backup dumping; AM4/VP4 document USB plus TRS MIDI; Axe II/AX8/FX8 document USB MIDI. | USB endpoint identity, editor-vs-generic-MIDI multiplexing, reconnect/session rules, FASLINK framing, and exact USB/DIN routing are not fully published. Do not treat “USB MIDI” in one manual as evidence for another. |
| **Preset/file** | Fractal-Bot and each named editor are official tools for firmware, preset/bank/system backup/restore, and (where documented) user-cab transfer. Device-specific slot/scene/channel facts are in the table above. | No official, cross-device file grammar, chunk size, checksum, acknowledgement, partial-transfer recovery, or save-to-location write protocol was found. A UI feature described in an editor manual is not a wire contract. |
| **Documented real-time/control MIDI** | Standard CC/PC assignments are documented for every target. Axe-Fx III has the only detailed official third-party command PDF found. FM9/FM3/Axe II manuals mention tempo/tuner realtime SysEx; FX8 explicitly receives MIDI clock only; Axe III PDF limits realtime pushes to MIDI OUT. | Message bytes, parameter IDs, status/error replies, high-rate telemetry cadence, and USB-vs-DIN semantics are generally absent. Do not derive them from a different model. |
| **Cab/IR** | User-cab capacities and transfer tools are documented per model where applicable. Axe III supports up to 2048; AX8 512; Axe II capacities vary by model; AM4 256. | Current DynaCab catalogs, VP4 post-v1.06 IR claims, firmware compatibility, chunking, and checksum/ack behavior need current release-note or lawful capture evidence. FX8 has no cab model. |
| **FC** | Axe-Fx III, FM9, and FM3 manuals document FC-6/FC-12 host workflows; the FC manual documents host/internal firmware and layouts. Axe II documents MFC-101 instead. | FASLINK/FC wire protocol and host discovery are not published. AM4/VP4/AX8/FX8 manuals do not establish FC-6/FC-12 support; VP4's official FAQ says no. |

## Documented control messages vs. undocumented full editor

The following is the safe protocol boundary for Topology:

1. **Documented and potentially implementable after device-specific transport
   verification:** standard MIDI program changes, documented CC assignments,
   MIDI clock direction, and the named Axe-Fx III third-party operations from
   revision 1.4. Even these need a device/firmware/transport matrix and
   read-back evidence before a write is advertised.
2. **Documented as an application capability, not as bytes:** Fractal-Bot and
   the named editors can update firmware, back up/restore, transfer presets and
   user cabs, and edit blocks. The manuals do not turn those actions into a
   public packet grammar.
3. **Undocumented full-editor behavior:** discovery/identity, parameter-ID
   catalogs, arbitrary parameter writes, grid coordinates, cable placement,
   split/merge operations, scene/channel internals, patch-name write framing,
   save-to-location, full preset/cab dump chunking, acknowledgements/errors,
   firmware-update framing, and FC/FASLINK commands. These require a lawful
   owner capture or an independently corroborated permissively licensed
   implementation. They stay read-only/unsupported until then.

The archived 2014 Axe II MIDI specification and the open-source projects listed
below are **published research material**, not permission to copy proprietary
editor binaries or to promote a community capture to vendor support. No source
reviewed here establishes that an Axe II command category applies unchanged to
AX8, FX8, AM4, VP4, FM3, FM9, or Axe-Fx III.

## Published secondary and open-source corroboration

These sources are useful leads and provenance references. They do not override
the official-device table.

* **Archived MIDI specification (secondary):** [Fractal MIDI Spec Public,
  2014-01-22](https://archive.axefx.fr/AxeFX%20II/Docs%20%26%20Manuals/Fractal%20MIDI%20Spec%20Public%2022%2001%202014%20resaved.htm)
  names Axe II-era query/dump/parameter/grid/cab/tempo categories. It is
  archived and firmware-specific; no byte strings are copied here.
* **Community wiki (secondary):** [MIDI SysEx](https://wiki.fractalaudio.com/wiki/index.php?title=MIDI_SysEx),
  [MIDI overview](https://wiki.fractalaudio.com/wiki/index.php?title=MIDI), and
  per-device release-note pages for [Axe-Fx III](https://wiki.fractalaudio.com/wiki/index.php?title=Firmware_release_notes_Axe-Fx_III),
  [FM9](https://wiki.fractalaudio.com/wiki/index.php?title=Firmware_release_notes_FM9),
  [VP4](https://wiki.fractalaudio.com/wiki/index.php?title=Firmware_release_notes_VP4),
  and [AM4](https://wiki.fractalaudio.com/wiki/index.php?title=Firmware_release_notes_AM4).
  The wiki pages were either inaccessible to the automated fetch or explicitly
  secondary; they are not release authority. The VP4 page's 4.03 claim conflicts
  with the official 4.02 page as of this report.
* **Axis:** [sKuhLight/Axis at commit
  `6b87bd2`](https://github.com/sKuhLight/Axis/tree/6b87bd2472fd88854421fda0dd1d2d7a02d2dd19),
  MIT-licensed community editor. Its README calls the project beta, says FM3
  firmware 12.0 was hardware-verified, and describes a Node/HTTP development
  stack. It is a research lead, not a Topology production dependency.
* **ForgeFX:** [sKuhLight/ForgeFX at commit
  `c22862a`](https://github.com/sKuhLight/ForgeFX/tree/c22862a5b2f2078f3cb92a2735e51f94c39a0062),
  MIT-licensed community beta. Its README claims gen-3 device auto-detection,
  grid, cab, telemetry, and backup behavior; its NOTICE credits Apache-2.0
  `mcp-midi-control`/`fractal-midi` sources. These are community claims and
  require separate hardware and provenance review.
* **mcp-midi-control / fractal-midi:** [pinned repository
  `5904717`](https://github.com/TheAndrewStaker/mcp-midi-control/tree/59047175cfc4f23e092931b54a7c54f2bffde3ea)
  and its [`packages/fractal-midi`](https://github.com/TheAndrewStaker/mcp-midi-control/tree/59047175cfc4f23e092931b54a7c54f2bffde3ea)
  package are Apache-2.0 with a NOTICE. The README reports AM4/Axe-II
  hardware-verified coverage and community-beta FM3/FM9/VP4/Axe III coverage,
  including owner-confirmed reads/continuous writes for some gen-3 paths and
  untested VP4 writes. Its NOTICE says protocol data came from published
  material plus community reverse engineering/editor-binary mining. Treat all
  behavior as secondary until the exact lawful capture and fixture provenance
  are available.

## Unknowns, gaps, and evidence required

| ID | Classification | Current gap | Evidence that would close it | Safe interim behavior |
| --- | --- | --- | --- | --- |
| G-01 | UNKNOWN/BLOCKER | Identity/discovery handshake, model/firmware query, endpoint selection for every target. | Vendor publication, or a lawful owner capture with device/firmware/transport sidecar and independent replay. | User-selected known device only; unknown firmware is read-only/unsupported. |
| G-02 | UNKNOWN | USB generic-MIDI versus editor-serial channels, especially FM3; reconnect/session timing and framing. | Official driver/editor documentation or lawful USB capture with descriptors and OS/driver versions. | Keep transport adapter device-specific; no cross-device endpoint inference. |
| G-03 | UNKNOWN/BLOCKER | Gen-3 full preset read/write, parameter IDs, grid/cabling, save-to-location, ack/error, chunk/checksum behavior. | Per-device lawful captures on declared firmware plus read-back and negative/error cases; compare against permissively licensed source. | Standard CC/PC only; no arbitrary full-editor writes. |
| G-04 | UNKNOWN/BLOCKER | AM4/VP4 editor SysEx, dump/restore, mode selection, parameter catalog, and current cab transfer. | Owner capture from AM4 2.01/VP4 4.02 (or official 4.03 package) and current editor; preserve provenance and license. | Standard CC/PC; no editor write or cab transfer claim. |
| G-05 | CONTRADICTION | VP4 official page lists 4.02; community release page lists 4.03. | Official Fractal 4.03 release page/package or corrected support listing. | Pin support-page 4.02; mark 4.03 unverified. |
| G-06 | UNKNOWN | Axe II archived 2014 command categories versus final Ares variants and AX8/FX8. | Variant/firmware captures or an updated official specification; test each command with non-destructive read/query first. | Treat archive as research lead only; no command portability. |
| G-07 | UNKNOWN | Realtime tuner/tempo message bytes, cadence, routing, and USB-vs-DIN behavior. | Device-specific official statement or captures with MIDI OUT/USB separately observed; include high-rate timing and no-signal cases. | Use local device UI; do not parse or synthesize undocumented telemetry. |
| G-08 | UNKNOWN | Current DynaCab catalogs, user-cab format/version, firmware compatibility and transfer semantics. | Current release notes/editor documentation plus checksummed, owner-authorized cab fixture metadata. | Use documented slot counts/tools only; no assumed format. |
| G-09 | UNKNOWN | FC-6/FC-12 FASLINK host discovery and command protocol; applicability outside Axe III/FM9/FM3. | FC manual plus lawful FASLINK/USB capture on each host, or Fractal-published protocol. | Treat FC workflows as UI capability only; no raw FC transport implementation. |
| G-10 | LEGAL/PROVENANCE BLOCKER | Community captures/editor-binary mining may contain vendor firmware, presets, or unknown-rights data. | Written capture/redistribution permission from hardware owner; sidecar with device, firmware, transport, date, operator, consent, hashes, and redaction. | Do not commit proprietary binaries, artwork, presets, or unlicensed capture. |
| G-11 | UNKNOWN | Firmware release pages and manuals are asynchronous (for example current gen-3 firmware is newer than manual “current as of” versions). | Re-run source map for every supported firmware profile; attach release-note hash and profile status. | Version-gate profiles; unknown firmware read-only. |

## Proposed follow-up work (not created by this packet)

These are proposed IDs for the parent to schedule; no production code or packet
was created here.

* **`TOP-ADR-004` — Official-vendor boundary:** define the evidence labels and
  read-only default when a manual documents a feature but not its wire format;
  explicitly prohibit porting Axe III third-party operations to FM9/FM3 without
  corroboration.
* **`TOP-RSCH-009` — Lawful discovery/transport capture:** capture identity,
  endpoint, reconnect, and realtime routing for one declared device/firmware at
  a time, with fixture sidecars and redaction review.
* **`TOP-RSCH-010` — Current firmware profile matrix:** reconcile support-page
  release notes with manual coverage and generate per-device profile status;
  resolve VP4 4.02 versus 4.03 first.
* **`TOP-RSCH-011` — Gen-3 editor boundary:** corroborate FM3/FM9/Axe III
  read/write and error behavior using owner-authorized captures and a
  permissively licensed codec; keep grid/cabling/save writes gated until proven.
* **`TOP-RSCH-012` — FC transport:** investigate FASLINK/FC host discovery and
  commands without assuming that a UI workflow is a transport specification.

## Source register

All web sources below were accessed 2026-08-08. Official support pages are the
authority for current firmware/date listings; linked PDFs are the authority for
the manual revision/coverage. Release-note packages are linked from those pages.

### Official Fractal sources

* [Support index](https://www.fractalaudio.com/support/)
* [AM4 downloads](https://www.fractalaudio.com/am4-downloads/) · [AM4 manual PDF](https://www.fractalaudio.com/downloads/manuals/AM4/AM4-Owners-Manual.pdf) · [AM4 v2.01 package](https://www.fractalaudio.com/downloads/firmware-presets/am4/2p0/AM4_firmware_v2p01.zip)
* [VP4 downloads](https://www.fractalaudio.com/vp4-downloads/) · [VP4 manual PDF](https://www.fractalaudio.com/downloads/manuals/VP4/VP4-Owners-Manual.pdf) · [VP4 v4.02 package](https://www.fractalaudio.com/downloads/firmware-presets/vp4/4p0/VP4_firmware_v4p02.zip)
* [Axe-Fx III downloads](https://www.fractalaudio.com/axe-fx-iii-downloads/) · [Axe-Fx III manual PDF](https://www.fractalaudio.com/downloads/manuals/axe-fx-3/Axe-Fx-III-Owners-Manual.pdf) · [Axe-Fx III MIDI for 3rd Party Devices, rev. 1.4](https://www.fractalaudio.com/downloads/misc/Axe-Fx%20III%20MIDI%20for%203rd%20Party%20Devices.pdf) · [Axe-Fx III 32.06 package](https://www.fractalaudio.com/downloads/firmware-presets/axe-fx-3/32p0/axefxiii_dsp_rel_32p06.zip)
* [FM9 downloads](https://www.fractalaudio.com/fm9-downloads/) · [FM9 manual PDF](https://www.fractalaudio.com/downloads/manuals/FM9/FM9-Owners-Manual.pdf) · [FM9 12.0 package](https://www.fractalaudio.com/downloads/firmware-presets/fm9/12p0/fm9_dsp_rel_12p00.zip)
* [FM3 downloads](https://www.fractalaudio.com/fm3-downloads/) · [FM3 manual PDF](https://www.fractalaudio.com/downloads/manuals/FM3/FM3-Owners-Manual.pdf) · [FM3 13.0 package](https://www.fractalaudio.com/downloads/firmware-presets/fm3/13p0/fm3_dsp_rel_13p0.zip)
* [Axe-Fx II downloads](https://www.fractalaudio.com/axe-fx-ii-downloads/) · [Axe-Fx II manual PDF](https://www.fractalaudio.com/downloads/manuals/axe-fx-2/Axe-Fx-II-Owners-Manual.pdf) · [Original/Mark II Ares 2.00 package](https://www.fractalaudio.com/downloads/firmware-presets/axe-fx-2/Ares/2.0/AxeFx2_Ares_2p00.zip) · [XL Ares 2.00 package](https://www.fractalaudio.com/downloads/firmware-presets/axe-fx-2/Ares/2.0/AxeFx2_XL_Ares_2p00.zip) · [XL+ Ares 2.00 package](https://www.fractalaudio.com/downloads/firmware-presets/axe-fx-2/Ares/2.0/AxeFx2_XLplus_Ares_2p00.zip)
* [AX8 downloads](https://www.fractalaudio.com/ax8-downloads/) · [AX8 manual PDF](https://www.fractalaudio.com/downloads/manuals/AX8/AX8-Owners-Manual.pdf) · [AX8 10.01 package](https://www.fractalaudio.com/downloads/firmware-presets/ax8/10.0/ax8_10p01.zip)
* [FX8 downloads](https://www.fractalaudio.com/fx8-downloads/) · [FX8 manual PDF](https://www.fractalaudio.com/downloads/manuals/FX8/FX8-Owners-Manual.pdf) · [FX8 Mark II 5.03 package](https://www.fractalaudio.com/downloads/firmware-presets/fx8/5.0/fx8MkII_5p03.zip) · [FX8 Original 5.03 package](https://www.fractalaudio.com/downloads/firmware-presets/fx8/5.0/fx8_5p03.zip)
* [FC downloads](https://www.fractalaudio.com/fc-download/) · [FC-6/FC-12 manual PDF](https://www.fractalaudio.com/downloads/manuals/FC-X/FC6%2B12-Owners-Manual.pdf)

### Published secondary/open-source sources

* [Fractal MIDI Spec Public archive (2014)](https://archive.axefx.fr/AxeFX%20II/Docs%20%26%20Manuals/Fractal%20MIDI%20Spec%20Public%2022%2001%202014%20resaved.htm)
* Community forum (secondary): [VP4 FAQ](https://forum.fractalaudio.com/threads/vp4-faq.208790/)
* [Fractal Wiki MIDI SysEx](https://wiki.fractalaudio.com/wiki/index.php?title=MIDI_SysEx) · [Wiki MIDI overview](https://wiki.fractalaudio.com/wiki/index.php?title=MIDI)
* [Axis pinned commit](https://github.com/sKuhLight/Axis/tree/6b87bd2472fd88854421fda0dd1d2d7a02d2dd19) ([MIT LICENSE](https://github.com/sKuhLight/Axis/blob/6b87bd2472fd88854421fda0dd1d2d7a02d2dd19/LICENSE)) · [ForgeFX pinned commit](https://github.com/sKuhLight/ForgeFX/tree/c22862a5b2f2078f3cb92a2735e51f94c39a0062) ([MIT LICENSE](https://github.com/sKuhLight/ForgeFX/blob/c22862a5b2f2078f3cb92a2735e51f94c39a0062/LICENSE), [NOTICE](https://github.com/sKuhLight/ForgeFX/blob/c22862a5b2f2078f3cb92a2735e51f94c39a0062/NOTICE)) · [mcp-midi-control pinned commit](https://github.com/TheAndrewStaker/mcp-midi-control/tree/59047175cfc4f23e092931b54a7c54f2bffde3ea) ([Apache-2.0 LICENSE](https://github.com/TheAndrewStaker/mcp-midi-control/blob/59047175cfc4f23e092931b54a7c54f2bffde3ea/LICENSE), [NOTICE](https://github.com/TheAndrewStaker/mcp-midi-control/blob/59047175cfc4f23e092931b54a7c54f2bffde3ea/NOTICE), [fractal-midi package metadata](https://github.com/TheAndrewStaker/mcp-midi-control/blob/59047175cfc4f23e092931b54a7c54f2bffde3ea/packages/fractal-midi/package.json))

## Verification limits and handoff

This is a source map, not hardware verification. No device was connected, no
write bytes were generated, and no proprietary firmware/editor binary or
unknown-rights preset/capture was copied. A community claim, simulator result,
or open-source implementation can guide the next lawful capture but cannot
upgrade a device profile to `HARDWARE_VERIFIED`.

The companion evidence directory records the packet, environment, source index,
and handoff. Because `TOP-RSCH-003` is a research packet, strict-TDD RED/GREEN
artifacts do not apply; the acceptance gate is source-backed evidence plus an
independent review.
