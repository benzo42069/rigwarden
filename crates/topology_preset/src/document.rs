//! Minimal normalized preset state for known metadata and opaque segments.

/// Known metadata carried by a normalized preset document.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PresetMetadata {
    name: String,
}

impl PresetMetadata {
    /// Create metadata with a known preset name.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    /// Return the known preset name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// An ordered segment whose contents are intentionally not interpreted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpaqueSegment {
    id: String,
    bytes: Vec<u8>,
}

impl OpaqueSegment {
    /// Create an opaque segment while retaining its identifier and bytes.
    pub fn new(id: impl Into<String>, bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            id: id.into(),
            bytes: bytes.into(),
        }
    }

    /// Return the stable segment identifier.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return the segment bytes without interpretation or conversion.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

/// A normalized preset document with known metadata beside ordered opaque data.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PresetDocument {
    metadata: PresetMetadata,
    opaque_segments: Vec<OpaqueSegment>,
}

impl PresetDocument {
    /// Create a document, retaining opaque segments in the supplied order.
    pub fn new(
        metadata: PresetMetadata,
        opaque_segments: impl IntoIterator<Item = OpaqueSegment>,
    ) -> Self {
        Self {
            metadata,
            opaque_segments: opaque_segments.into_iter().collect(),
        }
    }

    /// Return the known metadata.
    pub fn metadata(&self) -> &PresetMetadata {
        &self.metadata
    }

    /// Return opaque segments in their document order.
    pub fn opaque_segments(&self) -> &[OpaqueSegment] {
        &self.opaque_segments
    }

    /// Edit the known preset name without touching opaque segments.
    pub fn set_name(&mut self, name: impl Into<String>) {
        self.metadata.name = name.into();
    }
}
