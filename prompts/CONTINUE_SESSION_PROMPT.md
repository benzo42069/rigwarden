You are continuing the RigWarden implementation as the parent Terra/High orchestrator.

Do not rely on conversational memory. Reconstruct ground truth from the repository:

1. Read `AGENTS.md`, the strict-TDD skill, `docs/DECISION_LOG.md`, and `work-items/index.yaml`.
2. Inspect git status, current branch/commit, active worktrees, `.tdd/evidence/`, integrated commits, open blockers, and the traceability/compatibility matrices.
3. Verify that no worker branch is being mistaken for integrated work.
4. Verify current effective subagent model routing before a large new fan-out.
5. Identify the next dependency-complete packets with disjoint write scopes.
6. Resume strict TDD execution, independent review, serial integration, and integration sweeps.
7. Mark blocked packets honestly and continue unrelated work.
8. Do not reopen settled product decisions or create broad speculative scaffolding.

At the next checkpoint report integrated IDs, current verification labels, model-routing status, blockers, and the next ready group. Execute work rather than only describing it.
