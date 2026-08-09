# TOP-RSCH-007 handoff

**Status:** `REVIEW_APPROVED` (bounded research only)  
**Worker:** `/root/distribution_constraints`  
**Parent:** `/root`

## Delivered

- [docs/research/distribution-constraints.md](/Users/benzo/Desktop/Topology_Codex_Starter_Kit_v1/docs/research/distribution-constraints.md) maps current official Apple, Android/Google Play, F-Droid, and GitHub constraints.
- Recommended product floors: iOS/iPadOS 16.0 and Android 10/API 29, clearly labeled engineering recommendations rather than store minimums.
- Release SDK gates: Apple Xcode 26/iOS/iPadOS 26 SDK; Play target API 36 from 2026-08-31.
- Conditional permissions/entitlements for Core MIDI, BLE, USB, files, local network bridge, telemetry, and AI.
- F-Droid pure-open-source/offline flavor constraints and the static `INTERNET` permission blocker.
- Owner-controlled Apple/Play/GitHub/F-Droid signing, identity, package, and credential tasks.
- Privacy/data-safety inventory and prohibited assumptions.
- Proposed ADRs and future packet inputs; no production behavior or shared manifest changed.

## Evidence

- Official-source access log: `source-access-log.md`.
- Environment and credential boundary: `environment.txt`.
- Packet copy: `work-item.yaml`.
- Validation commands/results: `sweep-commands.txt`, `sweep.log`, `sweep-exit-statuses.txt`.
- Scope record: `files-changed.txt`.
- Independent review is still pending in `review.md`.

## Claims earned

- Current official source map at access date 2026-08-08.
- Distribution architecture recommendations and conditional permission/entitlement inventory.
- Explicit F-Droid flavor/signing constraints and release blockers.
- Account-owner boundary and privacy/data-safety checklist.

## Claims unavailable

- App Store/TestFlight/Play/F-Droid approval or inclusion.
- Legal/privacy-policy approval, trademark/package-name clearance, or regional compliance.
- Signing-key ownership, account identity verification, or package registration.
- Physical native transport, hardware, VoiceOver/TalkBack, store-install, or reproducible-build verification.
- Current Git commit/integration state: starter kit is not a Git worktree.

## Blockers and next actions

1. Parent/reviewer should independently review the report and record a decision in `review.md`.
2. Owner should resolve ADR-DIST-003: separate offline F-Droid artifact/package versus amended network-permission requirement.
3. Bootstrap should install Flutter/Dart/JDK/Gradle, initialize Git, and run native/store test packets.
4. Owner should schedule Apple/Google/GitHub account, signing, package registration, and test-track tasks; no credentials should be passed to agents.
