use topology_undo::Journal;

#[test]
fn preset_change_creates_isolated_journal_branch() {
    let mut journal = Journal::new_with_preset("preset-a");

    let preset_a_edit = journal.begin_parameter_change("amp-1.gain", 3.0, 4.5);
    journal
        .confirm(preset_a_edit)
        .expect("preset A edit should be confirmed");

    journal.switch_preset("preset-b");

    let preset_b_edit = journal.begin_parameter_change("amp-1.gain", 4.5, 6.0);
    journal
        .confirm(preset_b_edit)
        .expect("preset B edit should be confirmed");

    assert_eq!(journal.current_branch_name(), "preset-b");

    let current_entries = journal.completed_entries();
    assert_eq!(current_entries.len(), 1);
    assert_eq!(current_entries[0].previous_value(), 4.5);
    assert_eq!(current_entries[0].new_value(), 6.0);

    let preset_a_entries = journal
        .branch_entries("preset-a")
        .expect("preset A branch should remain inspectable");
    assert_eq!(preset_a_entries.len(), 1);
    assert_eq!(preset_a_entries[0].previous_value(), 3.0);
    assert_eq!(preset_a_entries[0].new_value(), 4.5);

    assert!(journal.branch_entries("preset-b").is_some());
}
