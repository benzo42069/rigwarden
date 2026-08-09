use topology_domain::{
    DeviceFamilyId, DeviceIdentity, DeviceModelId, FirmwareId, TransportEndpointId,
};

#[test]
fn device_identity_does_not_imply_write_capability() {
    let family = DeviceFamilyId::new("fractal-gen3").expect("valid family identifier");
    let model = DeviceModelId::new("axe-fx-iii").expect("valid model identifier");
    let firmware = FirmwareId::new("1.0 beta").expect("valid firmware identifier");
    let endpoint = TransportEndpointId::new("midi-endpoint-7").expect("valid endpoint identifier");

    let identity = DeviceIdentity::new(
        family.clone(),
        model.clone(),
        firmware.clone(),
        endpoint.clone(),
    );

    assert_eq!(identity.family(), &family);
    assert_eq!(identity.model(), &model);
    assert_eq!(identity.firmware(), &firmware);
    assert_eq!(identity.transport_endpoint(), &endpoint);
}
