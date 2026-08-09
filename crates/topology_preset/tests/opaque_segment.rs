use topology_preset::{OpaqueSegment, PresetDocument, PresetMetadata};

#[test]
fn editing_known_field_preserves_opaque_segment_exactly() {
    let mut document = PresetDocument::new(
        PresetMetadata::new("Original name"),
        vec![
            OpaqueSegment::new("known-before", vec![0x10]),
            OpaqueSegment::new("unknown-1", vec![0x01, 0x02, 0x03]),
            OpaqueSegment::new("known-after", vec![0x20, 0x21]),
        ],
    );

    document.set_name("Edited name");

    assert_eq!(document.metadata().name(), "Edited name");
    let segments = document.opaque_segments();
    assert_eq!(segments.len(), 3);
    assert_eq!(segments[0].id(), "known-before");
    assert_eq!(segments[0].bytes(), [0x10]);
    assert_eq!(segments[1].id(), "unknown-1");
    assert_eq!(segments[1].bytes(), [0x01, 0x02, 0x03]);
    assert_eq!(segments[2].id(), "known-after");
    assert_eq!(segments[2].bytes(), [0x20, 0x21]);
}
