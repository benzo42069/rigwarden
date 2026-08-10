# TOP-SIM-002 handoff

status: `INTEGRATED`
work_item: TOP-SIM-002
requirements: TRANSPORT-004, UNDO-007, SEC-001

## Behavior delivered

`ConnectionSession` assigns every request a monotonically increasing connection
generation. After reconnect, a reused request ID belongs to the new generation;
a response carrying the previous generation is returned as `IgnoredStale` and
does not remove pending state or create a confirmation. A matching current
generation response is accepted and stored as confirmed. Current-generation
responses without a pending request are rejected as `RejectedUnmatched`.

No retry, physical reconnect, protocol-byte interpretation, UI message, or
hardware behavior was added.

## TDD evidence

- Invalid first harness attempt: `red-invalid.log` / `red-invalid-exit-status.txt` preserve a command-engine test that imported `topology_simulator` without a packet-permitted Cargo.toml dev-dependency; it is explicitly rejected as harness evidence.
- Canonical RED: `cargo test -p topology-command-engine stale_response_cannot_confirm_new_connection_request -- --exact --nocapture` exited 101 because the intended `ConnectionSession`, `IncomingResponse`, and `ResponseDisposition` API was absent. See `red.log` and `red-exit-status.txt`.
- GREEN: the same focused command exited 0; see `green.log` and `green-exit-status.txt`.
- Required final fail-fast sweep: command-engine tests 0, simulator tests 0, formatter check 0, and command-engine Clippy with `-D warnings` 0. See `sweep.log`, `sweep-exit-statuses.txt`, and `sweep-commands.txt`.
- An initial wrapper's incomplete sweep transcript is preserved as `sweep-initial.log` and `sweep-initial-exit-statuses.txt`; the canonical rerun is complete and authoritative for this candidate.

## Scope and design

- Session correlation is keyed by `(ConnectionGeneration, request_id)`.
- Reconnect clears pending work from the disconnected lifetime and increments
  the generation; confirmed records remain keyed by their original generation.
- Generation mismatch is checked before pending/confirmed mutation, so stale
  responses cannot satisfy a reused request ID.
- Payloads remain opaque bytes; no vendor protocol or fixture claim is implied.
- No simulator transport source change was needed: the packet's focused
  command-session cycle exercises synthetic response payloads without adding a
  forbidden manifest dependency or untested simulator behavior.

## Claims

Candidate claim after review and integration rerun: `UNIT_VERIFIED` for the
deterministic command-session stale/current response behavior only. The focused
test does not drive a `topology-simulator` transport path, so
`SIMULATOR_VERIFIED` remains unavailable for this packet.

Not earned: `PLATFORM_DEVICE_VERIFIED`, `HARDWARE_VERIFIED`, protocol-byte or
fixture compatibility, physical reconnect, native transport, UI, or
accessibility claims.

## Blockers and next step

No environment, fixture, or hardware blocker. Independent security review is
`REVIEW_APPROVED`; parent reran the focused test, command-engine package,
simulator package, workspace format check, and command-engine Clippy with
`-D warnings`, all exit 0. The bounded candidate and status updates are
published to public main; see `integration-sweep.md`.
