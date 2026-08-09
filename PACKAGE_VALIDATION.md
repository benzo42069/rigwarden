# Package Validation Report

**Package:** RigWarden Codex Starter Kit v1.0  
**Prepared:** 2026-08-08  
**Validation status:** PASS

## Final package inventory

- The original starter-kit snapshot contained 121 files. The current
  RigWarden pre-alpha repository has evolved beyond that snapshot; its manifest
  covers every tracked repository file except the manifest itself.
- 62 executable work-item packets across five initial waves.
- Work-item status distribution:
  - 8 `READY`
  - 38 `BLOCKED_DEPENDENCY`
  - 8 `BLOCKED_FIXTURE`
  - 8 `BLOCKED_HARDWARE`
- Work-item kind distribution:
  - 9 research
  - 4 bootstrap
  - 40 implementation
  - 9 hardware
- 9 TOML files: project config plus eight custom agent profiles.
- 4 JSON Schemas.
- 67 YAML files: 62 packets, one packet template, one work-item index, and supporting templates.

## Checks performed

The final unpacked package passed `python tools/validate_starter_kit.py` with zero errors and zero warnings. The validator checked:

- JSON syntax and Draft 2020-12 schema validity;
- TOML syntax;
- YAML syntax;
- every work packet against `schemas/work-item.schema.json`;
- unique work-item IDs;
- recognized requirement namespaces;
- exact evidence-directory naming;
- nonempty RED and GREEN commands for executable packets;
- absence of unresolved command placeholders;
- existence of all declared dependencies;
- an acyclic dependency graph;
- one-to-one synchronization between `work-items/index.yaml` and packet files;
- index title, status, priority, dependency, and path consistency.

## Additional integrity checks

- `START_HERE_PROMPT.md` is byte-identical to `prompts/MASTER_ORCHESTRATOR_PROMPT.md`.
- Every internal reference required by the strict-TDD skill exists.
- `MANIFEST.sha256` contains a SHA-256 checksum for every tracked repository
  file except the manifest itself.
- Verify the current manifest with `shasum -a 256 -c MANIFEST.sha256`.
- A release ZIP still requires its own archive-integrity pass and clean
  extraction before publication.
- The extracted copy passed the starter-kit validator again.

## What this report does not claim

This validates the planning/orchestration package, not the future RigWarden product. It does not establish protocol correctness, mobile-platform behavior, physical-device support, accessibility on real devices, store acceptance, or any other product verification label. Those claims require the test and evidence layers defined by the included strict-TDD contract.
