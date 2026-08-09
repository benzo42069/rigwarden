# Open-source source and license audit

**Packet:** `TOP-RSCH-002`  
**Access date:** 2026-08-08 UTC  
**Status:** `REVIEW_APPROVED` (bounded research only)  
**Scope:** research and provenance only; this is not legal advice or protocol compatibility proof.

## Decision summary

RigWarden should implement its Rust/Flutter architecture independently. Axis, ForgeFX, `mcp-midi-control`, and `forgefx-midi` are valuable research sources, but no source layout, asset, vendor-derived catalog, manual prose, capture, or byte mapping is approved for direct reuse by this audit.

| Candidate | Exact revision / license evidence | Classification | Required boundary |
| --- | --- | --- | --- |
| [Axis](https://github.com/sKuhLight/Axis/tree/6b87bd2472fd88854421fda0dd1d2d7a02d2dd19) | `6b87bd2472fd88854421fda0dd1d2d7a02d2dd19`; [MIT LICENSE](https://github.com/sKuhLight/Axis/blob/6b87bd2472fd88854421fda0dd1d2d7a02d2dd19/LICENSE) | Reference-only. A narrowly audited helper could be ported with MIT notice, but independent Rust is preferred. | Reject UI/layout/art/font/icon copying; Axis uses Svelte/Electron/localhost ForgeFX architecture that conflicts with RigWarden’s mobile contract. |
| [ForgeFX](https://github.com/sKuhLight/ForgeFX/tree/c22862a5b2f2078f3cb92a2735e51f94c39a0062) | `c22862a5b2f2078f3cb92a2735e51f94c39a0062`; [MIT LICENSE](https://github.com/sKuhLight/ForgeFX/blob/c22862a5b2f2078f3cb92a2735e51f94c39a0062/LICENSE) | Reference-only / unresolved provenance. | Do not port Node server, protocol, layout, or catalog code until stale NOTICE and transitive provenance are reconciled. |
| [`mcp-midi-control` / `fractal-midi`](https://github.com/TheAndrewStaker/mcp-midi-control/tree/59047175cfc4f23e092931b54a7c54f2bffde3ea) | `59047175cfc4f23e092931b54a7c54f2bffde3ea`; root/package [Apache-2.0 license](https://github.com/TheAndrewStaker/mcp-midi-control/blob/59047175cfc4f23e092931b54a7c54f2bffde3ea/LICENSE) and [NOTICE](https://github.com/TheAndrewStaker/mcp-midi-control/blob/59047175cfc4f23e092931b54a7c54f2bffde3ea/NOTICE) | Independent reimplementation preferred; a future file-level port requires Apache notice, modified-file marking, exact provenance, and review. | Protocol facts remain reference-only until independently supported by a primary source or lawful fixture. |
| [`forgefx-midi`](https://github.com/sKuhLight/forgefx-midi/tree/553d24b7409302908d5c7e46a71b45e07dffdc05) | `553d24b7409302908d5c7e46a71b45e07dffdc05`; Apache-2.0 [LICENSE](https://github.com/sKuhLight/forgefx-midi/blob/553d24b7409302908d5c7e46a71b45e07dffdc05/LICENSE) and [NOTICE](https://github.com/sKuhLight/forgefx-midi/blob/553d24b7409302908d5c7e46a71b45e07dffdc05/NOTICE) | Reference-only. | Reject generated editor-derived cab/layout data and Wiki-derived lineage/prose without independent rights/provenance. |

## Findings

### Axis

- The [README](https://github.com/sKuhLight/Axis/blob/6b87bd2472fd88854421fda0dd1d2d7a02d2dd19/README.md), typed ForgeFX client, [editor state](https://github.com/sKuhLight/Axis/blob/6b87bd2472fd88854421fda0dd1d2d7a02d2dd19/src/lib/editor.svelte.ts), and [Electron main process](https://github.com/sKuhLight/Axis/blob/6b87bd2472fd88854421fda0dd1d2d7a02d2dd19/electron/main.cjs) show a Svelte/Electron UI coupled to a local Node service.
- Its ControlSurface, SignalGrid, BlockEditor, CabPicker, EQGraph, designs, PNG/SVG/icon assets, tokens, and visual layouts must not enter RigWarden. MIT permissiveness does not override RigWarden’s independent-identity and no-copied-layout rule.
- Upstream hardware statements are upstream claims only; they grant no RigWarden verification label.

### ForgeFX

- The current server owns MIDI/serial and Gen-3 paths under [`server/src`](https://github.com/sKuhLight/ForgeFX/tree/c22862a5b2f2078f3cb92a2735e51f94c39a0062/server/src); it is incompatible with the no-Node/local-HTTP mobile architecture.
- Its current [NOTICE](https://github.com/sKuhLight/ForgeFX/blob/c22862a5b2f2078f3cb92a2735e51f94c39a0062/NOTICE) names an old C# codec path removed by [commit `0478f68`](https://github.com/sKuhLight/ForgeFX/commit/0478f68). Device data later moved by [commit `a0b7c14`](https://github.com/sKuhLight/ForgeFX/commit/a0b7c14). Treat this as an unresolved attribution/provenance mismatch, not an infringement conclusion.
- `docs/LAYOUTS.md` describes device-authentic/editor-derived layouts. Do not copy layouts, generated controls, or data. Protocol assertions in codec/write documentation are research input only.

### mcp-midi-control / fractal-midi

- The package is TypeScript and includes codec/catalog coverage, but its [README](https://github.com/TheAndrewStaker/mcp-midi-control/blob/59047175cfc4f23e092931b54a7c54f2bffde3ea/packages/fractal-midi/README.md) distinguishes varying upstream evidence levels. Never promote those labels to RigWarden.
- The [provenance policy](https://github.com/TheAndrewStaker/mcp-midi-control/blob/59047175cfc4f23e092931b54a7c54f2bffde3ea/packages/fractal-midi/AGENTS.md), [capture inventory](https://github.com/TheAndrewStaker/mcp-midi-control/blob/59047175cfc4f23e092931b54a7c54f2bffde3ea/packages/fractal-midi/docs/research/captured-artifacts.md), and [manual policy](https://github.com/TheAndrewStaker/mcp-midi-control/blob/59047175cfc4f23e092931b54a7c54f2bffde3ea/packages/fractal-midi/docs/manuals/README.md) confirm that raw artifacts/captures are not general reusable fixtures.
- Do not reuse manual text extracts, Wiki quotes/lineage JSON, editor-derived labels/layouts/assets, raw decompile dumps, private captures, or `fractal-syx-codec` claims without an exact upstream pin and an independently lawful RigWarden source.
- GitHub API reported `NOASSERTION` while repository license files report Apache-2.0; retain that metadata contradiction in any future reuse review.

## Obligations and blockers

1. Any future MIT/Apache code import needs a reviewed file-by-file provenance record, copyright/license retention, required NOTICE text, and modified-file declaration.
2. No generated device catalog/layout or protocol fixture may be copied wholesale. Re-author factual labels from official sources and independently validate all protocol semantics.
3. Create an ADR or obtain legal advice before relying on: ForgeFX’s stale NOTICE, the missing exact `fractal-syx-codec` pin, editor-binary extraction guidance, capture redistribution, or manual/Wiki terms.
4. `TOP-RSCH-003` and `TOP-RSCH-008` must supply the primary-source and fixture gates before protocol implementation proceeds.

## Claims unavailable

This report grants no code reuse, byte compatibility, simulator, hardware, platform, or trademark-clearance claim.
