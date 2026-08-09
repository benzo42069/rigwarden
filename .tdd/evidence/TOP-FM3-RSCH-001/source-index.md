# TOP-FM3-RSCH-001 source index

All rows below were independently reopened or rechecked on **2026-08-09**.
The report is the bounded synthesis; this index preserves the direct URLs,
authority tier, and exact claim boundary.

## Primary FM3 sources

| ID | Tier | Source | Evidence used |
| --- | --- | --- | --- |
| F-FM3-MANUAL | Official Fractal manual | [FM3 Owner's Manual, v7.x (PDF)](https://www.fractalaudio.com/downloads/manuals/FM3/FM3-Owners-Manual.pdf) | pp. 20, 29–30: USB audio, not USB MIDI, `COMM over USB`, USB-B host port; pp. 110–117: realtime MIDI setting and 5-pin backup/dump limitation; p. 139: 5-pin MIDI control; p. 146: SysEx used extensively by FM3-Edit. |
| F-FM3-DOWNLOADS | Official Fractal support page | [FM3 Downloads](https://www.fractalaudio.com/fm3-downloads/) | Firmware 13.0, compatible FM3 Original/Mark II Turbo, July 16 2026; Windows USB bundle June 10 2026, audio 6.16 and serial 1.3, both required. |

## Primary Android sources

| ID | Tier | Source | Evidence used |
| --- | --- | --- | --- |
| A-USB | Official Android API guide | [USB host overview](https://developer.android.com/develop/connectivity/usb/host) and [`UsbManager`](https://developer.android.com/reference/android/hardware/usb/UsbManager) | Host enumeration, user permission, interface/endpoint selection, descriptor checks, claim, bulk/control transfer, off-UI-thread work, detach and close. Platform mechanics only; no FM3 identity or framing. |
| A-MIDI | Official Android API reference | [`android.media.midi`](https://developer.android.com/reference/android/media/midi/package-summary) | USB/BLE/virtual MIDI, hot-plug, arbitrary-length SysEx, and raw/partial message delivery caveat. Applies to MIDI devices, not FM3 COMM/USB serial. |
| A-BT | Official Android permissions guide | [Bluetooth permissions](https://developer.android.com/develop/connectivity/bluetooth/bt-permissions) | Android 12+ runtime `BLUETOOTH_SCAN`, `BLUETOOTH_CONNECT`, `BLUETOOTH_ADVERTISE`; location caveat and `neverForLocation` option. |
| A-FGS | Official Android service guide | [Connected-device foreground service](https://developer.android.com/develop/background-work/services/fgs/service-types) | Connected-device service type, prerequisites, USB/Bluetooth/network examples, and companion-device alternative. |
| A-LAN | Official Android privacy guide | [Local network permission](https://developer.android.com/privacy-and-security/local-network-permission) | Android 16 opt-in rollout and Android 17/target SDK 37+ `ACCESS_LOCAL_NETWORK` enforcement for raw LAN sockets. |

## Primary Apple sources

| ID | Tier | Source | Evidence used |
| --- | --- | --- | --- |
| APPLE-CORE | Official Apple API reference | [Core MIDI](https://developer.apple.com/documentation/coremidi) | USB, BLE, and network MIDI endpoint services; no generic FM3 COMM serial endpoint. |
| APPLE-SYSEX | Official Apple API reference | [`MIDISysexSendRequest`](https://developer.apple.com/documentation/coremidi/midisysexsendrequest) | Async SysEx requires a MIDI endpoint; it does not expose arbitrary serial USB. |
| APPLE-BLE | Official Apple API reference | [MIDI Bluetooth](https://developer.apple.com/documentation/coremidi/midi-bluetooth) | BLE-MIDI pairing/reconnect and Core Bluetooth activation boundary. |
| APPLE-NET | Official Apple API reference | [`MIDINetworkSession`](https://developer.apple.com/documentation/coremidi/midinetworksession) | Local-network MIDI session, contacts, UDP port, and Bonjour name. |
| APPLE-EA | Official Apple API reference | [External Accessory](https://developer.apple.com/documentation/externalaccessory) and [`EASession`](https://developer.apple.com/documentation/externalaccessory/easession) | MFi accessory and manufacturer-declared protocol requirement; not a generic FM3 serial API. |
| APPLE-IOUSBHOST | Official Apple API reference | [IOUSBHost](https://developer.apple.com/documentation/iousbhost) | Host-mode user-space USB driver objects, descriptor access, and pipes; not evidence of a normal iPhone app transport. |
| APPLE-USBKIT | Official Apple API reference | [USBDriverKit](https://developer.apple.com/documentation/usbdriverkit) | Custom/non-class-compliant USB driver availability on macOS and M-series iPadOS; not ordinary iPhone. |
| APPLE-IPAD-DRIVER | Official Apple article | [Creating drivers for iPadOS](https://developer.apple.com/documentation/driverkit/creating-drivers-for-ipados) | iPadOS 16+, M-series requirement, DriverKit target, user approval, and iPadOS user-client entitlement. |
| APPLE-BT-PERM | Official Apple API reference | [`NSBluetoothAlwaysUsageDescription`](https://developer.apple.com/documentation/bundleresources/information-property-list/nsbluetoothalwaysusagedescription) | Bluetooth usage description required for Core Bluetooth APIs. |
| APPLE-LAN | Official Apple API reference | [`NSLocalNetworkUsageDescription`](https://developer.apple.com/documentation/bundleresources/information-property-list/nslocalnetworkusagedescription) | User-facing local network usage description for direct/Bonjour/multicast use. |

## Secondary/open-source provenance leads

| ID | Tier | Source | Treatment |
| --- | --- | --- | --- |
| MCP-FRACTAL | Community Apache-2.0 source | [`mcp-midi-control` / `fractal-midi`, pinned commit `5904717`](https://github.com/TheAndrewStaker/mcp-midi-control/tree/59047175cfc4f23e092931b54a7c54f2bffde3ea/packages/fractal-midi), [LICENSE](https://github.com/TheAndrewStaker/mcp-midi-control/blob/59047175cfc4f23e092931b54a7c54f2bffde3ea/LICENSE), [NOTICE](https://github.com/TheAndrewStaker/mcp-midi-control/blob/59047175cfc4f23e092931b54a7c54f2bffde3ea/NOTICE) | Community VID label `2466` retained as `SECONDARY/UNVERIFIED` capture lead only. No code, bytes, or automatic matching. |
| FORGEFX | Community MIT source | [`ForgeFX`, pinned commit `c22862a`](https://github.com/sKuhLight/ForgeFX/tree/c22862a5b2f2078f3cb92a2735e51f94c39a0062), [LICENSE](https://github.com/sKuhLight/ForgeFX/blob/c22862a5b2f2078f3cb92a2735e51f94c39a0062/LICENSE), [NOTICE](https://github.com/sKuhLight/ForgeFX/blob/c22862a5b2f2078f3cb92a2735e51f94c39a0062/NOTICE) | Community PID/interface labels `8011` / `if03` (`MI_03`) retained as `SECONDARY/UNVERIFIED` capture leads only. No CDC/framing/baud inference. |
| WAVE-00-LICENSE-AUDIT | Repository research record | [Open-source source and license audit](../../../docs/research/open-source-source-and-license-audit.md) | Existing independent provenance review; community reports remain reference-only and do not authorize reuse or compatibility. |

## Source limitations

No physical FM3, endpoint descriptor, COMM frame, identity response, or
owner-authorized capture was available. Official sources describe device and
platform boundaries but do not publish the complete FM3-Edit wire protocol.
Community identifiers are provisional leads and are explicitly not profile
inputs. No source in this packet supplies permission to redistribute vendor
binaries, artwork, presets, or unknown-rights captures.
