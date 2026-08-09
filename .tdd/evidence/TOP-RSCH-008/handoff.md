# TOP-RSCH-008 handoff

Status: `REVIEW_APPROVED` (bounded research only)

- Added `docs/research/capture-and-fixture-plan.md` with a read-only-first capture workflow, explicit prohibition on unknown-write capture/derivation, sanitization tokens and byte-preserving rules, contributor redistribution declaration, raw/sanitized storage layers, independent expected-value derivation, fixture path/naming/versioning, and AM4/FM3 hardware/adapter rows.
- Audited current Fractal AM4/FM3 manuals/download pages, the published Axe-Fx III third-party MIDI PDF, Apple Core MIDI, Android USB host/MIDI APIs, GitHub licensing guidance, SPDX guidance, and exact-commit open-source method/license candidates. Sources and access date are recorded in the report and `research-sources.md`.
- No production code, shared manifests, device packs, captures, fixture bytes, vendor binaries, artwork, credentials, or personal data were created.
- Current environment has no physical AM4/FM3 and no lawful Topology fixture corpus, so identity/read fixture packets and hardware rows remain blocked until contributor evidence exists.
- Starter-kit validator sweep is blocked because PyYAML and jsonschema are missing; this is recorded per-command and is not presented as validation success.
- Independent review is recorded in `review.md`; retain all source, fixture, and hardware blockers before any implementation.
- Suggested next work: `TOP-BOOT-006`, `TOP-BOOT-007`, `TOP-BOOT-009`, then `TOP-AM4-FIX-001`/`002` and `TOP-FM3-FIX-001` after lawful hardware/read captures arrive.
