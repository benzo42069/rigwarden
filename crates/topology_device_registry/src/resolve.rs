use topology_domain::{DeviceFamilyId, DeviceIdentity, DeviceModelId, FirmwareId};

/// An exact numeric parameter definition owned by a device profile.
///
/// Values are represented in the profile's literal stored integer units. The
/// decimal precision is metadata for presentation/entry and never requires a
/// floating-point representation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NumericParameterMetadata {
    min_stored: i32,
    max_stored: i32,
    decimal_places: u8,
}

impl NumericParameterMetadata {
    /// Construct a numeric definition from literal stored bounds and precision.
    pub const fn new(min_stored: i32, max_stored: i32, decimal_places: u8) -> Self {
        Self {
            min_stored,
            max_stored,
            decimal_places,
        }
    }

    /// Return the inclusive minimum in the profile's stored integer units.
    pub const fn min_stored(self) -> i32 {
        self.min_stored
    }

    /// Return the inclusive maximum in the profile's stored integer units.
    pub const fn max_stored(self) -> i32 {
        self.max_stored
    }

    /// Return the number of decimal places used to present the stored value.
    pub const fn decimal_places(self) -> u8 {
        self.decimal_places
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NumericParameterDefinition {
    block_id: String,
    parameter_id: String,
    metadata: NumericParameterMetadata,
}

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
    numeric_parameters: Vec<NumericParameterDefinition>,
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
            numeric_parameters: Vec::new(),
        }
    }

    /// Add or replace one profile-owned numeric parameter definition.
    pub fn add_numeric_parameter(
        &mut self,
        block_id: impl Into<String>,
        parameter_id: impl Into<String>,
        metadata: NumericParameterMetadata,
    ) {
        let block_id = block_id.into();
        let parameter_id = parameter_id.into();

        if let Some(existing) = self.numeric_parameters.iter_mut().find(|definition| {
            definition.block_id == block_id && definition.parameter_id == parameter_id
        }) {
            existing.metadata = metadata;
            return;
        }

        self.numeric_parameters.push(NumericParameterDefinition {
            block_id,
            parameter_id,
            metadata,
        });
    }

    /// Add or replace a numeric parameter definition using builder-style chaining.
    pub fn with_numeric_parameter(
        mut self,
        block_id: impl Into<String>,
        parameter_id: impl Into<String>,
        metadata: NumericParameterMetadata,
    ) -> Self {
        self.add_numeric_parameter(block_id, parameter_id, metadata);
        self
    }

    /// Return an exact numeric definition owned by this profile, if present.
    pub fn numeric_parameter(
        &self,
        block_id: &str,
        parameter_id: &str,
    ) -> Option<&NumericParameterMetadata> {
        self.numeric_parameters
            .iter()
            .find(|definition| {
                definition.block_id == block_id && definition.parameter_id == parameter_id
            })
            .map(|definition| &definition.metadata)
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

    fn matches_device(&self, identity: &DeviceIdentity) -> bool {
        self.family == *identity.family() && self.model == *identity.model()
    }
}

/// Why a profile was selected for a session.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResolutionProvenance {
    /// All identity fields matched one profile exactly.
    ExactProfile,
    /// The family and model are known, but the observed firmware is unsupported.
    UnknownFirmware,
}

/// Machine-readable resolution status.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResolutionStatus {
    /// The returned profile matched family, model, and firmware exactly.
    ExactMatch,
    /// The family and model are known, but the observed firmware is unsupported.
    UnknownFirmware,
}

/// A profile together with the capability and provenance of its resolution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedProfile {
    profile: DeviceProfile,
    provenance: ResolutionProvenance,
    status: ResolutionStatus,
    unsupported_firmware: Option<FirmwareId>,
}

impl ResolvedProfile {
    fn exact(profile: DeviceProfile) -> Self {
        Self {
            profile,
            provenance: ResolutionProvenance::ExactProfile,
            status: ResolutionStatus::ExactMatch,
            unsupported_firmware: None,
        }
    }

    fn unknown_firmware(profile: DeviceProfile) -> Self {
        let unsupported_firmware = profile.firmware().clone();

        Self {
            profile,
            provenance: ResolutionProvenance::UnknownFirmware,
            status: ResolutionStatus::UnknownFirmware,
            unsupported_firmware: Some(unsupported_firmware),
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

    /// Return the observed firmware when resolution is read-only because it is unsupported.
    pub fn unsupported_firmware(&self) -> Option<&FirmwareId> {
        self.unsupported_firmware.as_ref()
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

    /// Resolve a known device into either an exact profile or an explicit
    /// read-only result for an unsupported firmware.
    pub fn resolve_session(&self, identity: &DeviceIdentity) -> Option<ResolvedProfile> {
        if let Some(exact) = self.resolve(identity) {
            return Some(exact);
        }

        self.profiles
            .iter()
            .any(|profile| profile.matches_device(identity))
            .then(|| {
                ResolvedProfile::unknown_firmware(DeviceProfile::new(
                    identity.family().clone(),
                    identity.model().clone(),
                    identity.firmware().clone(),
                    SessionCapabilities::new(false),
                    VerificationStatus::ReadOnly,
                ))
            })
    }
}
