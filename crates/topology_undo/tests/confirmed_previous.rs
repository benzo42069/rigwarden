use topology_undo::Journal;

#[test]
fn undo_entry_uses_confirmed_previous_value() {
    let mut journal = Journal::new();

    let pending = journal.begin_parameter_change("amp-1.gain", 3.0, 4.5);
    assert!(journal.completed_entries().is_empty());
    assert_eq!(journal.pending_count(), 1);

    journal
        .confirm(pending)
        .expect("confirmation should finalize one pending mutation");

    let entry = journal
        .completed_entries()
        .first()
        .expect("confirmed mutation should have one undo entry");
    assert_eq!(entry.target(), "amp-1.gain");
    assert_eq!(entry.previous_value(), 3.0);
    assert_eq!(entry.new_value(), 4.5);
    assert_eq!(journal.pending_count(), 0);

    let failed = journal.begin_parameter_change("amp-1.gain", 4.5, 6.0);
    journal
        .fail(failed)
        .expect("failed mutation should be removable");
    assert_eq!(journal.completed_entries().len(), 1);
    assert_eq!(journal.pending_count(), 0);
}
