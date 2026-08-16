//! Port-occupier fault — the loopback bind/hold/release lifecycle, exercised on an ephemeral port.
//! Never binds the real `:4317` (reserved for the live fault); tests use `:0` so they are
//! collision-free and deterministic (test-plan §10 zero-flakiness).

use std::net::{IpAddr, Ipv4Addr, TcpListener};

use conductor_core::{ObsSink, init_observability};
use conductor_faults::{FaultError, PortOccupier, OTLP_INGEST_PORT};
use serde_json::{Map, Value};

#[test]
fn occupies_a_loopback_port_and_reports_its_resolved_addr() {
    let occ = PortOccupier::occupy(0).expect("occupy an ephemeral loopback port");
    let addr = occ.local_addr();
    assert_eq!(addr.ip(), IpAddr::V4(Ipv4Addr::LOCALHOST));
    assert_ne!(addr.port(), 0, "an OS-assigned port is reported, not the :0 request");
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

/// The span brackets the bind in the self-obs stream, asserted on the line the subscriber writes —
/// the `fields(...)` we wrote in the macro prove nothing, since the allowlist gates them (obs-plan §4).
#[test]
fn the_hold_is_bracketed_by_a_fault_span_on_the_emitted_lines() {
    let dir = std::env::temp_dir().join(format!("conductor-faults-obs-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let path = dir.join("agent-latest.jsonl");
    init_observability("conductor", Some("RUN-OCCUPIER".to_string()), ObsSink::File(path.clone()));

    let mut occ = PortOccupier::occupy(0).expect("occupy an ephemeral loopback port");
    let port = occ.local_addr().port();
    occ.release();

    let body = std::fs::read_to_string(&path).expect("self-obs log written");
    let lines: Vec<Value> = body.lines().filter_map(|l| serde_json::from_str(l).ok()).collect();
    let record = |event: &str| -> Map<String, Value> {
        lines
            .iter()
            .filter_map(Value::as_object)
            .find(|o| {
                o.get("span") == Some(&Value::from("fault.port_occupier"))
                    && o.get("span_event") == Some(&Value::from(event))
            })
            .unwrap_or_else(|| panic!("no {event} record for fault.port_occupier: {lines:?}"))
            .clone()
    };

    let new = record("new");
    assert_eq!(new.get("fault_type"), Some(&Value::from("port_occupier")));
    assert_eq!(
        new.get("port"),
        Some(&Value::from(port)),
        "the bound port survives the allowlist: {new:?}"
    );
    record("close");

    let _ = std::fs::remove_dir_all(&dir);
}
