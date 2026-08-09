# Reading Order

The parent orchestrator must read these files before creating production behavior:

1. `README.md`
2. `AGENTS.md`
3. `docs/PROJECT_BRIEF.md`
4. `docs/DECISION_LOG.md`
5. `docs/PRODUCT_REQUIREMENTS.md`
6. `docs/ARCHITECTURE.md`
7. `docs/TRANSPORT_AND_COMPATIBILITY.md`
8. `docs/PROTOCOL_RESEARCH_AND_PROVENANCE.md`
9. `docs/ACCESSIBILITY.md`
10. `docs/AI_PIPELINE.md`
11. `docs/THEME_AND_ASSET_SYSTEM.md`
12. `docs/THREAT_MODEL.md`
13. `docs/GOVERNANCE_LICENSE_CONDUCT.md`
14. `docs/RELEASE_PLAN_AND_DEFINITION_OF_DONE.md`
15. `docs/MASTER_BACKLOG_BLUEPRINT.md`
16. `docs/CODEX_RUNTIME_AND_SUBAGENTS.md`
17. `.codex/skills/enforce-topology-strict-tdd/SKILL.md`
18. Every reference linked by that skill.
19. `work-items/README.md`
20. `work-items/index.yaml`

Before using an individual work packet, read the packet in full and verify that its dependencies are integrated, its paths still match the repository, and its fixtures or platform requirements actually exist.

## Authority order

When instructions conflict, use this order:

1. Current explicit user instruction.
2. Safety, legal, and platform constraints.
3. `AGENTS.md`.
4. Strict-TDD skill and references.
5. Product and architecture contracts.
6. Approved ADRs.
7. Work packet.
8. Existing implementation details.

Do not quietly reinterpret a product decision because a library makes another path easier. Record an ADR or blocker instead.
