# Evidence Record

## Directory

```text
.tdd/evidence/<WORK-ITEM-ID>/
```

Evidence belongs to the work item and must remain reproducible from the recorded commit.

## Required files

### `work-item.yaml`

Copy of the executed packet, including any approved amendment and amendment author/date.

### `environment.txt`

Include:

- UTC/local timestamp;
- OS and version;
- CPU architecture;
- repository path;
- branch/worktree;
- starting commit;
- dirty-state summary;
- Rust toolchain;
- Flutter/Dart;
- Xcode/iOS SDK;
- Android SDK/JDK/Gradle;
- relevant native compiler;
- Codex client/build;
- requested/effective subagent model metadata when relevant;
- emulator/device/hardware details;
- environment variables names used, with secrets redacted.

### `red-command.txt`

Exact focused command. Include shell, working directory, and necessary safe environment configuration.

### `red.log`

Unedited stdout/stderr. Redaction may replace secrets/personal identifiers with stable tokens such as `<REDACTED_SERIAL_1>` and must be documented.

### `red-exit-status.txt`

Numeric status and whether the worker accepted it as intended RED, with one-sentence reason.

### `green-command.txt`, `green.log`, `green-exit-status.txt`

Same requirements for focused GREEN.

### `sweep-commands.txt`

Every adjacent/matrix/format/lint/audit command in execution order.

### `sweep.log`

Command-separated output. Do not omit failures.

### `sweep-exit-statuses.txt`

One status per command. Do not report only the last shell status.

### `files-changed.txt`

`git diff --name-status` or equivalent plus explanation of generated/build artifacts.

### `review.md`

Reviewer identity/agent, commit, findings, decision, and verification-label audit.

### `handoff.md`

Summary:

- behavior delivered;
- files changed;
- design decisions;
- pitfalls;
- fixture/source references;
- claims earned;
- claims unavailable;
- shared-file changes proposed;
- next packet;
- blockers.

## Conditional files

- `fixture-provenance.yaml`
- `simulator-transcript.log`
- `platform-matrix.yaml`
- `hardware-matrix.yaml`
- `accessibility-results.md`
- `benchmark-results.json`
- `security-review.md`
- `screenshots/`
- `sanitized-captures/`
- `release-artifacts.sha256`

## Integrity

- Do not rewrite logs to look cleaner.
- Do not fabricate timestamps.
- Do not remove a failure from a composite sweep.
- Do not store secrets.
- Preserve checksums for fixtures/artifacts.
- Evidence may be compressed for release archives, but canonical repository evidence remains addressable.
- CI evidence should link to immutable run/artifact IDs when available.

## Compactness

Evidence should be complete, not noisy. Large irrelevant logs may be stored as artifacts with a concise indexed summary, provided exact raw output remains retrievable.
