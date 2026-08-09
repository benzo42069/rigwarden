use std::fmt;

/// A validated, stable identifier for a device family.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DeviceFamilyId(String);

/// Construction errors for [`DeviceFamilyId`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeviceFamilyIdError {
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
