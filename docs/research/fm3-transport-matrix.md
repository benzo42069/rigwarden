# FM3 transport and adapter matrix

**Packet:** `TOP-FM3-RSCH-001`  
**Requirements:** `TRANSPORT-005`, `PLAT-001`, `PLAT-002`  
**Research question:** What exact FM3 control transports are evidenced on
Android and iOS, including direct USB serial, MIDI interfaces, BLE adapters,
and a network bridge fallback?  
**Recorded/accessed:** **2026-08-09** (America/Chicago; sources were reopened
independently on this date)  
**Status:** `RESEARCH_COMPLETE — REVIEW_PENDING`  
**Research route:** OpenAI `gpt-5.6-luna`, `max`, bounded documentation and
provenance research only.

## Decision summary

The FM3 is **not a USB MIDI device**. The official manual describes 4x4 USB
audio and `COMM over USB` channels for Fractal-Bot and FM3-Edit, while the
MIDI ports used for ordinary MIDI control are 5-pin. The same manual says that
backup/dump over 5-pin is unsupported (transmit-only is possible but so slow it
is advised against) and that SysEx is used extensively by FM3-Edit. The current
Fractal downloads page lists FM3 firmware **13.0 (2026-07-16)** and Windows
audio **6.16** plus serial **1.3** drivers (both required for Windows USB
operation). [F-FM3-MANUAL] [F-FM3-DOWNLOADS]

This produces four distinct boundaries:

1. **Android raw USB host:** `UsbManager` can enumerate devices, request user
   permission, inspect interfaces/endpoints, claim an interface, and perform
   bulk/control transfers. That is a platform fact only. No FM3 descriptors,
   serial framing, identity exchange, or physical capture is available here;
   direct Android FM3 USB is therefore a **hardware-gated hypothesis** and not
   a CDC, serial, editor, or compatibility claim. [A-USB]
2. **Ordinary iPhone direct USB serial:** **blocked** by the current public API
   boundary. CoreMIDI addresses MIDI endpoints, not the FM3's COMM channel;
   ExternalAccessory is for MFi accessories with manufacturer-declared
   protocols; Apple's IOUSBHost/USBDriverKit host-driver path is not a normal
   iPhone app transport. [APPLE-CORE] [APPLE-SYSEX] [APPLE-EA] [APPLE-IOUSBHOST]
   [APPLE-USBKIT]
3. **M-series iPadOS direct USB:** **conditional**, not generally available.
   Apple documents USBDriverKit on iPadOS 16+ for M-series iPads, with a
   custom driver, DriverKit transport/user-client entitlements, and user
   approval. No FM3 driver or COMM protocol is supplied by Apple or Fractal;
   this requires a separate feasibility and entitlement packet. [APPLE-IPAD-DRIVER]
4. **5-pin MIDI, BLE MIDI, and network bridge:** platform/API paths are
   plausible through a class-compliant bidirectional MIDI interface, a BLE-MIDI
   adapter, or an explicitly paired local bridge. They remain **hypotheses** for
   FM3 until a real FM3 capture proves endpoint identity, message routing,
   fragmentation, and read-back. Fractal has not published native FM3 BLE or
   network transport in the reviewed sources.

No raw protocol bytes, editor code, vendor binaries, captures, or compatibility
claims are created by this report.

## Evidence labels

| Label | Meaning in this report |
| --- | --- |
| `FACT_DEVICE` | Stated by the current FM3 manual or Fractal support/download page, with section/page and source link. |
| `FACT_PLATFORM` | Guaranteed only by an Android or Apple API contract; it does not identify or validate an FM3 endpoint. |
| `HYPOTHESIS` | A bounded path worth testing with exact hardware, firmware, OS, adapter, and capture evidence. It cannot enable writes. |
| `UNKNOWN` | The reviewed sources do not establish the behavior. |
| `BLOCKED` | The ordinary platform/device boundary does not expose the requested path; a different product boundary is required. |
| `CONDITIONAL` | Possible only under a stated platform/entitlement/device condition; no implementation or support claim follows. |
| `SECONDARY/UNVERIFIED` | A community/open-source identifier or report retained as a capture lead only. |

## FM3 facts that constrain every transport

| Fact | Evidence and safe interpretation |
| --- | --- |
| USB function | `FACT_DEVICE`: FM3 Owner's Manual v7.x, pp. 29–30: 4x4 USB audio; FM3 is **not** a USB MIDI device; `COMM over USB` channels are used by Fractal-Bot/FM3-Edit; USB type-B is the host connection and USB type-A is unused. [F-FM3-MANUAL] |
| Desktop editor | `FACT_DEVICE`: the same manual, p. 20, identifies FM3-Edit as the full-featured editor/librarian and Fractal-Bot as the firmware/backup/restore tool. This documents a desktop application capability, not a public wire contract. [F-FM3-MANUAL] |
| Windows drivers | `FACT_DEVICE`: FM3 downloads page, accessed 2026-08-09, lists firmware 13.0, dated 2026-07-16, and a Windows USB driver bundle dated 2026-06-10 containing audio 6.16 and serial 1.3; both drivers are required for Windows 10+ USB operation. [F-FM3-DOWNLOADS] |
| 5-pin MIDI | `FACT_DEVICE`: MIDI messages are received at MIDI IN and transmitted at MIDI OUT/THRU using 5-pin cables; the manual lists PC/CC, scene, bypass, channel, parameter, clock receive, and realtime tap/tuner settings. This is not evidence that FM3-Edit's full editor protocol is available over mobile MIDI. [F-FM3-MANUAL] |
| Backup/dump over DIN | `FACT_DEVICE`: pp. 116–117 say backup and dumping are **not supported** over 5-pin MIDI; transmit-only backup transfer can take a very long time and is advised against. Do not plan mobile full-backup support on this path without a separate, approved experiment. [F-FM3-MANUAL] |
| SysEx boundary | `FACT_DEVICE`: p. 146 says SysEx is used extensively for FM3-Edit and records Fractal Audio real-time/non-real-time categories. The manual does not publish the editor framing, identity request, parameter map, checksum, acknowledgement, or read-back grammar. [F-FM3-MANUAL] |
| Firmware/profile | `FACT_DEVICE`: the current support page names FM3 Original and Mark II Turbo as compatible with 13.0. Do not infer that a manual's older “current as of 7.x” coverage proves v13 protocol compatibility. [F-FM3-DOWNLOADS] [F-FM3-MANUAL] |

## FM3 transport × host matrix

The matrix is deliberately split by **transport capability** and **editor
feature capability**. A platform row marked `FACT_PLATFORM` does not promote
the adjacent FM3 row above `HYPOTHESIS` or `UNKNOWN`.

| Transport path | Android phone/tablet | iPhone / ordinary iPadOS | M-series iPadOS 16+ | FM3-specific status | Adapter, permission, and power requirements | Evidence still required |
| --- | --- | --- | --- | --- | --- | --- |
| **FM3 USB COMM/serial directly** | `HYPOTHESIS`: Android USB host can enumerate and open a non-MIDI interface after permission. No physical FM3 descriptor or framing capture exists. Do not label it USB MIDI or assume CDC semantics. [A-USB] | `BLOCKED`: no ordinary public generic-serial app path in the reviewed source set; CoreMIDI cannot turn COMM into a MIDI endpoint, and ExternalAccessory requires an MFi accessory protocol. [APPLE-CORE] [APPLE-EA] | `CONDITIONAL`: custom USBDriverKit driver path exists for M-series iPads, but requires a DriverKit extension, device matching, entitlements, user approval, and an FM3-specific driver/protocol. [APPLE-USBKIT] [APPLE-IPAD-DRIVER] | **FM3 USB editor transport is device-specific and unverified on mobile.** | Android: USB host feature, OTG/data cable, possible powered hub, user USB grant, descriptor inspection, interface claim, bounded I/O off the UI thread. iPadOS: M-series iPad, USB-C host path, driver extension and entitlement review. | Exact FM3 Original/Mark II Turbo, firmware, VID/PID/interface descriptors, attach/detach, permission, open/close, and a provenance-approved read/identity exchange. No outbound probe until the capture packet permits it. |
| **Direct USB MIDI to FM3** | `BLOCKED`: the FM3 manual explicitly says it is not USB MIDI. `android.media.midi` is irrelevant unless a separate class-compliant MIDI interface is present. [F-FM3-MANUAL] [A-MIDI] | `BLOCKED`: same FM3 boundary; CoreMIDI USB support does not change the device class. [F-FM3-MANUAL] [APPLE-CORE] | `BLOCKED` for the same reason; DriverKit does not make the FM3 a USB MIDI device. | **Do not create a direct-USB-MIDI profile row.** | None; a class-compliant MIDI interface is a different path and is listed below. | Physical descriptor capture must confirm any unexpected device mode before the matrix can change. |
| **FM3 5-pin MIDI → class-compliant USB-MIDI interface** | `HYPOTHESIS`: Android MIDI supports USB/BLE MIDI and arbitrary-length SysEx, but the FM3 interface, adapter, power, message fragmentation, and editor behavior remain untested. [A-MIDI] | `HYPOTHESIS`: CoreMIDI exposes USB MIDI endpoints and SysEx operations for a connected class-compliant interface. This is the primary ordinary iOS path for documented basic MIDI control. [APPLE-CORE] [APPLE-SYSEX] | `HYPOTHESIS`: same interface path; no need to invoke DriverKit if the adapter is class-compliant, but physical iPad testing remains required. | **Basic PC/CC/clock control is plausible; full editor/SysEx is `UNKNOWN`.** FM3's 5-pin backup/dump limitation remains in force. | Bidirectional class-compliant interface with MIDI IN and OUT, correct 5-pin cables, host adapter (USB-C/Lightning as applicable), bus power or powered hub, endpoint permission/selection. | Capture FM3 responses for a documented/read-only control, check ordered SysEx fragmentation and maximum message size, measure reconnect and busy-port behavior, and keep all writes read-only until fixture/HIL review. |
| **FM3 5-pin MIDI → BLE-MIDI adapter** | `HYPOTHESIS`: Android MIDI supports BLE MIDI; Android 12+ scanning/connection requires runtime `BLUETOOTH_SCAN`/`BLUETOOTH_CONNECT`, with the documented location caveat. [A-MIDI] [A-BT] | `HYPOTHESIS`: Core MIDI supports BLE-MIDI peripherals; iOS 16+ can automatically reconnect paired peripherals, while non-pairing peripherals require a Core Bluetooth discovery/activation path. `NSBluetoothAlwaysUsageDescription` is required for Bluetooth APIs. [APPLE-BLE] [APPLE-BT-PERM] | `HYPOTHESIS`: same BLE API path; no FM3-native BLE evidence. | **Adapter-to-5-pin only; no Fractal-native BLE claim. Full SysEx/editor/realtime behavior is `UNKNOWN`.** | User-owned BLE-MIDI adapter, 5-pin cabling, pairing flow, platform Bluetooth permission, foreground-first session, reconnect/idle timeout handling. | Test BLE MTU fragmentation, ordering, cancellation, idle disconnect, reconnect, large SysEx, and 5-pin FM3 read-only controls on exact OS/adapter/firmware tuples. |
| **Local network bridge → FM3** | `HYPOTHESIS`: a user-run local bridge could expose a network or virtual MIDI endpoint, but no Fractal-native network API is documented. Android local-network policy is changing: Android 16 is opt-in, while Android 17/target SDK 37+ requires `ACCESS_LOCAL_NETWORK` or a privacy-preserving picker. [A-LAN] | `HYPOTHESIS`: Core MIDI provides `MIDINetworkSession` over a local network; direct sockets/Bonjour require `NSLocalNetworkUsageDescription`. The bridge protocol, pairing, and FM3 side are not supplied by Fractal. [APPLE-CORE] [APPLE-NET] [APPLE-LAN] | `HYPOTHESIS`: same network path; DriverKit is not required for a network bridge. | **Bridge is a separate product boundary, not FM3 compatibility.** Security/session ownership and all editor features are `UNKNOWN`. | Explicitly paired local bridge (desktop/Raspberry Pi class), LAN permission/prompt, authenticated/encrypted protocol, version negotiation, replay protection, rate limits, no cloud relay. | Define and review the bridge protocol before implementation; test pairing, ownership conflict, reconnect, malformed input, and no-write behavior. |

### What the matrix does not say

* Android `UsbManager` support does not identify an FM3 interface, prove a
  serial class, or prove a Fractal editor session.
* CoreMIDI USB/BLE/network support does not expose arbitrary FM3 COMM/USB
  serial access on iPhone or ordinary iPadOS.
* A class-compliant MIDI adapter can carry MIDI bytes, but a note, CC, or short
  SysEx observation is not evidence of full editor, backup, cab, parameter,
  grid, acknowledgement, or firmware-update support.
* A BLE or network adapter is not an FM3-native transport. It is a third-party
  transport boundary with its own permissions, security, fragmentation, and
  lifecycle tests.

## Feature capability matrix

| Feature | Android direct USB COMM | 5-pin via USB-MIDI/BLE interface | iPhone direct USB | iPhone/iPad via class-compliant MIDI or BLE | M-series iPadOS DriverKit |
| --- | --- | --- | --- | --- | --- |
| Enumerate endpoint | `HYPOTHESIS` pending FM3 descriptor capture | `HYPOTHESIS` pending interface/adapter capture | `BLOCKED` for ordinary generic serial | `FACT_PLATFORM` for MIDI endpoint enumeration; FM3 match is `HYPOTHESIS` | `CONDITIONAL` driver discovery; no FM3 driver exists |
| Open/close/lifecycle | `HYPOTHESIS`; permission, detach, claim, cancellation required | `HYPOTHESIS`; adapter and OS behavior untested | `BLOCKED` ordinary serial | `HYPOTHESIS` for real adapter/bridge | `CONDITIONAL`; driver approval and user-client lifecycle required |
| Documented PC/CC/clock control | `UNKNOWN`; COMM framing and routing not published | `HYPOTHESIS` over 5-pin; FM3 manual documents device-side MIDI behavior | `BLOCKED` direct serial | `HYPOTHESIS` through the external interface | `UNKNOWN` until custom driver exposes a proven path |
| Identity/discovery exchange | `UNKNOWN` | `UNKNOWN` | `BLOCKED` direct serial | `UNKNOWN` | `UNKNOWN` |
| Full FM3-Edit parameter/grid/editor | `UNKNOWN` | `UNKNOWN` | `BLOCKED` direct serial | `UNKNOWN` | `UNKNOWN` |
| Backup/dump | `UNKNOWN` and do not attempt without a device-specific capture | `FACT_DEVICE` limitation: unsupported/very slow over 5-pin; no mobile promise | `BLOCKED` direct serial | `FACT_DEVICE` limitation still applies | `UNKNOWN`; driver transport does not remove FM3's stated limitation |
| Realtime tuner/tempo | `UNKNOWN` | `UNKNOWN` route/format; manual exposes a transmit setting but no mobile framing | `BLOCKED` direct serial | `UNKNOWN` | `UNKNOWN` |
| Write/undo/read-back | `BLOCKED` pending approved fixture, exact profile, and HIL | `BLOCKED` pending fixture/HIL and safe write procedure | `BLOCKED` | `BLOCKED` | `BLOCKED` |
| Large SysEx/throughput | `UNKNOWN` | `UNKNOWN`; BLE fragmentation and 5-pin rate are material risks | `BLOCKED` direct serial | `UNKNOWN` | `UNKNOWN` |

The report therefore earns only transport research guidance. It does not earn
`BYTE_FIXTURE_VERIFIED`, `PLATFORM_DEVICE_VERIFIED`, `CAPTURE_VERIFIED`, or
`HARDWARE_VERIFIED`.

## Secondary identifier and provenance leads

Two pinned community sources retain provisional USB metadata useful for a later
capture plan. They are **not** vendor specifications and must not be used for
automatic matching or compatibility claims:

| Source | Provisional report | Treatment |
| --- | --- | --- |
| [`mcp-midi-control` / `fractal-midi`, commit `5904717`](https://github.com/TheAndrewStaker/mcp-midi-control/tree/59047175cfc4f23e092931b54a7c54f2bffde3ea/packages/fractal-midi) ([Apache-2.0](https://github.com/TheAndrewStaker/mcp-midi-control/blob/59047175cfc4f23e092931b54a7c54f2bffde3ea/LICENSE), [NOTICE](https://github.com/TheAndrewStaker/mcp-midi-control/blob/59047175cfc4f23e092931b54a7c54f2bffde3ea/NOTICE)) | Reports a Fractal USB vendor identifier of `2466` in community code/data. | `SECONDARY/UNVERIFIED`; retain as a search lead only. Re-open exact source and capture the physical descriptor before any profile uses it. No codec bytes or code are copied. |
| [`ForgeFX`, commit `c22862a`](https://github.com/sKuhLight/ForgeFX/tree/c22862a5b2f2078f3cb92a2735e51f94c39a0062) ([MIT](https://github.com/sKuhLight/ForgeFX/blob/c22862a5b2f2078f3cb92a2735e51f94c39a0062/LICENSE), [NOTICE](https://github.com/sKuhLight/ForgeFX/blob/c22862a5b2f2078f3cb92a2735e51f94c39a0062/NOTICE)) | Reports `PID 8011` / interface `if03` (`MI_03`) as a possible FM3 serial interface. | `SECONDARY/UNVERIFIED`; descriptor/protocol provenance is unresolved. Do not call it CDC, do not assume nominal baud/framing, and do not match a unit from these labels alone. |

The earlier source/provenance audit records the licenses and reuse boundary;
it does not turn a community identifier into a fixture or protocol approval.
[WAVE-00-LICENSE-AUDIT]

## Native boundary and follow-up work

This packet writes no native implementation. The parent should schedule the
following bounded work, preserving the matrix labels above:

### Existing FM3 packets

* **`TOP-FM3-NATIVE-001`** — Android: enumerate a matching USB serial endpoint
  **without opening it**. Compare physical descriptors against the provisional
  community labels only as a capture lead; do not match by label alone.
* **`TOP-FM3-NATIVE-002`** — Android: open, cancel, and close only after
  `NATIVE-001` proves the exact endpoint and user permission. No guessed
  framing or write path.
* **`TOP-FM3-IOS-001`** — iOS: enumerate the evidenced FM3-compatible
  **external MIDI/interface path**. Treat ordinary direct FM3 USB serial as
  blocked; do not silently reinterpret this packet as generic serial.
* **`TOP-FM3-FIX-001`** — acquire an owner-authorized FM3 identity/read fixture
  on the selected transport with a provenance sidecar.
* **`TOP-FM3-PROTO-001`** — decode only the approved identity response after
  the fixture packet is integrated.
* **`TOP-FM3-HIL-001` / `TOP-FM3-HIL-002`** — verify Android direct USB and the
  approved iOS interface path on exact hardware/firmware/OS/adapter tuples.

### Proposed bounded follow-ups (not created by this packet)

* **`TOP-FM3-IOS-DRIVERKIT-001`** — separate M-series iPadOS 16+ DriverKit
  feasibility/entitlement packet. It must first establish whether an FM3
  driver can be built and distributed under the project's Apple account and
  whether the user-approved driver can expose the COMM interface. It must not
  be merged into ordinary iPhone/CoreMIDI work.
* **`TOP-FM3-BLE-001`** — BLE-MIDI adapter fragmentation, reconnect, idle
  timeout, and large-SysEx capability matrix using a user-owned interface.
* **`TOP-FM3-BRIDGE-001`** — local bridge pairing/authentication, protocol
  versioning, replay/session ownership, permission prompts, and no-write
  failure behavior.
* **Proposed `TOP-ADR-004` (ADR to schedule)** — record
  the FM3 distinction between generic iPhone serial (`BLOCKED`), class-compliant
  5-pin MIDI (`HYPOTHESIS`), and M-series iPadOS DriverKit (`CONDITIONAL`), and
  require exact device/firmware/transport evidence before any promotion.

## Capture and verification procedure

The next physical work must be read-only and device-specific:

1. Record FM3 Original versus Mark II Turbo, firmware, host OS/build, adapter
   model/firmware, cable/power path, and whether FM3-Edit/Fractal-Bot or another
   editor is closed.
2. For Android direct USB, enumerate first and retain sanitized descriptors,
   interface/endpoint metadata, permission result, hot-plug, and power behavior
   without opening or transmitting. A matching label from a community source is
   not enough.
3. For 5-pin, BLE, or bridge paths, select the endpoint explicitly and run only
   an approved `READ_PROBE` once a lawful identity/read vector exists. Keep
   transport identity in the fixture sidecar; never substitute a USB capture for
   a DIN/BLE/bridge capture.
4. Record whether responses are complete, fragmented, delayed, stale after
   reconnect, or absent. Preserve unknown data and do not normalize bytes in a
   way that hides a mismatch.
5. Promote a fixture only after independent expected-value derivation,
   sanitization, redistribution permission, checksum, and reviewer approval.
   Any write remains blocked until exact-profile hardware verification uses a
   disposable preset, bounded mutation, acknowledgement/read-back, undo, and
   reconnect checks.

## Source register (all independently reopened/accessed 2026-08-09)

### Fractal Audio (primary)

* **[F-FM3-MANUAL]** [FM3 Owner's Manual, v7.x, July 12, 2023](https://www.fractalaudio.com/downloads/manuals/FM3/FM3-Owners-Manual.pdf), especially pp. 20, 29–30, 110–117, 139, and 146.
* **[F-FM3-DOWNLOADS]** [FM3 Downloads](https://www.fractalaudio.com/fm3-downloads/), firmware 13.0 (July 16, 2026) and Windows USB Drivers Bundle (June 10, 2026; audio 6.16, serial 1.3).

### Android (primary)

* **[A-USB]** [USB host overview](https://developer.android.com/develop/connectivity/usb/host) and [`UsbManager`](https://developer.android.com/reference/android/hardware/usb/UsbManager): enumeration, user permission, interface/endpoint selection, claim, bulk/control transfer, detach, and close.
* **[A-MIDI]** [`android.media.midi`](https://developer.android.com/reference/android/media/midi/package-summary): USB, BLE, and virtual MIDI transports; device hot-plug; arbitrary-length SysEx; partial/raw message delivery caveat.
* **[A-BT]** [Bluetooth permissions](https://developer.android.com/develop/connectivity/bluetooth/bt-permissions): Android 12+ `BLUETOOTH_SCAN`, `BLUETOOTH_CONNECT`, `BLUETOOTH_ADVERTISE`, runtime approval, and location caveat for older targets.
* **[A-FGS]** [Connected-device foreground service](https://developer.android.com/develop/background-work/services/fgs/service-types): connected-device type, USB/Bluetooth/network prerequisites, and companion-device alternative.
* **[A-LAN]** [Local network permission](https://developer.android.com/privacy-and-security/local-network-permission), last updated July 13, 2026: Android 16 opt-in and Android 17/target SDK 37+ `ACCESS_LOCAL_NETWORK` enforcement.

### Apple (primary)

* **[APPLE-CORE]** [Core MIDI](https://developer.apple.com/documentation/coremidi): MIDI devices/endpoints, USB, BLE, and network MIDI services.
* **[APPLE-SYSEX]** [`MIDISysexSendRequest`](https://developer.apple.com/documentation/coremidi/midisysexsendrequest): asynchronous SysEx to a MIDI endpoint; this does not make a serial device a MIDI endpoint.
* **[APPLE-BLE]** [MIDI Bluetooth](https://developer.apple.com/documentation/coremidi/midi-bluetooth): BLE-MIDI pairing/reconnect and Core Bluetooth activation boundary.
* **[APPLE-NET]** [`MIDINetworkSession`](https://developer.apple.com/documentation/coremidi/midinetworksession): local network MIDI session and UDP/Bonjour model.
* **[APPLE-EA]** [External Accessory](https://developer.apple.com/documentation/externalaccessory) and [`EASession`](https://developer.apple.com/documentation/externalaccessory/easession): MFi accessory and manufacturer-declared protocol requirement.
* **[APPLE-IOUSBHOST]** [IOUSBHost](https://developer.apple.com/documentation/iousbhost): host-mode user-space USB driver objects and descriptor/pipe access; this is not evidence of a normal iPhone app API.
* **[APPLE-USBKIT]** [USBDriverKit](https://developer.apple.com/documentation/usbdriverkit): custom/non-class-compliant USB driver framework; availability is macOS and M-series iPadOS, not ordinary iPhone.
* **[APPLE-IPAD-DRIVER]** [Creating drivers for iPadOS](https://developer.apple.com/documentation/driverkit/creating-drivers-for-ipados): iPadOS 16+, M-series requirement, driver target, user approval, and iPadOS user-client entitlement boundary.
* **[APPLE-BT-PERM]** [`NSBluetoothAlwaysUsageDescription`](https://developer.apple.com/documentation/bundleresources/information-property-list/nsbluetoothalwaysusagedescription): Bluetooth usage-description requirement.
* **[APPLE-LAN]** [`NSLocalNetworkUsageDescription`](https://developer.apple.com/documentation/bundleresources/information-property-list/nslocalnetworkusagedescription): local-network prompt for direct/Bonjour/multicast use.

### Secondary/open-source provenance leads

* **[WAVE-00-LICENSE-AUDIT]** [Wave-00 source/license audit](open-source-source-and-license-audit.md), which pins the exact community commits, licenses, NOTICE files, and reuse boundary.
* **[MCP-FRACTAL]** [`mcp-midi-control` / `fractal-midi`, pinned `5904717`](https://github.com/TheAndrewStaker/mcp-midi-control/tree/59047175cfc4f23e092931b54a7c54f2bffde3ea/packages/fractal-midi) — Apache-2.0/NOTICE; community identifier lead only.
* **[FORGEFX]** [`ForgeFX`, pinned `c22862a`](https://github.com/sKuhLight/ForgeFX/tree/c22862a5b2f2078f3cb92a2735e51f94c39a0062) — MIT/NOTICE; provisional interface metadata only.

## Claims and blockers

### Claims earned by this research packet

* The FM3 USB/editor distinction is source-backed: USB audio plus COMM over
  USB, not generic USB MIDI.
* Android raw USB host and Android/Apple MIDI API mechanics are accurately
  separated from FM3 device behavior.
* Ordinary iPhone direct FM3 serial is blocked in this public API boundary;
  M-series iPadOS DriverKit is a separate conditional path.
* 5-pin, BLE-adapter, and local-bridge paths are explicitly labeled hypotheses
  with required permissions/adapters and evidence gates.
* Community VID/PID/interface identifiers are retained as secondary,
  unverified capture leads only.

### Claims not earned

* No Android, iPhone, iPad, BLE, USB, network, editor, SysEx, backup, realtime,
  throughput, firmware, or hardware compatibility claim.
* No `CAPTURE_VERIFIED`, `BYTE_FIXTURE_VERIFIED`, `PLATFORM_DEVICE_VERIFIED`, or
  `HARDWARE_VERIFIED` label.
* No permission to reuse community codec code, captures, proprietary binaries,
  vendor artwork, or unknown-rights fixtures.

### Current blockers

1. No attached FM3, Android phone/tablet, iPhone/iPad, class-compliant MIDI
   interface, BLE adapter, powered USB hub, or local bridge was available for a
   physical capture.
2. No approved FM3 identity/read fixture, USB descriptor capture, COMM framing,
   or independent expected-value witness exists in this packet.
3. iPhone direct serial remains blocked; an M-series iPadOS DriverKit path needs
   a separate design, entitlement, distribution, and hardware experiment.
4. Community identifier provenance is secondary/unverified; it cannot authorize
   endpoint matching or writes.

The independent reviewer must confirm source quality, the iPhone/iPadOS split,
secondary identifier treatment, and that no protocol bytes or compatibility
claims slipped into the report.
