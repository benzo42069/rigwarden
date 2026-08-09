use topology_command_engine::{
    MutationValidationError, ParameterMutationRequest, validate_parameter_mutation,
};
use topology_device_registry::{
    DeviceProfile, DeviceRegistry, NumericParameterMetadata, ResolutionStatus, SessionCapabilities,
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

fn writable_profile() -> DeviceProfile {
    DeviceProfile::new(
        DeviceFamilyId::new("test-family").expect("family fixture should be valid"),
        DeviceModelId::new("test-model").expect("model fixture should be valid"),
        FirmwareId::new("1.0").expect("firmware fixture should be valid"),
        SessionCapabilities::new(true),
        VerificationStatus::Experimental,
    )
    .with_numeric_parameter("amp-1", "gain", NumericParameterMetadata::new(0, 100, 1))
}

#[test]
fn read_only_session_cannot_plan_a_write() {
    let registry = DeviceRegistry::from_profiles([writable_profile()]);
    let unknown_identity = identity("test-family", "test-model", "1.1");
    let resolved = registry
        .resolve_session(&unknown_identity)
        .expect("known family/model with unknown firmware should resolve read-only");

    assert_eq!(resolved.status(), ResolutionStatus::UnknownFirmware);
    assert!(!resolved.capabilities().can_write());
    assert_eq!(resolved.verification_status(), VerificationStatus::ReadOnly);

    let error = validate_parameter_mutation(
        resolved.profile(),
        ParameterMutationRequest::new("amp-1", "gain", 45),
    )
    .expect_err("unknown-firmware read-only sessions must not produce a write plan");

    assert_eq!(
        error,
        MutationValidationError::ReadOnly {
            firmware: "1.1".to_owned(),
        }
    );
}

#[test]
fn read_only_status_cannot_plan_a_write_even_if_capability_is_true() {
    let profile = DeviceProfile::new(
        DeviceFamilyId::new("test-family").expect("family fixture should be valid"),
        DeviceModelId::new("test-model").expect("model fixture should be valid"),
        FirmwareId::new("1.1").expect("firmware fixture should be valid"),
        SessionCapabilities::new(true),
        VerificationStatus::ReadOnly,
    )
    .with_numeric_parameter("amp-1", "gain", NumericParameterMetadata::new(0, 100, 1));

    let error =
        validate_parameter_mutation(&profile, ParameterMutationRequest::new("amp-1", "gain", 45))
            .expect_err("read-only status must fail closed before planning a write");

    assert_eq!(
        error,
        MutationValidationError::ReadOnly {
            firmware: "1.1".to_owned(),
        }
    );
}
