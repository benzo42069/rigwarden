# Hardware Verification

## Principle

Physical compatibility is a matrix and a procedure, not a statement that “it worked for me.”

## Prerequisites

- relevant unit/byte/simulator/native tests green;
- exact device/firmware profile;
- declared transport and adapter;
- safe test preset/slot;
- recovery plan;
- other editors closed if exclusive;
- capture sanitization ready;
- no unknown write exploration;
- user aware of level/output risks.

## Record

- work-item ID;
- app commit/build;
- device model/variant/serial redacted token;
- firmware;
- mobile/host model;
- OS;
- transport;
- adapter model/firmware;
- power/cable path where relevant;
- exact steps;
- expected behavior;
- actual behavior;
- timings;
- reconnect/reset behavior;
- final state;
- recovery;
- sanitized logs/captures/checksums;
- verifier/date.

## Minimum write verification

For a parameter write:

1. Read current value.
2. Record journal prior state.
3. Send one bounded known-safe change.
4. Receive acknowledgement/read-back as profile requires.
5. Confirm UI state.
6. Read independently again if supported.
7. Undo.
8. Confirm original value restored.
9. Disconnect/reconnect and confirm consistent state where relevant.

## Structural verification

Use a disposable test preset. Verify:

- read;
- add/move/connect;
- device confirmation;
- preset save semantics;
- undo/reversal;
- partial failure behavior;
- no adjacent preset corruption.

## Soak

For beta flagship profiles:

- sustained editing;
- repeated parameter changes;
- repeated scene/channel changes;
- large preset read;
- unplug/replug;
- app background/foreground;
- cancellation;
- other-editor conflict;
- transport-specific stress.

## Labels

`HARDWARE_VERIFIED` applies only to the tested matrix and features. It does not automatically cover:

- another firmware;
- another OS;
- another adapter;
- another transport;
- another device variant;
- untested feature families.

## Community confirmation

Community evidence may become `COMMUNITY_CONFIRMED` after review. It remains distinct from founder/project-run `HARDWARE_VERIFIED` if the project chooses that public distinction.

## Failure

A failed test:

- preserves logs;
- reconciles hardware;
- creates a bug/regression packet;
- does not update the profile to claim support;
- does not weaken the procedure.
