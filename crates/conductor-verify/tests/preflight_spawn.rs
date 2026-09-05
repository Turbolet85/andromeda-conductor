//! Live child-spawn preflight test: spawns the real `stub_pulse_mcp` child binary and drives
//! `preflight_boot` over its stdio — proving the connect→initialize→preflight→canary path against a
//! true child process speaking Pulse's RAW JSON-RPC shapes, not just an in-process duplex. Built only
//! under `--features stub-server` (the gate that builds the stub bin + sets `CARGO_BIN_EXE_stub_pulse_mcp`).
#![cfg(feature = "stub-server")]

mod common;

use tokio::process::Command;

use common::bounded;
use conductor_core::RunContractStatus;
use conductor_verify::{CanaryMarker, CanaryPoll, ContractManifest, preflight_boot};

const CANARY: &str = "conductor-canary-7f3a";
/// Must match `stub_pulse_mcp.rs`'s `CANARY_FP` (the fidelity carrier).
const CANARY_FP: &str = "0123456789abcdef";

fn manifest() -> ContractManifest {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../contracts/mcp-contract.toml");
    ContractManifest::load(&path).expect("pinned manifest loads")
}

#[tokio::test(flavor = "current_thread")]
async fn live_child_spawn_preflight_is_ready() {
    let command = Command::new(env!("CARGO_BIN_EXE_stub_pulse_mcp"));

    let ready = bounded(preflight_boot(
        command,
        &manifest(),
        &RunContractStatus::satisfied(),
        &CanaryMarker::new(CANARY, CANARY_FP),
        "/test/data-dir",
        CanaryPoll::immediate(),
    ))
    .await
    .expect("preflight runs");

    assert!(ready.ready, "blocked against the stub child: {:?}", ready.blocked_precondition);
    assert_eq!(ready.negotiated_protocol_version.as_deref(), Some("2024-11-05"));
}
