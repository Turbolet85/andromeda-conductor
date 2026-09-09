//! Correlation on the read-back wire: the request-id counter, and the notification that follows
//! `initialize`.
//!
//! Neither is visible from a return value. The stub ECHOES the id it received — as Pulse's own
//! hand-rolled sidecar does — so it answers a counter that walks backwards or never moves exactly as
//! agreeably as one that advances; the id SEQUENCE on the wire is the only witness. And `notify`'s
//! single call site discards its result by design (Pulse tolerates the notification's absence), so
//! there the wire is the only witness there has ever been. Driven through the public
//! `connect_transport` seam over an in-process duplex — the session is crate-private by design.

mod common;

use common::{DECOY_TOOL, StubConfig, WireEntry, WireLog, bounded, serve_stub};
use conductor_verify::ReadbackClient;

#[tokio::test(flavor = "current_thread")]
async fn request_ids_advance_so_a_response_pairs_to_exactly_one_request() {
    let log = WireLog::default();
    let config = StubConfig {
        wire_log: Some(log.clone()),
        ..StubConfig::default()
    };
    let (client_io, server_io) = tokio::io::duplex(4096);
    let server = tokio::spawn(serve_stub(server_io, config));

    let client = bounded(ReadbackClient::connect_transport(client_io))
        .await
        .expect("client connects");
    bounded(client.list_tools())
        .await
        .expect("tools/list answers");
    bounded(client.query_incident_list(None))
        .await
        .expect("query_incident_list answers");

    drop(client);
    server.abort();

    let ids = log.request_ids();
    assert_eq!(
        ids.len(),
        3,
        "initialize + tools/list + one tools/call reached the wire: {ids:?}"
    );
    assert_eq!(ids[0], 1, "a session's first request carries id 1: {ids:?}");
    for pair in ids.windows(2) {
        assert!(
            pair[1] > pair[0],
            "ids must advance strictly: a repeated id makes one response pairable to more than one \
             request, and a decreasing one re-spends ids already in flight: {ids:?}"
        );
    }
}

#[tokio::test(flavor = "current_thread")]
async fn a_stale_response_is_skipped_rather_than_paired_to_a_later_request() {
    // The decoy carries the id the FIRST request spent. Skipping it is the whole purpose of the
    // id-mismatch guard in the response loop; a client whose id never advances consumes it instead.
    let config = StubConfig {
        decoy_before_nth_request: Some(2),
        ..StubConfig::default()
    };
    let expected = config.tools.clone();
    let (client_io, server_io) = tokio::io::duplex(4096);
    let server = tokio::spawn(serve_stub(server_io, config));

    let client = bounded(ReadbackClient::connect_transport(client_io))
        .await
        .expect("client connects");
    let tools = bounded(client.list_tools())
        .await
        .expect("tools/list answers");

    drop(client);
    server.abort();

    assert!(
        !tools.iter().any(|name| name == DECOY_TOOL),
        "the stale response was paired to a later request: {tools:?}"
    );
    assert_eq!(
        tools, expected,
        "the real response is the one that answers: {tools:?}"
    );
}

#[tokio::test(flavor = "current_thread")]
async fn the_initialized_notification_reaches_the_wire_after_initialize() {
    let log = WireLog::default();
    let config = StubConfig {
        wire_log: Some(log.clone()),
        ..StubConfig::default()
    };
    let (client_io, server_io) = tokio::io::duplex(4096);
    let server = tokio::spawn(serve_stub(server_io, config));

    let client = bounded(ReadbackClient::connect_transport(client_io))
        .await
        .expect("client connects");
    // A second request the stub must read AFTER the notification: the stub consumes its input in
    // arrival order, so awaiting this makes the observation an ordering fact, never a timing race.
    bounded(client.list_tools())
        .await
        .expect("tools/list answers");

    drop(client);
    server.abort();

    let entries = log.entries();
    let initialized = entries
        .iter()
        .position(
            |entry| matches!(entry, WireEntry::Request { method, .. } if method == "initialize"),
        )
        .unwrap_or_else(|| panic!("the session opens with an initialize request: {entries:?}"));
    let notified = entries
        .iter()
        .position(|entry| {
            matches!(entry, WireEntry::Notification { method } if method == "notifications/initialized")
        })
        .unwrap_or_else(|| {
            panic!("notifications/initialized never reached the wire: {entries:?}")
        });

    assert!(
        notified > initialized,
        "the handshake step follows initialize, never precedes it: {entries:?}"
    );
}
