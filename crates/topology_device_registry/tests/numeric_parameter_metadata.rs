use topology_device_registry::{
    DeviceProfile, DeviceRegistry, NumericParameterMetadata, SessionCapabilities,
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
fn exact_numeric_parameter_metadata_is_profile_owned() {
    let profile = DeviceProfile::new(
        DeviceFamilyId::new("test-family").expect("family fixture should be valid"),
        DeviceModelId::new("test-model").expect("model fixture should be valid"),
        FirmwareId::new("1.0").expect("firmware fixture should be valid"),
        SessionCapabilities::new(true),
        VerificationStatus::Experimental,
    )
    .with_numeric_parameter("amp-1", "gain", NumericParameterMetadata::new(0, 100, 1));
    let registry = DeviceRegistry::from_profiles([profile]);

    let resolved = registry
        .resolve_session(&identity("test-family", "test-model", "1.0"))
        .expect("the exact profile should resolve");
    let gain = resolved
        .profile()
        .numeric_parameter("amp-1", "gain")
        .expect("the profile owns the amp-1/gain definition");

    assert_eq!(gain.min_stored(), 0);
    assert_eq!(gain.max_stored(), 100);
    assert_eq!(gain.decimal_places(), 1);
    assert!(
        resolved
            .profile()
            .numeric_parameter("amp-1", "missing")
            .is_none()
    );

    let unknown = registry
        .resolve_session(&identity("test-family", "test-model", "1.1"))
        .expect("known family/model with unknown firmware should resolve read-only");
    assert!(
        unknown
            .profile()
            .numeric_parameter("amp-1", "gain")
            .is_none()
    );
}
