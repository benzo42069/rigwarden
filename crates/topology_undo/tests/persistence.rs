use std::panic::{AssertUnwindSafe, catch_unwind};

use topology_undo::{Journal, JournalSnapshotError};

const EXPECTED_MAGIC: &[u8; 4] = b"RWJS";
const EXPECTED_SCHEMA_VERSION: u16 = 1;

#[test]
fn confirmed_entry_survives_deterministic_save_reload() {
    let mut journal = Journal::new_with_preset("preset-a");
    let mutation = journal.begin_parameter_change("amp-1/gain", 3.0, 4.5);
    journal.confirm(mutation).expect("mutation should confirm");
    journal.switch_preset("preset-b");
    journal.switch_preset("preset-a");

    let encoded = journal
        .encode_snapshot()
        .expect("confirmed journal should encode");
    assert_eq!(&encoded[..EXPECTED_MAGIC.len()], EXPECTED_MAGIC);
    assert_eq!(
        u16::from_le_bytes([encoded[4], encoded[5]]),
        EXPECTED_SCHEMA_VERSION
    );

    let restored = Journal::decode_snapshot(&encoded).expect("snapshot should decode");
    assert_eq!(restored.current_branch_name(), "preset-a");
    let entries = restored
        .branch_entries("preset-a")
        .expect("active branch should survive");
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].target(), "amp-1/gain");
    assert_eq!(entries[0].previous_value().to_bits(), 3.0f64.to_bits());
    assert_eq!(entries[0].new_value().to_bits(), 4.5f64.to_bits());
    assert!(restored.branch_entries("preset-b").is_some());

    let reencoded = restored
        .encode_snapshot()
        .expect("restored journal should reencode");
    assert_eq!(
        reencoded, encoded,
        "snapshot encoding must be deterministic"
    );

    let truncated = &encoded[..encoded.len() - 1];
    assert!(matches!(
        decode_without_panic(truncated),
        JournalSnapshotError::Truncated { .. }
    ));

    let mut invalid_magic = encoded.clone();
    invalid_magic[..EXPECTED_MAGIC.len()].copy_from_slice(b"NOPE");
    assert!(matches!(
        decode_without_panic(&invalid_magic),
        JournalSnapshotError::InvalidMagic { .. }
    ));

    let mut invalid_schema = encoded.clone();
    invalid_schema[4..6].copy_from_slice(&2_u16.to_le_bytes());
    assert!(matches!(
        decode_without_panic(&invalid_schema),
        JournalSnapshotError::UnsupportedSchemaVersion(2)
    ));

    let mut oversized_branch_count = encoded[..6].to_vec();
    oversized_branch_count.extend_from_slice(&u32::MAX.to_le_bytes());
    assert!(matches!(
        decode_without_panic(&oversized_branch_count),
        JournalSnapshotError::CountTooLarge { .. }
    ));

    let mut pending_mutation = Journal::new_with_preset("preset-a");
    pending_mutation.begin_parameter_change("amp-1/gain", 3.0, 4.5);
    assert_eq!(
        pending_mutation.encode_snapshot(),
        Err(JournalSnapshotError::PendingMutation)
    );

    let mut pending_restoration = Journal::new_with_preset("preset-a");
    let mutation = pending_restoration.begin_parameter_change("amp-1/gain", 3.0, 4.5);
    pending_restoration
        .confirm(mutation)
        .expect("mutation should confirm");
    pending_restoration
        .prepare_undo()
        .expect("confirmed entry should prepare undo");
    assert_eq!(
        pending_restoration.encode_snapshot(),
        Err(JournalSnapshotError::PendingRestoration)
    );
}

fn decode_without_panic(input: &[u8]) -> JournalSnapshotError {
    let result = catch_unwind(AssertUnwindSafe(|| Journal::decode_snapshot(input)));
    assert!(result.is_ok(), "malformed snapshot must not panic");
    result
        .expect("panic result checked")
        .expect_err("malformed snapshot must be rejected")
}
