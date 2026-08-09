# Governance, Licensing, and Conduct

## 1. Project identity

RigWarden is an independent open-source community project. It is not an official Fractal Audio product and must display a clear compatibility disclaimer.

Do not use vendor trademarks in the primary project name, package ID, icon, or logo. Device names may be used factually to describe compatibility.

## 2. Early governance

Initial governance is founder-led:

- the founder is final product decision-maker during the first year or until a documented transition;
- maintainers earn responsibility through sustained, trustworthy contribution;
- architectural and product decisions are recorded as ADRs;
- security and legal concerns may override ordinary roadmap preference;
- governance evolves deliberately rather than by accidental commit access.

This is not permission for arbitrary undocumented decisions. The decision log remains authoritative.

## 3. Licensing

Intended policy:

- original RigWarden source: MIT;
- original declarative device/theme packs: MIT unless a specific compatible data license is selected by ADR;
- reused or derived permissive code/data: original license retained;
- Apache-2.0 components retain notices and attribution;
- documentation and fixtures include explicit ownership/provenance;
- no unknown-rights content is merged.

Required repository files before public release:

- `LICENSE`
- `THIRD_PARTY_NOTICES.md`
- `NOTICE` when required
- `CONTRIBUTING.md`
- `CODE_OF_CONDUCT.md`
- `SECURITY.md`
- `TRADEMARKS.md`
- provenance manifests
- contributor fixture declaration

## 4. Free distribution

- Source is free.
- Official App Store and Play Store builds are free.
- Signed GitHub releases are free.
- F-Droid build is free.
- Donations/sponsorship may later fund hardware, signing, and maintenance, but core functionality is not paywalled.
- No mandatory account or subscription.

## 5. Conduct

The project welcomes criticism of software and engineering decisions. It does not permit:

- personal harassment;
- public campaigns against named competitors;
- review bombing;
- dogpiling;
- threats;
- publishing private information;
- humiliating novice contributors;
- using the repository to settle personal disputes.

Public communication should focus on:

- features;
- evidence;
- reproducibility;
- accessibility;
- compatibility;
- licensing;
- community benefit.

## 6. Competitive boundaries

RigWarden may:

- implement compatible behavior lawfully;
- compare public features factually;
- explain its own open-source model;
- document reproducible bugs;
- audit permissively licensed code;
- provide migration/import tools where lawful.

RigWarden must not:

- copy a competitor’s UI assets or distinctive decorative layout;
- impersonate an official vendor app;
- coordinate negative reviews;
- include insulting commentary in source/docs;
- use confidential or unlawfully obtained information;
- claim superiority without evidence.

## 7. Contribution model

Contributions require:

- requirement/work-item linkage;
- strict TDD evidence for production behavior;
- provenance for fixtures/profile data;
- license declaration;
- accessibility impact;
- compatibility claim at the correct level;
- independent review;
- no unexplained generated blobs.

Profile contributions additionally require:

- exact device/firmware;
- transport;
- source/capture declaration;
- checksum;
- sanitized fixture;
- test results;
- verification label;
- contributor permission to redistribute.

## 8. Naming

“RigWarden” remains a working name until a preliminary and then professional collision/trademark review is complete.

Do not reserve public package IDs, publish store listings, purchase domains, or announce final branding before `TOP-RSCH-005` is approved.
