# Protocol Fixture Provenance

## Acceptance rule

A protocol fixture cannot enter the compatibility suite without a valid sidecar and reviewer approval.

## Sidecar minimum

```yaml
fixture_id:
device:
  family:
  model:
  variant:
firmware:
transport:
direction:
feature:
captured_at:
source:
  category:
  reference:
  license:
  commit:
derivation:
sanitization:
redistribution:
  permitted:
  basis:
sha256:
expected:
confidence:
verification_status:
contributor:
review:
```

## Source priority

1. Vendor specification/manual.
2. User-owned direct hardware capture.
3. Permissively licensed implementation/vector.
4. Community capture with explicit permission.
5. Independently generated simulator vector, labeled simulator-only.

## Sanitization

Remove or replace:

- serials;
- account/user names;
- local paths;
- API keys;
- unrelated MIDI;
- preset names containing personal information;
- timestamps when unnecessary;
- device identifiers not required for behavior.

Document every replacement. Do not alter bytes required for the protocol expectation without clearly deriving a sanitized equivalent.

## Redistribution

The contributor states that they:

- own or are authorized to provide the capture;
- grant redistribution under the declared fixture license;
- did not include vendor binaries/artwork/confidential data;
- understand the fixture will be public.

Unknown permission means `BLOCKED_FIXTURE`.

## Confidence

Suggested values:

- `published_spec`
- `direct_capture_single`
- `direct_capture_repeated`
- `cross_implementation`
- `community_unreplicated`
- `simulator_only`

Confidence is not verification status.

## Review

Reviewer confirms:

- checksum;
- source exists;
- license/permission;
- expected semantic value independent from decoder;
- sanitization;
- firmware/device exactness;
- no secret/personal data;
- correct verification label.

## Fixture changes

Fixtures are immutable. Corrections create a new fixture version/ID and document why the prior one was wrong. Do not silently replace bytes under an existing checksum.
