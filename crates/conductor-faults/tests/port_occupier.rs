//! Port-occupier fault — the loopback bind/hold/release lifecycle, exercised on an ephemeral port.
//! Never binds the real `:4317` (reserved for the live fault); tests use `:0` so they are
//! collision-free and deterministic (test-plan §10 zero-flakiness).

use std::net::{IpAddr, Ipv4Addr, TcpListener};

use conductor_faults::{FaultError, OTLP_INGEST_PORT, PortOccupier};

#[test]
fn occupies_a_loopback_port_and_reports_its_resolved_addr() {
    let occ = PortOccupier::occupy(0).expect("occupy an ephemeral loopback port");
    let addr = occ.local_addr();
    assert_eq!(addr.ip(), IpAddr::V4(Ipv4Addr::LOCALHOST));
    assert_ne!(
        addr.port(),
        0,
        "an OS-assigned port is reported, not the :0 request"
    );
}

#[test]
fn a_held_port_rejects_a_second_bind() {
    let occ = PortOccupier::occupy(0).expect("occupy an ephemeral loopback port");
    let addr = occ.local_addr();
    // While the occupier holds the address, the OS denies any competing bind — the exclusivity that
    // denies Pulse's receiver bind (P-003).
    assert!(TcpListener::bind(addr).is_err());
    drop(occ);
}

#[test]
fn occupying_a_held_port_is_a_typed_error_not_a_panic() {
    let held = PortOccupier::occupy(0).expect("occupy an ephemeral loopback port");
    let port = held.local_addr().port();
    let result = PortOccupier::occupy(port);
    assert!(matches!(result, Err(FaultError::Bind { .. })));
    drop(held);
}

#[test]
fn release_frees_the_port_and_is_idempotent() {
    let mut occ = PortOccupier::occupy(0).expect("occupy an ephemeral loopback port");
    let addr = occ.local_addr();
    occ.release();
    let rebound = TcpListener::bind(addr).expect("the address rebinds after release");
    drop(rebound);
    occ.release(); // a second release is a no-op — never a panic.
}

#[test]
fn drop_releases_the_port() {
    let addr = {
        let occ = PortOccupier::occupy(0).expect("occupy an ephemeral loopback port");
        occ.local_addr()
    };
    TcpListener::bind(addr).expect("the address rebinds after the occupier is dropped");
}

#[test]
fn default_target_is_the_otlp_ingest_port() {
    // occupy_default() binds 4317; a test must never bind the real ingest port, so assert the
    // documented constant instead.
    assert_eq!(OTLP_INGEST_PORT, 4317);
}
