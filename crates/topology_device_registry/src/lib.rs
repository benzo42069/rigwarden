//! Deterministic in-memory device-profile resolution for RigWarden.

mod resolve;

pub use resolve::{
    DeviceProfile, DeviceRegistry, ResolutionProvenance, ResolutionStatus, ResolvedProfile,
    SessionCapabilities, VerificationStatus,
};
