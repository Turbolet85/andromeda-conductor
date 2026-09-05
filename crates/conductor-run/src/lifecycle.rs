//! Runtime-STATE read-back fidelity: the `mark_incident_resolved` write and what it changes.
//!
//! Payload fidelity is out of reach under deterministic L4, but a write Conductor MAKES and the
//! consequence it observes are not — [`probe_resolve_lifecycle`] resolves an incident and reads the
//! active set back, and [`attribute_by_liveness`] excludes Pulse's own idle resolver by construction
//! (arch §Established Decisions — Read-Back Dependency Posture).






use conductor_verify::{
    ReadbackClient, VerifyError,
};

/// What one resolve-lifecycle probe observed: the active set before the write, the id it resolved,
/// and the active set after. The two sets are what the verdict is computed from — the write's own
/// `{resolved, incident_id}` answer says only that Pulse accepted the call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifecycleObservation {
    pub before: Vec<i64>,
    pub resolved: i64,
    pub after: Vec<i64>,
}

/// The verdict a lifecycle probe earns. The two `Proven*` arms attribute the active-set change to
/// Conductor's write; every other arm names why it cannot.
///
/// `Eq` is deliberately absent: `ProvenByLiveness` carries the measured idle seconds, and a float
/// has no total equality. Comparisons in tests use `PartialEq` on exact captured values.
#[derive(Debug, Clone, PartialEq)]
pub enum LifecycleVerdict {
    /// The resolved id left the active set AND a control id stayed — Pulse's own auto-resolver
    /// cannot produce this, because it would have to spare exactly the incident we did not write to.
    ///
    /// UNREACHABLE against Pulse as measured (2026-09-01, HEAD `83d4060`): a second concurrent
    /// incident cannot be formed, because the producer dedupes against any OPEN incident regardless
    /// of fingerprint. Kept because it is the stronger attribution if that ever changes.
    Proven { control: i64 },
    /// The resolved id left the active set while its telemetry was FRESH. Pulse auto-resolves only
    /// an incident idle for `AUTO_RESOLVE_IDLE_SECONDS`, so a removal inside that window cannot be
    /// the resolver's — which makes Conductor's write the only remaining cause. This is the
    /// attribution the single-incident reality actually supports.
    ProvenByLiveness { idle_seconds: f64 },
    /// The resolved id left, but no control survived. Indistinguishable from the 120s idle
    /// auto-resolve (30s resolver tick), so it proves nothing on its own.
    Unattributable,
    /// The resolved id is still active — the write did not take effect.
    StillActive,
    /// Fewer than two incidents were active, so no control could be held back.
    NoControl,
}

/// Choose the incident to resolve, keeping a control behind. Returns `(resolve, control)` — the
/// LAST id is resolved and the FIRST held, so the control is the older incident (typically the
/// preflight canary's, which is already open when a scenario's own storm forms).
///
/// Fewer than two active incidents yields `None`: a single-incident probe cannot separate Conductor's
/// write from Pulse's auto-resolver, and running it anyway would mint evidence that reads as proof.
pub fn select_resolve_target(active: &[i64]) -> Option<(i64, i64)> {
    match active {
        [control, .., resolve] => Some((*resolve, *control)),
        _ => None,
    }
}

/// Pulse's auto-resolve idle threshold. An incident whose telemetry is fresher than this cannot be
/// the resolver's to take, which is what lets a single-incident leg attribute the removal.
pub const AUTO_RESOLVE_IDLE_SECONDS: f64 = 120.0;

/// Grade a lifecycle observation by LIVENESS — the attribution available when only one incident can
/// be active at a time.
///
/// `idle_seconds` is the age of the resolved incident's most recent emission at the instant of the
/// write. Below [`AUTO_RESOLVE_IDLE_SECONDS`] the auto-resolver is excluded by construction, so the
/// id leaving the active set is attributable to Conductor's write. At or above it the observation is
/// real but `Unattributable`: the resolver could have produced exactly the same disappearance.
pub fn attribute_by_liveness(
    observation: &LifecycleObservation,
    idle_seconds: f64,
) -> LifecycleVerdict {
    if observation.before.is_empty() {
        return LifecycleVerdict::NoControl;
    }
    if observation.after.contains(&observation.resolved) {
        return LifecycleVerdict::StillActive;
    }
    if idle_seconds < AUTO_RESOLVE_IDLE_SECONDS {
        LifecycleVerdict::ProvenByLiveness { idle_seconds }
    } else {
        LifecycleVerdict::Unattributable
    }
}

/// Grade a lifecycle observation. The control's SURVIVAL is the load-bearing half: `query_incident_list`
/// is active-only, so a resolved incident leaves the surface — but so does an auto-resolved one, and
/// only a spared control separates the two.
pub fn evaluate_lifecycle(observation: &LifecycleObservation, control: i64) -> LifecycleVerdict {
    if observation.before.len() < 2 {
        return LifecycleVerdict::NoControl;
    }
    if observation.after.contains(&observation.resolved) {
        return LifecycleVerdict::StillActive;
    }
    if observation.after.contains(&control) {
        LifecycleVerdict::Proven { control }
    } else {
        LifecycleVerdict::Unattributable
    }
}

/// Read the active set, resolve one incident by id, and read it back — the first production caller of
/// `mark_incident_resolved`.
///
/// ORDERING: `query_incident_list` is active-only, so this empties what it resolves. Run it LAST in a
/// run, or in a run of its own; anything reading back afterwards sees the shrunken set.
///
/// A declined write is Pulse REFUSING, not a Conductor fault — it returns `Err(VerifyError::JsonRpc)`
/// for the caller to grade, never a panic.
pub async fn probe_resolve_lifecycle(
    client: &ReadbackClient,
    resolve: i64,
) -> Result<LifecycleObservation, VerifyError> {
    let before = active_incident_ids(client).await?;
    // The incident id is NOT in `conductor-core::redact::ALLOWLISTED_FIELDS`, so it rides the
    // allowlisted `message` rather than a span attribute the processor stage would drop.
    tracing::info!(message = %format!("resolve-lifecycle: resolving incident {resolve}"));
    let _ = client.resolve_incident(resolve).await?;
    let after = active_incident_ids(client).await?;
    Ok(LifecycleObservation { before, resolved: resolve, after })
}

/// The active incident ids from a `query_incident_list` result.
///
/// Accepts EITHER item key. The live sidecar emits `incident_id` (measured 2026-09-01 against Pulse
/// HEAD `83d4060`); the in-process stub emits `id`. Reading only one of them yields an empty list on
/// a populated corpus — a silent degrade indistinguishable downstream from a genuinely empty active
/// set, which is exactly how this reader shipped its first live leg. `conductor-verify`'s own
/// `incident_ids` already carries the same tolerance.
async fn active_incident_ids(client: &ReadbackClient) -> Result<Vec<i64>, VerifyError> {
    let list = client.query_incident_list(None).await?;
    Ok(list
        .get("items")
        .and_then(|v| v.as_array())
        .map(|items| {
            items
                .iter()
                .filter_map(|i| {
                    i.get("incident_id").or_else(|| i.get("id")).and_then(|v| v.as_i64())
                })
                .collect()
        })
        .unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testkit::*;

    #[test]
    fn liveness_attribution_turns_on_the_idle_threshold_exactly() {
        let observation = resolved_away(vec![7], 7);

        // Strictly BELOW the threshold Pulse's 120s auto-resolver is excluded by construction, so
        // the id leaving the active set is attributable to Conductor's write...
        assert_eq!(
            attribute_by_liveness(&observation, AUTO_RESOLVE_IDLE_SECONDS - 0.5),
            LifecycleVerdict::ProvenByLiveness { idle_seconds: AUTO_RESOLVE_IDLE_SECONDS - 0.5 }
        );

        // ...and AT the threshold it is not. This is the only value where `<` and `<=` disagree, so
        // it is the only case that separates the shipped bound from a relaxed one.
        assert_eq!(
            attribute_by_liveness(&observation, AUTO_RESOLVE_IDLE_SECONDS),
            LifecycleVerdict::Unattributable,
            "at the threshold the resolver could have produced the same disappearance"
        );
    }

    /// Serve a line-delimited JSON-RPC session over one end of an in-process duplex, answering
    /// `query_incident_list` with `items` — supplied independently of the request, because a stub
    /// that echoes what it was given answers a mutated reader as agreeably as a correct one
    /// (`.claude/rules/testing.md` 2026-08-20).
    async fn serve_incident_list(server_io: tokio::io::DuplexStream, items: Vec<i64>) {
        use tokio::io::{AsyncBufReadExt as _, AsyncWriteExt as _};

        let (reader, mut writer) = tokio::io::split(server_io);
        let mut lines = tokio::io::BufReader::new(reader).lines();
        while let Ok(Some(line)) = lines.next_line().await {
            let Ok(req) = serde_json::from_str::<serde_json::Value>(&line) else { continue };
            // A notification carries no id and earns no response.
            let Some(id) = req.get("id").cloned() else { continue };
            let result = match req.get("method").and_then(serde_json::Value::as_str) {
                Some("initialize") => serde_json::json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {},
                    "serverInfo": { "name": "stub", "version": "0" },
                }),
                _ => serde_json::json!({
                    "items": items
                        .iter()
                        .map(|id| serde_json::json!({ "incident_id": id }))
                        .collect::<Vec<_>>(),
                }),
            };
            let response = serde_json::json!({ "jsonrpc": "2.0", "id": id, "result": result });
            if writer.write_all(format!("{response}\n").as_bytes()).await.is_err() {
                break;
            }
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn active_incident_ids_reads_every_id_the_corpus_returns() {
        // The set is deliberately unlike each degenerate replacement — two entries, neither 0 nor 1
        // nor -1 — so `Ok(vec![])`, `Ok(vec![0])`, `Ok(vec![1])` and `Ok(vec![-1])` all differ from
        // it simultaneously.
        let expected = vec![41_i64, 42_i64];
        let (client_io, server_io) = tokio::io::duplex(8192);
        let server = tokio::spawn(serve_incident_list(server_io, expected.clone()));

        let client =
            ReadbackClient::connect_transport(client_io).await.expect("the stub session initializes");
        let ids = active_incident_ids(&client).await.expect("the active set reads back");

        drop(client);
        server.abort();

        assert_eq!(ids, expected, "every id the corpus returned must reach the caller");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn active_incident_ids_is_empty_only_when_the_corpus_is() {
        let (client_io, server_io) = tokio::io::duplex(8192);
        let server = tokio::spawn(serve_incident_list(server_io, Vec::new()));

        let client =
            ReadbackClient::connect_transport(client_io).await.expect("the stub session initializes");
        let ids = active_incident_ids(&client).await.expect("an empty active set reads back");

        drop(client);
        server.abort();

        assert!(ids.is_empty(), "an empty corpus yields an empty set, got {ids:?}");
    }
}
