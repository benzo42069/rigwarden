use topology_undo::Journal;

#[test]
fn confirmed_undo_restores_exact_prior_value_only_after_confirmation() {
    let mut journal = Journal::new_with_preset("preset-a");

    let edit = journal.begin_parameter_change("amp-1/gain", 3.0, 4.5);
    journal
        .confirm(edit)
        .expect("the confirmed edit should become undoable");

    let proposal = journal
        .prepare_undo()
        .expect("a confirmed edit should produce one restoration proposal");
    assert_eq!(proposal.target(), "amp-1/gain");
    assert_eq!(proposal.restoration_value(), 3.0);

    let pending_entry = journal
        .completed_entries()
        .first()
        .expect("the completed entry remains visible while restoration is pending");
    assert_eq!(pending_entry.target(), "amp-1/gain");
    assert_eq!(pending_entry.previous_value(), 3.0);
    assert_eq!(pending_entry.new_value(), 4.5);

    journal
        .confirm_undo(proposal)
        .expect("confirmed restoration should consume the completed entry");
    assert!(journal.completed_entries().is_empty());
}

#[test]
fn undo_proposal_cannot_confirm_against_another_journal() {
    let mut first = Journal::new_with_preset("preset-a");
    let first_edit = first.begin_parameter_change("amp-1/gain", 3.0, 4.5);
    first
        .confirm(first_edit)
        .expect("the first confirmed edit should become undoable");

    let mut second = Journal::new_with_preset("preset-a");
    let second_edit = second.begin_parameter_change("amp-1/gain", 3.0, 4.5);
    second
        .confirm(second_edit)
        .expect("the second confirmed edit should become undoable");

    let _first_proposal = first
        .prepare_undo()
        .expect("the first journal should hold its own pending proposal");
    let foreign_proposal = second
        .prepare_undo()
        .expect("the second journal should produce a restoration proposal");
    assert!(first.confirm_undo(foreign_proposal).is_err());
    assert_eq!(first.completed_entries().len(), 1);
    assert_eq!(second.completed_entries().len(), 1);
}
