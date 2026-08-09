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
fn exact_profile_match_can_enable_write() {
    let profile = DeviceProfile::new(
        DeviceFamilyId::new("test-family").expect("family fixture should be valid"),
        DeviceModelId::new("test-model").expect("model fixture should be valid"),
        FirmwareId::new("1.0").expect("firmware fixture should be valid"),
        SessionCapabilities::new(true),
        VerificationStatus::Experimental,
    );
    let registry = DeviceRegistry::from_profiles([profile.clone()]);

    let resolved = registry
        .resolve(&identity("test-family", "test-model", "1.0"))
        .expect("an exact family, model, and firmware profile should resolve");

    assert_eq!(resolved.profile(), &profile);
    assert!(resolved.capabilities().can_write());
    assert_eq!(resolved.provenance(), ResolutionProvenance::ExactProfile);
    assert_eq!(resolved.status(), ResolutionStatus::ExactMatch);
    assert_eq!(
        resolved.verification_status(),
        VerificationStatus::Experimental
    );

    assert!(
        registry
            .resolve(&identity("test-family", "other-model", "1.0"))
            .is_none(),
        "a different model must not resolve through an exact profile"
    );
}
