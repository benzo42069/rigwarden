//! Deterministic domain value objects for RigWarden.

mod device;
mod firmware;

pub use device::{DeviceFamilyId, DeviceFamilyIdError};
pub use firmware::{FirmwareId, FirmwareIdError};
