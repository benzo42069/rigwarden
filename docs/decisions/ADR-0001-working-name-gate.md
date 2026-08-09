# ADR-0001: Keep `Topology` internal pending public-name replacement

- Status: Accepted for the starter-kit/bootstrap phase only
- Date: 2026-08-08
- Owners: integration owner; counsel owns public-name clearance
- Requirements: `QA-001`, `SEC-007`
- Work items: `TOP-RSCH-005`, `TOP-BOOT-001`

## Context

The preliminary collision screen found active, materially adjacent uses of the
exact term `Topology` in music/audio and software contexts. That screen is not
legal clearance, and no public identifier has been reserved.

## Decision

Use `Topology` only as an internal working identifier while establishing the
repository baseline. Do not publish, reserve, submit, purchase, or claim any
public mark, package ID, repository slug, domain, social handle, or store title
using the exact name. A replacement public identity (or a counsel-reviewed
modified mark) is a separate decision and clearance gate.

## Alternatives considered

- Publish the exact name now: rejected; it would turn a known collision risk
  into external state without clearance.
- Stop all local bootstrap work: rejected; original repository/legal baseline
  work can proceed without any public use or reservation.

## Consequences

### Positive

- Removes the name-screen block from private bootstrap work.
- Preserves the evidence-backed public-identity gate.

### Negative

- Public-facing metadata and release automation remain blocked pending a
  replacement-name decision and counsel review.

### Risks

- Local files containing the working identifier must not be treated as public
  clearance or copied into a release/store identity without review.

## Verification

This is a governance constraint, not executable behavior. Verify the absence of
public reservation/publication actions and retain the naming gate in future
release packets.

## Migration or rollback

Replace the working name only through a reviewed naming decision; no external
state exists to roll back.

## References

- `docs/research/topology-name-screen.md`
- `.tdd/evidence/TOP-RSCH-005/review.md`
- `docs/DECISION_LOG.md` (`DEC-067`)
