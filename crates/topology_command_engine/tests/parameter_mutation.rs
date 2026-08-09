use topology_command_engine::{
    MutationValidationError, ParameterMutationRequest, validate_parameter_mutation,
};
use topology_device_registry::{
    DeviceProfile, NumericParameterMetadata, SessionCapabilities, VerificationStatus,
};
use topology_domain::{DeviceFamilyId, DeviceModelId, FirmwareId};

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
fn valid_parameter_mutation_is_typed_but_not_encoded() {
    let mutation = validate_parameter_mutation(
        &writable_profile(),
        ParameterMutationRequest::new("amp-1", "gain", 45),
    )
    .expect("4.5 should be valid against the profile-owned 0.0..10.0 gain range");

    assert_eq!(mutation.block_id(), "amp-1");
    assert_eq!(mutation.parameter_id(), "gain");
    assert_eq!(mutation.stored_value(), 45);
    assert_eq!(mutation.decimal_places(), 1);

    let out_of_range = validate_parameter_mutation(
        &writable_profile(),
        ParameterMutationRequest::new("amp-1", "gain", 101),
    )
    .expect_err("10.1 must be rejected by the literal profile range");

    assert_eq!(
        out_of_range,
        MutationValidationError::OutOfRange {
            requested_stored: 101,
            min_stored: 0,
            max_stored: 100,
        }
    );
}
