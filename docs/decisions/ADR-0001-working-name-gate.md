# ADR-0001: Retire the superseded working-name candidate

- Status: Superseded for public identity by ADR-0003
- Date: 2026-08-08
- Owners: integration owner; counsel owns public-name clearance
- Requirements: `QA-001`, `SEC-007`
- Work items: `TOP-RSCH-005`, `TOP-BOOT-001`

## Context

The preliminary collision screen found active, materially adjacent uses of a
superseded candidate in music/audio and software contexts. The screen remains
historical evidence only; it is not legal clearance.

## Decision

Do not revive, publish, reserve, submit, purchase, or claim the superseded
candidate as a public mark, package ID, repository slug, domain, social handle,
or store title. ADR-0003 selects RigWarden only for the bounded pre-alpha
community repository and retains its separate clearance gate.

## Alternatives considered

- Publish the superseded candidate: rejected; it would turn a known collision
  risk into external state without clearance.
- Stop all local bootstrap work: rejected; original repository/legal baseline
  work can proceed without any public use or reservation.

## Consequences

### Positive

- Preserves the rejected-candidate evidence without presenting it as current
  RigWarden research.
- Keeps the public-identity gate in ADR-0003 explicit.

### Negative

- Store, package, domain, handle, and trademark actions remain blocked pending
  current clearance and counsel review.

### Risks

- Historical evidence must not be represented as a RigWarden clearance result.

## Verification

This is a governance constraint, not executable behavior. Verify the absence of
public reservation/publication actions and retain the naming gate in future
release packets.

## Migration or rollback

Do not reuse the superseded candidate. Update current clearance evidence before
any public identity expansion beyond the pre-alpha repository.

## References

- `docs/research/topology-name-screen.md`
- `.tdd/evidence/TOP-RSCH-005/review.md`
- `docs/DECISION_LOG.md` (`DEC-067`)
