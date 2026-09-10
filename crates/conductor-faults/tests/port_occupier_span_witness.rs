//! The port-occupier span witness, alone in its own test binary.
//!
//! `conductor_core::init_observability` installs a PROCESS-GLOBAL, first-install-wins subscriber, so a
//! per-test temp sink does not isolate it: five of this test's former siblings in `port_occupier.rs`
//! construct a `PortOccupier`, and each emits its own `fault.port_occupier` span into whichever file won
//! the install. Measured on the shared binary — six `new` records carrying six distinct ephemeral ports,
//! so the first-match lookup below returned a sibling's bind and the allowlist assertion compared the
//! wrong port; green under nextest, which is process-per-test, and green under `--test-threads=1`.
//! Serializing the file would hide that shared state rather than remove it (test-plan §11). One test per
//! binary holds under both runners.

use conductor_core::{ObsSink, init_observability};
use conductor_faults::PortOccupier;
use serde_json::{Map, Value};

/// The span brackets the bind in the self-obs stream, asserted on the line the subscriber writes —
/// the `fields(...)` we wrote in the macro prove nothing, since the allowlist gates them (obs-plan §4).
#[test]
fn the_hold_is_bracketed_by_a_fault_span_on_the_emitted_lines() {
    let dir = std::env::temp_dir().join(format!("conductor-faults-obs-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let path = dir.join("agent-latest.jsonl");
    init_observability(
        "conductor",
        Some("RUN-OCCUPIER".to_string()),
        ObsSink::File(path.clone()),
    );

    let mut occ = PortOccupier::occupy(0).expect("occupy an ephemeral loopback port");
    let port = occ.local_addr().port();
    occ.release();

    let body = std::fs::read_to_string(&path).expect("self-obs log written");
    let lines: Vec<Value> = body
        .lines()
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect();
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
