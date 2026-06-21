//! The MCP `initialize` preflight readiness gate (arch §Standard Contracts).
//!
//! Three assertions over a [`ReadbackClient`] — protocol-version pin, required-tool presence, and a
//! data-dir canary read-back — each producing a distinct [`ReportState::Blocked`] precondition on
//! failure (never a silent downgrade; the preflight-integrity invariant). The canary's *emission* is
//! the caller's job (the Epoch-8 cli suite runner); this gate asserts only that the known canary
//! reads back from the shared corpus, proving the `ANDROMEDA_PULSE_DATA_DIR` wiring.

use std::collections::BTreeMap;

use rmcp::service::RoleClient;
use rmcp::transport::IntoTransport;
use serde::Serialize;

use conductor_core::{ReportState, now_rfc3339, redact_value};

use crate::client::ReadbackClient;
use crate::error::VerifyError;
use crate::manifest::ContractManifest;

/// Whether a required read-back tool was advertised by the server.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolPresence {
    Present,
    Absent,
}

/// The data-dir canary round-trip outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CanaryOutcome {
    Ok,
    Failed,
    Skipped,
}

/// The known canary incident the upstream emitter stamped into the live Pulse's corpus; the gate
/// asserts `query_incident_list` returns an incident bearing this marker.
#[derive(Debug, Clone)]
pub struct CanaryMarker {
    pub marker: String,
}

impl CanaryMarker {
    pub fn new(marker: impl Into<String>) -> Self {
        Self { marker: marker.into() }
    }
}

/// The readiness-gate result — a serializable value (arch §Standard Contracts). `ready:false` is the
/// signal every dependent scenario is reported [`ReportState::Blocked`]; only a true harness fault is
/// a `Result::Err`.
#[derive(Debug, Clone, Serialize)]
pub struct ReadyState {
    pub ready: bool,
    pub negotiated_protocol_version: Option<String>,
    pub expected_protocol_version: String,
    pub required_tools: BTreeMap<String, ToolPresence>,
    pub data_dir: String,
    pub canary_round_trip: CanaryOutcome,
    pub blocked_precondition: Option<String>,
    pub checked_at: String,
}

impl ReadyState {
    /// The report state every dependent scenario inherits — `Blocked` until the gate is ready.
    pub fn report_state(&self) -> ReportState {
        if self.ready {
            ReportState::Pass
        } else {
            ReportState::Blocked
        }
    }
}

/// The named precondition surfaced when the MCP read-back path is unreachable (arch §Standard
/// Contracts) — the canary cannot run, so the gate is `Blocked`, never a silent pass.
const UNREACHABLE_PRECONDITION: &str =
    "mcp-server cargo feature + ANDROMEDA_PULSE_MCP_ENABLED + ANDROMEDA_PULSE_DATA_DIR == live Pulse's data-dir";

/// Run the three readiness assertions over an already-connected client. Returns `Ok(ReadyState)` for
/// every readiness outcome (including `Blocked`); MCP call failures are caught into the relevant
/// `Blocked` leg — never propagated as `Err`, never a panic (the verdict/error wall).
#[tracing::instrument(name = "verify.readback.preflight", skip_all)]
pub async fn run_preflight(
    client: &ReadbackClient,
    manifest: &ContractManifest,
    canary: &CanaryMarker,
    data_dir: &str,
) -> Result<ReadyState, VerifyError> {
    let checked_at = now_rfc3339();
    let data_dir = redact_value(data_dir).into_owned();
    let expected = manifest.expected_protocol_version.clone();

    let negotiated = client.negotiated_protocol_version().map(|v| v.as_str().to_string());
    let version_ok = negotiated.as_deref() == Some(expected.as_str());

    let (required_tools, tools_unverifiable): (BTreeMap<String, ToolPresence>, Option<String>) =
        match client.list_tools().await {
            Ok(tools) => {
                let map = manifest
                    .required_tools
                    .iter()
                    .map(|name| {
                        let present = tools.iter().any(|t| t.name == name.as_str());
                        let presence =
                            if present { ToolPresence::Present } else { ToolPresence::Absent };
                        (name.clone(), presence)
                    })
                    .collect();
                (map, None)
            }
            Err(e) => {
                let map = manifest
                    .required_tools
                    .iter()
                    .map(|name| (name.clone(), ToolPresence::Absent))
                    .collect();
                (map, Some(redact_value(&e.to_string()).into_owned()))
            }
        };
    let tools_ok =
        tools_unverifiable.is_none() && required_tools.values().all(|p| *p == ToolPresence::Present);

    let canary_round_trip = match client.query_incident_list(None).await {
        Ok(result) if result.is_error != Some(true) => {
            let body = serde_json::to_string(&result.content).unwrap_or_default();
            if body.contains(&canary.marker) {
                CanaryOutcome::Ok
            } else {
                CanaryOutcome::Failed
            }
        }
        _ => CanaryOutcome::Failed,
    };

    let blocked_precondition = if !version_ok {
        Some(format!(
            "protocol version mismatch: expected {expected}, got {}",
            negotiated.as_deref().unwrap_or("<none>")
        ))
    } else if let Some(reason) = tools_unverifiable {
        Some(format!("required tools unverifiable: {reason}"))
    } else if !tools_ok {
        let absent: Vec<&str> = required_tools
            .iter()
            .filter(|(_, p)| **p == ToolPresence::Absent)
            .map(|(name, _)| name.as_str())
            .collect();
        Some(format!("required tool(s) absent: {}", absent.join(", ")))
    } else if canary_round_trip != CanaryOutcome::Ok {
        Some("canary round-trip failed: incident not found in corpus".to_string())
    } else {
        None
    };

    let ready = blocked_precondition.is_none();
    if let Some(precondition) = &blocked_precondition {
        tracing::info!(state = ReportState::Blocked.label(), "preflight blocked: {precondition}");
    }

    Ok(ReadyState {
        ready,
        negotiated_protocol_version: negotiated,
        expected_protocol_version: expected,
        required_tools,
        data_dir,
        canary_round_trip,
        blocked_precondition,
        checked_at,
    })
}

/// Connect over an injected transport, then run the gate. A connect / `initialize` failure (the MCP
/// read-back path unreachable) maps to a `Blocked` `ReadyState` with the named precondition — not a
/// harness `Err`. The entrypoint the Epoch-8 `conductor preflight` verb and the live child-spawn test
/// both drive.
#[tracing::instrument(name = "verify.readback.preflight_boot", skip_all)]
pub async fn preflight_boot<T, E, A>(
    transport: T,
    manifest: &ContractManifest,
    canary: &CanaryMarker,
    data_dir: &str,
) -> Result<ReadyState, VerifyError>
where
    T: IntoTransport<RoleClient, E, A>,
    E: std::error::Error + Send + Sync + 'static,
{
    match ReadbackClient::connect_transport(transport).await {
        Ok(client) => run_preflight(&client, manifest, canary, data_dir).await,
        Err(e) => {
            tracing::info!(
                state = ReportState::Blocked.label(),
                "preflight blocked: mcp read-back unreachable ({})",
                redact_value(&e.to_string())
            );
            Ok(ReadyState {
                ready: false,
                negotiated_protocol_version: None,
                expected_protocol_version: manifest.expected_protocol_version.clone(),
                required_tools: manifest
                    .required_tools
                    .iter()
                    .map(|name| (name.clone(), ToolPresence::Absent))
                    .collect(),
                data_dir: redact_value(data_dir).into_owned(),
                canary_round_trip: CanaryOutcome::Skipped,
                blocked_precondition: Some(UNREACHABLE_PRECONDITION.to_string()),
                checked_at: now_rfc3339(),
            })
        }
    }
}
