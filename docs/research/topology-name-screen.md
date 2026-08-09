# Retired-candidate name screen (historical record)

**Packet:** `TOP-RSCH-005`  
**Question:** Is a now-retired candidate usable as the working/public name for an open modeler editor, and which package IDs, repository names, domains, and store titles avoid obvious collisions?  
**Screen date:** 2026-08-08 (America/Chicago; source pages and APIs were accessed on this date)  
**Status:** `REVIEW_APPROVED` for a preliminary screen only; not public-name or legal clearance  
**Decision owner:** parent/integration owner; counsel owns any legal clearance decision

> **Historical-evidence notice:** This report documents a discarded candidate and its source quotations/URLs. It is not a RigWarden screen, trademark clearance opinion, legal advice, or a finding that any party has enforceable rights. It is not comprehensive. A trademark attorney should perform a current comprehensive search before any public launch, registration, store submission, domain purchase, or rights-holder contact.

## Decision summary

- **Internal working name: KEEP for now.** It is useful in the starter kit and does not by itself publish a product or reserve an identifier.
- **Public exact display mark `Topology`: REPLACE is the safest recommendation.** The exact word is already used for active music/audio activity and a Canadian registration covering musical recordings, music-recording/reproduction apparatus, downloadable music, and music services; it also appears in active US software marks and an active Apple/Google app title. Those are materially close to a local modeler/editor audience. See [CIPO record 1864695 / TMA1077623](https://ised-isde.canada.ca/cipo/trademark-search/1864695), [Topology Music](https://topologymusic.com/), [USPTO 75359122](https://tsdr.uspto.gov/#caseNumber=75359122&caseSearchType=US_APPLICATION&caseType=DEFAULT&searchType=statusSearch), and [Google Play `com.skopei.topology`](https://play.google.com/store/apps/details?id=com.skopei.topology).
- **Fallback if product direction requires the word:** `MODIFY` the public mark with a genuinely distinctive house term and descriptor, then rerun the screen and obtain counsel review. `Topology — Fractal Editor` is a descriptor proposal only; it is **not** cleared and should not be published on the strength of this document.
- **Package/repository IDs:** use namespaced provisional IDs internally; do not publish or reserve the exact `topology` slug. Exact package and repository namespaces are already occupied or crowded. See the provisional-ID table below.
- **No external state was changed:** no domain was purchased, store listing was created, repository/package was published, or rights holder was contacted.

### Risk rubric used in this report

| Label | Engineering meaning for this screen | What it does **not** mean |
| --- | --- | --- |
| **High** | Exact `Topology` wording plus materially overlapping software, music/audio, or modeler/editor audience; public naming should stop for counsel/research. | A legal infringement, likelihood-of-confusion, or registration result. |
| **Medium** | Exact title/namespace with different goods, or close modeling/network/editor usage that can create search/store/package confusion. | That the use is legally actionable. |
| **Low** | Remote category or historical/adjacent use; still relevant to search noise. | Permission to publish or an availability guarantee. |

## Method and evidence boundary

The sweep followed the packet's source priority: trademark databases; Apple App Store and Google Play; GitHub/GitLab; crates.io and pub.dev; music/audio/modeler products; domains and social handles. Exact and adjacent searches used the strings `TOPOLOGY`, `topology`, `Topology Editor`, `topology project`, and obvious package/domain variants. API result counts and metadata are snapshots as of 2026-08-08 and can change. Direct record URLs are provided so a reviewer can repeat the checks.

The [USPTO federal-search guidance](https://www.uspto.gov/trademarks/search/federal-trademark-searching) explicitly describes searching the exact wording and related goods/services and warns that a comprehensive search is complex. The [WIPO Global Brand Database](https://www.wipo.int/en/web/global-brand-database/index) says its collections should be supplemented with national/regional-office searches and professional advice. The WIPO interactive quick search was reached but its Altcha challenge prevented a deterministic `TOPOLOGY` query in this environment; that is a recorded limitation, not a clean result.

Source facts are marked **FACT**. Ratings and recommendations are **ENGINEERING INFERENCE**. Unresolved items are **UNKNOWN / NOT CLEARED**.

## Exact and materially close trademark findings

The USPTO query used the exact wordmark field (`WM` phrase `TOPOLOGY`) through the public trademark search service. The search returned 24 wordmark-related records at the time of access; the rows below are the records most relevant to software, music/audio, or public discoverability. Status and goods are taken from the linked USPTO TSDR records.

| Source and jurisdiction | FACT observed on 2026-08-08 | Category relevance | Risk / action |
| --- | --- | --- | --- |
| [USPTO application 99634140](https://tsdr.uspto.gov/#caseNumber=99634140&caseSearchType=US_APPLICATION&caseType=DEFAULT&searchType=statusSearch), United States | Exact standard-character `TOPOLOGY`; Relatent, Inc.; filed 2026-02-04; alive and published for opposition; IC 009 downloadable software/mobile apps for social networking. | Software/app title and store-search surface. | **High.** Exact public software wording is close enough to block an unreviewed exact app/editor title; counsel must assess goods/services. |
| [USPTO application 99634135](https://tsdr.uspto.gov/#caseNumber=99634135&caseSearchType=US_APPLICATION&caseType=DEFAULT&searchType=statusSearch), United States | Exact `TOPOLOGY`; Relatent, Inc.; filed 2026-02-04; alive application; IC 042 online/non-downloadable software for integration, data processing, application/data security, and infrastructure. | Software platform naming. | **High.** Exact software mark in a neighboring technical field; do not treat the pending status as permission. |
| [USPTO registration 2252682 / application 75359122](https://tsdr.uspto.gov/#caseNumber=75359122&caseSearchType=US_APPLICATION&caseType=DEFAULT&searchType=statusSearch), United States/United Kingdom owner | Exact `TOPOLOGY`; Topology Limited (UK); registered 1999 and shown as registered/renewed; IC 009 software for file-system, disk, document, data, system, and configuration management. | Exact registered software wording. | **High.** It is not audio software, but it is a long-lived exact software identity; public exact name needs counsel review or replacement. |
| [USPTO registration 5294303 / application 87177038](https://tsdr.uspto.gov/#caseNumber=87177038&caseSearchType=US_APPLICATION&caseType=DEFAULT&searchType=statusSearch), United States | Exact `TOPOLOGY`; Bespoke, Inc.; registered 2017; IC 009 software/mobile apps for virtual eyewear, image manipulation, and prescription-frame/lens design. | Exact app/software title, different product. | **Medium.** Search/store noise and exact-mark adjacency; lower goods overlap than the rows above. |
| [USPTO registration 7528859 / application 98256471](https://tsdr.uspto.gov/#caseNumber=98256471&caseSearchType=US_APPLICATION&caseType=DEFAULT&searchType=statusSearch), United States | Exact `TOPOLOGY`; Topology NJ LLC; registered 2024; IC 037/042 real-estate development, land-use, and urban-design planning. | Exact professional-services name, different field. | **Low–Medium.** Relevant to global search results, not a close product category on this screen. |
| [USPTO registration 6580226 / application 90399005](https://tsdr.uspto.gov/#caseNumber=90399005&caseSearchType=US_APPLICATION&caseType=DEFAULT&searchType=statusSearch), United States/United Kingdom owner | Exact `TOPOLOGY`; Hebe Studio Ltd.; registered 2021; IC 020/024 pet beds and bedding. | Exact word only; remote goods. | **Low.** Record it as noise; no product-direction conclusion. |
| [CIPO application 1864695 / registration TMA1077623](https://ised-isde.canada.ca/cipo/trademark-search/1864695), Canada; registered owner in Australia | Exact word mark `TOPOLOGY`; CIPO status **REGISTERED**, TM5 **LIVE/REGISTRATION/Issued and Active**; filed 2017-10-26, registered 2020-11-18, expiry shown as 2030-11-18; owner `Topology Inc`, The Gap, Queensland, Australia. Nice classes 9, 16, 25, 41 include musical recordings, music-recording/transmission/reproduction apparatus (including interfaces, amplifiers, microphones, headphones), downloadable music, music instruction materials, clothing, live/music entertainment, recording-studio, audio/video production, publishing, composition, and music education. | **Direct music/audio overlap** with an editor for modelers and audio hardware. | **High.** This is the strongest collision signal. It does not decide legal rights, but it makes an unreviewed public exact mark a bad engineering bet. |
| [USPTO registration 2422496 / application 75405755](https://tsdr.uspto.gov/#caseNumber=75405755&caseSearchType=US_APPLICATION&caseType=DEFAULT&searchType=statusSearch), United States (historical) | `TWIN TOPOLOGY` for professional audio recording electronics was marked cancelled (Section 8). | Historical audio use of the word. | **Low / historical only.** It is not treated as an active conflict; it reinforces that “topology” has appeared in audio equipment naming before. |

The CIPO record and the Australian [Topology Music site](https://topologymusic.com/) are both exact `Topology` uses. It is an **inference**, not a verified identity match, that they are connected: the CIPO owner is `Topology Inc` in Queensland, while the site describes a Brisbane/Australia ensemble. The [support page](https://topologymusic.com/support/) describes ongoing albums, performances, commissions, and music education. Do not collapse those facts into a legal ownership conclusion without counsel and registry documents.

## Store titles and music/modeler adjacency

| Surface | FACT observed on 2026-08-08 | Risk / action |
| --- | --- | --- |
| [Apple App Store — `Topology`](https://apps.apple.com/us/app/topology/id1525860385) | Exact title `Topology`; seller shown as Skopei Group B.V.; business app for booking/shared workspaces and vehicles. App metadata/version can vary by country and date. | **Medium–High store collision.** Goods differ, but an exact title occupies search/discovery and app identity. Use a modifier or replacement after counsel; do not rely on a store subtitle to cure the exact-title collision. |
| [Google Play — `com.skopei.topology`](https://play.google.com/store/apps/details?id=com.skopei.topology) | Exact title `Topology`; developer SKOPEI/Skopei Group B.V.; business booking/sharing app; listing showed 10K+ downloads and an active 2026 update. | **Medium–High store/package collision.** The Android package ID is already occupied; exact title should not be submitted. |
| [Google Play — Tone Trace: Rig Analyzer](https://play.google.com/store/apps/details?id=org.remnant.workshop.inc.tonetrace) | Music/audio app that models a complete guitar signal chain (amps, effects loops, MIDI routing, pedalboard/canvas-style workflow). | **Medium category adjacency.** Not an exact name conflict, but it shows an existing audience and product vocabulary around rig topology/editor workflows; differentiate product naming and claims. |
| [Google Play — Packet Road](https://play.google.com/store/apps/details?id=com.pakedou.app) | Network topology editor that places/connects routers, switches, firewalls, and packet flows. | **Medium category adjacency.** Not a mark conflict; reinforces that “topology editor” is a crowded descriptive product concept. |
| [Topology Music](https://topologymusic.com/) and [Bandcamp album](https://topologymusic.bandcamp.com/album/airwaves) | Active Brisbane/Australia ensemble using `Topology` since the late 1990s; site advertises releases, events, collaborations, and education. | **High audience adjacency.** A modeler/editor marketed to musicians would be discovered alongside an exact active music identity. |
| [Topology — Enterprise AI Governance](https://www.topology.sh/) | Active software site using exact `Topology` for identity-native AI-agent governance. | **Medium exact-software collision.** Different product, but exact technical software branding adds global search noise and counsel work. |
| [Topologic](https://topologic.app/) | Active SDK/plugin whose stated purpose is logical, hierarchical, topological representation of spaces/entities. | **Medium spelling/modeling adjacency.** Not the same word; relevant to modeler/software audience and to domain/search confusion. |

## Code-host and package namespace findings

These are namespace/discoverability facts, not trademark findings. Repository owners can change names, and a 404 is not a reservation or legal clearance.

### GitHub and GitLab

- [GitHub repository search for `topology`](https://api.github.com/search/repositories?q=topology&per_page=100) returned **23,297** matching repositories at access time. Exact or strongly relevant examples include [1257any/topology](https://github.com/1257any/topology) (canvas/TypeScript diagram framework), [zhaodabao/topology](https://github.com/zhaodabao/topology) (HTML5 network-topology diagram), [wenyuan/jtopo_topology](https://github.com/wenyuan/jtopo_topology) (topology diagram editor), [ParmEd/ParmEd](https://github.com/ParmEd/ParmEd) (parameter/topology editor and molecular simulator), and [Raynos/topology](https://github.com/Raynos/topology) (network topologies). 
- [GitLab API search for `topology`](https://gitlab.com/api/v4/projects?search=topology&per_page=100&order_by=star_count&sort=desc) returned many projects. Relevant examples include [domke/TopologyGenerator](https://gitlab.com/domke/TopologyGenerator) (create/modify network topologies for simulation), [jonnystorm/giraphe](https://gitlab.com/jonnystorm/giraphe) (discover/visualize L2/L3 topology), and [cumulus-consulting/tools/topology_converter](https://gitlab.com/cumulus-consulting/tools/topology_converter) (network simulation topology building). A direct request for the GitLab `topology` handle was served an anti-bot page, so ownership/availability of that exact handle is **UNKNOWN**.
- **Risk:** **Medium** for repository search, issue links, package docs, and contributor discoverability. Exact `topology` is not a credible public repository namespace to depend on. A namespaced organization/project slug is preferable, but must be manually checked immediately before publication.

### crates.io, pub.dev, and npm supporting evidence

| Registry | FACT observed on 2026-08-08 | Risk / action |
| --- | --- | --- |
| [crates.io search API `topology`](https://crates.io/api/v1/crates?q=topology&per_page=100) and [crate `topology`](https://crates.io/crates/topology) | Search reported 2,591 matches; exact crate `topology` exists (`0.1.0`, 1,692 total downloads) and describes process-topology helpers. [topology-traits](https://crates.io/crates/topology-traits) also exists (113,611 downloads shown) and [surge-topology](https://crates.io/crates/surge-topology) is an active adjacent topology engine. | **High exact package collision.** Do not publish a Rust crate named `topology`; use a project namespace and check ownership immediately before any release. |
| [pub.dev search API `topology`](https://pub.dev/api/search?q=topology) | Search returns multiple active packages including [device_topology_view](https://pub.dev/packages/device_topology_view), [flutter_topology_view](https://pub.dev/packages/flutter_topology_view), [flutter_topo_canvas](https://pub.dev/packages/flutter_topo_canvas), and [topology_view_icons](https://pub.dev/packages/topology_view_icons). The exact `https://pub.dev/api/packages/topology` endpoint returned 404 during this check. | **Medium–High namespace/category collision.** A 404 is not an availability or clearance result; use a namespaced Dart package ID and repeat the check before publishing. |
| [npm search API `topology`](https://registry.npmjs.org/-/v1/search?text=topology&size=100) and [npm `topology`](https://www.npmjs.com/package/topology) | Exact npm package `topology` exists and points to the Raynos network-topology repository; many `topology-*` and scoped topology UI packages also appear. | **Medium package/discoverability collision.** npm is supporting evidence, not a substitute for trademark review. |

## Domains and social handles

RDAP checks were made through [rdap.org](https://rdap.org/) on 2026-08-08. RDAP status means a domain object was returned; it does not establish the registrant's rights or a product relationship. HTTP pages were not treated as proof of ownership beyond the visible site/handle metadata.

| Surface | FACT observed | Risk / action |
| --- | --- | --- |
| [topology.com RDAP](https://rdap.org/domain/topology.com) | Exact `.com` object returned active; registration 1997-11-06 and expiration 2027-11-05 shown. | **High.** Exact root domain is occupied; do not plan around acquisition. |
| [topology.net RDAP](https://rdap.org/domain/topology.net), [topology.app RDAP](https://rdap.org/domain/topology.app), [topology.dev RDAP](https://rdap.org/domain/topology.dev), [topology.nl RDAP](https://rdap.org/domain/topology.nl) | Exact `.net`, `.app`, `.dev`, and `.nl` objects returned active/protected statuses and existing registration events. `topology.nl` is also the support domain linked by the Skopei Google Play listing. | **High exact-domain collision.** Treat every exact root as occupied or controlled by another party. |
| [topologymusic.com RDAP](https://rdap.org/domain/topologymusic.com) | Active domain, registered 2004-08-10; it resolves to the exact music identity described above. | **High audience adjacency.** Avoid close variants that imply affiliation. |
| [topology.sh](https://www.topology.sh/) and [topologic.app](https://topologic.app/) | Active sites use exact/near-exact software identities. RDAP for `.sh` was not deterministic in this environment; the live site is enough to record use, not ownership. | **Medium.** Exact/near-exact software search noise; no domain purchase or contact was attempted. |
| Candidate checks: `topology.audio`, `topology.fm`, `topologyeditor.com`, `topologyproject.org`, `topologyproject.dev` | RDAP returned no object or no deterministic response at check time for these strings. | **UNKNOWN / NOT CLEARED.** A 404, DNS failure, or missing RDAP object is not a registration/availability guarantee. Do not register until counsel and a registrar/registry check approve. |
| [GitHub `topology`](https://github.com/topology) | HTTP page is an existing account/profile named `Topology` with repositories. | **High handle collision.** Do not assume the account is available or affiliated with this project. |
| [X `@topology`](https://x.com/topology), [Instagram `@topology`](https://www.instagram.com/topology/), [Threads `@topology`](https://www.threads.net/@topology) | Existing public profiles/metadata use the exact handle; the visible profiles do not identify this project. | **High handle collision.** Use a namespaced provisional handle only after platform-by-platform manual confirmation. |
| [GitLab `topology`](https://gitlab.com/topology), [YouTube `@topology`](https://www.youtube.com/@topology), [TikTok `@topology`](https://www.tiktok.com/@topology), [Bluesky `topology`](https://bsky.app/profile/topology) | Anti-bot or generic pages prevented deterministic ownership/availability verification in this environment. | **UNKNOWN / NOT CLEARED.** Never infer availability from a generic page or HTTP 404. |

## Provisional IDs (internal only; not availability-cleared)

These are deliberately namespaced strings that can be used in local notes, branches, and unpublished fixtures. The endpoint checks below were non-publishing reads on 2026-08-08. A 404 only records that the queried resource was not returned at that moment; it is not a reservation, permission, or legal clearance.

| Surface | Provisional string | Check and current interpretation |
| --- | --- | --- |
| GitHub repository | `topologyproject/editor` | [GitHub path](https://github.com/topologyproject/editor) returned 404 at check; **not cleared**. Keep as an internal candidate only. |
| GitHub repository | `topologyproject/topology-editor` | [GitHub path](https://github.com/topologyproject/topology-editor) returned 404 at check; **not cleared**. |
| Rust crate | `topologyproject-core` | [crates API exact lookup](https://crates.io/api/v1/crates/topologyproject-core) returned 404 at check; **not cleared**. |
| Rust crate | `topologyproject-protocol` | [crates API exact lookup](https://crates.io/api/v1/crates/topologyproject-protocol) returned 404 at check; **not cleared**. |
| Dart package | `topologyproject_editor` | [pub.dev API exact lookup](https://pub.dev/api/packages/topologyproject_editor) returned 404 at check; **not cleared**. |
| Local application identifier | `org.topologyproject.editor` | A local reverse-domain-style placeholder only; no registry check or publication. **Not a public brand clearance.** |
| Store descriptor proposal | `Topology — Fractal Editor` | A possible modified display title; no store reservation and no trademark screen for the combination. **Do not publish without counsel and a fresh screen.** |
| Domain candidates | `topologyproject.org`, `topologyproject.dev`, `topologyeditor.com`, `topology.audio`, `topology.fm` | No deterministic RDAP object in this environment; **not available/cleared**. Registrar/registry status must be checked immediately before any authorized purchase. |

No candidate should be turned into a public repository, package, app listing, domain, or social handle until the naming decision and counsel review are complete. Candidate strings such as `topoforge`, `rigtopology`, `modelerforge`, and `fractalforge` were not adopted: the first already has GitHub uses and all would require their own collision screen.

## Recommended disposition

1. **Now:** keep `Topology` as an internal codename in this starter kit; use the namespaced provisional IDs above for local implementation scaffolding only.
2. **Before a public repository/package/store/domain/handle:** choose `REPLACE` as the default public naming path. If replacement is not acceptable, choose `MODIFY` with a distinctive house term, then rerun trademark, store, registry, domain, and handle searches for the full combination.
3. **Do not reserve or publish:** exact `Topology` app titles, exact `topology` package names, exact root domains, or exact social handles.
4. **Counsel gate:** obtain a comprehensive clearance search covering the jurisdictions and goods/services below before external launch. Counsel—not this report—decides whether any candidate is usable.

## Uncertainties, contradictions, and counsel flags

- The WIPO quick-search interaction was blocked by Altcha; no WIPO global exact-word result is asserted. The WIPO method page itself says to search national/regional offices and consult professional advice.
- This screen did not complete a comprehensive USPTO design/phonetic/similarity search, state/common-law search, EUIPO search, IP Australia search, or a complete UK/Canadian marketplace search. The linked records are a prioritized sample, not an exhaustive result set.
- The CIPO owner and the Australian Topology Music site are plausibly related but not proven to be the same legal entity. Treat that relationship as an open fact for counsel.
- App Store/Play metadata, download counts, package ownership, domain status, repository activity, and social profiles can change after the access date. Country storefronts can show different versions or sellers.
- The final live recheck of crates.io returned HTTP 403 (rate limiting/edge policy) even though the initial 2026-08-08 API capture returned JSON; the crate facts are therefore a dated snapshot and must be rechecked before publication.
- A package API 404, GitHub/GitLab 404, generic social page, DNS failure, or absent RDAP object is not an availability promise. Names may be reserved, unpublished, protected, or unavailable through a different route.
- No rights holder was contacted, no domain was purchased, no account was reserved, and no product was published. Any future contact or purchase is a separate authorized action.
- This document does not interpret trademark classes, fair use, priority, likelihood of confusion, dilution, registration eligibility, or infringement.

## Follow-up ADRs and packets

- **`TOP-ADR-NAMING-001` — Public identity decision:** record the owner-approved keep/modify/replace decision and the chosen display mark only after counsel review.
- **`TOP-RSCH-005A` — Expanded clearance screen (proposed):** rerun exact, phonetic, design, common-law, WIPO, EUIPO, UKIPO, IP Australia, CIPO, and USPTO searches for the selected full name; include product/store/domain/handle evidence and date-stamped captures.
- **`TOP-BOOTSTRAP-NAMING-GATE` (proposed implementation packet):** block public package, repository, store, domain, and social reservation until `TOP-ADR-NAMING-001` is approved; keep local IDs explicitly provisional.
- **Requirement-traceability update:** the integration owner may map this report to PLAT-005/PLAT-006 in the shared matrix. This worker did not edit shared matrices, manifests, or production code.

## Source index (all accessed 2026-08-08)

The links in the findings tables are the source-to-claim map. Primary method and registry sources are repeated here for reviewer convenience:

- [USPTO trademark search](https://www.uspto.gov/trademarks/search) and [federal search guidance](https://www.uspto.gov/trademarks/search/federal-trademark-searching).
- [WIPO Global Brand Database](https://www.wipo.int/en/web/global-brand-database/index) (interactive quick search blocked by Altcha in this environment).
- [CIPO trademark record 1864695](https://ised-isde.canada.ca/cipo/trademark-search/1864695).
- [Apple App Store search record](https://apps.apple.com/us/app/topology/id1525860385) and [Google Play record](https://play.google.com/store/apps/details?id=com.skopei.topology).
- [GitHub repository search API](https://api.github.com/search/repositories?q=topology&per_page=100) and [GitLab project search API](https://gitlab.com/api/v4/projects?search=topology&per_page=100&order_by=star_count&sort=desc).
- [crates.io search API](https://crates.io/api/v1/crates?q=topology&per_page=100), [pub.dev search API](https://pub.dev/api/search?q=topology), and [npm search API](https://registry.npmjs.org/-/v1/search?text=topology&size=100).
- [RDAP](https://rdap.org/) domain records linked in the domain table.

**Handoff boundary:** This report earns a preliminary, source-linked research result only. It does not earn a `LEGAL_CLEARED`, `PUBLIC_NAME_APPROVED`, `PACKAGE_PUBLISHED`, `STORE_SUBMITTED`, `DOMAIN_REGISTERED`, or `HANDLE_RESERVED` status.
