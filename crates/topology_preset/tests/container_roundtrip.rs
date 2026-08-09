use std::panic::{AssertUnwindSafe, catch_unwind};

use topology_preset::{
    CONTAINER_SCHEMA_VERSION, DeviceMetadata, GraphEdge, KnownValue, OfflineContainer,
    OpaqueSegment, PresetDocument, PresetMetadata,
};

fn synthetic_container() -> OfflineContainer {
    let document = PresetDocument::new(
        PresetMetadata::new("Synthetic offline preset"),
        [OpaqueSegment::new("unknown-1", [0xde, 0xad, 0xbe, 0xef])],
    );

    OfflineContainer::new(
        DeviceMetadata::new(
            "synthetic-family",
            "synthetic-model",
            "firmware-opaque",
            "offline",
        ),
        document,
        [GraphEdge::new("input-1", "out", "amp-1", "in")],
        [KnownValue::new("amp-1.input_drive", 450)],
    )
}

#[test]
fn offline_container_preserves_normalized_and_opaque_data() {
    let original = synthetic_container();
    let encoded = original
        .serialize()
        .expect("synthetic container should serialize");

    assert_eq!(&encoded[..4], b"RWOC", "container magic is explicit");
    assert_eq!(
        &encoded[4..6],
        &[0x01, 0x00],
        "container schema version is explicit",
    );

    let parsed = OfflineContainer::deserialize(&encoded).expect("container should parse");
    assert_eq!(parsed.schema_version(), 1);
    assert_eq!(CONTAINER_SCHEMA_VERSION, 1);

    assert_eq!(parsed.device().family(), "synthetic-family");
    assert_eq!(parsed.device().model(), "synthetic-model");
    assert_eq!(parsed.device().firmware(), "firmware-opaque");
    assert_eq!(parsed.device().transport_endpoint(), "offline");
    assert_eq!(
        parsed.document().metadata().name(),
        "Synthetic offline preset"
    );

    let edge = &parsed.graph_edges()[0];
    assert_eq!(edge.source_node(), "input-1");
    assert_eq!(edge.source_port(), "out");
    assert_eq!(edge.destination_node(), "amp-1");
    assert_eq!(edge.destination_port(), "in");

    let known_value = &parsed.known_values()[0];
    assert_eq!(known_value.key(), "amp-1.input_drive");
    assert_eq!(known_value.value(), 450);

    let opaque = &parsed.document().opaque_segments()[0];
    assert_eq!(opaque.id(), "unknown-1");
    assert_eq!(opaque.bytes(), [0xde, 0xad, 0xbe, 0xef]);
}

#[test]
fn offline_container_rejects_truncated_input_without_panic() {
    let encoded = synthetic_container()
        .serialize()
        .expect("synthetic container should serialize");
    let truncated = &encoded[..encoded.len() - 1];

    let result = catch_unwind(AssertUnwindSafe(|| {
        OfflineContainer::deserialize(truncated)
    }));

    assert!(result.is_ok(), "truncated input must not panic");
    assert!(
        result.expect("panic result was checked above").is_err(),
        "truncated input must return a structured error"
    );
}
