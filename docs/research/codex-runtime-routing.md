# Codex runtime and subagent-routing audit

**Packet:** `TOP-RSCH-001`  
**Recorded:** 2026-08-08 UTC  
**Status:** `REVIEW_PENDING` — revised after independent review

## Result

The requested project-scoped `topology_explorer` route is verified for child **model and reasoning effort**. The parent explicitly selected that agent type and the selected child’s runtime metadata records `gpt-5.6-luna` and `max`; this is not based on child self-identification. The runtime did not honor that role’s `read-only` sandbox request.

| Field | Requested | Runtime-visible evidence |
| --- | --- | --- |
| Parent model / effort | `gpt-5.6-terra` / `high` | Parent runtime rows `306832976–306832977`. |
| Custom-agent selection | `topology_explorer` | Parent runtime row `306862840` records `spawn_agent` with `agent_type:"topology_explorer"`. |
| Child model / effort | `gpt-5.6-luna` / `max` | Child thread `019fe35d-47a1-7f42-b721-0a2fbb85f51c`, runtime row `306864064`, records both `codex.turn.reasoning_effort=max` and `model="gpt-5.6-luna"`. |
| Effective provider | unverified | Runtime records `auth_mode=Some(Chatgpt)`; it does not expose a provider field. OpenAI is configuration, not runtime proof. |
| Multi-agent interface | enabled | Runtime row `306864064` lists `Collab,MultiAgentV2`; this session exposes agent spawn/wait/status controls. |
| Child sandbox / approvals | `read-only` / inherited | Same runtime row records `sandbox_policy=DangerFullAccess`, `approval_policy=Never`. Read-only isolation is not verified. |

The first generic `explorer` probe remains supplemental only. It was not treated as proof for the project custom role. The selected custom-role probe establishes model/effort routing, but not sandbox enforcement.

Official [Codex subagent documentation](https://learn.chatgpt.com/docs/agent-configuration/subagents) describes project custom-agent TOML files, model/effort precedence, and sandbox configuration; [OpenAI model guidance](https://developers.openai.com/api/docs/guides/latest-model) documents GPT-5.6 Luna and `max` reasoning (accessed 2026-08-08).

## Installed environment

| Component | Observed result |
| --- | --- |
| ChatGPT desktop app / build | `26.803.41515` / `6321` |
| Codex CLI / framework build | `0.147.0-alpha.6.5` / `151.0.7922.76` |
| macOS | 26.5, build 25F71, arm64 |
| Rust / Cargo | 1.97.0 / 1.97.0 |
| Xcode | 26.5 (17F42) |
| Android tooling | ADB 37.0.0-14910828; Android SDK and `sdkmanager` absent; both SDK environment variables unset |
| Python | 3.14.6 |
| Flutter / Dart | missing at the initial audit; later local installation is separately verified before bootstrap |
| Java / JDK / Gradle | absent at the initial audit |
| Git | 2.50.1, but the extracted starter kit is **not a Git worktree** |
| Starter-kit validator | blocked: `PyYAML` and `jsonschema` absent (exit status 2) |

## Operating decision

Model/effort routing is verified. Sandbox isolation is not. Consequently, do not create further write-capable fan-out until isolated worktrees or verified sandboxing exists. The already-issued research writers own disjoint documentation/evidence paths and must receive scope review before integration. Before bootstrap, initialize a Git worktree and install documented validator/Flutter/JDK/Android-SDK prerequisites.

## Unavailable claims

- No operating-system read-only isolation claim.
- No effective model-provider claim beyond configured OpenAI and observed ChatGPT authentication.
- No Flutter, Android, iOS, hardware, product-code, or compatibility claim.

## Follow-up

`TOP-BOOT-001` may start only after all wave-00 reports are independently reviewed and integrated.
