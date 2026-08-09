# TOP-RSCH-002 independent review

Reviewer: `/root/review_sources` (`topology_reviewer`, OpenAI `gpt-5.6-luna` / `max`)  
Reviewed: 2026-08-08  
Decision: `REVIEW_APPROVED`

The report is within its research-only packet: no production behavior, fixtures,
binaries, captures, shared-file edits, hardware claim, protocol-compatibility
claim, or direct reuse approval. Four candidates are pinned with addressable
license/NOTICE evidence: Axis `6b87bd2472fd88854421fda0dd1d2d7a02d2dd19`
(MIT), ForgeFX `c22862a5b2f2078f3cb92a2735e51f94c39a0062` (MIT; stale
NOTICE unresolved), mcp-midi-control/fractal-midi
`59047175cfc4f23e092931b54a7c54f2bffde3ea` (Apache-2.0), and forgefx-midi
`553d24b7409302908d5c7e46a71b45e07dffdc05` (Apache-2.0). External pinned
commit/license spot checks resolved.

The classifications remain conservative: independent implementation or
reference-only by default; UI/layout/art/font/icon/manual/catalog/capture/byte
reuse is rejected or blocked. The ForgeFX NOTICE mismatch and missing exact
fractal-syx-codec pin remain provenance questions, not legal conclusions.
Environment evidence records timestamp/timezone, OS/CPU, non-Git-worktree
limits, toolchains, and source access. No RED/GREEN applies to this research
packet. This approval earns research/provenance guidance only; it does not
approve reuse, protocol compatibility, platform/hardware behavior, or legal
clearance.
