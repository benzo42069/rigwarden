//! Harness for deterministic RigWarden preset-document behavior.

mod container;
mod document;

pub use container::{
    CONTAINER_SCHEMA_VERSION, ContainerError, DeviceMetadata, GraphEdge, KnownValue,
    OfflineContainer,
};
pub use document::{OpaqueSegment, PresetDocument, PresetMetadata};
