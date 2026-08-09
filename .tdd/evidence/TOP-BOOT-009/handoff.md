# TOP-BOOT-009 implementer handoff

Status: INTEGRATED. Candidate public commit: `c4c4ecc4c2019702228c6db658d48c59bdcb28dc`; post-landing checks are recorded in `integration-sweep.*`.

## Behavior delivered

`topology-devtools validate-fixture <path>` reads one local provenance sidecar and delegates validation to the existing `fixture::validate_yaml` library validator. A permitted sidecar prints exactly `valid` on stdout and exits zero. A denied sidecar prints deterministic `<path>: <field> [<code>] <message>` diagnostics on stderr and exits nonzero. Missing/extra arguments use a stable usage diagnostic; file-read failures are nonzero. The command performs no network access.

## TDD evidence

- RED: `cargo test -p topology-devtools fixture_cli_returns_truthful_exit_status -- --exact --nocapture`, exit 101. The integration test reached the package and failed because the binary target was absent (`CARGO_BIN_EXE_topology-devtools` was not available).
- `red-preflight-shell-variable.log` is retained only as a discarded shell-variable preflight diagnostic; it is not RED evidence. The canonical accepted RED is `red.log` with its matching command and status record.
- GREEN: same focused command, exit 0 after adding the binary.
- Post-format focused rerun: exit 0.
- Required sweeps: `cargo test -p topology-devtools`, `cargo fmt --all -- --check`, and `cargo clippy -p topology-devtools --all-targets -- -D warnings` all exit 0.

## Scope and design

Only the packet-authorized binary and integration test were changed. No CLI framework or dependency was added. The CLI does not parse raw protocol bytes, scan repositories, verify signatures, or make network requests.

## Claims

Earned after integration/review: `UNIT_VERIFIED`, `CLI_INTEGRATION_VERIFIED` (packet-local labels; parent must rerun after landing). Not earned: `BYTE_FIXTURE_VERIFIED`, `CAPTURE_VERIFIED`, physical hardware or firmware compatibility.

## Next step / blockers

Independent `topology_reviewer` review is required. Parent integration must apply the two source/test files and evidence, rerun the focused test and packet sweeps, then update the packet/index status. No hardware, fixture, credential, or network dependency blocks this cycle.
