# Topology distribution constraints

**Packet:** `TOP-RSCH-007`  
**Recorded:** 2026-08-08  
**Status:** `REVIEW_APPROVED` (bounded research only)  
**Scope:** current official Apple, Android/Google Play, GitHub, and F-Droid distribution constraints. This is engineering research, not legal advice or a store-submission record.

## Executive result

### Recommended release matrix

| Channel | Product/runtime target | Current distribution constraint | Recommendation |
| --- | --- | --- | --- |
| iOS/iPadOS, TestFlight, App Store | **Minimum iOS/iPadOS 16.0** (project recommendation); build with **Xcode 26 / iOS and iPadOS 26 SDK or later** | Apple says uploads from 2026-04-28 require the iOS/iPadOS 26 SDK or later [A1]. Xcode 26's upload deployment-target range starts at iOS/iPadOS 15 [A2]. | Keep the existing 16.0 product floor, pin a released Xcode 26 toolchain in CI, and test iPhone/iPad transport and privacy behavior on physical devices. The 16.0 floor is an engineering choice, not an Apple minimum. |
| Android, Google Play | **Minimum Android 10 / API 29** (project recommendation); **compile/target API 36** for Play submissions after 2026-08-31 | New apps and updates must target Android 16 / API 36 from 2026-08-31; existing apps need API 35 to remain visible to new users on newer Android versions [G1][G2]. | Keep the requested Android 10 floor, but make target API 36 a release gate. `targetSdkVersion` is not `minSdkVersion`; this does not prove every API-29 device has the required USB/BLE hardware. |
| GitHub Releases (APK) | Android 10/API 29 minimum, same as the Android product | GitHub releases package Git tags and binary assets; GitHub does not provide Android APK signing merely by hosting an asset [H1]. Artifact attestations can bind a binary to repository, workflow, commit, and environment [H2]. | Publish a developer-signed APK, SHA-256 manifest, signed release tag, and GitHub artifact attestation. Publish the signing-certificate SHA-256 fingerprint and an offline verification recipe. |
| F-Droid | Separate **pure-open-source offline flavor**; minimum API should remain aligned with Android product unless F-Droid build review requires otherwise | F-Droid expects public source, a recognized FOSS license, FOSS dependencies/tooling, source-buildable metadata, and isolated build success [F1][F2]. Tracking, proprietary dependencies/services, and non-free assets are surfaced as Anti-Features [F3]. | Build and review an `fdroid` flavor from tagged source with no telemetry, AI/provider network, profile-update network, GMS/Firebase, or proprietary SDK. Decide package/signing identity before first public release. |

**Why these floors:** the project decision log selects iOS/iPadOS 16+ and Android 10+ as its baseline (DEC-029). The current Apple toolchain can upload an iOS 16 deployment target, and Android's USB host APIs exist from API 12 while BLE permission behavior has a clear API-31 boundary [A2][G3][G4]. Nothing in the reviewed store policy requires Android 10 or iOS 16; those floors remain product and maintenance decisions and must be rechecked during the bootstrap/toolchain audit.

### Immediate release-order consequences

1. Treat the Apple SDK requirement and Android target-API deadline as build configuration gates, not as app minimum-OS claims.
2. Keep iOS/iPadOS, Android/Play, GitHub APK, and F-Droid artifacts as distinct signed outputs with recorded hashes.
3. Decide the F-Droid package/signing model before the first Android release. Android APK update continuity follows signing keys; a Play-signed APK and an F-Droid-signed APK cannot be assumed to cross-update.
4. Do not add optional permissions or entitlements speculatively. Request them only in the feature path that needs them and provide a useful read/offline path when a user declines.
5. Treat store forms, privacy policy text, developer verification, and signing as owner-controlled release work. This packet does not submit, publish, or handle credentials.

## Verified facts and engineering implications

### Apple targets, SDK, and beta distribution

- **VERIFIED_FACT [A1][A2]:** As of 2026-04-28, App Store Connect uploads for iOS/iPadOS must be built with the iOS/iPadOS 26 SDK or later. Apple's Xcode system table lists Xcode 26's upload deployment target range as iOS/iPadOS 15 through 26.x. The installed environment here has Xcode 26.5, which is in the Xcode 26 release line; the final release workflow still needs a pinned, reproducible toolchain record.
- **RECOMMENDATION:** Keep `IPHONEOS_DEPLOYMENT_TARGET`/iPadOS deployment target at 16.0 for the first supported product matrix. This preserves the project decision while remaining inside Xcode 26's documented upload range. Raise or lower it only through a reviewed ADR backed by device-usage and native-API evidence.
- **VERIFIED_FACT [A1][A15][A16][A17]:** TestFlight is Apple's beta service. Internal testers are team members with eligible roles; external testing uses a build submitted for TestFlight review, with up to 10,000 external testers. Uploaded builds expire for TestFlight after 90 days [A17].
- **VERIFIED_FACT [A3]:** Apple reviews apps and updates for privacy, security, safety, reliability, and policy compliance. App Review notes must explain non-obvious features and provide access to any hardware or backend needed to review the app.
- **UNKNOWN:** Whether every planned Core MIDI, BLE MIDI, direct USB adapter, and local-bridge path is acceptable in a given App Store review context. A working simulator or a Core MIDI API reference does not grant a physical-device or entitlement claim.

### Android targets, Play testing, and future platform changes

- **VERIFIED_FACT [G1][G2]:** Starting 2026-08-31, new Play apps and updates must target Android 16/API 36 or higher. Existing apps targeting API 34 or lower become unavailable to new users on devices with newer Android versions; existing-app visibility requires API 35 or higher. Google documents an extension path to 2026-11-01, but it is an account action, not a design assumption.
- **RECOMMENDATION:** Set Play `targetSdkVersion`/compile SDK to API 36 for the first Play release. Keep `minSdkVersion` at API 29 (Android 10) as the product floor from DEC-029. Re-run the Flutter/Rust/native compatibility audit before finalizing that floor.
- **VERIFIED_FACT [G13]:** Newly created personal Play developer accounts must run a closed test with at least 12 testers opted in continuously for 14 days before applying for production access. Internal testing is available without that gate; open testing requires production access.
- **VERIFIED_FACT [G14][G15]:** Google is introducing developer identity and package-name registration. Play packages must be registered by 2026-09-30; Android's initial enforcement begins in Brazil, Indonesia, Singapore, and Thailand, with broader rollout planned for 2027. This is owner/account work and applies to Play package identity and, where applicable, outside-Play package registration.
- **WATCH:** Android 16 makes local-network protection an opt-in migration feature; Android 17/API 37 is documented as requiring `ACCESS_LOCAL_NETWORK` for broad local-network access or a user-mediated NSD picker path [G7][G8]. A target-API-36 release must still test the Android 17 behavior before claiming a long-lived network-bridge contract.

### GitHub Releases and provenance

- **VERIFIED_FACT [H1]:** A GitHub release is a package around a Git tag, release notes, and binary assets. Write access is required to create/manage releases; assets are downloadable files, not an Android trust anchor.
- **VERIFIED_FACT [H2]:** GitHub artifact attestations use signed provenance (repository, workflow, environment, commit SHA, and event) and can include an SBOM. Public repositories use GitHub's Sigstore public-good instance; consumers must verify attestations for them to provide value.
- **VERIFIED_FACT [H3]:** GitHub can mark GPG-, SSH-, or S/MIME-signed commits/tags as verified. A verified Git tag is not the same as an APK signature or an attestation for the APK bytes.
- **RECOMMENDATION:** For each GitHub APK release publish: (a) APK signed with the project Android release key, (b) `SHA256SUMS` signed or covered by an attestation, (c) signed version tag, (d) GitHub artifact attestation for the APK and SBOM, and (e) public certificate fingerprint plus `apksigner verify`/`gh attestation verify` instructions. Never store private signing keys or tokens in the repository.

### F-Droid inclusion, flavors, and signing

- **VERIFIED_FACT [F1][F2]:** F-Droid's quick-start requirements include a public source repository, a recognized FOSS license, only FOSS dependencies/build tools (Firebase and GMS are called out as examples of non-FOSS libraries), author notification/permission, metadata, tagged releases, and an isolated build that produces a functional APK. Inclusion is reviewed by F-Droid maintainers and is not automatic.
- **VERIFIED_FACT [F3]:** F-Droid labels undesirable behavior with Anti-Features. Tracking includes sending crash reports or checking for updates without user knowledge; an opt-in, disabled-by-default feature can avoid the Tracking flag when it has informed consent and minimizes PII. Non-free dependencies, non-free assets, non-free/tethered network services, and unsafe signing are separately labeled.
- **VERIFIED_FACT [F4][F5]:** Reproducible builds are described as best practice rather than an absolute inclusion requirement, but changing to a different signing key later can force users to reinstall. F-Droid supports publishing upstream developer-signed APKs when the APK matches the F-Droid build recipe, or F-Droid-signed APKs under its repository key model.
- **RECOMMENDATION:** The `fdroid` flavor should be built from source in an offline/isolated build and should contain no GMS/Firebase, proprietary SDK, telemetry/crash transport, update checker, AI/provider client, or profile-update endpoint. Keep the local editor, local files, and local USB/BLE/MIDI transports available where the Android platform permits them.
- **BLOCKER:** DEC-060 requires the F-Droid flavor to have no network permission until a user enables profile updates or AI. Android's `INTERNET` permission is declared in the manifest and is not a user-toggleable runtime grant [G6]. A single APK cannot start with no `INTERNET` permission and add it later when the user enables AI/profile updates. To honor the existing decision, choose one of: (1) a separate offline `fdroid` package/flavor with no `INTERNET` declaration and no network AI/profile-update feature; (2) a separate network-enabled package with an explicit consent flow; or (3) a reviewed amendment to the “no network permission” decision. Do not claim option (1) while shipping one manifest for all channels.
- **UNKNOWN:** Whether F-Droid maintainers would accept a network-enabled flavor with a disabled-by-default opt-in, and which Anti-Feature labels they would apply. Ask maintainers only through the normal inclusion review after the flavor/build recipe exists; do not treat a local build as inclusion approval.

## Permissions and entitlements by capability

The table lists the minimum expected declarations. It intentionally distinguishes a platform requirement from a proposed implementation. A row marked “conditional” must not be added until that capability is implemented and tested.

| Capability | iOS/iPadOS | Android | F-Droid / privacy note |
| --- | --- | --- | --- |
| Core MIDI / class-compliant USB MIDI | Core MIDI provides hardware, dock-connector/network, Bluetooth MIDI, and MIDI-network APIs [A7]. No separate generic “MIDI permission” is established by that page. Direct adapter compatibility and any MFi limitations remain physical-test questions. | USB host requires `<uses-feature android:name="android.hardware.usb.host"/>`; Android asks the user for access to each attached device [G4]. USB host is hardware-dependent even though the API exists from API 12. | Local transport can remain in the offline flavor. Do not log serials/descriptors or send them to telemetry by default. |
| BLE / Bluetooth MIDI | If the implementation uses Core Bluetooth, include `NSBluetoothAlwaysUsageDescription`; iOS 12-and-earlier compatibility would require the deprecated peripheral key [A8][A9]. Add `UIBackgroundModes=bluetooth-central` only for a tested background-central workflow; background execution is constrained and not indefinite [A8]. Core MIDI BLE MIDI support does not, by itself, prove that direct Core Bluetooth usage is needed. | For target API 31+, request only `BLUETOOTH_SCAN` and/or `BLUETOOTH_CONNECT` needed by the feature; `BLUETOOTH_ADVERTISE` is only for advertising. These are runtime Nearby devices permissions. For API <=30 scans, legacy Bluetooth and location permissions may be required. `neverForLocation` is valid only when the app never derives location; it can filter some beacons [G3]. | Treat device names/addresses/serials as potentially identifying. Keep BLE local by default; request permission in context and preserve an offline/no-BLE path. |
| User files / preset import-export | Use `UIDocumentPickerViewController` and security-scoped URLs/bookmarks for external documents; do not assume a broad filesystem entitlement [A12]. App Review asks file-capable apps to include Files/iCloud documents where appropriate [A3]. | Storage Access Framework (`ACTION_OPEN_DOCUMENT`, `ACTION_CREATE_DOCUMENT`, `ACTION_OPEN_DOCUMENT_TREE`) gives access to user-selected URIs without a storage permission [G5]. Do not request `MANAGE_EXTERNAL_STORAGE`; Play treats broad file access as high-risk and review-gated [G10]. | Files stay local unless the user explicitly previews and approves AI/provider transfer. Sanitization must remove paths, serials, names, and secrets from logs/captures. |
| Local network bridge | Any local-network use needs `NSLocalNetworkUsageDescription`; Bonjour discovery needs `NSBonjourServices`. Multicast/broadcast needs `com.apple.developer.networking.multicast`; ordinary outgoing TCP/UDP unicast to a local address still triggers local-network privacy [A10][A11]. No Network Extension entitlement is assumed. | `INTERNET` is a normal manifest permission for sockets [G6]. Wi-Fi management/discovery APIs on target API 33+ may require runtime `NEARBY_WIFI_DEVICES`; selected older scan APIs need `ACCESS_FINE_LOCATION` [G6]. Android 17 broad local-network traffic is planned to require `ACCESS_LOCAL_NETWORK` or a picker path [G7][G8]. | This is the main F-Droid flavor split. A self-hostable, user-addressed bridge is preferable to a mandatory proprietary service. No cloud/account dependency is implied. |
| Optional telemetry/crash reports | App Store Connect privacy details must include app and third-party data practices; a privacy policy URL is required for iOS [A4][A13]. Privacy manifests must be valid and required-reason APIs declared where used [A5][A6]. | Play Data Safety is required for published apps, including testing tracks other than internal-only, and covers third-party SDK collection/sharing [G9]. | Topology policy requires explicit opt-in, local sanitized logs by default, and inspect/export/delete controls. The F-Droid flavor must leave telemetry transport out of the binary; an opt-in network client would need a separately reviewed flavor. |
| Optional AI/provider requests | Apple guideline 5.1.2 requires clear disclosure of third-party sharing, including third-party AI, and explicit permission before sharing personal data [A3]. | Play User Data/permissions policy requires data access to be necessary, incremental, consented, and disclosed; if AI generates content, Play's AI policy and reporting/safety requirements also apply [G10][G11]. | Show the exact fields/bytes leaving the device, strip identifiers, require provider and cost confirmation, and keep AI out of the offline F-Droid flavor. BYOK secrets belong in OS secure storage and never in logs or prompts unless the user explicitly includes them. |

### Permissions that are specifically not assumed

- No account/login permission or mandatory cloud service: normal editor use remains offline and account-free.
- No contacts, camera, microphone, photos, location, notification, or background-location permission unless a separately approved feature actually needs it.
- No `MANAGE_EXTERNAL_STORAGE`, no Android accessibility service, no VPN, no Network Extension, and no iOS multicast entitlement unless the implementation and review packet demonstrate the exact use.
- No claim that a class-compliant USB cable, BLE adapter, or network bridge works on every device; each is a transport/adapter matrix item.
- No downloaded executable packs, self-update code, or runtime native libraries. Google Play prohibits self-updates outside Play and downloading executable code from outside Play [G16]. Topology packs remain signed declarative data.

## Signing, identity, and owner-controlled tasks

These actions require the project/account owner or an explicitly delegated account administrator. This worker did not perform them.

| Surface | Owner-controlled task | Evidence to retain | Credential boundary |
| --- | --- | --- | --- |
| Apple Developer / App Store Connect | Enroll and accept agreements; choose legal developer identity and bundle ID; create App ID, distribution certificate, provisioning profile, and only needed capabilities; create App Store record; provide privacy-policy URL; configure TestFlight groups; submit builds/review metadata. Apple says only the Account Holder can sign legal agreements, renew membership, request API access, and create Developer ID certificates; distribution certificates are sensitive [A13][A14]. | Team ID, bundle ID, certificate/profile fingerprints and expiry dates (not private keys), Xcode version/SDK, App Store/TestFlight build IDs, review notes. | Account Holder/Admin keeps Apple credentials, 2FA, certificates, profiles, and App Store Connect API keys. Never commit `.p12`, `.mobileprovision`, JWT private keys, or passwords. |
| Google Play / Android | Create and verify Play developer account; complete package-name registration; enroll in Play App Signing; generate/store upload key; provide privacy policy and Data Safety form; create internal/closed/open tracks; satisfy personal-account closed test; apply for production; complete developer verification by the deadline. Play App Signing separates the developer upload key from Google's app-signing key [G12]. | Package name, signing certificate SHA-256, upload-key rotation record, Play artifact IDs, target API, test cohort evidence, Data Safety form revision. | Account owner controls legal identity, payments/D-U-N-S or government ID, Play Console roles, signing keys, and package registration. Never put `.jks`/keystore passwords or service-account keys in source. |
| GitHub | Choose repository owner/organization; protect release branch/tags; enable/require signed tags as appropriate; configure Actions permissions for `id-token` and `attestations`; create immutable/draft-then-publish releases; publish APK, checksums, SBOM, and attestation. | Release URL, tag/object ID, asset SHA-256, attestation bundle, workflow run ID, public signing certificate. | Owner/admin controls repository tokens, Actions secrets, release signing key, and branch protection. A GitHub token is not an APK signing key. |
| F-Droid | Authorize inclusion proposal/MR; choose upstream-signed reproducible versus F-Droid-signed model; approve package ID, metadata, Anti-Features, build recipe, and update policy; provide public signing certificate and source tags. | `fdroiddata` metadata revision, build log, reproducibility result, signing certificate fingerprint, inclusion decision. | Keep private Android release/F-Droid keys offline or in owner-controlled signing infrastructure. Inclusion is maintainer-reviewed, not self-certified. |

## Privacy and data-safety disclosure inventory

This inventory is a release-input checklist, not a completed store declaration. Store forms describe the union of practices across versions/flavors where the store requires a package-level declaration.

| Data/operation | What can leave the device | Required disclosure/control | Default for Topology |
| --- | --- | --- | --- |
| Telemetry/crash | Crash stack, app/device/OS/build metadata, optional logs, endpoint names, user-entered context if accidentally included | Explain collection, purpose, retention, linkage, third-party processor, opt-in/out, export/delete. Apple App Privacy details include third-party SDKs [A4]; Play Data Safety covers app and SDKs and is developer-responsibility [G9]. | No mandatory telemetry; local sanitized logs; crash reporting explicit opt-in; no preset/AI content by default. |
| AI | User-selected preset/SysEx fields, names/notes, parameter graph, reference-audio features, provider account/BYOK request metadata | Preview exact payload, strip identifiers/secrets, name provider and purpose, obtain explicit consent for third-party AI (Apple 5.1.2 [A3]), cap cost, make provider/network state visible, and document retention/deletion. | Disabled in F-Droid offline flavor; opt-in BYOK/provider path only in network-enabled flavor. |
| BLE | Nearby-device identifiers, names, service data, connection timing; location inference risk on older Android | Request platform permissions in context; do not claim “location-free” unless `neverForLocation` assertion is true; do not transmit identifiers to telemetry without separate disclosure. | Local transport only; no background scan unless a reviewed workflow requires it. |
| USB | Device descriptors, vendor/product IDs, serials, permission grants, raw MIDI/SysEx | Explain USB/MIDI use; ask Android per-device consent; redact serials and unrelated SysEx from logs and fixtures; never upload raw captures by default. | Local transport only; user-visible read/write confirmation and safe recovery. |
| Files | Preset bytes, paths, names, cloud-provider URIs, metadata, imported vendor/user content | Use OS pickers; disclose if file bytes or metadata are sent to AI/telemetry; preserve local-only behavior when declined. Apple requires purpose strings for protected data [A3]; Android SAF avoids broad storage permission [G5]. | Local-first import/export. No broad storage access. |
| Network bridge | Local IP/port, Bonjour/mDNS service name, bridge/device identifiers, protocol traffic; optionally provider traffic if bridge is remote | Explain local-network access and endpoint purpose. On Apple include local-network/Bonjour declarations as needed [A10][A11]. On Android account for `INTERNET`, nearby Wi-Fi, and Android 17 local-network changes [G6][G7]. | Optional, user-started, self-hostable bridge; no mandatory cloud or remote telemetry. |

**Important store-form rule [A4][G9]:** “No data collected” is only correct for a specific artifact/practice when all off-device collection is absent. A local-only file or BLE operation is not Play “collection,” but an opt-in AI request or crash SDK may be. A single Play package's Data Safety declaration covers the collection/sharing practices of versions distributed through that package; flavors/package IDs need an explicit declaration strategy.

## Prohibited assumptions and uncertainty ledger

| ID | Classification | Constraint or uncertainty | Consequence |
| --- | --- | --- | --- |
| U-01 | **PROHIBITED_ASSUMPTION** | A store target SDK is the same as a minimum supported OS. | Keep `targetSdkVersion=36` separate from `minSdkVersion=29`; keep Apple SDK 26 separate from iOS deployment target 16. |
| U-02 | **PROHIBITED_ASSUMPTION** | Xcode/Core MIDI documentation alone proves direct USB, BLE MIDI, or Fractal adapter compatibility. | Require native-platform and physical-device matrices; no entitlement or hardware claim before evidence. |
| U-03 | **PROHIBITED_ASSUMPTION** | `BLUETOOTH_SCAN` always avoids location permission. | API <=30 scanning has location implications; `neverForLocation` is an assertion with filtering tradeoffs [G3]. |
| U-04 | **PROHIBITED_ASSUMPTION** | User-selected Android files require all-files/storage permission. | Use SAF; `MANAGE_EXTERNAL_STORAGE` is restricted and review-gated [G5][G10]. |
| U-05 | **PROHIBITED_ASSUMPTION** | `INTERNET` can be granted later after an in-app opt-in. | Manifest permissions are static for this purpose; split the F-Droid offline artifact or amend the decision. |
| U-06 | **PROHIBITED_ASSUMPTION** | A verified GitHub tag proves the APK is authentic. | Verify APK signature, hash, and artifact attestation separately [H1][H2][H3]. |
| U-07 | **PROHIBITED_ASSUMPTION** | F-Droid inclusion follows automatically from MIT source. | Maintainers review dependencies, assets, build isolation, Anti-Features, metadata, and update process [F1][F2]. |
| U-08 | **PROHIBITED_ASSUMPTION** | Play package IDs and signing keys can be chosen after public release. | Package registration and signature continuity are release architecture; decide before first public APK [G12][G14]. |
| U-09 | **HYPOTHESIS** | Android API 29 is the best product floor. | It is the existing project baseline and leaves USB/BLE API room, but Flutter/native toolchain, device usage, and test cost must validate it. |
| U-10 | **WATCH** | Android 17/API 37 broad local-network permission will affect bridge discovery/connection. | Design a picker/user-addressed path now and run API-37 tests before claiming long-term bridge support [G7][G8]. |
| U-11 | **UNKNOWN** | Exact Apple entitlement/review posture for each direct USB adapter and background MIDI workflow. | Keep capabilities conditional; test on physical iOS/iPadOS hardware and document App Review notes. |
| U-12 | **LEGAL_BOUNDARY** | Privacy policy wording, AI provider terms, regional data law, and trademark/package-name clearance. | Engineering can inventory flows; owner/counsel must approve legal text and regional availability. |

## ADR and follow-up packet inputs

These are proposed decisions; none are implemented by this research packet.

1. **ADR-DIST-001 — Supported OS and SDK matrix:** ratify iOS/iPadOS 16.0 and Android API 29 minimums; pin Xcode 26 and target API 36; define annual review and end-of-support policy.
2. **ADR-DIST-002 — Android package and signing topology:** decide whether Play, GitHub, and F-Droid share an application ID; define update/migration behavior and certificate rotation before first public APK.
3. **ADR-DIST-003 — F-Droid offline flavor:** resolve the static `INTERNET` permission conflict. Preferred design is a separately identified offline flavor with no network feature code/permission, plus a network-enabled flavor for Play/GitHub.
4. **ADR-DIST-004 — Release provenance:** require Android APK signatures, SHA-256 manifests, signed tags, GitHub artifact/SBOM attestations, and reproducibility evidence before each release.
5. **ADR-DIST-005 — Store privacy/data inventory:** maintain one versioned data-flow inventory feeding Apple Privacy Nutrition Labels, `PrivacyInfo.xcprivacy`, Play Data Safety, in-app disclosures, and the public privacy policy.
6. **ADR-DIST-006 — Local-network bridge:** choose explicit endpoint entry versus Bonjour/NSD discovery; include Apple local-network declarations, Android API-36 test coverage, and Android API-37 migration tests.
7. **ADR-DIST-007 — Native transport capability gates:** map Core MIDI/BLE/USB entitlements, Android runtime permissions, adapters, hardware, and background behavior to physical test packets.
8. **Proposed implementation packet (new, not in current index):** build and validate the `fdroid` flavor from a clean clone using fdroidserver metadata, no network at build/runtime, and reproducibility/signing evidence.
9. **Proposed release packet (new, not in current index):** execute owner-supplied Apple/Google/GitHub signing and store test tracks; no agent should receive private credentials.

## Blockers

- **BLOCKED_ENVIRONMENT:** this extracted starter kit is not a Git worktree; Flutter/Dart, Java/JDK, and Gradle are absent in the current environment. No store or native build claim is made.
- **BLOCKED_DECISION:** the existing F-Droid requirement cannot be represented by one Android APK that adds `INTERNET` only after user opt-in. An owner-approved flavor/package ADR is required.
- **BLOCKED_OWNER_ACTION:** Apple Developer/App Store Connect enrollment, Google Play identity/package registration and signing, GitHub repository/release permissions, and any F-Droid inclusion/signing decision require the account owner. This packet deliberately does not request or handle credentials.
- **BLOCKED_LEGAL_REVIEW:** privacy-policy/AI-provider/region-specific legal language may require counsel. This report records engineering disclosures and does not give a legal conclusion.

## Source register (official sources, accessed 2026-08-08)

### Apple

- **[A1]** Apple Developer, “Submitting your apps and games today” — <https://developer.apple.com/app-store/submitting/>.
- **[A2]** Apple Developer, “SDKs and system requirements — Xcode” — <https://developer.apple.com/xcode/system-requirements>.
- **[A3]** Apple Developer, “App Review Guidelines” — <https://developer.apple.com/app-store/review/guidelines/>.
- **[A4]** Apple Developer, “App Privacy Details” — <https://developer.apple.com/app-store/app-privacy-details/>.
- **[A5]** Apple Developer, “Adding a privacy manifest to your app or third-party SDK” — <https://developer.apple.com/documentation/bundleresources/adding-a-privacy-manifest-to-your-app-or-third-party-sdk>.
- **[A6]** Apple Developer, “Describing use of required reason API” — <https://developer.apple.com/documentation/bundleresources/describing-use-of-required-reason-api>.
- **[A7]** Apple Developer, “Core MIDI” — <https://developer.apple.com/documentation/coremidi>.
- **[A8]** Apple Developer, “Core Bluetooth” — <https://developer.apple.com/documentation/CoreBluetooth>.
- **[A9]** Apple Developer, “NSBluetoothAlwaysUsageDescription” — <https://developer.apple.com/documentation/bundleresources/information-property-list/nsbluetoothalwaysusagedescription>.
- **[A10]** Apple Developer, “TN3179: Understanding local network privacy” — <https://developer.apple.com/documentation/technotes/tn3179-understanding-local-network-privacy>.
- **[A11]** Apple Developer, “NSLocalNetworkUsageDescription” — <https://developer.apple.com/documentation/bundleresources/information-property-list/nslocalnetworkusagedescription>.
- **[A12]** Apple Developer, “UIDocumentPickerViewController” — <https://developer.apple.com/documentation/uikit/uidocumentpickerviewcontroller>.
- **[A13]** Apple Developer, “Overview of accounts and roles” — <https://developer.apple.com/help/app-store-connect/manage-your-team/overview-of-accounts-and-roles/>.
- **[A14]** Apple Developer, “Certificates overview” — <https://developer.apple.com/help/account/certificates/certificates-overview>.
- **[A15]** Apple Developer, “Upload builds” — <https://developer.apple.com/help/app-store-connect/manage-builds/upload-builds/>.
- **[A16]** Apple Developer, “TestFlight” — <https://developer.apple.com/testflight/>.
- **[A17]** Apple Developer, “App build statuses” — <https://developer.apple.com/help/app-store-connect/reference/app-build-statuses/>.

### Android and Google Play

- **[G1]** Android Developers, “Meet Google Play's target API level requirement” — <https://developer.android.com/google/play/requirements/target-sdk>.
- **[G2]** Google Play Console Help, “Google Play's Target API Level Policy” — <https://support.google.com/googleplay/android-developer/answer/16561298>.
- **[G3]** Android Developers, “Bluetooth permissions” — <https://developer.android.com/develop/connectivity/bluetooth/bt-permissions>.
- **[G4]** Android Developers, “USB host overview” — <https://developer.android.com/develop/connectivity/usb/host>.
- **[G5]** Android Developers, “Access documents and other files from shared storage” — <https://developer.android.com/training/data-storage/shared/documents-files>.
- **[G6]** Android Developers, “Manifest.permission” and “Request permission to access nearby Wi-Fi devices” — <https://developer.android.com/reference/android/Manifest.permission.html> and <https://developer.android.com/develop/connectivity/wifi/wifi-permissions>.
- **[G7]** Android Developers, “Local network permission” — <https://developer.android.com/privacy-and-security/local-network-permission>.
- **[G8]** Android Developers, “NsdManager” — <https://developer.android.com/reference/android/net/nsd/NsdManager>.
- **[G9]** Google Play Console Help, “Provide information for Google Play's Data safety section” — <https://support.google.com/googleplay/android-developer/answer/10787469>.
- **[G10]** Google Play Console Help, “Permissions and APIs that Access Sensitive Information” — <https://support.google.com/googleplay/android-developer/answer/16558241>.
- **[G11]** Google Play Console Help, “Understanding Google Play's AI-Generated Content policy” — <https://support.google.com/googleplay/android-developer/answer/14094294>.
- **[G12]** Google Play Console Help, “Use Play App Signing” — <https://support.google.com/googleplay/android-developer/answer/9842756>.
- **[G13]** Google Play Console Help, “App testing requirements for new personal developer accounts” — <https://support.google.com/googleplay/android-developer/answer/14151465>.
- **[G14]** Google Play Console Help, “Registering Play package names” — <https://support.google.com/googleplay/android-developer/answer/16984799>.
- **[G15]** Android Developers, “Register on Google Play Console — Android developer verification” — <https://developer.android.com/developer-verification/guides/google-play-console>.
- **[G16]** Google Play Console Help, “Device and Network Abuse” — <https://support.google.com/googleplay/android-developer/answer/16559646>.

### F-Droid

- **[F1]** F-Droid, “Inclusion Policy” — <https://f-droid.org/en/docs/Inclusion_Policy/>.
- **[F2]** F-Droid, “Submitting to F-Droid Quick Start Guide” — <https://f-droid.org/en/docs/Submitting_to_F-Droid_Quick_Start_Guide/>.
- **[F3]** F-Droid, “Anti-Features” — <https://f-droid.org/en/docs/Anti-Features/>.
- **[F4]** F-Droid, “Reproducible Builds” — <https://f-droid.org/en/docs/Reproducible_Builds/>.
- **[F5]** F-Droid, “Release Channels and Signing Keys” — <https://f-droid.org/en/docs/Release_Channels_and_Signing_Keys/>.

### GitHub

- **[H1]** GitHub Docs, “About releases” — <https://docs.github.com/en/repositories/releasing-projects-on-github/about-releases>.
- **[H2]** GitHub Docs, “Artifact attestations” and “Using artifact attestations to establish provenance for builds” — <https://docs.github.com/en/actions/concepts/security/artifact-attestations> and <https://docs.github.com/en/actions/how-tos/secure-your-work/use-artifact-attestations/use-artifact-attestations>.
- **[H3]** GitHub Docs, “About commit signature verification” — <https://docs.github.com/en/authentication/managing-commit-signature-verification/about-commit-signature-verification>.

## Claims available and unavailable

**Available after this research:** current official policy/source map; proposed iOS/Android floors and SDK targets; conditional permission/entitlement inventory; F-Droid flavor constraints; owner credential boundary; privacy/data-safety checklist; release provenance recommendations; explicit ADR inputs and blockers.

**Unavailable:** store approval, TestFlight acceptance, Play production access, F-Droid inclusion, legal clearance, package-name/trademark clearance, signing-key ownership, physical iOS/Android/BLE/USB/network behavior, reproducibility of a Topology build, and any hardware compatibility claim.
