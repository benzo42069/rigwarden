# Mobile transport feasibility and capability matrix

Packet: `TOP-RSCH-004`  
Research date/access date: **2026-08-08**  
Status: `REVIEW_APPROVED` (research layer only; no mobile or hardware compatibility claim)

This is a feasibility report, not a compatibility promise. It covers the modern first-class families (`Axe-Fx III`, `FM9`, `FM3`, `AM4`, `VP4`) and the legacy families named by the project (`Axe-Fx II`, `AX8`, `FX8`, `Axe-Fx Standard/Ultra`). The repository has no physical-hardware captures for this packet. A simulator pass, a single note-on, or an endpoint enumeration is not editor-grade SysEx, realtime, or firmware compatibility proof.

## Claim labels

* **FACT** — stated by an official platform/device source or an API contract.
* **HYPOTHESIS** — the platform and connector shape make the path plausible, but this packet has no device capture or end-to-end test.
* **UNKNOWN** — the source set does not establish the behavior. Do not expose the capability in a profile until tested.
* **BLOCKED** — the requested path is not provided by the public API/device transport described by the sources (or requires an unowned MFi/proprietary bridge).

The matrix labels the *mobile transport path*, not the existence of a desktop editor. A `HYPOTHESIS` cell means “worth testing,” not “works.” Feature claims below are intentionally stricter than transport claims.

## What the Fractal manuals establish

| Device | Official USB and DIN/TRS facts | Official editor/realtime boundary |
|---|---|---|
| **Axe-Fx III** | USB is 8x8 audio and two-way MIDI; Fractal describes it as class-compliant (the Windows driver remains a host-install concern). Five-pin MIDI IN/OUT/THRU are present. [F-A3-OWN] | The third-party MIDI guide documents PC/CC/SysEx status/control. `PUSH DATA` tempo/tuner realtime SysEx is sent to physical MIDI OUT when enabled and explicitly is **not** sent over MIDI-over-USB. [F-A3-MIDI] A USB editor transport therefore does not imply USB realtime tuner. |
| **FM9** | USB audio plus two-way MIDI for FM9-Edit/Fractal-Bot; five-pin MIDI IN and OUT. [F-FM9] | A `Send Realtime Sysex` setting exists, but the manual does not establish whether that stream is emitted over USB. Treat realtime-over-USB as **UNKNOWN** until captured. |
| **FM3** | The manual says it is **not USB MIDI**: editor/Fractal-Bot use “COMM over USB.” Windows uses USB-Serial/Audio drivers. Five-pin MIDI IN and OUT/THRU are present. [F-FM3] | Desktop COMM is a device-specific serial protocol, not a CoreMIDI endpoint. Five-pin backup/dump is documented as unsupported or very slow. [F-FM3] Android raw USB host access is a possible transport experiment; framing, permissions, and editor protocol remain **UNKNOWN**. |
| **AM4** | USB-C is USB 2.0 class-compliant audio (4x4) with added MIDI-over-USB for AM4-Edit/Fractal-Bot/DAW. MIDI uses 3.5-mm Type-A TRS MIDI In/Out; a Type-A TRS-to-five-pin adapter is required. [F-AM4] | The manual advertises editor transport but publishes no editor SysEx framing or realtime tuner stream. Full mobile editor behavior is **UNKNOWN** pending capture. |
| **VP4** | USB-C is USB 2.0 class-compliant audio (2x2) with added MIDI-over-USB for VP4-Edit/Fractal-Bot/DAW. MIDI uses 3.5-mm Type-A TRS MIDI In/Out; a Type-A adapter is required. [F-VP4] | As with AM4, the manual does not publish a mobile/editor protocol or USB realtime tuner contract. **UNKNOWN** pending capture. |
| **Axe-Fx II** | USB provides two-way audio/MIDI; the manual describes a class-compliant USB connection with host-software/driver caveats. Seven-pin MIDI IN and five-pin MIDI OUT/THRU are present. [F-A2] | The manual describes MIDI SysEx tuner/tempo and USB-adapter behavior, but the required host/firmware combinations and mobile behavior need hardware verification. Full mobile editor is **UNKNOWN**. |
| **AX8** | USB is two-way MIDI (no USB audio); five-pin DIN IN/OUT/THRU. [F-AX8] | AX8-Edit uses SysEx extensively. The transport is plausible through a class-compliant interface, but mobile enumeration, large SysEx, and firmware variants are **UNKNOWN**. |
| **FX8** | USB is two-way MIDI; five-pin DIN IN/OUT/THRU. [F-FX8] | FX8-Edit uses SysEx extensively. Mobile transport and editor-scale transfers are **UNKNOWN** until tested. |
| **Axe-Fx Standard/Ultra** | No built-in USB editor transport is documented; the product page/manual route editing through five-pin MIDI and an external MIDI interface (the manual describes preset SysEx dumps). [F-GEN1] | Direct phone-to-device USB MIDI is **BLOCKED** because the device has no USB MIDI port. Five-pin through an external class-compliant interface is a plausible transport; editor-scale SysEx, BLE, and bridge behavior are **UNKNOWN**. |

## Device × mobile OS × transport matrix

Each cell is `iOS / Android`. It describes the transport path only. `H` is a hardware-tested-required hypothesis; `B` is blocked by the documented device/API boundary; `N/A` means the device does not expose that transport. A cell marked `H` never grants editor, backup, or realtime capability.

| Device | Direct USB MIDI | Direct USB serial | Class-compliant USB-MIDI interface | Five-pin/Type-A MIDI through interface | BLE MIDI adapter | Local network bridge |
|---|---|---|---|---|---|---|
| Axe-Fx III | `H / H` — CoreMIDI and Android MIDI support USB MIDI; physical endpoint and power test required. [APPLE-AUDIO][ANDROID-MIDI][F-A3-OWN] | `N/A / N/A` — no serial transport is documented. | `H / H` — interface must expose DIN; SysEx/realtime still feature-gated. | `H / H` — direct five-pin requires the external interface. | `H / H` — Fractal does not document BLE; use a third-party BLE-MIDI adapter and test fragmentation. | `H / H` — bridge is an application/desktop/Raspberry Pi feature, not a Fractal-native path. |
| FM9 | `H / H` — USB MIDI is documented; mobile capture required. [F-FM9] | `N/A / N/A` | `H / H` | `H / H` | `H / H` — adapter required; no Fractal BLE claim. | `H / H` |
| FM3 | `B / B` — device explicitly is not USB MIDI. [F-FM3] | `B / H` — generic iOS serial is not provided; Android `UsbManager` can address raw endpoints, but FM3 framing/protocol is unknown. [APPLE-EA][ANDROID-USB] | `H / H` — external MIDI interface to the FM3 five-pin ports. | `H / H` | `H / H` — adapter to five-pin; not native FM3 BLE. | `H / H` |
| AM4 | `H / H` — USB MIDI is documented, but no mobile capture. [F-AM4] | `N/A / N/A` | `H / H` | `H / H` — Type-A TRS-to-DIN adapter plus interface. [F-AM4] | `H / H` | `H / H` |
| VP4 | `H / H` — USB MIDI is documented, but no mobile capture. [F-VP4] | `N/A / N/A` | `H / H` | `H / H` — Type-A TRS-to-DIN adapter plus interface. [F-VP4] | `H / H` | `H / H` |
| Axe-Fx II | `H / H` — USB MIDI is documented; host/firmware and mobile power must be tested. [F-A2] | `N/A / N/A` | `H / H` | `H / H` | `H / H` | `H / H` |
| AX8 | `H / H` — USB MIDI is documented; mobile endpoint and SysEx test required. [F-AX8] | `N/A / N/A` | `H / H` | `H / H` | `H / H` | `H / H` |
| FX8 | `H / H` — USB MIDI is documented; mobile endpoint and SysEx test required. [F-FX8] | `N/A / N/A` | `H / H` | `H / H` | `H / H` | `H / H` |
| Axe-Fx Standard/Ultra | `B / B` — no built-in USB MIDI. [F-GEN1] | `N/A / N/A` | `H / H` — external interface to five-pin is required. | `H / H` | `H / H` — third-party adapter only. | `H / H` |

### Why the mobile cells are hypotheses

Apple states that Core MIDI handles USB, Bluetooth, and network MIDI and exposes MIDI endpoints and SysEx operations. [APPLE-CORE][APPLE-AUDIO] Android's `android.media.midi` handles USB/BLE/virtual MIDI and arbitrary-length data; Android's USB host API can enumerate interfaces and perform bulk/control transfers. [ANDROID-MIDI][ANDROID-USB] Those facts establish platform capability, not a Fractal device profile. We must still capture VID/PID, endpoint topology, hot-plug/power behavior, framing, maximum message/chunk size, and a known device response on each target OS.

The FM3 exception is material: its official manual calls the USB path COMM/USB-Serial, so it cannot be treated as USB MIDI. iOS has no generic public serial-host API in the cited sources; Apple's ExternalAccessory path is for MFi accessories with manufacturer-declared protocols, and BLE is handled by CoreBluetooth instead. [F-FM3][APPLE-EA] Android raw USB serial is therefore a bounded experiment, not a compatibility claim.

## Permissions, entitlements, adapters, and lifecycle

| Concern | iOS | Android | Engineering consequence |
|---|---|---|---|
| USB MIDI | CoreMIDI owns USB MIDI devices and exposes endpoints; the phone/tablet still needs a data-capable host adapter/cable and enough power. Apple documents USB-C/adapter data and accessory-unlock behavior. [APPLE-CORE][APPLE-USB] | Declare `android.hardware.usb.host`; enumerate with `UsbManager`; request per-device user permission; perform transfers off the UI thread. [ANDROID-USB] | Native adapter must surface permission denial, attach/detach, power failure, and endpoint metadata. Do not silently retry a revoked grant. |
| Generic USB serial | No generic serial path is established here. `ExternalAccessory` is MFi wired/Bluetooth Classic and manufacturer-protocol based; BLE is not ExternalAccessory. [APPLE-EA] | `UsbManager`/`UsbDeviceConnection` can inspect interfaces and claim/bulk-transfer endpoints after permission. [ANDROID-USB][ANDROID-USB-CONN] | FM3 needs a dedicated Android adapter and a provenance-approved framing fixture. iOS FM3 USB is a blocker unless a user-owned MFi/bridge accessory is in scope. |
| BLE MIDI | Core MIDI presents a paired BLE MIDI endpoint as an ordinary MIDI device; QA1831 requires secure pairing and notes idle links can terminate. CoreAudioKit pairing UI is unavailable in the iOS Simulator. [APPLE-BLE] CoreBluetooth apps need `NSBluetoothAlwaysUsageDescription` (older deployment targets also use `NSBluetoothPeripheralUsageDescription`). [APPLE-BLE-PERM] | `android.media.midi` supports BLE MIDI. Target Android 12+ requires runtime `BLUETOOTH_SCAN`/`BLUETOOTH_CONNECT` (and `BLUETOOTH_ADVERTISE` when advertising); location is only needed when deriving location, with the documented `neverForLocation` caveat. Targets through Android 11 use legacy `BLUETOOTH`/`BLUETOOTH_ADMIN` plus runtime `ACCESS_FINE_LOCATION` unless a documented Companion Device Manager path applies. [ANDROID-MIDI][ANDROID-BT] | Treat BLE MTU/fragmentation, pairing, reconnect, and large SysEx as unknown. A note-on proves only a small message path. |
| Network MIDI / bridge | `MIDINetworkSession` manages local network MIDI sources/destinations over UDP/Bonjour. iOS/iPadOS 14+ requires `NSLocalNetworkUsageDescription`; multicast additionally requires `com.apple.developer.networking.multicast`. [APPLE-NET][APPLE-LAN] | A bridge may use sockets/NSD. Android's current rollout makes local-network access opt-in on Android 16 and requires `ACCESS_LOCAL_NETWORK` for target SDK 37+ on Android 17; older targets currently rely on `INTERNET`. [ANDROID-LAN] | Bridge protocol must define pairing, mutual authentication, encryption, protocol negotiation, session ownership, replay/timeout behavior, and no cloud relay. Permission behavior is part of the capability snapshot. |
| Background / lock screen | UIKit normally suspends an app shortly after it enters background. Core Bluetooth can wake a declared `bluetooth-central`/`bluetooth-peripheral` app, but the app may still be terminated and has bounded work after wake. [APPLE-BG][APPLE-BG-BLE] | Long-lived connected-device work uses a connected-device foreground service (with `FOREGROUND_SERVICE_CONNECTED_DEVICE` and the relevant runtime permission) or a companion-device flow. Android 12+ restricts starting a foreground service from the background; newer targets enforce service-type checks. [ANDROID-FGS][ANDROID-FGS-LAUNCH] | Continuous editor transfers should be foreground-first, resumable, and cancellable. “Background supported” is not a transport fact; verify on locked devices and after process death. |
| Exclusive ports | CoreMIDI endpoint enumeration/ports do not establish ownership against another editor. [APPLE-CORE] | Android MIDI is brokered, while raw USB interface claiming can conflict with another claimant; the API does not establish Fractal-editor semantics. [ANDROID-MIDI][ANDROID-USB-CONN] | Detect busy/disconnect and explain the conflict. Never force-claim, fight another editor, or call an endpoint exclusive without a measured device-specific rule. |

### Adapter and power requirements

* USB-C iPhone/iPad testing needs a data-capable cable and, for bus-powered Fractal units/interfaces, a powered USB-C hub. Lightning devices need Apple's camera/USB adapter plus power. These are connection/power constraints, not protocol proof. [APPLE-USB]
* AM4 and VP4 use Type-A 3.5-mm TRS MIDI; the manual requires a Type-A TRS-to-five-pin adapter before a DIN interface is involved. [F-AM4][F-VP4]
* Five-pin-only units require a class-compliant USB-MIDI interface or a BLE-MIDI/network bridge. A named adapter is a test input, not a compatibility guarantee.

## Transport versus editor/realtime features

| Feature | Evidence level and boundary |
|---|---|
| Enumerate/open/close, attach/detach | Platform APIs document enumeration and lifecycle for their supported classes. [APPLE-CORE][ANDROID-MIDI][ANDROID-USB] Device-specific mobile endpoint, power, and reconnect behavior remains **HYPOTHESIS** until hardware tests. |
| PC/CC preset, scene, bypass, channel control | Axe-Fx III's official third-party guide documents standard PC/CC and SysEx status/control. [F-A3-MIDI] Other families' manuals document MIDI control at varying levels. A mobile transport endpoint does not prove a particular firmware command set. |
| Identity query, parameter read/write, preset dump/editor | CoreMIDI and Android MIDI can carry SysEx (Android explicitly permits arbitrary-length data). [APPLE-CORE][ANDROID-MIDI] Device framing, checksums, response timing, chunking, and destructive-write verification are **UNKNOWN** unless backed by an official vector or a provenance-approved user-owned capture. |
| Large SysEx | USB MIDI and CoreMIDI APIs provide a transport mechanism. BLE MIDI MTU fragmentation, Android/iOS buffering, bridge framing, and every device's maximum frame/chunk are **UNKNOWN**. A successful note-on or short CC is not evidence. |
| Realtime tuner/tempo | Axe-Fx III is the hard boundary: `PUSH DATA` is sent to physical MIDI OUT when enabled and the guide says it does not stream over MIDI-over-USB. [F-A3-MIDI] FM9's setting is documented but its USB route is not established; FM3/AM4/VP4/legacy mobile routes are **UNKNOWN** until captured. |
| Background editing/streaming | iOS suspension and Android foreground-service rules limit lifecycle; neither platform document promises an always-on editor session. [APPLE-BG][ANDROID-FGS] This must be a per-transport/per-feature capability, not a global “mobile supported” flag. |

### Apparent source tensions and their resolution

* The Axe-Fx III manual describes USB MIDI as providing the same general control functions as five-pin MIDI, while the separate third-party guide gives the narrower realtime rule: `PUSH DATA` tuner/tempo goes to physical MIDI OUT and not USB. The specific realtime statement wins for that feature; the general USB control statement must not be extended to realtime streaming. [F-A3-OWN][F-A3-MIDI]
* Fractal manuals advertise desktop editors over USB for several families. That is evidence of a device-side transport/editor pair, not evidence that iOS or Android can open the same session, sustain its lifecycle, or reproduce its proprietary SysEx. The matrix therefore keeps mobile cells as `H` and feature rows as `UNKNOWN`.
* Android's raw USB host API can transfer bytes on an interface even when a device is not USB MIDI. That does not contradict FM3's “not USB MIDI” statement: it only makes FM3 serial a possible Android transport experiment, with framing and protocol still unknown. [F-FM3][ANDROID-USB]
* Older manuals and firmware-specific notes are not silently generalized to newer hardware. Where a manual exposes a setting but omits its USB/BLE route (for example, FM9 realtime SysEx), the report records `UNKNOWN` rather than inferring parity.

## Recommended native boundary

Keep Flutter presentation and Rust protocol/state deterministic. Native code should be a thin transport adapter with no Fractal command guessing:

1. **Swift adapter**: CoreMIDI enumeration (`MIDIGetNumberOfDevices`/entities/endpoints), open/close, input callbacks, `MIDISend`/`MIDISysexSendRequest`, cancellation, and hot-plug. Add `MIDINetworkSession` for the network bridge and CoreBluetooth only for BLE pairing/discovery. Do not route FM3 generic USB serial through `ExternalAccessory` without an in-scope MFi accessory and declared protocol. [APPLE-CORE][APPLE-BLE][APPLE-NET][APPLE-EA]
2. **Kotlin adapter**: `MidiManager`/`MidiDeviceInfo`/MIDI input-output ports for USB/BLE MIDI; `UsbManager` + permission + descriptor inspection + `UsbDeviceConnection.claimInterface`/bulk/control transfers for FM3 serial; sockets/NSD for a bridge. Put long sessions behind connected-device foreground service/companion lifecycle. [ANDROID-MIDI][ANDROID-USB][ANDROID-FGS]
3. **Rust contract**: normalize endpoint identity, transport kind, VID/PID, device family/firmware, permission state, maximum message/chunk, SysEx support, realtime route, background/reconnect limits, and busy/exclusive status into a capability snapshot. Rust owns framing, checksums, timeout/retry/cancel, transaction correlation, response validation, and write verification. Unknown firmware remains read-only/unsupported.
4. **Security boundary**: AI and UI receive decoded domain data only; no AI path receives raw transport handles or bytes. The bridge must be local-first with authenticated pairing and no cloud relay.

## Physical test matrix and experiments

### Suggested test set

| Platform | Devices/adapters | Minimum evidence |
|---|---|---|
| iOS USB-C iPhone/iPad | Powered USB-C hub; Axe-Fx III, FM9, FM3 (negative serial case), AM4, VP4, AX8/FX8; also a class-compliant DIN interface | VID/PID/endpoint capture, permission/unplug/lock behavior, known identity/SysEx query and response, large transfer, write verification. |
| iOS Lightning (if supported) | Apple camera/USB adapter + power; same interface/device set | Same evidence; record adapter power and accessory-unlock state. |
| Android USB host phone/tablet | Current Pixel/Samsung-class USB-C host, OTG/powered hub; same Fractal set | `UsbDevice` descriptors and grants; MIDI endpoint capture; FM3 interface/framing observation; process death/reconnect and foreground-service behavior. |
| Five-pin and TRS | One class-compliant interface (for example, a Roland UM-ONE-class or CME U2MIDI-class adapter); Type-A TRS-to-DIN adapter for AM4/VP4 | DIN/ TRS polarity and round-trip tests; no adapter model is certified by this report. |
| BLE | A user-owned BLE-MIDI adapter (for example, a WIDI-class adapter) on five-pin/TRS; iOS CoreAudioKit and Android MIDI pairing | Pair/reconnect, idle timeout, MTU fragmentation, ordered large SysEx, cancellation, and locked-screen behavior. |
| Network bridge | Raspberry Pi or desktop bridge on the same LAN; iOS `MIDINetworkSession`, Android socket/NSD client | Pairing/authentication, local-network prompts, protocol negotiation, session ownership, loss/reconnect, and no-cloud verification. |

### Required experiments and blockers

1. Enumerate every direct USB target on both OSes; retain descriptors, VID/PID, endpoint names, power draw/failure, hot-plug, and firmware version. **Blocker:** no hardware in this packet.
2. Start with a provenance-approved identity/query vector. For Axe-Fx III, use only the command and checksum documented in the official third-party guide. **Do not infer editor support from note-on/CC.**
3. Exercise a large preset/status SysEx on USB MIDI, DIN-through-interface, BLE, and bridge; record fragmentation, MTU, maximum chunk, ordering, timeout, retry, cancellation, and reconnect.
4. FM3: inspect Android USB descriptors and capture the user-owned COMM framing without guessing writes. Mark iOS generic serial as blocked unless an in-scope MFi/bridge accessory is supplied.
5. Capture realtime tuner/tempo independently per device and transport. Confirm the Axe-Fx III negative USB case and do not generalize it to FM9 or newer units.
6. Run conflict tests with Fractal-Edit/Fractal-Bot, a second app, and a second endpoint. Detect busy/ownership behavior without force-claiming.
7. Run lock/background/process-death tests: iOS BLE wake/termination and Android connected-device FGS/companion. Treat each result as transport- and feature-specific.
8. Test cable/adapter/powered-hub combinations and iOS accessory unlock. Record physical setup as part of the evidence, not as a hidden prerequisite.
9. Promote only hardware-in-loop evidence to the L8 hardware rung. Simulator/CoreAudioKit-unavailable evidence is L3/API evidence; it cannot close a hardware compatibility claim. [APPLE-BLE]

Follow-up work should be split into bounded packets/ADRs: native transport boundary; mobile USB-MIDI adapter; FM3 Android serial capture; BLE SysEx fragmentation/capabilities; local bridge security/session ownership; per-device capability/profile gating; and hardware verification for each firmware/OS/adapter tuple.

## Sources (official, accessed 2026-08-08)

### Apple

* **[APPLE-CORE]** [Core MIDI](https://developer.apple.com/documentation/coremidi) and [MIDI services](https://developer.apple.com/documentation/coremidi/midi-services) — MIDI clients, devices/entities/endpoints, ports, and SysEx operations.
* **[APPLE-AUDIO]** [Audio and music technology overview](https://developer.apple.com/documentation/technologyoverviews/audio-and-music) — Core MIDI over USB, Bluetooth, and network connections.
* **[APPLE-BLE]** [QA1831: Using Bluetooth LE MIDI](https://developer.apple.com/library/archive/qa/qa1831/_index.html) — secure pairing, ordinary MIDI endpoint behavior, idle disconnect, and simulator limitation.
* **[APPLE-BLE-PERM]** [Core Bluetooth](https://developer.apple.com/documentation/corebluetooth) — Bluetooth usage-description keys.
* **[APPLE-NET]** [MIDINetworkSession](https://developer.apple.com/documentation/coremidi/midinetworksession) — local network MIDI session and UDP/Bonjour model.
* **[APPLE-LAN]** [Local network privacy](https://developer.apple.com/documentation/technotes/tn3179-understanding-local-network-privacy) and [`NSLocalNetworkUsageDescription`](https://developer.apple.com/documentation/bundleresources/information-property-list/nslocalnetworkusagedescription) — iOS/iPadOS local-network prompt and multicast entitlement boundary.
* **[APPLE-EA]** [External Accessory](https://developer.apple.com/documentation/externalaccessory) and [QA1657](https://developer.apple.com/library/archive/qa/qa1657/_index.html) — MFi accessory/protocol boundary; BLE is handled by CoreBluetooth, not ExternalAccessory.
* **[APPLE-BG]** [Configuring background execution modes](https://developer.apple.com/documentation/Xcode/configuring-background-execution-modes) and [UIKit background execution](https://developer.apple.com/documentation/uikit/extending-your-apps-background-execution-time) — declared modes and normal suspension.
* **[APPLE-BG-BLE]** [Core Bluetooth background processing](https://developer.apple.com/library/archive/documentation/NetworkingInternetWeb/Conceptual/CoreBluetooth_concepts/CoreBluetoothBackgroundProcessingForIOSApps/PerformingTasksWhileYourAppIsInTheBackground.html) — wake/termination and bounded background behavior.
* **[APPLE-USB]** [Connect iPhone to USB accessories](https://support.apple.com/en-us/109044), [USB-C accessories](https://support.apple.com/en-mide/105099), and [unlock accessories](https://support.apple.com/en-gb/111806) — data/power/unlock caveats for physical setup.

### Android

* **[ANDROID-MIDI]** [`android.media.midi`](https://developer.android.com/reference/android/media/midi/package-summary) — USB/BLE/virtual MIDI, enumeration, notifications, timestamps, and arbitrary-length data.
* **[ANDROID-USB]** [USB host overview](https://developer.android.com/develop/connectivity/usb/host) — host feature, `UsbManager`, permission, interfaces/endpoints, and transfers.
* **[ANDROID-USB-CONN]** [`UsbDeviceConnection`](https://developer.android.com/reference/android/hardware/usb/UsbDeviceConnection) — interface claiming and endpoint I/O boundary.
* **[ANDROID-BT]** [Bluetooth permissions](https://developer.android.com/develop/connectivity/bluetooth/bt-permissions) — Android 12+ runtime permissions and location caveat.
* **[ANDROID-FGS]** [Connected-device foreground service](https://developer.android.com/develop/background-work/services/fgs/service-types) — connected-device service type, permission, and alternatives.
* **[ANDROID-FGS-LAUNCH]** [Foreground-service launch restrictions](https://developer.android.com/develop/background-work/services/fgs/launch) — background-start and target-SDK checks.
* **[ANDROID-LAN]** [Local network permission](https://developer.android.com/privacy-and-security/local-network-permission) — Android 16/17 rollout and `ACCESS_LOCAL_NETWORK` boundary.

### Fractal Audio

* **[F-A3-OWN]** [Axe-Fx III Owner's Manual](https://www.fractalaudio.com/downloads/manuals/axe-fx-3/Axe-Fx-III-Owners-Manual.pdf).
* **[F-A3-MIDI]** [Axe-Fx III MIDI for third-party devices](https://www.fractalaudio.com/downloads/misc/Axe-Fx%20III%20MIDI%20for%203rd%20Party%20Devices.pdf).
* **[F-FM9]** [FM9 Owner's Manual](https://www.fractalaudio.com/downloads/manuals/FM9/FM9-Owners-Manual.pdf).
* **[F-FM3]** [FM3 Owner's Manual](https://www.fractalaudio.com/downloads/manuals/FM3/FM3-Owners-Manual.pdf).
* **[F-AM4]** [AM4 Owner's Manual](https://www.fractalaudio.com/downloads/manuals/AM4/AM4-Owners-Manual.pdf).
* **[F-VP4]** [VP4 Owner's Manual](https://www.fractalaudio.com/downloads/manuals/VP4/VP4-Owners-Manual.pdf).
* **[F-A2]** [Axe-Fx II Owner's Manual](https://www.fractalaudio.com/downloads/manuals/axe-fx-2/archive/Axe-Fx_II_Owners_Manual_-_901.pdf).
* **[F-AX8]** [AX8 Owner's Manual](https://www.fractalaudio.com/downloads/manuals/AX8/AX8-Owners-Manual.pdf).
* **[F-FX8]** [FX8 Owner's Manual](https://www.fractalaudio.com/downloads/manuals/FX8/FX8-Owners-Manual.pdf).
* **[F-GEN1]** [Axe-Fx Gen 1 product page](https://www.fractalaudio.com/axe-fx-gen-1/) and [Standard/Ultra User Manual](https://www.fractalaudio.com/downloads/manuals/axe-fx/User-Manual-Axe-Fx-Ultra-Standard.pdf).

## Handoff boundary

* This report creates no native code, protocol fixture, profile, or compatibility guarantee.
* The only direct mobile `BLOCKED` claim is generic iOS USB serial for FM3 (no public generic serial path in the cited API set) and direct USB for Standard/Ultra (no device USB port). An MFi or network bridge would be a separate product boundary.
* All `H` cells require physical hardware-in-loop evidence with exact firmware, OS build, adapter/cable/power, and capture artifacts before becoming `FACT`.
* Realtime tuner/tempo, large SysEx, editor write verification, background continuity, and exclusive ownership are separate capabilities. They must not be inferred from transport enumeration or note-on success.
