//! Live `TokioChildProcess` child-spawn preflight test (chunk-1's deferred follow-up): spawns the
//! real `stub_pulse_mcp` child binary and drives `preflight_boot` over its stdio — proving the
//! connect→initialize→preflight→canary path against a true child process, not just an in-process
//! duplex. Built only under `--features stub-server` (the gate that builds the stub bin + sets
//! `CARGO_BIN_EXE_stub_pulse_mcp`).
#![cfg(feature = "stub-server")]

use rmcp::transport::TokioChildProcess;
use tokio::process::Command;

use conductor_verify::{CanaryMarker, ContractManifest, preflight_boot};

const CANARY: &str = "conductor-canary-7f3a";

fn manifest() -> ContractManifest {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../contracts/mcp-contract.toml");
    ContractManifest::load(&path).expect("pinned manifest loads")
}

#[tokio::test(flavor = "current_thread")]
async fn live_child_spawn_preflight_is_ready() {
    let command = Command::new(env!("CARGO_BIN_EXE_stub_pulse_mcp"));
    let transport = TokioChildProcess::new(command).expect("spawn stub child");

    let ready = preflight_boot(transport, &manifest(), &CanaryMarker::new(CANARY), "/test/data-dir")
        .await
        .expect("preflight runs");

    assert!(ready.ready, "blocked against the stub child: {:?}", ready.blocked_precondition);
    assert_eq!(ready.negotiated_protocol_version.as_deref(), Some("2024-11-05"));
}
