use topology_simulator::{
    ScriptedRequest, ScriptedResponse, ScriptedTransport, SimulatorIdentity, SyntheticPayload,
    TranscriptEntry,
};

#[test]
fn scripted_exchange_correlates_expected_response() {
    // These payloads are synthetic, non-vendor test values.
    let request = ScriptedRequest::new(7, SyntheticPayload::from_bytes(b"read-state"));
    let response = ScriptedResponse::new(7, SyntheticPayload::from_bytes(b"state-ok"));
    let mut simulator = ScriptedTransport::new(request.clone(), response.clone());

    assert_eq!(simulator.identity(), SimulatorIdentity::SyntheticScripted);

    let actual_response = simulator
        .exchange(request.clone())
        .expect("matching scripted response should complete request 7");
    assert_eq!(actual_response, response);
    assert_eq!(
        simulator.transcript(),
        &[
            TranscriptEntry::Sent(request.clone()),
            TranscriptEntry::Received(response.clone()),
        ]
    );

    let unexpected = ScriptedRequest::new(8, SyntheticPayload::from_bytes(b"wrong-request"));
    let mut mismatch_simulator = ScriptedTransport::new(request, response);
    let mismatch = mismatch_simulator
        .exchange(unexpected.clone())
        .expect_err("unexpected scripted request should fail clearly");
    assert_eq!(mismatch.expected_id, 7);
    assert_eq!(mismatch.expected_payload.as_bytes(), b"read-state");
    assert_eq!(mismatch.actual_id, 8);
    assert_eq!(mismatch.actual_payload, unexpected.payload);
    assert_eq!(
        mismatch_simulator.transcript(),
        &[TranscriptEntry::Sent(unexpected)]
    );
}
