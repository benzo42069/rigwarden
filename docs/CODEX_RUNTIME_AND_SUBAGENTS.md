# Codex Runtime and Subagent Operating Contract

## 1. Intended setup

- Parent/orchestrator: `gpt-5.6-terra`
- Parent reasoning: `high`
- Bounded workers: `gpt-5.6-luna`
- Worker reasoning: `max`
- Project agent files: `.codex/agents/*.toml`
- Project defaults: `.codex/config.toml`

The parent is responsible for decomposition, dependency order, conflict prevention, integration, and truthful status. Luna workers receive narrow packets with an objective definition of done.

## 2. Verify effective routing before fan-out

Do not trust configuration text or a child’s self-description alone.

Before launching a large wave:

1. Read the current Codex model/subagent documentation available to the environment.
2. Inspect the project agent files.
3. Spawn one read-only probe using the intended Luna custom agent.
4. Inspect runtime-visible thread metadata, logs, or UI details for effective model and reasoning effort.
5. Record:
   - requested model/effort;
   - effective model/effort if visible;
   - Codex version/build;
   - multi-agent mode;
   - sandbox;
   - thread ID;
   - evidence location.
6. If the runtime cannot verify the model:
   - label routing `UNVERIFIED`;
   - warn before expensive fan-out;
   - use a small trial wave;
   - do not claim Luna/Max was proven.
7. If the child inherits Terra or another expensive model unexpectedly:
   - stop the large wave;
   - preserve completed useful work;
   - report the mismatch;
   - continue only with deliberate user-approved or budget-safe routing.

## 3. Parallelism policy

Use high parallelism for:

- read-only research;
- codebase mapping;
- independent fixture review;
- independent work packets with disjoint write scopes;
- test execution;
- independent reviews;
- documentation validation;
- compatibility matrix analysis.

Use low or serialized parallelism for:

- root manifests;
- lockfiles;
- generated bindings;
- shared registries;
- global traceability matrix;
- release manifests;
- database migrations;
- architectural refactors;
- integration.

The parent waits for every agent in a declared parallel group before advancing dependent work.

## 4. Agent roles

### topology_explorer

Read-only. Maps code, requirements, dependencies, and existing behavior. Does not implement.

### topology_fixture_researcher

Read-only. Audits documentation, source licenses, captures, provenance, and independent expected values. Never invents bytes.

### topology_implementer

Workspace-write. Executes one ready packet through strict observed TDD. Does not self-approve.

### topology_reviewer

Read-only. Audits behavior, test integrity, scope, evidence, and verification labels.

### topology_accessibility_reviewer

Read-only. Audits semantic/nonvisual equivalence and test coverage.

### topology_security_reviewer

Read-only. Audits untrusted input, writes, secrets, packs, network, and AI boundaries.

### topology_test_runner

Workspace-write only because build tools may emit artifacts. Must not edit source. Runs declared sweeps and records exact output.

### topology_release_verifier

Workspace-write for packaging output only. Validates clean-clone, signing inputs, manifests, store flavors, and reproducibility. Does not implement features.

The parent or a dedicated integration thread owns shared files and lands patches.

## 5. Worktree policy

Prefer isolated worktrees/branches for write-capable agents.

Each packet defines:

- read scope;
- exclusive write scope;
- forbidden paths;
- dependency commit;
- evidence path.

Do not assign overlapping write scopes concurrently.

## 6. Context policy

The parent keeps:

- decisions;
- dependency graph;
- integrated status;
- blockers;
- user-facing updates.

Workers keep:

- command output;
- local traces;
- detailed test logs;
- implementation notes.

Workers return concise evidence-backed summaries plus the evidence path and commit/patch reference.

## 7. No vague delegation

Invalid:

> Implement the routing editor.

Valid:

> Execute `TOP-GRAPH-004`: write the focused cycle that rejects a prohibited cycle, modify only the declared files, capture RED/GREEN/sweeps, and return the evidence directory and commit.

## 8. Parent completion report

At each checkpoint, report:

- integrated work-item IDs;
- verification labels achieved;
- current parallel group;
- blockers and why;
- tests/sweeps;
- unexpected decisions/ADRs;
- next ready packets;
- whether child model routing was verified.
