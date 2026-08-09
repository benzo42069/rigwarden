/// Configure flutter_rust_bridge's generated runtime without exposing a
/// product-domain endpoint. TOP-FFI-001 adds the first typed API.
#[flutter_rust_bridge::frb(init)]
pub fn init_rigwarden_bridge() {
    flutter_rust_bridge::setup_default_user_utils();
}
