use topology_command_engine::{ConnectionSession, IncomingResponse, ResponseDisposition};

#[test]
fn stale_response_cannot_confirm_new_connection_request() {
    let mut session = ConnectionSession::new();
    let first_request = session.issue_request(7);
    assert_eq!(first_request.generation(), 1);

    session.reconnect();
    let current_request = session.issue_request(7);
    assert_eq!(current_request.generation(), 2);

    let stale_outcome = session.process_response(IncomingResponse::new(1, 7, b"state-old"));
    assert_eq!(stale_outcome, ResponseDisposition::IgnoredStale);
    assert!(session.is_pending(current_request));
    assert!(session.confirmed_response(current_request).is_none());

    let current_outcome = session.process_response(IncomingResponse::new(2, 7, b"state-new"));
    assert_eq!(current_outcome, ResponseDisposition::Confirmed);
    assert!(!session.is_pending(current_request));
    assert_eq!(
        session
            .confirmed_response(current_request)
            .expect("matching generation should confirm request")
            .payload(),
        b"state-new"
    );
}
