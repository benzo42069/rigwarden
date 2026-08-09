use topology_domain::{
    DeviceFamilyId, DeviceIdentity as DomainDeviceIdentity, DeviceModelId, FirmwareId,
    TransportEndpointId,
};

/// Configure flutter_rust_bridge's generated runtime.
#[flutter_rust_bridge::frb(init)]
pub fn init_rigwarden_bridge() {
    flutter_rust_bridge::setup_default_user_utils();
}

/// A read-only bridge handle for the validated domain identity.
///
/// The wrapped domain value remains the source of truth; this type exposes
/// only copied display fields and has no Dart-side constructor or mutation.
#[flutter_rust_bridge::frb(opaque)]
pub struct DeviceIdentityHandle(DomainDeviceIdentity);

impl DeviceIdentityHandle {
    #[flutter_rust_bridge::frb(sync, getter)]
    pub fn family(&self) -> String {
        self.0.family().as_str().to_owned()
    }

    #[flutter_rust_bridge::frb(sync, getter)]
    pub fn model(&self) -> String {
        self.0.model().as_str().to_owned()
    }

    #[flutter_rust_bridge::frb(sync, getter)]
    pub fn firmware(&self) -> String {
        self.0.firmware().as_str().to_owned()
    }

    #[flutter_rust_bridge::frb(sync, getter)]
    pub fn transport_endpoint(&self) -> String {
        self.0.transport_endpoint().as_str().to_owned()
    }
}

/// Return one validated identity fixture through the typed Rust-Dart bridge.
pub fn read_fixture_device_identity() -> DeviceIdentityHandle {
    DeviceIdentityHandle(DomainDeviceIdentity::new(
        DeviceFamilyId::new("AM4").expect("fixture family is nonblank"),
        DeviceModelId::new("AM4").expect("fixture model is nonblank"),
        FirmwareId::new("1.00").expect("fixture firmware is nonblank"),
        TransportEndpointId::new("fixture://am4").expect("fixture endpoint is nonblank"),
    ))
}
