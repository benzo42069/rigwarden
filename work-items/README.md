# Topology Work Items

## Purpose

A work item is an executable contract for one bounded behavior. Luna workers should not receive epics or vague TODOs.

## Waves

- `wave-00-research` — read-only evidence and decisions required before implementation.
- `wave-01-bootstrap` — minimum executable Rust/Flutter/evidence harnesses; no product behavior.
- `wave-02-core-vertical-slice` — first domain/simulator/UI/accessibility slice.
- `wave-03-am4-bootstrap` — fixture- and hardware-dependent AM4 path.
- `wave-04-fm3-bootstrap` — transport-, fixture-, and hardware-dependent FM3 path.

The later full product is in `docs/MASTER_BACKLOG_BLUEPRINT.md`. Terra must generate and review equally detailed packets before assigning later work.

## Status

Only packets with `status: READY` and fully integrated dependencies may run.

Research packets may be ready before the repository has code. Implementation packets often start `BLOCKED_DEPENDENCY`, `BLOCKED_FIXTURE`, or `BLOCKED_HARDWARE` and become ready only when the parent verifies the condition.

## Editing a packet

The parent/integration owner may amend a packet before execution when:

- actual toolchain/package path differs;
- research invalidates an assumption;
- dependency API changes;
- a path conflict appears;
- the test selector needs a mechanical correction.

Record the amendment in the packet/evidence. Do not weaken behavior or claims merely to make it easy.

## Execution

1. Parent confirms packet and dependencies.
2. Worker copies packet to `.tdd/evidence/<id>/work-item.yaml`.
3. Worker follows strict TDD.
4. Reviewer audits.
5. Integration owner lands patch.
6. Integration owner reruns.
7. Index/traceability updates.
8. Dependents may become ready.

## Research packets

Research packets do not manufacture RED/GREEN. They produce bounded, cited evidence and an independently reviewed conclusion. Any executable validator or generator created from the research requires its own TDD packet.

## Shared-file warning

Root manifests, lockfiles, global indexes, generated registries, and traceability are integration-owned unless the packet explicitly grants them.
