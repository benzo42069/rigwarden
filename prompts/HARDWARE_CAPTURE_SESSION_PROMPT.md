Run a Topology hardware capture/verification session for the explicitly connected device.

Before sending anything:

1. Read the applicable work packet, hardware-verification contract, transport matrix, and fixture-provenance contract.
2. Record device model/variant, firmware, host/mobile device, OS, transport, adapter, app commit, and session generation.
3. Confirm whether the packet is read-only or write-capable.
4. Confirm other official editors are closed when the endpoint is exclusive.
5. Confirm logs/captures will sanitize serials, paths, credentials, and unrelated traffic.
6. Confirm every write is bounded and has acknowledgement/read-back policy.
7. Never explore unknown write opcodes on valuable user presets. Use an agreed test preset/slot and the minimum safe mutation.

Execute only the packet’s declared steps. Capture exact commands/actions and observed responses. Reconcile final hardware state. Produce:

- sanitized capture;
- fixture sidecar;
- checksum;
- hardware matrix entry;
- pass/fail per step;
- recovery performed;
- verification label justified.

A missing device, adapter, permission, or fixture is a blocker, not a pass.
