# TOP-BOOT-008 handoff

Status: REVIEW_APPROVED; integration rerun and public status update pending.

Delivered behavior: `validate_manifest` rejects each declared production static asset whose extension is `.svg`, case-insensitively, with stable `production_svg_forbidden` diagnostics. PNG declarations are accepted. Test-only SVG fixtures, dynamically rendered functional graphics, and declared procedural knobs are accepted because they are not production static assets.

Design boundaries: this validates only explicitly declared entries. It does not scan repository files or caches, create art, render knobs, or establish visual completeness.

Files changed: `crates/topology_devtools/src/lib.rs`, `src/assets.rs`, `tests/asset_policy.rs`, and this evidence directory.

Evidence: the first focused RED exits 101 solely for the missing asset-manifest API; the second focused RED exits 101 solely for the packet-required procedural-knob constructor; `final-green.*` and `final-sweep.*` were recaptured against the final candidate and all exit 0. Earlier green/sweep records are retained rather than rewritten.

Claims after review/integration: `UNIT_VERIFIED` only. Unavailable: `VISUAL_ASSET_COMPLETE`, `PLATFORM_DEVICE_VERIFIED`, and any hardware verification.

Shared-file proposals: none. Next independent bootstrap packet: TOP-BOOT-009 once its status is made READY from its already integrated dependencies.
