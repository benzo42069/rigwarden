use topology_domain::{DeviceFamilyId, DeviceFamilyIdError};

#[test]
fn blank_device_family_id_is_rejected() {
    assert_eq!(
        DeviceFamilyId::new(""),
        Err(DeviceFamilyIdError::BlankOrWhitespace)
    );
    assert_eq!(
        DeviceFamilyId::new(" \t\n"),
        Err(DeviceFamilyIdError::BlankOrWhitespace)
    );

    let id = DeviceFamilyId::new("fractal-gen3").expect("valid family identifier");
    assert_eq!(id.as_str(), "fractal-gen3");
}
