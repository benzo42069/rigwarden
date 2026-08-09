# Product Decision Log

This file records interview decisions already made. Do not reopen them merely because implementation would be easier another way.

| ID | Decision |
|---|---|
| DEC-001 | Public pre-alpha identity is **RigWarden**, subject to current collision and trademark research. |
| DEC-002 | Tagline is **“An open editor for modern modelers.”** |
| DEC-003 | RigWarden is an independent community project with no Abyssal Audio branding or operational tie. |
| DEC-004 | Initial architecture is vendor-neutral internally but Fractal-focused publicly. |
| DEC-005 | Modern first-class scope is AM4, VP4, Axe-Fx III, FM9, and FM3. |
| DEC-006 | Legacy devices matter and may launch under honest community/experimental labels without owner-verified testing by the founder. |
| DEC-007 | Existing Axis, ForgeFX, and fractal-midi work must be audited as research/reuse candidates, but RigWarden will use an independent monorepo and architecture. |
| DEC-008 | No requirement to contact upstream maintainers or preserve upstream compatibility. |
| DEC-009 | Monorepo. |
| DEC-010 | Original code uses a permissive MIT license; derivative or reused material retains its compatible original license and notices. |
| DEC-011 | No account is required for normal use. |
| DEC-012 | Source and official store builds are free. |
| DEC-013 | Optional telemetry is opt-in. |
| DEC-014 | Complete editor on phones and tablets. |
| DEC-015 | Offline editing is required. |
| DEC-016 | All listed editor features are in the target product scope. |
| DEC-017 | No automatic preset backup before first write. |
| DEC-018 | Persistent undo/redo across crashes and disconnects is required. |
| DEC-019 | Manual snapshots and optional snapshot reminders are acceptable. |
| DEC-020 | Custom performance panels are required and portable. |
| DEC-021 | All technically feasible transport paths are targeted; unsupported combinations are represented in a capability matrix. |
| DEC-022 | It is acceptable to document that some units require a third-party MIDI interface or bridge. |
| DEC-023 | Distribution target includes TestFlight, App Store, Play testing/production, signed GitHub APKs, and F-Droid. |
| DEC-024 | Founder acts as final product decision-maker early, then adds trusted maintainers. |
| DEC-025 | Community conduct prohibits harassment, personal campaigns, and review bombing. |
| DEC-026 | UI architecture: Flutter presentation, Rust core, narrow Swift/Kotlin native transport modules. |
| DEC-027 | Production mobile builds contain no Node runtime, local HTTP service, or WebView editor shell. |
| DEC-028 | Desktop clients should later ship from the same monorepo. |
| DEC-029 | iOS/iPadOS is the lead reference implementation; baseline recommendation is iOS/iPadOS 16+ and Android 10+, subject to toolchain audit. |
| DEC-030 | Every major screen supports phone/tablet and portrait/landscape. |
| DEC-031 | Tablet mouse, keyboard, trackpad, stylus precision, drag-and-drop, and split-screen are targeted. |
| DEC-032 | Accessibility is non-negotiable, especially blind-user workflows. |
| DEC-033 | Stage Mode may keep the screen awake, lock navigation, and preserve connection where the OS permits. |
| DEC-034 | One active editor target initially; architecture prepares for multi-device rigs. |
| DEC-035 | Simulator and Capture Lab exist from the beginning. |
| DEC-036 | Before public beta, USB MIDI, Android FM3 USB serial, class-compliant interfaces, 5-pin MIDI interfaces, BLE MIDI, and a network bridge are targeted. BLE/bridge may initially be experimental. |
| DEC-037 | Automatic detection plus manual fallback. |
| DEC-038 | Explicit conflict diagnostics when another editor owns an exclusive port. |
| DEC-039 | Developer-mode protocol monitor allowed; arbitrary raw transmission requires a separate danger toggle. |
| DEC-040 | Device and firmware packs update independently as signed declarative data, never downloaded executable code. |
| DEC-041 | Unsigned local packs allowed only in developer mode with a prominent warning. |
| DEC-042 | Vendor binaries, artwork, and copied layouts are never distributed. |
| DEC-043 | Current plus two previous major firmware versions should ship where possible; retain valid community profiles beyond that. |
| DEC-044 | Writes use acknowledgement or read-back where supported; partial completion is represented truthfully. |
| DEC-045 | Normal manual editing is live; AI, import, conversion, and large structural changes use staged preview. |
| DEC-046 | Persistent undo records prior confirmed state; it is not treated as an automatic user-visible preset backup. |
| DEC-047 | Preset changes create named session branches rather than blind cross-preset undo. |
| DEC-048 | UI distinguishes live edited, last-read, last-device-saved, and offline-library states. |
| DEC-049 | Offline container preserves original SysEx, normalized data, metadata, unknown bytes, and history. |
| DEC-050 | Local library includes folders, tags, favorites, search, versions, diff, duplicate detection, setlists, and compatibility filtering. |
| DEC-051 | Cloud storage is accessed through OS file pickers; RigWarden does not run a project cloud. |
| DEC-052 | Complete FC-6/FC-12 editor is in scope. |
| DEC-053 | Full user-cab management and DynaCab are in scope; dedicated multi-IR mixing follows first beta. |
| DEC-054 | AI supports BYOK OpenRouter plus a generic OpenAI-compatible provider interface; secrets use Keychain/Keystore. |
| DEC-055 | Multi-parameter AI changes require semantic preview; exact simple changes may use a validated fast path. |
| DEC-056 | AI privacy includes data preview, identifier stripping, provider allowlists, cost estimates/caps, and telemetry exclusion by default. |
| DEC-057 | Reference-audio tone matching follows the deterministic editor and text-driven AI features. |
| DEC-058 | Public beta contains no placeholder buttons or decorative mock screens counted as implemented. |
| DEC-059 | AM4 and FM3 must be hardware-verified end-to-end before beta; other devices may be explicitly experimental/community-confirmed. |
| DEC-060 | F-Droid flavor is pure open source, telemetry off, and has no network permission until a user enables profile updates or AI. |
| DEC-061 | Release progression: simulator builds → private hardware alpha → GitHub/TestFlight alpha → Play open test → public beta → production stores/F-Droid. |
| DEC-062 | Codex continues autonomously until a real blocker: missing hardware/captures, credentials/signing, legal uncertainty, or a decision that cannot safely be inferred. |
| DEC-063 | Default theme is Studio Carbon; Stage Amber, Console Ivory, and Electric Slate also ship. |
| DEC-064 | All decorative/control/icon assets are PNG. Knobs may be procedurally rendered. Live functional visualizations may be dynamically drawn. No production SVG assets. |
| DEC-065 | The project uses strict observed RED–GREEN–REFACTOR TDD for every production behavior. |
| DEC-066 | Terra/High is the intended parent orchestration model; Luna/Max is requested for bounded subagents, with runtime verification before large fan-out. |
| DEC-067 | The superseded working-name candidate is retired. Do not publish, reserve, or ship it as a public mark, store title, package ID, repository slug, domain, or handle. |
| DEC-068 | `RigWarden` is the provisional public pre-alpha GitHub identity. It is not legal clearance or approval for store/package/domain/handle/trademark use. |
| DEC-069 | The first typed Rust–Dart bridge uses stable Flutter Rust Bridge 2.12.0 with the Cargokit backend; Native Assets is reconsidered only after a stable release and clean mobile-build comparison. |
