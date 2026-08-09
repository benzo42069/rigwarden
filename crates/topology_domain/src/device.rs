use std::fmt;

/// A validated, stable identifier for a device family.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DeviceFamilyId(String);

/// A validated, stable identifier for a device model.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DeviceModelId(String);

/// A validated, stable identifier for a transport endpoint.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TransportEndpointId(String);

/// The identity of a device observed on a transport endpoint.
///
/// Identity is descriptive only. It does not grant a write capability or
/// imply that the endpoint has been opened.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DeviceIdentity {
    family: DeviceFamilyId,
    model: DeviceModelId,
    firmware: crate::FirmwareId,
    transport_endpoint: TransportEndpointId,
}

/// Construction errors for [`DeviceFamilyId`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeviceFamilyIdError {
    /// The supplied identifier contains no non-whitespace characters.
    BlankOrWhitespace,
}

/// Construction errors for [`DeviceModelId`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeviceModelIdError {
    /// The supplied identifier contains no non-whitespace characters.
    BlankOrWhitespace,
}

/// Construction errors for [`TransportEndpointId`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransportEndpointIdError {
    /// The supplied identifier contains no non-whitespace characters.
    BlankOrWhitespace,
}

impl DeviceFamilyId {
    /// Construct an identifier while preserving the supplied text exactly.
    pub fn new(value: impl Into<String>) -> Result<Self, DeviceFamilyIdError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(DeviceFamilyIdError::BlankOrWhitespace);
        }

        Ok(Self(value))
    }

    /// Return the identifier text without normalization.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for DeviceFamilyId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl DeviceModelId {
    /// Construct an identifier while preserving the supplied text exactly.
    pub fn new(value: impl Into<String>) -> Result<Self, DeviceModelIdError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(DeviceModelIdError::BlankOrWhitespace);
        }

        Ok(Self(value))
    }

    /// Return the identifier text without normalization.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for DeviceModelId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TransportEndpointId {
    /// Construct an identifier while preserving the supplied text exactly.
    pub fn new(value: impl Into<String>) -> Result<Self, TransportEndpointIdError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(TransportEndpointIdError::BlankOrWhitespace);
        }

        Ok(Self(value))
    }

    /// Return the identifier text without normalization.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for TransportEndpointId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl DeviceIdentity {
    /// Construct a descriptive identity from already validated typed values.
    pub fn new(
        family: DeviceFamilyId,
        model: DeviceModelId,
        firmware: crate::FirmwareId,
        transport_endpoint: TransportEndpointId,
    ) -> Self {
        Self {
            family,
            model,
            firmware,
            transport_endpoint,
        }
    }

    /// Return the device family identifier.
    pub fn family(&self) -> &DeviceFamilyId {
        &self.family
    }

    /// Return the device model identifier.
    pub fn model(&self) -> &DeviceModelId {
        &self.model
    }

    /// Return the firmware identifier.
    pub fn firmware(&self) -> &crate::FirmwareId {
        &self.firmware
    }

    /// Return the transport endpoint identifier separately from device IDs.
    pub fn transport_endpoint(&self) -> &TransportEndpointId {
        &self.transport_endpoint
    }
}

impl fmt::Display for DeviceFamilyIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BlankOrWhitespace => {
                formatter.write_str("device family identifier is blank or whitespace-only")
            }
        }
    }
}

impl std::error::Error for DeviceFamilyIdError {}

impl fmt::Display for DeviceModelIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BlankOrWhitespace => {
                formatter.write_str("device model identifier is blank or whitespace-only")
            }
        }
    }
}

impl std::error::Error for DeviceModelIdError {}

impl fmt::Display for TransportEndpointIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BlankOrWhitespace => {
                formatter.write_str("transport endpoint identifier is blank or whitespace-only")
            }
        }
    }
}

impl std::error::Error for TransportEndpointIdError {}
