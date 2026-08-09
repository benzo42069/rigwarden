use topology_devtools::assets::{AssetEntry, AssetManifest, validate_manifest};

#[test]
fn production_svg_asset_is_rejected() {
    let manifest = AssetManifest::new([
        AssetEntry::production("icons/device.SvG"),
        AssetEntry::dynamic("routing/cable"),
        AssetEntry::procedural_knob("gain"),
    ]);

    let errors = validate_manifest(&manifest).expect_err("production SVG must be rejected");
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].code, "production_svg_forbidden");
    assert_eq!(errors[0].path, "icons/device.SvG");

    let png_only = AssetManifest::new([AssetEntry::production("icons/device.PNG")]);
    validate_manifest(&png_only).expect("PNG production assets must be accepted");

    let test_only_svg = AssetManifest::new([AssetEntry::test_only("fixtures/icon.svg")]);
    validate_manifest(&test_only_svg).expect("test-only SVG fixtures remain configurable");

    let functional = AssetManifest::new([
        AssetEntry::dynamic("routing/cable"),
        AssetEntry::procedural_knob("gain"),
    ]);
    validate_manifest(&functional).expect("dynamic functional graphics must be accepted");
}
