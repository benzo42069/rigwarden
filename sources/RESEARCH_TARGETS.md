# Initial Research Targets

Reverify availability, version, license, and current documentation before relying on any source.

## Official OpenAI/Codex

- Codex subagents and custom agents: `https://learn.chatgpt.com/docs/agent-configuration/subagents`
- Codex model selection: `https://learn.chatgpt.com/docs/models`

Use these to validate current custom-agent schema and model/reasoning support. Runtime evidence still controls the actual child configuration.

## Fractal Audio primary sources

- Fractal Audio support/manuals: `https://www.fractalaudio.com/support/`
- Axe-Fx III MIDI for third-party devices: `https://www.fractalaudio.com/downloads/misc/Axe-Fx%20III%20MIDI%20for%203rd%20Party%20Devices.pdf`
- Current product manuals and firmware release notes from Fractal’s official support pages.

Do not assume one product’s MIDI document fully specifies complete editor behavior for another product or firmware.

## Open-source projects to audit

- Axis: `https://github.com/sKuhLight/Axis`
- ForgeFX: `https://github.com/sKuhLight/ForgeFX`
- mcp-midi-control / fractal-midi: `https://github.com/TheAndrewStaker/mcp-midi-control`

For each:

- record exact commit;
- inspect license at repository and package/file level;
- identify provenance of protocol maps;
- distinguish code/data that may be reused from behavior that should be independently reimplemented;
- preserve notices;
- do not copy visual assets or distinctive layouts.

## Apple

- Core MIDI: `https://developer.apple.com/documentation/coremidi`
- App lifecycle, background execution, Keychain, document picker, accessibility/VoiceOver, TestFlight, App Store privacy and entitlements.

Use current official Apple documentation for deployment targets and capabilities.

## Android

- MIDI: `https://developer.android.com/develop/connectivity/usb/midi`
- USB host: `https://developer.android.com/develop/connectivity/usb/host`
- Bluetooth/BLE MIDI, Keystore, Storage Access Framework, accessibility/TalkBack, Play policies, and foreground/background execution from current Android documentation.

## Flutter and Rust bridge

- Flutter platform integration: `https://docs.flutter.dev/platform-integration`
- Flutter accessibility/testing docs.
- `flutter_rust_bridge` official repository/docs if selected after audit.

Dependency selection requires an ADR covering maintenance, mobile support, code generation, cancellation, binary size, and testability.

## Distribution

- Apple App Store current review/privacy requirements.
- Google Play current target-SDK, data-safety, and testing requirements.
- F-Droid inclusion/build requirements.
- GitHub release signing and provenance options.

## Naming

Search:

- USPTO and relevant international trademark sources;
- Apple App Store and Google Play;
- GitHub/GitLab;
- crates.io, pub.dev, npm, package IDs;
- domains and social handles;
- existing music/audio/modeler products.

The result is a preliminary collision screen, not legal clearance.
