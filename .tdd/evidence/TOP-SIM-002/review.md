# TOP-SIM-002 independent security review

Reviewer: `/root/sim002_review` (`topology_security_reviewer`, OpenAI
`gpt-5.6-luna` / max)
Review timestamp: `2026-08-10T06:12:02Z` (local `2026-08-10T01:12:02-0500`)
Candidate basis: shared dirty worktree at starting commit
`536d8901ac91ecdbc15e09356800d9f46be401dd`; candidate source is not integrated.

Decision: `REVIEW_APPROVED` for the amended, bounded L1 command-session behavior,
with the evidence correction and P2 hardening follow-up below required before
the candidate is treated as integrated truth.

## Findings and behavior audit

- The packet and frozen evidence copy now contain one ordered `amendments` list
  with both the dependency-ready amendment and the `lib.rs` export-scope
  amendment, plus the 01:08 claim/layer amendment. A Ruby YAML parse returns all
  three entries in both files; no amendment is silently lost.
- `crates/topology_command_engine/src/lib.rs:3-5,12-14` declares and exports
  `session` exactly under the parent-approved amendment. The implementation,
  focused test, and evidence are within the packet paths. The simulator source
  is unchanged; there is no new manifest, lockfile, app, native, device-pack,
  index, or traceability change attributable to this packet.
- `crates/topology_command_engine/src/session.rs:126-131` advances the
  connection generation and clears the disconnected generation's pending set.
  `:140-153` compares the response generation before constructing a current
  request key or mutating `pending`/`confirmed`. Therefore a generation-1
  response for reused request ID 7 is returned as `IgnoredStale`, leaves the
  generation-2 request pending, and cannot create a confirmation. A generation-
  2 response then removes only the current pending key and stores its opaque
  payload as `Confirmed`.
- `crates/topology_command_engine/tests/stale_response.rs:5-27` uses the same
  request ID across generations, asserts stale disposition, pending-state
  preservation, absence of a current confirmation, current confirmation, and
  the literal current payload. A removal of the generation guard would make
  the stale call return `Confirmed` and fail line 14; moving the guard after
  `pending.remove` would fail line 15. No production mutation was made for
  this review; this is a direct test-strength/mutation reasoning check.
- Current-generation unmatched responses are fail-closed (`:147-150`), and
  no retry, partial-batch, hardware-write, firmware-selection, protocol-byte,
  network, secret, telemetry, profile-pack, or AI path is present. The payload
  remains explicitly opaque (`:86-89`).

### P2 — bounded ingress and session queues remain a deferred security gap

`crates/topology_command_engine/src/session.rs:68-73` copies every supplied
payload with `to_vec()` before `process_response` can reject a stale generation.
The `pending` and `confirmed` collections at `:103-109` also have no count or
byte budget. Once a real transport forwards untrusted frames, a peer can send
an oversized stale payload or drive many outstanding requests and cause memory
pressure/OOM; this is an availability failure, not a stale-confirmation bypass
in the current synthetic unit path. The global threat model requires bounded
buffers and pending queues. This packet does not add a transport ingress or
parser, so the fix belongs in a bounded-input/transport follow-up rather than
this cycle.

Missing tests for that follow-up: maximum frame/payload rejection before copy,
bounded pending/confirmed accounting, oversized stale input, duplicate request
IDs within one generation, and generation-overflow behavior. Do not claim
arbitrary untrusted-frame safety from this packet until those boundaries exist.

### Evidence consistency check

The parent corrected `handoff.md` to match the 01:08 amendment: it now records
`UNIT_VERIFIED` only and explicitly withholds `SIMULATOR_VERIFIED`. The packet,
frozen work-item copy, handoff, and this review therefore agree on the L1 claim
boundary.

## Independent reruns

All commands were run from `/Users/benzo/Desktop/Topology_Codex_Starter_Kit_v1`
against the frozen candidate without production edits:

1. `cargo test -p topology-command-engine stale_response_cannot_confirm_new_connection_request -- --exact --nocapture` — exit `0`.
2. `cargo test -p topology-command-engine` — exit `0` (all package tests and doctests).
3. `cargo test -p topology-simulator` — exit `0` (scripted simulator test and doctests).
4. `cargo fmt --all -- --check` — exit `0`.
5. `cargo clippy -p topology-command-engine --all-targets -- -D warnings` — exit `0`.
6. Supplemental `cargo clippy -p topology-simulator --all-targets -- -D warnings` — exit `0`.

The preserved canonical RED is valid (exit `101` for the intentionally absent
session API); the earlier invalid harness attempt is correctly excluded. The
focused GREEN and required sweep evidence are reproducible.

## Verification-label audit

After parent integration reruns the focused test and required sweeps from the
immutable integration commit, the only supported claim is `UNIT_VERIFIED` for
the in-memory connection-generation correlation behavior exercised by this
test. `SIMULATOR_VERIFIED` is unavailable because the test directly constructs
`IncomingResponse` and does not drive `ScriptedTransport`; the parent’s 01:08
amendment correctly records that boundary. `PLATFORM_DEVICE_VERIFIED`,
`HARDWARE_VERIFIED`, byte/protocol compatibility, physical reconnect, firmware
matching, writes, retries, partial completion, UI/accessibility, secrets,
telemetry, network, pack trust, and AI claims remain unavailable.

Parent integration must land the candidate, rerun the focused test and packet
sweeps, preserve this review, and only then promote TOP-SIM-002 to `INTEGRATED`.
