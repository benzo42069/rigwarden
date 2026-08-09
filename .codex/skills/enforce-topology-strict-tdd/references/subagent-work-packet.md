# Subagent Work Packet

## Goal

A packet removes ambiguity so a bounded worker can execute strict TDD without inventing product or architecture.

## Required fields

- `id`
- `kind`
- `epic`
- `title`
- `status`
- `priority`
- `requirement_ids`
- `depends_on`
- `parallel_group`
- `agent`
- `scope.read`
- `scope.write`
- `scope.forbidden`
- `behavior`
- `non_goals`
- `preconditions`
- `test`
- `minimum_green`
- `required_sweeps`
- `acceptance`
- `available_claims_after_completion`
- `unavailable_claims`
- `evidence_directory`

Research packets replace RED/GREEN with a bounded evidence question, approved sources, deliverables, and review.

## Packet sizing

Good:

- one parser message;
- one graph invariant;
- one semantics behavior;
- one transport lifecycle case;
- one migration;
- one AI validation rule.

Too broad:

- whole protocol;
- whole device;
- whole screen;
- “all accessibility”;
- “set up CI” without individual observable rules;
- multi-platform transport implementation.

A packet may include up to three tightly coupled cycles when splitting would create artificial shared-state conflict. Each cycle still needs separate RED/GREEN evidence.

## Path ownership

The packet should make concurrent safety obvious.

- Grant exact files where possible.
- Grant a narrow directory only when files do not exist and the behavior determines them.
- Mark root/shared files forbidden.
- State generated file ownership.
- Workers propose root dependency changes through handoff.

## Dependency truth

`depends_on` references integrated work, not a worker branch.

The parent checks:

- dependency status;
- commit;
- expected API;
- no pending integration changes.

## Amendments

Only the parent/integration owner may amend a packet before execution or after a worker reports it stale.

Record:

- old field;
- new field;
- reason;
- author;
- timestamp;
- whether test/claim changed.

An implementer does not silently amend its packet.

## Research packets

Research can run before executable TDD when it creates no production behavior.

It must still be bounded:

- exact question;
- source priority;
- prohibited sources/material;
- deliverable;
- uncertainty;
- decision impact;
- reviewer.

Research findings do not automatically become implementation truth. They feed reviewed packets/ADRs/fixtures.

## Handoff

Worker returns:

- status;
- exact RED/GREEN;
- changed files;
- evidence path;
- commit;
- claims;
- blockers;
- shared changes proposed;
- next packet suggestion.

No prose claim without repository evidence.
