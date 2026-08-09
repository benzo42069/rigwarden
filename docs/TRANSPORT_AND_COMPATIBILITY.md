# Transport and Compatibility Contract

## 1. “All transports” means a matrix

Topology does not promise that every device connects through every method on every OS. It promises to implement and document every technically feasible path for the selected device/platform combination.

Compatibility is a four-dimensional claim:

```text
device + firmware + operating system + transport/adapter
```

Some features add a fifth dimension because they may be available only over a particular physical port or message mode.

## 2. Target transport families

1. Direct USB MIDI.
2. Direct Android USB host/serial where a device exposes a serial control channel.
3. Class-compliant USB MIDI interfaces.
4. Five-pin MIDI through compatible interfaces.
5. BLE MIDI adapters.
6. Desktop or Raspberry Pi network bridge.
7. Later desktop-native MIDI/serial paths.

Before implementation, research packets must verify each device path against primary vendor/platform documentation and lawful captures. Do not convert a community claim into a public fact without evidence.

## 3. Transport capability model

Each open session exposes machine-readable capabilities such as:

- discovery;
- identity query;
- preset list;
- preset dump;
- parameter read/write;
- routing write;
- large SysEx transfer;
- realtime tuner stream;
- tempo stream;
- looper status;
- cab transfer;
- FC editing;
- maximum safe frame or chunk size;
- exclusive access;
- background availability;
- write verification method.

UI must explain missing capabilities.

## 4. Required adapter interface

A transport adapter should provide conceptually:

```text
enumerate() -> endpoints
open(endpoint, options) -> session
session.send(frame, cancellation) -> send receipt
session.events() -> bytes/lifecycle/errors
session.close()
session.capabilities()
```

Requirements:

- stable endpoint IDs where possible;
- no implicit selection of the first matching device;
- bounded buffers;
- bounded retries;
- explicit timeout;
- cancellation;
- connection generation IDs so stale responses cannot satisfy new requests;
- sanitizable diagnostics;
- deterministic fake adapter for tests.

## 5. Exclusive-port behavior

When another editor owns an exclusive port:

- do not fight for the endpoint;
- do not repeatedly reopen in a tight loop;
- identify likely conflict;
- show exact steps to close the other editor;
- retain the user’s endpoint selection;
- allow explicit retry;
- do not mislabel the device as unsupported.

## 6. Network bridge

The network bridge is optional infrastructure, not a mandatory cloud dependency.

Requirements:

- local network by default;
- explicit pairing;
- mutual authentication;
- encrypted transport;
- no public relay required;
- protocol version negotiation;
- device/endpoint capability relay;
- bounded queueing;
- reconnect and session ownership;
- clear distinction between bridge and direct connection;
- headless Raspberry Pi/desktop packaging;
- no ability for an unpaired network client to send device writes.

It may launch as `EXPERIMENTAL`, but must be an actual working path with tests.

## 7. BLE MIDI

BLE MIDI support must test:

- discovery and permission;
- pairing/reconnect;
- MTU/fragmentation;
- large SysEx behavior;
- latency and ordering;
- background suspension;
- adapter-specific quirks;
- cancellation;
- long transfer reliability.

Do not claim that a simple note-on test proves editor-grade SysEx support.

## 8. Mobile lifecycle

The app must survive:

- screen rotation;
- split-screen size changes;
- app background/foreground;
- screen lock;
- USB unplug/replug;
- Bluetooth disconnect;
- device reboot;
- permission revocation;
- OS memory pressure;
- process death and journal recovery.

Stage Mode may request screen-awake/background behavior, but must accurately explain OS limitations.

## 9. Compatibility evidence

A compatibility record includes:

- app version/commit;
- device and hardware variant;
- firmware;
- mobile/desktop device;
- OS version;
- transport;
- adapter and firmware when applicable;
- feature matrix;
- test suite version;
- date;
- verifier identity or pseudonymous contributor ID;
- sanitized logs/capture checksums;
- status label.

Compatibility reports are generated from these records.

## 10. Initial beta gate

Before public beta:

- AM4 has a complete hardware-verified end-to-end vertical slice.
- FM3 has a complete hardware-verified end-to-end vertical slice.
- Every advertised transport is implemented or explicitly removed by a reviewed product decision.
- BLE MIDI and network bridge may remain experimental.
- Other device profiles may be experimental or community-confirmed.
- No unverified device is described as fully supported.
