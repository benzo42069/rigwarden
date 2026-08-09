use topology_domain::{DeviceFamilyId, DeviceIdentity, DeviceModelId, FirmwareId};

/// The capabilities granted to a resolved device session.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionCapabilities {
    write: bool,
}

impl SessionCapabilities {
    /// Construct capabilities with an explicit write permission.
    pub const fn new(write: bool) -> Self {
        Self { write }
    }

    /// Return whether the resolved session may issue writes.
    pub const fn can_write(self) -> bool {
        self.write
    }
}

/// Verification status recorded by a device profile.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VerificationStatus {
    /// The profile is implemented but has not completed physical verification.
    Experimental,
    /// The profile is intentionally limited to read-only operations.
    ReadOnly,
    /// The profile is backed by lawful protocol captures.
    CaptureVerified,
    /// The profile has been confirmed by a community contributor.
    CommunityConfirmed,
    /// The profile has passed the declared physical hardware matrix.
    HardwareVerified,
    /// The profile is not supported for the identified device.
    Unsupported,
}

/// One exact device-family, model, and opaque-firmware profile.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeviceProfile {
    family: DeviceFamilyId,
    model: DeviceModelId,
    firmware: FirmwareId,
    capabilities: SessionCapabilities,
    verification_status: VerificationStatus,
}

impl DeviceProfile {
    /// Construct an in-memory typed profile with explicit capabilities/status.
    pub fn new(
        family: DeviceFamilyId,
        model: DeviceModelId,
        firmware: FirmwareId,
        capabilities: SessionCapabilities,
        verification_status: VerificationStatus,
    ) -> Self {
        Self {
            family,
            model,
            firmware,
            capabilities,
            verification_status,
        }
    }

    /// Return the profile's exact family identifier.
    pub fn family(&self) -> &DeviceFamilyId {
        &self.family
    }

    /// Return the profile's exact model identifier.
    pub fn model(&self) -> &DeviceModelId {
        &self.model
    }

    /// Return the profile's opaque firmware identifier.
    pub fn firmware(&self) -> &FirmwareId {
        &self.firmware
    }

    /// Return the capabilities explicitly declared by this profile.
    pub const fn capabilities(&self) -> SessionCapabilities {
        self.capabilities
    }

    /// Return the profile's recorded verification status.
    pub const fn verification_status(&self) -> VerificationStatus {
        self.verification_status
    }

    fn matches(&self, identity: &DeviceIdentity) -> bool {
        self.family == *identity.family()
            && self.model == *identity.model()
            && self.firmware == *identity.firmware()
    }
}

/// Why a profile was selected for a session.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResolutionProvenance {
    /// All identity fields matched one profile exactly.
    ExactProfile,
}

/// Machine-readable resolution status.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResolutionStatus {
    /// The returned profile matched family, model, and firmware exactly.
    ExactMatch,
}

/// A profile together with the capability and provenance of its resolution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedProfile {
    profile: DeviceProfile,
    provenance: ResolutionProvenance,
    status: ResolutionStatus,
}

impl ResolvedProfile {
    fn exact(profile: DeviceProfile) -> Self {
        Self {
            profile,
            provenance: ResolutionProvenance::ExactProfile,
            status: ResolutionStatus::ExactMatch,
        }
    }

    /// Return the selected exact profile.
    pub fn profile(&self) -> &DeviceProfile {
        &self.profile
    }

    /// Return capabilities copied from the selected profile.
    pub const fn capabilities(&self) -> SessionCapabilities {
        self.profile.capabilities()
    }

    /// Return how the profile was selected.
    pub const fn provenance(&self) -> ResolutionProvenance {
        self.provenance
    }

    /// Return the machine-readable resolution status.
    pub const fn status(&self) -> ResolutionStatus {
        self.status
    }

    /// Return the selected profile's verification status.
    pub const fn verification_status(&self) -> VerificationStatus {
        self.profile.verification_status()
    }
}

/// An in-memory collection of typed device profiles.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DeviceRegistry {
    profiles: Vec<DeviceProfile>,
}

impl DeviceRegistry {
    /// Construct a registry from typed profiles.
    pub fn from_profiles(profiles: impl IntoIterator<Item = DeviceProfile>) -> Self {
        Self {
            profiles: profiles.into_iter().collect(),
        }
    }

    /// Resolve an identity only against an exact family, model, and firmware profile.
    pub fn resolve(&self, identity: &DeviceIdentity) -> Option<ResolvedProfile> {
        self.profiles
            .iter()
            .find(|profile| profile.matches(identity))
            .cloned()
            .map(ResolvedProfile::exact)
    }
}
