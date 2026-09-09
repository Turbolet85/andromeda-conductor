//! The MCP `initialize` preflight readiness gate (arch §Standard Contracts).
//!
//! Four assertions over a [`ReadbackClient`] — protocol-version pin, required-tool presence, the
//! recorded run contract's terms, and a data-dir canary round-trip — each producing a distinct
//! [`ReportState::Blocked`] precondition on failure (never a silent downgrade; the
//! preflight-integrity invariant). The canary's *emission* (a unique fingerprint-storm) is the
//! composition root's job (`conductor-run`); this gate asserts the storm read back — Pulse opened an
//! incident AFTER the storm was emitted — proving the `ANDROMEDA_PULSE_DATA_DIR` corpus wiring end to
//! end and attributing the incident to this run rather than to residue.
//!
//! Freshness carries the assertion because no read-back field varies with the emitted payload: titles
//! are scrubbed, and `retrieve_telemetry_slice.fingerprint_refs` — the former carrier — is populated
//! from the L4 model's `evidence_refs`, which the deterministic-L4 fixture pins to `[]`. Pulse's own
//! computed fingerprint lands in a `span_events` column no MCP tool reads.

use std::collections::BTreeMap;

use serde::Serialize;
use tokio::process::Command;

use conductor_core::{ReportState, RunContractStatus, now_rfc3339, redact_value};

use crate::client::ReadbackClient;
use crate::error::VerifyError;
use crate::extract::{call_error_reason, incident_ids, log_observed_keys, opened_at_unix_nanos};
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

/// The canary the bridge emitted (a unique fingerprint-storm). The gate asserts the live Pulse opened
/// an incident AFTER `emitted_at_unix_nano` — the storm's own emission instant, so the incident is
/// attributable to this run rather than to corpus residue.
///
/// `marker` (the unique `exception.type`) and `fingerprint` are carried for identity and logging only.
/// Neither can serve as the fidelity carrier: Pulse scrubs incident titles, and
/// `retrieve_telemetry_slice.fingerprint_refs` is populated from the L4 model's `evidence_refs`
/// (`pulse-app/src/inference_runtime.rs:684-701`), which the deterministic-L4 fixture pins to `[]`
/// (`pulse-app/src/deterministic_inference.rs:35`) — Pulse's own computed fingerprint reaches no
/// read-back surface at all.
#[derive(Debug, Clone)]
pub struct CanaryMarker {
    pub marker: String,
    pub fingerprint: String,
    pub emitted_at_unix_nano: i64,
}

impl CanaryMarker {
    pub fn new(
        marker: impl Into<String>,
        fingerprint: impl Into<String>,
        emitted_at_unix_nano: i64,
    ) -> Self {
        Self {
            marker: marker.into(),
            fingerprint: fingerprint.into(),
            emitted_at_unix_nano,
        }
    }
}

/// How long the canary leg waits for Pulse to ingest the storm and raise the incident before reporting
/// `Blocked` — bounded so the gate never hangs. `immediate` (one attempt, no wait) is the deterministic
/// in-process / stub-child test budget; the live caller derives attempts from `CONDUCTOR_PREFLIGHT_TIMEOUT`.
#[derive(Debug, Clone, Copy)]
pub struct CanaryPoll {
    pub attempts: u32,
    pub interval: std::time::Duration,
}

impl CanaryPoll {
    /// A single attempt with no wait — deterministic for the stub tests (no real-clock dependency).
    pub fn immediate() -> Self {
        Self {
            attempts: 1,
            interval: std::time::Duration::ZERO,
        }
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
const UNREACHABLE_PRECONDITION: &str = "mcp-server cargo feature + ANDROMEDA_PULSE_MCP_ENABLED + ANDROMEDA_PULSE_DATA_DIR == live Pulse's data-dir";

/// The named precondition for a corpus that returns no incidents at all (arch §Standard Contracts).
/// A workspace-key divergence and a genuinely empty corpus are indistinguishable on the wire — the
/// sidecar keys its query on `ANDROMEDA_PULSE_DATA_DIR` while `pulse-app` keys incidents on its
/// detected workspace root, so a divergence returns zero rows forever — hence both causes are named
/// and neither is claimed as measured.
const WORKSPACE_KEY_PRECONDITION: &str = "pulse-app and the spawned MCP sidecar must resolve the same incident workspace key — the sidecar keys on ANDROMEDA_PULSE_DATA_DIR, pulse-app on its detected workspace root — or Pulse raised no incident for the canary";

/// The named precondition when incidents exist but every one predates the canary's emission — the
/// corpus is reachable and non-empty, yet this run's storm raised nothing, so read-back would be
/// grading residue (arch §Standard Contracts). Distinct from an empty corpus.
const CANARY_STALE_CORPUS_PRECONDITION: &str = "no incident opened after the canary storm was emitted — every incident in the corpus predates it, so Pulse did not raise one for this run";

/// The named precondition when the recorded run contract's terms are not satisfied (arch §Standard
/// Contracts). Each unmet term is named individually, carrying its condition AND its candidate
/// causes: Conductor launches no Pulse process, so a term about `pulse-app` is reported as a
/// condition to satisfy, never as a measurement Conductor cannot make.
const RUN_CONTRACT_PRECONDITION: &str = "unmet run-contract terms";

/// Compose the run-contract precondition, naming each unmet term individually.
fn run_contract_precondition(contract: &RunContractStatus) -> String {
    let terms: Vec<String> = contract
        .unmet()
        .iter()
        .map(|t| format!("[{}] {} — {}", t.id, t.statement, t.causes))
        .collect();
    format!("{RUN_CONTRACT_PRECONDITION}: {}", terms.join("; "))
}

/// Run the three readiness assertions over an already-connected client. Returns `Ok(ReadyState)` for
/// every readiness outcome (including `Blocked`); MCP call failures are caught into the relevant
/// `Blocked` leg — never propagated as `Err`, never a panic (the verdict/error wall).
#[tracing::instrument(name = "verify.readback.preflight", skip_all)]
pub async fn run_preflight(
    client: &ReadbackClient,
    manifest: &ContractManifest,
    contract: &RunContractStatus,
    canary: &CanaryMarker,
    data_dir: &str,
    poll: CanaryPoll,
) -> Result<ReadyState, VerifyError> {
    let checked_at = now_rfc3339();
    let data_dir = redact_value(data_dir).into_owned();
    let expected = manifest.expected_protocol_version.clone();

    let negotiated = client.negotiated_protocol_version().map(|v| v.to_string());
    let version_ok = negotiated.as_deref() == Some(expected.as_str());

    let (required_tools, tools_unverifiable): (BTreeMap<String, ToolPresence>, Option<String>) =
        match client.list_tools().await {
            Ok(tools) => {
                let map = manifest
                    .required_tools
                    .iter()
                    .map(|name| {
                        let present = tools.iter().any(|t| t == name);
                        let presence = if present {
                            ToolPresence::Present
                        } else {
                            ToolPresence::Absent
                        };
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
    let tools_ok = tools_unverifiable.is_none()
        && required_tools.values().all(|p| *p == ToolPresence::Present);

    // Fidelity is on the emitted fingerprint, not a title substring (Pulse scrubs titles): poll
    // `query_incident_list` for the storm's incident, then assert the fingerprint reads back from its
    // telemetry slice. A genuine call/transport/JSON-RPC error stays distinct from "not found yet" (the
    // masking this chunk's prerequisite fixed); both fold into the precondition cascade below.
    // An unmet launch condition explains a failed canary, so polling first would spend the whole
    // budget only to report the downstream symptom — the failure mode the workspace-key probe hit.
    let (canary_round_trip, canary_call_error, canary_cause) = if contract.is_satisfied() {
        poll_canary(client, canary, poll).await
    } else {
        (CanaryOutcome::Skipped, None, None)
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
    } else if !contract.is_satisfied() {
        Some(run_contract_precondition(contract))
    } else if let Some(reason) = canary_call_error {
        Some(format!("MCP read-back call failed: {reason}"))
    } else if canary_round_trip != CanaryOutcome::Ok {
        Some(
            canary_cause
                .unwrap_or(NotFound::EmptyCorpus)
                .precondition()
                .to_string(),
        )
    } else {
        None
    };

    let ready = blocked_precondition.is_none();
    if let Some(precondition) = &blocked_precondition {
        tracing::info!(
            state = ReportState::Blocked.label(),
            "preflight blocked: {precondition}"
        );
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
pub async fn preflight_boot(
    command: Command,
    manifest: &ContractManifest,
    contract: &RunContractStatus,
    canary: &CanaryMarker,
    data_dir: &str,
    poll: CanaryPoll,
) -> Result<ReadyState, VerifyError> {
    match ReadbackClient::connect_command(command).await {
        Ok(client) => run_preflight(&client, manifest, contract, canary, data_dir, poll).await,
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

/// Why the canary was not found — the two causes the precondition cascade keeps apart.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NotFound {
    /// The corpus returned no incidents at all.
    EmptyCorpus,
    /// Incidents exist, but every one predates the canary's emission.
    StaleCorpus,
}

impl NotFound {
    /// The named precondition this cause surfaces (arch §Standard Contracts).
    fn precondition(self) -> &'static str {
        match self {
            Self::EmptyCorpus => WORKSPACE_KEY_PRECONDITION,
            Self::StaleCorpus => CANARY_STALE_CORPUS_PRECONDITION,
        }
    }
}

/// The canary fidelity outcome for one read-back attempt.
enum CanaryFidelity {
    /// An incident opened after the canary storm was emitted.
    Ok,
    /// No incident yet, or none newer than the storm — retryable within the poll budget.
    NotYet(NotFound),
    /// A read-back call/transport/JSON-RPC error — not retryable (its own precondition).
    CallError(String),
}

/// Poll the canary fidelity check within the [`CanaryPoll`] budget, returning the
/// `(outcome, call_error, not-found cause)` the precondition cascade consumes. Only the "not yet" case
/// retries (Pulse ingest latency); a call error surfaces immediately.
async fn poll_canary(
    client: &ReadbackClient,
    canary: &CanaryMarker,
    poll: CanaryPoll,
) -> (CanaryOutcome, Option<String>, Option<NotFound>) {
    let attempts = poll.attempts.max(1);
    let mut last_cause = NotFound::EmptyCorpus;
    let mut witness = ShapeWitness::default();
    for attempt in 0..attempts {
        match assert_canary(client, canary, &mut witness).await {
            CanaryFidelity::Ok => return (CanaryOutcome::Ok, None, None),
            CanaryFidelity::CallError(reason) => {
                return (CanaryOutcome::Failed, Some(reason), None);
            }
            CanaryFidelity::NotYet(cause) => {
                last_cause = cause;
                if attempt + 1 < attempts {
                    tokio::time::sleep(poll.interval).await;
                }
            }
        }
    }
    (CanaryOutcome::Failed, None, Some(last_cause))
}

/// One-shot shape witness for the poll loop: the tool's key set is recorded the FIRST time it answers
/// — not once per attempt, which would emit one identical line per second of the poll budget, and not
/// on attempt 0 only, which would never witness an answer first reached late in the poll.
///
/// Only `query_incident_list` is witnessed here; the canary no longer calls
/// `retrieve_telemetry_slice`, whose shape witness lives on the per-check extraction path.
#[derive(Default)]
struct ShapeWitness {
    list: bool,
}

impl ShapeWitness {
    fn list(&mut self, value: &serde_json::Value) {
        if !std::mem::replace(&mut self.list, true) {
            log_observed_keys("query_incident_list", value);
        }
    }
}

/// One canary fidelity attempt: assert the corpus carries an incident opened AFTER the storm was
/// emitted, which proves both that the sidecar reads the live Pulse's corpus and that this run's storm
/// raised something — the two facts the gate exists to establish.
///
/// Freshness, not payload identity, is the carrier because no read-back field varies with what
/// Conductor emitted: titles are scrubbed, and every L4-authored field (`title` / `severity` /
/// `fingerprint` / `evidence_refs`) is a fixture constant under the deterministic-L4 mode a verifiable
/// Pulse runs in. The honest limit is that a concurrent unrelated incident inside the poll window
/// would also satisfy this.
async fn assert_canary(
    client: &ReadbackClient,
    canary: &CanaryMarker,
    witness: &mut ShapeWitness,
) -> CanaryFidelity {
    let list = match client.query_incident_list(None).await {
        Ok(value) => value,
        Err(e) => return CanaryFidelity::CallError(call_error_reason(&e)),
    };
    witness.list(&list);
    if incident_ids(&list).is_empty() {
        return CanaryFidelity::NotYet(NotFound::EmptyCorpus);
    }
    if opened_at_unix_nanos(&list)
        .iter()
        .any(|opened| *opened > canary.emitted_at_unix_nano)
    {
        return CanaryFidelity::Ok;
    }
    CanaryFidelity::NotYet(NotFound::StaleCorpus)
}

// `call_error_reason` / `incident_ids` / `opened_at_unix_nanos` live in `crate::extract` — the canary and
// the per-check extraction read the same Pulse shapes, so they read them through one definition.
