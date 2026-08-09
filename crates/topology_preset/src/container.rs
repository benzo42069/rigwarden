//! Versioned, bounded bytes for the initial offline RigWarden container.
//!
//! This is an internal synthetic container for normalized test data. It is not
//! a vendor file or SysEx format and does not establish compatibility with any
//! physical device.

use std::fmt;

use crate::{OpaqueSegment, PresetDocument, PresetMetadata};

/// Magic prefix identifying an initial RigWarden offline container.
const MAGIC: &[u8; 4] = b"RWOC";

/// Explicit schema version for the initial internal container.
pub const CONTAINER_SCHEMA_VERSION: u16 = 1;

const MAX_STRING_BYTES: usize = 4 * 1024;
const MAX_OPAQUE_BYTES: usize = 1024 * 1024;
const MAX_ITEMS: usize = 1024;

/// Device metadata retained by an offline document without interpreting it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeviceMetadata {
    family: String,
    model: String,
    firmware: String,
    transport_endpoint: String,
}

impl DeviceMetadata {
    /// Construct metadata from the four descriptive identity fields.
    pub fn new(
        family: impl Into<String>,
        model: impl Into<String>,
        firmware: impl Into<String>,
        transport_endpoint: impl Into<String>,
    ) -> Self {
        Self {
            family: family.into(),
            model: model.into(),
            firmware: firmware.into(),
            transport_endpoint: transport_endpoint.into(),
        }
    }

    /// Return the family text exactly as stored.
    pub fn family(&self) -> &str {
        &self.family
    }

    /// Return the model text exactly as stored.
    pub fn model(&self) -> &str {
        &self.model
    }

    /// Return the firmware text exactly as stored.
    pub fn firmware(&self) -> &str {
        &self.firmware
    }

    /// Return the transport endpoint text exactly as stored.
    pub fn transport_endpoint(&self) -> &str {
        &self.transport_endpoint
    }
}

/// One directed graph edge retained as normalized endpoint identities.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphEdge {
    source_node: String,
    source_port: String,
    destination_node: String,
    destination_port: String,
}

impl GraphEdge {
    /// Construct one normalized graph edge.
    pub fn new(
        source_node: impl Into<String>,
        source_port: impl Into<String>,
        destination_node: impl Into<String>,
        destination_port: impl Into<String>,
    ) -> Self {
        Self {
            source_node: source_node.into(),
            source_port: source_port.into(),
            destination_node: destination_node.into(),
            destination_port: destination_port.into(),
        }
    }

    /// Return the source node identity.
    pub fn source_node(&self) -> &str {
        &self.source_node
    }

    /// Return the source port identity.
    pub fn source_port(&self) -> &str {
        &self.source_port
    }

    /// Return the destination node identity.
    pub fn destination_node(&self) -> &str {
        &self.destination_node
    }

    /// Return the destination port identity.
    pub fn destination_port(&self) -> &str {
        &self.destination_port
    }
}

/// One known normalized value retained by an offline document.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KnownValue {
    key: String,
    value: i64,
}

impl KnownValue {
    /// Construct a known value without assigning vendor semantics to it.
    pub fn new(key: impl Into<String>, value: i64) -> Self {
        Self {
            key: key.into(),
            value,
        }
    }

    /// Return the stable value key.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Return the integer value.
    pub const fn value(&self) -> i64 {
        self.value
    }
}

/// Initial offline container document.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OfflineContainer {
    schema_version: u16,
    device: DeviceMetadata,
    document: PresetDocument,
    graph_edges: Vec<GraphEdge>,
    known_values: Vec<KnownValue>,
}

impl OfflineContainer {
    /// Construct a container at the current explicit schema version.
    pub fn new(
        device: DeviceMetadata,
        document: PresetDocument,
        graph_edges: impl IntoIterator<Item = GraphEdge>,
        known_values: impl IntoIterator<Item = KnownValue>,
    ) -> Self {
        Self {
            schema_version: CONTAINER_SCHEMA_VERSION,
            device,
            document,
            graph_edges: graph_edges.into_iter().collect(),
            known_values: known_values.into_iter().collect(),
        }
    }

    /// Return the explicit schema version carried by this document.
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Return retained device metadata.
    pub fn device(&self) -> &DeviceMetadata {
        &self.device
    }

    /// Return the normalized document.
    pub fn document(&self) -> &PresetDocument {
        &self.document
    }

    /// Return graph edges in their document order.
    pub fn graph_edges(&self) -> &[GraphEdge] {
        &self.graph_edges
    }

    /// Return known values in their document order.
    pub fn known_values(&self) -> &[KnownValue] {
        &self.known_values
    }

    /// Serialize the internal container using bounded length-prefixed fields.
    pub fn serialize(&self) -> Result<Vec<u8>, ContainerError> {
        let mut writer = Writer::default();
        writer.bytes.extend_from_slice(MAGIC);
        writer.u16(self.schema_version);

        writer.string("device.family", self.device.family())?;
        writer.string("device.model", self.device.model())?;
        writer.string("device.firmware", self.device.firmware())?;
        writer.string(
            "device.transport_endpoint",
            self.device.transport_endpoint(),
        )?;
        writer.string("preset.name", self.document.metadata().name())?;

        writer.count("graph_edges", self.graph_edges.len())?;
        for edge in &self.graph_edges {
            writer.string("graph_edge.source_node", edge.source_node())?;
            writer.string("graph_edge.source_port", edge.source_port())?;
            writer.string("graph_edge.destination_node", edge.destination_node())?;
            writer.string("graph_edge.destination_port", edge.destination_port())?;
        }

        writer.count("known_values", self.known_values.len())?;
        for value in &self.known_values {
            writer.string("known_value.key", value.key())?;
            writer.i64(value.value());
        }

        writer.count("opaque_segments", self.document.opaque_segments().len())?;
        for segment in self.document.opaque_segments() {
            writer.string("opaque_segment.id", segment.id())?;
            writer.bytes_field("opaque_segment.bytes", segment.bytes())?;
        }

        Ok(writer.bytes)
    }

    /// Parse an internal container, rejecting malformed or truncated input.
    pub fn deserialize(input: &[u8]) -> Result<Self, ContainerError> {
        let mut reader = Reader::new(input);
        let magic = reader.take_exact(MAGIC.len(), "magic")?;
        if magic != MAGIC {
            let mut found = [0_u8; 4];
            found.copy_from_slice(magic);
            return Err(ContainerError::InvalidMagic { found });
        }

        let schema_version = reader.u16("schema_version")?;
        if schema_version != CONTAINER_SCHEMA_VERSION {
            return Err(ContainerError::UnsupportedSchemaVersion(schema_version));
        }

        let device = DeviceMetadata::new(
            reader.string("device.family")?,
            reader.string("device.model")?,
            reader.string("device.firmware")?,
            reader.string("device.transport_endpoint")?,
        );
        let preset_name = reader.string("preset.name")?;

        let edge_count = reader.count("graph_edges")?;
        let mut graph_edges = Vec::with_capacity(edge_count);
        for _ in 0..edge_count {
            graph_edges.push(GraphEdge::new(
                reader.string("graph_edge.source_node")?,
                reader.string("graph_edge.source_port")?,
                reader.string("graph_edge.destination_node")?,
                reader.string("graph_edge.destination_port")?,
            ));
        }

        let value_count = reader.count("known_values")?;
        let mut known_values = Vec::with_capacity(value_count);
        for _ in 0..value_count {
            known_values.push(KnownValue::new(
                reader.string("known_value.key")?,
                reader.i64("known_value.value")?,
            ));
        }

        let opaque_count = reader.count("opaque_segments")?;
        let mut opaque_segments = Vec::with_capacity(opaque_count);
        for _ in 0..opaque_count {
            opaque_segments.push(OpaqueSegment::new(
                reader.string("opaque_segment.id")?,
                reader.bytes_field("opaque_segment.bytes")?,
            ));
        }

        reader.finish()?;

        Ok(Self {
            schema_version,
            device,
            document: PresetDocument::new(PresetMetadata::new(preset_name), opaque_segments),
            graph_edges,
            known_values,
        })
    }
}

/// Structured failures produced by container encoding and decoding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContainerError {
    /// The first four bytes do not identify this container family.
    InvalidMagic { found: [u8; 4] },
    /// The bytes use a schema version not supported by this build.
    UnsupportedSchemaVersion(u16),
    /// A field ended before its complete value was available.
    Truncated { field: &'static str },
    /// A length-prefixed string is not valid UTF-8.
    InvalidUtf8 { field: &'static str },
    /// A field exceeds the bounded size for its kind.
    LengthTooLarge {
        field: &'static str,
        length: usize,
        maximum: usize,
    },
    /// A collection count exceeds the bounded item count.
    CountTooLarge {
        field: &'static str,
        count: usize,
        maximum: usize,
    },
    /// Input contains bytes after the final field.
    TrailingBytes { count: usize },
}

impl fmt::Display for ContainerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMagic { found } => {
                write!(formatter, "invalid container magic: {found:02x?}")
            }
            Self::UnsupportedSchemaVersion(version) => {
                write!(formatter, "unsupported container schema version: {version}")
            }
            Self::Truncated { field } => write!(formatter, "truncated container field: {field}"),
            Self::InvalidUtf8 { field } => {
                write!(formatter, "invalid UTF-8 in container field: {field}")
            }
            Self::LengthTooLarge {
                field,
                length,
                maximum,
            } => write!(
                formatter,
                "container field {field} length {length} exceeds {maximum}"
            ),
            Self::CountTooLarge {
                field,
                count,
                maximum,
            } => write!(
                formatter,
                "container field {field} count {count} exceeds {maximum}"
            ),
            Self::TrailingBytes { count } => {
                write!(formatter, "container has {count} trailing byte(s)")
            }
        }
    }
}

impl std::error::Error for ContainerError {}

#[derive(Default)]
struct Writer {
    bytes: Vec<u8>,
}

impl Writer {
    fn u16(&mut self, value: u16) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    fn i64(&mut self, value: i64) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    fn u32(&mut self, value: u32) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    fn string(&mut self, field: &'static str, value: &str) -> Result<(), ContainerError> {
        let bytes = value.as_bytes();
        if bytes.len() > MAX_STRING_BYTES {
            return Err(ContainerError::LengthTooLarge {
                field,
                length: bytes.len(),
                maximum: MAX_STRING_BYTES,
            });
        }
        self.u32(bytes.len() as u32);
        self.bytes.extend_from_slice(bytes);
        Ok(())
    }

    fn bytes_field(&mut self, field: &'static str, value: &[u8]) -> Result<(), ContainerError> {
        if value.len() > MAX_OPAQUE_BYTES {
            return Err(ContainerError::LengthTooLarge {
                field,
                length: value.len(),
                maximum: MAX_OPAQUE_BYTES,
            });
        }
        self.u32(value.len() as u32);
        self.bytes.extend_from_slice(value);
        Ok(())
    }

    fn count(&mut self, field: &'static str, count: usize) -> Result<(), ContainerError> {
        if count > MAX_ITEMS {
            return Err(ContainerError::CountTooLarge {
                field,
                count,
                maximum: MAX_ITEMS,
            });
        }
        self.u32(count as u32);
        Ok(())
    }
}

struct Reader<'a> {
    input: &'a [u8],
    offset: usize,
}

impl<'a> Reader<'a> {
    const fn new(input: &'a [u8]) -> Self {
        Self { input, offset: 0 }
    }

    fn take_exact(
        &mut self,
        length: usize,
        field: &'static str,
    ) -> Result<&'a [u8], ContainerError> {
        let end = self
            .offset
            .checked_add(length)
            .ok_or(ContainerError::Truncated { field })?;
        if end > self.input.len() {
            return Err(ContainerError::Truncated { field });
        }
        let bytes = &self.input[self.offset..end];
        self.offset = end;
        Ok(bytes)
    }

    fn u16(&mut self, field: &'static str) -> Result<u16, ContainerError> {
        let bytes = self.take_exact(2, field)?;
        Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
    }

    fn u32(&mut self, field: &'static str) -> Result<u32, ContainerError> {
        let bytes = self.take_exact(4, field)?;
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    fn i64(&mut self, field: &'static str) -> Result<i64, ContainerError> {
        let bytes = self.take_exact(8, field)?;
        Ok(i64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }

    fn string(&mut self, field: &'static str) -> Result<String, ContainerError> {
        let bytes = self.variable(field, MAX_STRING_BYTES)?;
        String::from_utf8(bytes.to_vec()).map_err(|_| ContainerError::InvalidUtf8 { field })
    }

    fn bytes_field(&mut self, field: &'static str) -> Result<Vec<u8>, ContainerError> {
        Ok(self.variable(field, MAX_OPAQUE_BYTES)?.to_vec())
    }

    fn variable(
        &mut self,
        field: &'static str,
        maximum: usize,
    ) -> Result<&'a [u8], ContainerError> {
        let length = self.u32(field)? as usize;
        if length > maximum {
            return Err(ContainerError::LengthTooLarge {
                field,
                length,
                maximum,
            });
        }
        self.take_exact(length, field)
    }

    fn count(&mut self, field: &'static str) -> Result<usize, ContainerError> {
        let count = self.u32(field)? as usize;
        if count > MAX_ITEMS {
            return Err(ContainerError::CountTooLarge {
                field,
                count,
                maximum: MAX_ITEMS,
            });
        }
        Ok(count)
    }

    fn finish(&self) -> Result<(), ContainerError> {
        let count = self.input.len() - self.offset;
        if count == 0 {
            Ok(())
        } else {
            Err(ContainerError::TrailingBytes { count })
        }
    }
}
