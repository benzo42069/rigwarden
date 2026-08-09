use topology_device_registry::{
    DeviceProfile, DeviceRegistry, ResolutionProvenance, ResolutionStatus, SessionCapabilities,
    VerificationStatus,
};
use topology_domain::{
    DeviceFamilyId, DeviceIdentity, DeviceModelId, FirmwareId, TransportEndpointId,
};

fn identity(family: &str, model: &str, firmware: &str) -> DeviceIdentity {
    DeviceIdentity::new(
        DeviceFamilyId::new(family).expect("family fixture should be valid"),
        DeviceModelId::new(model).expect("model fixture should be valid"),
        FirmwareId::new(firmware).expect("firmware fixture should be valid"),
        TransportEndpointId::new("test-endpoint").expect("endpoint fixture should be valid"),
    )
}

#[test]
fn unknown_firmware_never_inherits_write_capability() {
    let profile = DeviceProfile::new(
        DeviceFamilyId::new("test-family").expect("family fixture should be valid"),
        DeviceModelId::new("test-model").expect("model fixture should be valid"),
        FirmwareId::new("1.0").expect("firmware fixture should be valid"),
        SessionCapabilities::new(true),
        VerificationStatus::Experimental,
    );
    let registry = DeviceRegistry::from_profiles([profile]);
    let unknown_identity = identity("test-family", "test-model", "1.1");
    let unsupported_firmware = unknown_identity.firmware().clone();

    let resolved = registry
        .resolve_session(&unknown_identity)
        .expect("a known device with unknown firmware should return a session result");

    assert_eq!(resolved.status(), ResolutionStatus::UnknownFirmware);
    assert_eq!(resolved.provenance(), ResolutionProvenance::UnknownFirmware);
    assert!(!resolved.capabilities().can_write());
    assert_eq!(resolved.verification_status(), VerificationStatus::ReadOnly);
    assert_eq!(resolved.unsupported_firmware(), Some(&unsupported_firmware));
    assert_eq!(resolved.profile().family(), unknown_identity.family());
    assert_eq!(resolved.profile().model(), unknown_identity.model());
    assert_eq!(resolved.profile().firmware(), &unsupported_firmware);
    assert!(
        registry.resolve(&unknown_identity).is_none(),
        "unknown firmware must not inherit the nearest exact profile"
    );
}
