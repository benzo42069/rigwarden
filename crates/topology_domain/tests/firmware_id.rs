use topology_domain::{FirmwareId, FirmwareIdError};

#[test]
fn firmware_id_preserves_nonblank_vendor_text() {
    assert_eq!(
        FirmwareId::new(" \t\n"),
        Err(FirmwareIdError::BlankOrWhitespace)
    );

    let id = FirmwareId::new(" 1.02 beta 3 ").expect("valid firmware identifier");
    assert_eq!(id.as_str(), "1.02 beta 3");
}
