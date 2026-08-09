//! Deterministic domain value objects for RigWarden.

mod device;
mod firmware;

pub use device::{
    DeviceFamilyId, DeviceFamilyIdError, DeviceIdentity, DeviceModelId, DeviceModelIdError,
    TransportEndpointId, TransportEndpointIdError,
};
pub use firmware::{FirmwareId, FirmwareIdError};
