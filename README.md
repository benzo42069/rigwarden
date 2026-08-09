# RigWarden

*An open editor for modern modelers.*

RigWarden is a free, local-first, open-source project building a capable editor
for modern guitar modelers, beginning with Fractal Audio hardware. The goal is
complete mobile editing on iPhone, iPad, and Android—not a watered-down preset
picker—with accessible nonvisual workflows and honest compatibility labels.

There is no account requirement, mandatory cloud service, or project-owned AI
backend. Your rigs and files should remain yours unless you explicitly export
or share them.

## Current state

RigWarden is **pre-alpha**. It is a public foundation for contributors, not a
working editor release.

What is real today:

- A minimal Rust workspace and executable test harness.
- A minimal Flutter iOS/Android application harness with a passing widget test.
- iOS/iPadOS 16.0 and Android API 29 minimum / API 36 target configuration.
- Research and decision records covering Fractal model families, mobile
  transport constraints, accessibility, source provenance, privacy, release
  constraints, and contribution safety.
- An initial, dependency-aware implementation backlog and strict observed-TDD
  evidence process.

What is not here yet:

- A usable editor UI or preset browser.
- Rust/Flutter FFI, native MIDI/USB/BLE transport, device profiles, or protocol
  implementation.
- Emulator, physical-device, or modeler hardware verification.
- Any claim of supported Fractal firmware, hardware compatibility, or store
  readiness.

## Visual direction — concept references only

These are asset and UI references for future work, **not screenshots of
working software**. The controls, workflows, and modeler support depicted here
are not implemented or verified.

| Studio Carbon | Stage Amber |
|---|---|
| ![RigWarden Studio Carbon concept](docs/concepts/rigwarden-studio-carbon-concept.png) | ![RigWarden Stage Amber concept](docs/concepts/rigwarden-stage-amber-concept.png) |

| Console Ivory | Electric Slate |
|---|---|
| ![RigWarden Console Ivory concept](docs/concepts/rigwarden-console-ivory-concept.png) | ![RigWarden Electric Slate concept](docs/concepts/rigwarden-electric-slate-concept.png) |

## Run the current harnesses

The repository currently proves only that its Rust and Flutter bootstrap
harnesses run.

```bash
cargo test --workspace

cd apps/mobile_flutter
flutter analyze
flutter test
```

The Flutter shell is not an Android or iOS build claim. Android build/device
verification still requires the Android SDK, JDK, and Gradle; physical mobile
and modeler testing come later through explicit packets.

## Contributing

Good contributions are small, evidence-backed, and honest about their proof
layer. Before opening work, read:

1. [CONTRIBUTING.md](CONTRIBUTING.md)
2. [docs/00-READING-ORDER.md](docs/00-READING-ORDER.md)
3. [docs/PROJECT_BRIEF.md](docs/PROJECT_BRIEF.md)
4. [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)
5. [docs/ACCESSIBILITY.md](docs/ACCESSIBILITY.md)
6. [docs/PROTOCOL_RESEARCH_AND_PROVENANCE.md](docs/PROTOCOL_RESEARCH_AND_PROVENANCE.md)

The short version:

- Start with a focused failing test, then make it green.
- Do not guess protocol behavior or claim compatibility without evidence.
- A simulator, mock, screenshot, or semantics-tree pass is not hardware,
  VoiceOver, or TalkBack verification.
- Do not copy vendor/competitor artwork, layouts, binaries, captures, or other
  material with unclear rights.
- Keep normal usage local-first and preserve clear read-only/write-safe
  boundaries around physical hardware.

See [work-items/README.md](work-items/README.md) for the executable work-packet
workflow and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for community standards.

## Project principles

- **Complete editing, not phone-lite control.** Phones and tablets ultimately
  receive the complete editor through adaptive workflows.
- **Accessible by design.** A visual routing canvas can never be the only way
  to inspect or edit a preset.
- **Local first.** No account, mandatory cloud, or project AI proxy.
- **Truthful compatibility.** Support is labeled hardware-verified, community
  confirmed, capture-verified, simulator-verified, experimental, read-only, or
  unsupported—never hand-waved.
- **Community-extensible.** Device and firmware support must be declarative,
  provenance-aware, and independently testable.

## Project status and naming

RigWarden is the provisional pre-alpha GitHub identity. It is not legal
clearance and is not approved for a store listing, package ID, domain, handle,
or trademark claim. See
[ADR-0003](docs/decisions/ADR-0003-rigwarden-pre-alpha-identity.md).

The planning and integrity materials remain available in `docs/`,
`work-items/`, `schemas/`, and `templates/`. They exist to keep the project
safe and reproducible; they are not a substitute for tested product behavior.
