# TOP-RSCH-004 source inventory

Access date for every URL below: **2026-08-08**. The report is the readable artifact; this file records the source-to-claim boundary for review.

## Apple

* CoreMIDI and MIDI services: <https://developer.apple.com/documentation/coremidi>, <https://developer.apple.com/documentation/coremidi/midi-services>
* USB/Bluetooth/network MIDI overview: <https://developer.apple.com/documentation/technologyoverviews/audio-and-music>
* BLE MIDI, pairing, idle disconnect, simulator limitation: <https://developer.apple.com/library/archive/qa/qa1831/_index.html>
* CoreBluetooth permission keys: <https://developer.apple.com/documentation/corebluetooth>
* MIDI network session: <https://developer.apple.com/documentation/coremidi/midinetworksession>
* Local network privacy and usage description: <https://developer.apple.com/documentation/technotes/tn3179-understanding-local-network-privacy>, <https://developer.apple.com/documentation/bundleresources/information-property-list/nslocalnetworkusagedescription>
* ExternalAccessory/MFi boundary: <https://developer.apple.com/documentation/externalaccessory>, <https://developer.apple.com/library/archive/qa/qa1657/_index.html>
* Background execution/Core Bluetooth lifecycle: <https://developer.apple.com/documentation/Xcode/configuring-background-execution-modes>, <https://developer.apple.com/documentation/uikit/extending-your-apps-background-execution-time>, <https://developer.apple.com/library/archive/documentation/NetworkingInternetWeb/Conceptual/CoreBluetooth_concepts/CoreBluetoothBackgroundProcessingForIOSApps/PerformingTasksWhileYourAppIsInTheBackground.html>
* Physical USB-C/adapter/unlock caveats: <https://support.apple.com/en-us/109044>, <https://support.apple.com/en-mide/105099>, <https://support.apple.com/en-gb/111806>

## Android

* MIDI API (USB, BLE, virtual and arbitrary-length data): <https://developer.android.com/reference/android/media/midi/package-summary>
* USB host, permission, endpoints and transfers: <https://developer.android.com/develop/connectivity/usb/host>
* USB interface claim/I/O: <https://developer.android.com/reference/android/hardware/usb/UsbDeviceConnection>
* Bluetooth runtime permissions: <https://developer.android.com/develop/connectivity/bluetooth/bt-permissions>
* Connected-device foreground service and launch restrictions: <https://developer.android.com/develop/background-work/services/fgs/service-types>, <https://developer.android.com/develop/background-work/services/fgs/launch>
* Local network permission rollout: <https://developer.android.com/privacy-and-security/local-network-permission>

## Fractal Audio

* Axe-Fx III manual and MIDI guide: <https://www.fractalaudio.com/downloads/manuals/axe-fx-3/Axe-Fx-III-Owners-Manual.pdf>, <https://www.fractalaudio.com/downloads/misc/Axe-Fx%20III%20MIDI%20for%203rd%20Party%20Devices.pdf>
* FM9 manual: <https://www.fractalaudio.com/downloads/manuals/FM9/FM9-Owners-Manual.pdf>
* FM3 manual: <https://www.fractalaudio.com/downloads/manuals/FM3/FM3-Owners-Manual.pdf>
* AM4 manual: <https://www.fractalaudio.com/downloads/manuals/AM4/AM4-Owners-Manual.pdf>
* VP4 manual: <https://www.fractalaudio.com/downloads/manuals/VP4/VP4-Owners-Manual.pdf>
* Axe-Fx II manual: <https://www.fractalaudio.com/downloads/manuals/axe-fx-2/archive/Axe-Fx_II_Owners_Manual_-_901.pdf>
* AX8 manual: <https://www.fractalaudio.com/downloads/manuals/AX8/AX8-Owners-Manual.pdf>
* FX8 manual: <https://www.fractalaudio.com/downloads/manuals/FX8/FX8-Owners-Manual.pdf>
* Standard/Ultra product and manual: <https://www.fractalaudio.com/axe-fx-gen-1/>, <https://www.fractalaudio.com/downloads/manuals/axe-fx/User-Manual-Axe-Fx-Ultra-Standard.pdf>

## Claim boundary

Official sources establish operating-system API capabilities and the Fractal device connector/editor descriptions. They do not establish a Topology mobile implementation, a specific firmware's complete SysEx protocol, BLE large-SysEx behavior, realtime USB routing, or exclusive ownership. Those remain `HYPOTHESIS` or `UNKNOWN` in the report until hardware-in-loop evidence exists.
