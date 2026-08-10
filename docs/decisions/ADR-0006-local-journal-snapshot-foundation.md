# ADR-0006: Use a versioned local journal snapshot for the deterministic core slice

- Status: Accepted for the pre-alpha deterministic core only
- Date: 2026-08-10
- Owner: parent orchestrator
- Requirements: `UNDO-002`, `UNDO-003`, `QA-001`

## Context

The core journal records confirmed semantic edits only in memory. That does not
meet the milestone's persistent-record checkpoint or the product requirement
to preserve journal state across a restart. The broader architecture still
recommends SQLite behind a Rust storage interface, but no storage-selection
ADR, crate, or migration layer exists yet. Pretending an in-memory journal is
durable would be bullshit.

## Decision

Introduce a small, versioned, deterministic journal-snapshot codec followed by
a local file adapter that writes and reloads one snapshot. The snapshot is
strictly internal application state: it has no vendor protocol bytes, preset
backup representation, network behavior, or Dart ownership.

The codec must preserve only confirmed branch history and its active branch.
It must reject a save while a mutation or undo restoration is in flight rather
than silently losing uncertain work. Parsing uses explicit magic/schema,
bounded length-prefixed fields, checked arithmetic, exact `f64` bit storage,
and structured errors for malformed input.

The file adapter must use a temporary sibling followed by rename where the
host filesystem supports it, and it must surface I/O errors. It is tested by
writing, dropping the source journal, and loading a fresh journal from the
declared path.

## Consequences and boundaries

- This establishes a real local persistence boundary for the small deterministic
  slice; it does **not** select the final library storage architecture.
- It does not prove fsync durability through power loss, cross-process locking,
  encryption, migration, compaction, corruption repair, reconnect recovery,
  redo, or a full crash matrix. Those remain follow-up storage work.
- A codec round-trip is not a protocol-byte claim. The file/reload test earns
  only the narrow unit evidence named in its packet.
- The final storage backend remains an ADR-backed decision before beta, with
  SQLite or a documented alternative evaluated against migration, concurrency,
  mobile-build, and recovery requirements.

## Alternatives considered

### Keep the journal in memory until SQLite is selected

Rejected. It leaves the milestone's persistent undo record untrue and makes
the E2E composition path overstate its recovery behavior.

### Select and integrate a full SQLite library now

Deferred. It is a broader storage architecture decision than the immediate
one-entry persistent vertical slice and needs its own focused evaluation.

### Serialize transient pending operations

Rejected. A pending mutation or restoration has not been confirmed and must
not become a completed undo record during persistence.

## Verification plan

`TOP-UNDO-004` first proves the versioned byte snapshot and malformed-input
rejection. `TOP-UNDO-005` then proves a local write/drop/reload path. Both must
remain below platform, protocol, and hardware verification claims.
