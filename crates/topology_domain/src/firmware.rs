use std::fmt;

/// A validated firmware identifier whose vendor-defined text is otherwise opaque.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FirmwareId(String);

/// Construction errors for [`FirmwareId`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FirmwareIdError {
    /// The supplied identifier contains no non-whitespace characters.
    BlankOrWhitespace,
}

impl FirmwareId {
    /// Construct an identifier after removing only outer whitespace.
    pub fn new(value: impl Into<String>) -> Result<Self, FirmwareIdError> {
        let value = value.into();
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(FirmwareIdError::BlankOrWhitespace);
        }

        Ok(Self(trimmed.to_owned()))
    }

    /// Return the vendor-defined identifier text without semantic normalization.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for FirmwareId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for FirmwareId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Display for FirmwareIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BlankOrWhitespace => {
                formatter.write_str("firmware identifier is blank or whitespace-only")
            }
        }
    }
}

impl std::error::Error for FirmwareIdError {}
