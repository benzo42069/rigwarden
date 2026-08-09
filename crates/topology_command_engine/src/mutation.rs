use topology_device_registry::{DeviceProfile, VerificationStatus};

/// One requested numeric edit in the profile's literal stored units.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParameterMutationRequest {
    block_id: String,
    parameter_id: String,
    stored_value: i32,
}

impl ParameterMutationRequest {
    /// Construct a semantic request without encoding it for any transport.
    pub fn new(
        block_id: impl Into<String>,
        parameter_id: impl Into<String>,
        stored_value: i32,
    ) -> Self {
        Self {
            block_id: block_id.into(),
            parameter_id: parameter_id.into(),
            stored_value,
        }
    }
}

/// A validated local mutation; it has no protocol or transport representation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedParameterMutation {
    block_id: String,
    parameter_id: String,
    stored_value: i32,
    decimal_places: u8,
}

impl ValidatedParameterMutation {
    /// Return the profile block identifier selected by this mutation.
    pub fn block_id(&self) -> &str {
        &self.block_id
    }

    /// Return the profile parameter identifier selected by this mutation.
    pub fn parameter_id(&self) -> &str {
        &self.parameter_id
    }

    /// Return the accepted value in the profile's stored integer units.
    pub const fn stored_value(&self) -> i32 {
        self.stored_value
    }

    /// Return the profile-owned decimal precision used to present the value.
    pub const fn decimal_places(&self) -> u8 {
        self.decimal_places
    }
}

/// Why an in-memory semantic mutation cannot be accepted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MutationValidationError {
    /// The selected profile has no explicit write capability.
    ProfileNotWritable,
    /// The selected session is read-only because its firmware is unknown.
    ReadOnly { firmware: String },
    /// The exact block/parameter pair has no numeric declaration on the profile.
    ParameterNotDeclared {
        block_id: String,
        parameter_id: String,
    },
    /// The requested literal stored value is outside the inclusive profile range.
    OutOfRange {
        requested_stored: i32,
        min_stored: i32,
        max_stored: i32,
    },
}

/// Validate one local numeric mutation against an exact writable profile.
pub fn validate_parameter_mutation(
    profile: &DeviceProfile,
    request: ParameterMutationRequest,
) -> Result<ValidatedParameterMutation, MutationValidationError> {
    if profile.verification_status() == VerificationStatus::ReadOnly {
        return Err(MutationValidationError::ReadOnly {
            firmware: profile.firmware().as_str().to_owned(),
        });
    }

    if !profile.capabilities().can_write() {
        return Err(MutationValidationError::ProfileNotWritable);
    }

    let metadata = profile
        .numeric_parameter(&request.block_id, &request.parameter_id)
        .ok_or_else(|| MutationValidationError::ParameterNotDeclared {
            block_id: request.block_id.clone(),
            parameter_id: request.parameter_id.clone(),
        })?;

    if request.stored_value < metadata.min_stored() || request.stored_value > metadata.max_stored()
    {
        return Err(MutationValidationError::OutOfRange {
            requested_stored: request.stored_value,
            min_stored: metadata.min_stored(),
            max_stored: metadata.max_stored(),
        });
    }

    Ok(ValidatedParameterMutation {
        block_id: request.block_id,
        parameter_id: request.parameter_id,
        stored_value: request.stored_value,
        decimal_places: metadata.decimal_places(),
    })
}
