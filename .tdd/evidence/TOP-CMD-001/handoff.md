# TOP-CMD-001 handoff

Status: `REVIEW_APPROVED_INTEGRATION_PENDING_STATUS_COMMIT`

## Delivered

One in-memory command-engine validator reads the exact profile-owned numeric
metadata added by TOP-REG-003. A writable profile accepts `amp-1/gain` stored
value `45` with precision `1` (4.5) and returns a typed semantic mutation. A
stored value of `101` is rejected with `OutOfRange`; no protocol bytes or
transport operations exist in this crate.

## TDD and verification

- Canonical RED: intended missing validator imports, exit 101.
- Canonical GREEN: focused valid/out-of-range behavior, exit 0.
- Final required package, registry, formatter, Clippy, and scoped whitespace
  sweeps pass. The initial formatting-only failure is preserved separately.

## Claim boundary

Candidate may claim `UNIT_VERIFIED` only after independent review and parent
integration rerun. It does not claim byte fixtures, simulator, platform,
hardware, AI, or protocol behavior.

## Next packet

TOP-CMD-002 can add the unknown-firmware write rejection after this candidate
is independently reviewed and integrated.
