//! Deterministic in-memory device-profile resolution for RigWarden.

mod resolve;

pub use resolve::{
    DeviceProfile, DeviceRegistry, NumericParameterMetadata, ResolutionProvenance,
    ResolutionStatus, ResolvedProfile, SessionCapabilities, VerificationStatus,
};
