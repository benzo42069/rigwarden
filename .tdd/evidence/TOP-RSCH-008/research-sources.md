# TOP-RSCH-008 source audit

Access date for all URLs below: 2026-08-08. The report links these as `[S1]`–`[S17]`.

## Official sources

- Fractal AM4 downloads: <https://www.fractalaudio.com/am4-downloads/> — current page lists AM4 firmware 2.01, AM4 manual, and Windows USB driver.
- Fractal AM4 Owner's Manual: <https://www.fractalaudio.com/downloads/manuals/AM4/AM4-Owners-Manual.pdf> — v1.0.2 dated 2025-12-16; documents USB MIDI, Type-A TRS MIDI, AM4-Edit/SysEx, Scene MIDI physical-output distinction, and MIDI implementation.
- Fractal FM3 downloads: <https://www.fractalaudio.com/fm3-downloads/> — current page lists FM3 firmware 13.0 and Windows serial/audio driver bundle.
- Fractal FM3 Owner's Manual: <https://www.fractalaudio.com/downloads/manuals/FM3/FM3-Owners-Manual.pdf> — documents that FM3 is not a USB MIDI device, uses COMM-over-USB/USB-serial channels, and has 5-pin MIDI In/Out/Thru.
- Fractal Axe-Fx III third-party MIDI PDF: <https://fractalaudio.com/downloads/misc/Axe-Fx%20III%20MIDI%20for%203rd%20Party%20Devices.pdf> — published third-party surface; not treated as a complete AM4/FM3 editor protocol.
- Apple Core MIDI: <https://developer.apple.com/documentation/coremidi>
- Apple MIDI Services: <https://developer.apple.com/documentation/coremidi/midi-services>
- Apple MIDI Bluetooth: <https://developer.apple.com/documentation/coremidi/midi-bluetooth>
- Android USB host: <https://developer.android.com/develop/connectivity/usb/host>
- Android MIDI API: <https://developer.android.com/reference/android/media/midi/package-summary>

## Open-source method/license audits

`git ls-remote <repo> HEAD` was used to pin the following exact commits; no source tree was copied into Topology.

- TheAndrewStaker/mcp-midi-control: `59047175cfc4f23e092931b54a7c54f2bffde3ea` — Apache-2.0 root/package license; audited `CONTRIBUTING.md`, `docs/contributing/SAFETY.md`, `TIERS.md`, and `EVIDENCE.md` for method examples only.
  - <https://github.com/TheAndrewStaker/mcp-midi-control/tree/59047175cfc4f23e092931b54a7c54f2bffde3ea>
- sKuhLight/Axis: `6b87bd2472fd88854421fda0dd1d2d7a02d2dd19` — MIT root license; reuse candidate only; no Topology-compatible fixture sidecar found in audited tree.
  - <https://github.com/sKuhLight/Axis/tree/6b87bd2472fd88854421fda0dd1d2d7a02d2dd19>
- sKuhLight/ForgeFX: `c22862a5b2f2078f3cb92a2735e51f94c39a0062` — MIT root license; test fixture files exist, but no Topology-compatible redistribution sidecar was found in audited tree.
  - <https://github.com/sKuhLight/ForgeFX/tree/c22862a5b2f2078f3cb92a2735e51f94c39a0062>

## Licensing/provenance guidance

- GitHub licensing guidance: <https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/licensing-a-repository> — absence of an explicit license leaves default copyright; GitHub disclaims legal-advice status.
- SPDX license identifiers: <https://spdx.dev/learn/handling-license-info/> — exact SPDX IDs/expressions make obligations machine-readable.

## Boundary applied

The external capture guides include active editor-to-device capture methods for decoding unknown writes. This packet explicitly does **not** adopt those methods: the work-item non-goal and user instruction prohibit capturing or deriving unknown write operations. Only read-only or already documented/approved write verification is retained in the Topology plan.
