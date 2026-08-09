Review the completed RigWarden work-item wave as an independent gate.

Read the strict-TDD skill and every packet/evidence directory in the wave. Spawn separate read-only reviewers for:

1. behavioral correctness;
2. test integrity and RED/GREEN evidence;
3. protocol/fixture provenance;
4. accessibility;
5. security/privacy;
6. architecture/scope and shared-file conflicts.

Wait for all reviewers. Consolidate findings by work-item ID and severity.

For each packet decide exactly one:

- `REVIEW_APPROVED`
- `REVIEW_FAILED`
- `BLOCKED_EVIDENCE`

Do not fix the implementation in the review pass. Cite files, tests, commands, evidence gaps, and reproduction steps. Reject false verification labels, circular fixtures, skipped required tests, tests-after behavior, inaccessible visual-only workflows, and out-of-scope edits.

Only approved patches may proceed to serial integration and integration-worktree reruns.
