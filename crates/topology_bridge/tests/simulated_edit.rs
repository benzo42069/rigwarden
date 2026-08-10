use topology_bridge::simulated_edit::{
    compose_synthetic_parameter_edit, EditState, SemanticTranscriptEntry, SimulatedEditError,
};
use topology_command_engine::MutationValidationError;
use topology_device_registry::{
    DeviceProfile, NumericParameterMetadata, SessionCapabilities, VerificationStatus,
};
use topology_domain::{DeviceFamilyId, DeviceModelId, FirmwareId};

fn synthetic_writable_profile() -> DeviceProfile {
    DeviceProfile::new(
        DeviceFamilyId::new("synthetic-family").expect("synthetic family is valid"),
        DeviceModelId::new("synthetic-model").expect("synthetic model is valid"),
        FirmwareId::new("synthetic-1").expect("synthetic firmware is valid"),
        SessionCapabilities::new(true),
        VerificationStatus::Experimental,
    )
    .with_numeric_parameter("amp-1", "gain", NumericParameterMetadata::new(0, 100, 1))
}

fn synthetic_read_only_profile() -> DeviceProfile {
    DeviceProfile::new(
        DeviceFamilyId::new("synthetic-family").expect("synthetic family is valid"),
        DeviceModelId::new("synthetic-model").expect("synthetic model is valid"),
        FirmwareId::new("synthetic-1").expect("synthetic firmware is valid"),
        SessionCapabilities::new(true),
        VerificationStatus::ReadOnly,
    )
}

#[test]
fn synthetic_parameter_edit_confirms_then_undo_restores_prior_value() {
    let summary = compose_synthetic_parameter_edit(&synthetic_writable_profile(), 30, 45)
        .expect("the synthetic edit and its confirmed undo should succeed");

    assert_eq!(
        summary.simulator_identity().as_str(),
        "rigwarden.synthetic-scripted-simulator"
    );
    assert_eq!(summary.target(), "amp-1/gain");
    assert_eq!(summary.final_state().stored_value(), 30);
    assert_eq!(summary.final_state().decimal_places(), 1);
    assert_eq!(summary.final_state().display_value(), 3.0);
    assert_eq!(summary.exchange_count(), 2);
    assert_eq!(summary.journal_entry_before_undo().target(), "amp-1/gain");
    assert_eq!(
        summary.journal_entry_before_undo().previous_stored_value(),
        30
    );
    assert_eq!(summary.journal_entry_before_undo().new_stored_value(), 45);
    assert_eq!(summary.journal_entries_before_undo(), 1);
    assert_eq!(summary.journal_entries_remaining(), 0);
    assert_eq!(
        summary.states(),
        &[
            EditState::PendingEdit { stored_value: 45 },
            EditState::ConfirmedEdit { stored_value: 45 },
            EditState::PendingRestoration { stored_value: 30 },
            EditState::ConfirmedRestoration { stored_value: 30 },
        ]
    );
    assert_eq!(
        summary.transcript(),
        &[
            SemanticTranscriptEntry::Request { stored_value: 45 },
            SemanticTranscriptEntry::Confirmed { stored_value: 45 },
            SemanticTranscriptEntry::Request { stored_value: 30 },
            SemanticTranscriptEntry::Confirmed { stored_value: 30 },
        ]
    );

    let invalid = compose_synthetic_parameter_edit(&synthetic_writable_profile(), 30, 101)
        .expect_err("the validator must reject an out-of-range edit before exchange");
    assert_eq!(
        invalid,
        SimulatedEditError::Validation {
            source: MutationValidationError::OutOfRange {
                requested_stored: 101,
                min_stored: 0,
                max_stored: 100,
            },
            exchange_count: 0,
        }
    );
    assert_eq!(invalid.exchange_count(), 0);

    let read_only = compose_synthetic_parameter_edit(&synthetic_read_only_profile(), 30, 45)
        .expect_err("a read-only profile must be rejected before any exchange");
    assert_eq!(
        read_only,
        SimulatedEditError::Validation {
            source: MutationValidationError::ReadOnly {
                firmware: "synthetic-1".to_owned(),
            },
            exchange_count: 0,
        }
    );
    assert_eq!(read_only.exchange_count(), 0);
}
