//! The 2026-09-29 real-model drive series, pinned: each drive's capture block verbatim from its
//! committed `evidence/rm-capture-*.txt`, everything the capture printed after the rule record from its
//! first line to its last (the 2026-09-23 pin's cut). A `tests/` subdirectory module, so it is never a
//! test target of its own; the harvest holds each block byte-equal to its committed file.

/// One drive of the series: its ledger label, its committed capture's file name, and its block.
pub struct Drive {
    pub label: &'static str,
    pub file: &'static str,
    pub block: &'static str,
}

/// The committed captures' directory, relative to the workspace root.
pub const EVIDENCE: &str =
    "conductor-0.3.0/chunks/2026-09-29-diagnostic-quality-cluster-off-the-drift-pin/evidence";

pub const SERIES: [Drive; 6] = [
    Drive {
        label: "a1-pipeline-fault",
        file: "rm-capture-a1-pipeline-fault.txt",
        block: r######"real-model capture
run_id: 2026-09-29T15-56-51-623
preflight: preflight blocked: pulse-app and the spawned MCP sidecar must resolve the same incident workspace key — the sidecar keys on ANDROMEDA_PULSE_DATA_DIR, pulse-app on its detected workspace root — or Pulse raised no incident for the canary
preflight: preflight blocked: readiness gate not satisfied
emission: none
trace: spans=db.insert_run,emit.batch,report.generate,scenario.run,verify.readback.call_tool,verify.readback.connect,verify.readback.connect_command,verify.readback.list_tools,verify.readback.preflight wire_shape_lines=15 retrieve_report_witness=false
envelope: {"fingerprints":null,"journal_emitted_at":null,"latency_ms":null,"p_ids":["P-018","P-031","P-033","P-034","P-044"],"read_back_observed_at":null,"run_id":"2026-09-29T15-56-51-623","scenario":"real-model-interpretation","seed":4317033,"slo_tier":"<90s","state":"Blocked","verdict":null}
envelope fingerprints: 0, det- prefixed: 0
attribution: none (the scenario never emitted)
pulse-log file: agent-latest.jsonl.2026-09-29
pulse-log inference_mode: real
pulse-log workspace basename carries conductor: false
pulse-log bootstrap_window.override (whole file): 0
pulse-log window: 200093 lines since the leg's first self-obs line
pulse-log interpretation.prompt.assemble: 5
pulse-log   interpretation.prompt.assemble t=2026-09-29T15:56:55.609Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T15:57:36.810Z prompt_version=v2.2 token_count=6312 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T15:57:55.612Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T16:00:55.587Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T16:05:55.589Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log interpretation.json.parse: 0
pulse-log interpretation.incident.created: 0
pulse-log interpretation.inference.error: 5
pulse-log   interpretation.inference.error t=2026-09-29T15:56:55.609Z error_category=model_not_configured recovery_action=skip_digest model_tier=primary
pulse-log   interpretation.inference.error t=2026-09-29T15:57:36.810Z error_category=model_not_configured recovery_action=skip_digest model_tier=primary
pulse-log   interpretation.inference.error t=2026-09-29T15:57:55.612Z error_category=model_not_configured recovery_action=skip_digest model_tier=primary
pulse-log   interpretation.inference.error t=2026-09-29T16:00:55.587Z error_category=model_not_configured recovery_action=skip_digest model_tier=primary
pulse-log   interpretation.inference.error t=2026-09-29T16:05:55.590Z error_category=model_not_configured recovery_action=skip_digest model_tier=primary
pulse-log interpretation.inference.skipped: 7
pulse-log   interpretation.inference.skipped t=2026-09-29T15:58:55.591Z reason=backoff_active model_tier=primary backoff_seconds_remaining=60
pulse-log   interpretation.inference.skipped t=2026-09-29T15:59:55.601Z reason=backoff_active model_tier=primary backoff_seconds_remaining=0
pulse-log   interpretation.inference.skipped t=2026-09-29T16:01:55.590Z reason=backoff_active model_tier=primary backoff_seconds_remaining=239
pulse-log   interpretation.inference.skipped t=2026-09-29T16:02:55.589Z reason=backoff_active model_tier=primary backoff_seconds_remaining=179
pulse-log   interpretation.inference.skipped t=2026-09-29T16:03:55.588Z reason=backoff_active model_tier=primary backoff_seconds_remaining=119
pulse-log   interpretation.inference.skipped t=2026-09-29T16:04:55.591Z reason=backoff_active model_tier=primary backoff_seconds_remaining=59
pulse-log   interpretation.inference.skipped t=2026-09-29T16:06:55.586Z reason=backoff_active model_tier=primary backoff_seconds_remaining=60
pulse-log triage.pattern.storm.detected: 2
pulse-log   triage.pattern.storm.detected t=2026-09-29T15:57:36.779Z severity_hint=suggested occurrence_count=5 fingerprint_hex=<fingerprint>
pulse-log   triage.pattern.storm.detected t=2026-09-29T15:57:36.785Z severity_hint=autonomous occurrence_count=10 fingerprint_hex=<fingerprint>
pulse-log digest.runtime.cadence_tick: 12
pulse-log   digest.runtime.cadence_tick t=2026-09-29T15:56:55.594Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T15:57:36.798Z mode=tier1 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T15:57:55.600Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T15:58:55.585Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T15:59:55.590Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:00:55.579Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:01:55.584Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:02:55.582Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:03:55.582Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:04:55.583Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:05:55.582Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:06:55.579Z mode=tier3 cue_present=false
pulse-log digest.assemble.request: 12
pulse-log   digest.assemble.request t=2026-09-29T15:56:55.594Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T15:57:36.798Z mode=tier1 cue_kind=retry_storm cue_priority_tier=autonomous
pulse-log   digest.assemble.request t=2026-09-29T15:57:55.600Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T15:58:55.585Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T15:59:55.590Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:00:55.579Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:01:55.584Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:02:55.582Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:03:55.582Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:04:55.583Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:05:55.582Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:06:55.579Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log digest.corpus.retrieve: 12
pulse-log   digest.corpus.retrieve t=2026-09-29T15:56:55.608Z query_id=incidents_for_workspace_since row_count_returned=2 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T15:57:36.810Z query_id=incidents_for_workspace_since row_count_returned=2 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T15:57:55.612Z query_id=incidents_for_workspace_since row_count_returned=2 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T15:58:55.590Z query_id=incidents_for_workspace_since row_count_returned=2 duration_ms=1
pulse-log   digest.corpus.retrieve t=2026-09-29T15:59:55.601Z query_id=incidents_for_workspace_since row_count_returned=2 duration_ms=6
pulse-log   digest.corpus.retrieve t=2026-09-29T16:00:55.586Z query_id=incidents_for_workspace_since row_count_returned=2 duration_ms=3
pulse-log   digest.corpus.retrieve t=2026-09-29T16:01:55.590Z query_id=incidents_for_workspace_since row_count_returned=2 duration_ms=2
pulse-log   digest.corpus.retrieve t=2026-09-29T16:02:55.589Z query_id=incidents_for_workspace_since row_count_returned=2 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T16:03:55.588Z query_id=incidents_for_workspace_since row_count_returned=2 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T16:04:55.590Z query_id=incidents_for_workspace_since row_count_returned=2 duration_ms=2
pulse-log   digest.corpus.retrieve t=2026-09-29T16:05:55.589Z query_id=incidents_for_workspace_since row_count_returned=2 duration_ms=2
pulse-log   digest.corpus.retrieve t=2026-09-29T16:06:55.586Z query_id=incidents_for_workspace_since row_count_returned=2 duration_ms=1
pulse-log heartbeat ingest.tick (15 s): 44
creating digest prompt_version: unknown
creating digest corpus retrieval rows: unknown
canary: pipeline-fault t=2026-09-29T15:57:36.798Z cue_kind=retry_storm cue_priority_tier=autonomous parse=none created=none deduped=none
canary other cue-bearing digests: 0 ()
"######,
    },
    Drive {
        label: "a1",
        file: "rm-capture-a1.txt",
        block: r######"real-model capture
run_id: 2026-09-29T16-11-01-456
preflight: preflight blocked: pulse-app and the spawned MCP sidecar must resolve the same incident workspace key — the sidecar keys on ANDROMEDA_PULSE_DATA_DIR, pulse-app on its detected workspace root — or Pulse raised no incident for the canary
preflight: preflight blocked: readiness gate not satisfied
emission: none
trace: spans=db.insert_run,emit.batch,report.generate,scenario.run,verify.readback.call_tool,verify.readback.connect,verify.readback.connect_command,verify.readback.list_tools,verify.readback.preflight wire_shape_lines=15 retrieve_report_witness=false
envelope: {"fingerprints":null,"journal_emitted_at":null,"latency_ms":null,"p_ids":["P-018","P-031","P-033","P-034","P-044"],"read_back_observed_at":null,"run_id":"2026-09-29T16-11-01-456","scenario":"real-model-interpretation","seed":4317033,"slo_tier":"<90s","state":"Blocked","verdict":null}
envelope fingerprints: 0, det- prefixed: 0
attribution: none (the scenario never emitted)
pulse-log file: agent-latest.jsonl.2026-09-29
pulse-log inference_mode: real
pulse-log workspace basename carries conductor: false
pulse-log bootstrap_window.override (whole file): 0
pulse-log window: 202804 lines since the leg's first self-obs line
pulse-log interpretation.prompt.assemble: 4
pulse-log   interpretation.prompt.assemble t=2026-09-29T16:11:33.742Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T16:11:46.573Z prompt_version=v2.2 token_count=6312 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T16:12:33.742Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T16:13:33.728Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log interpretation.json.parse: 4
pulse-log   interpretation.json.parse t=2026-09-29T16:11:38.490Z parse_outcome=ok output_bytes=546 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T16:11:50.127Z parse_outcome=ok output_bytes=405 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T16:12:37.872Z parse_outcome=ok output_bytes=1007 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T16:13:38.428Z parse_outcome=ok output_bytes=1572 duration_ms=0
pulse-log interpretation.incident.created: 0
pulse-log interpretation.inference.error: 0
pulse-log interpretation.inference.skipped: 0
pulse-log triage.pattern.storm.detected: 2
pulse-log   triage.pattern.storm.detected t=2026-09-29T16:11:46.536Z severity_hint=suggested occurrence_count=5 fingerprint_hex=<fingerprint>
pulse-log   triage.pattern.storm.detected t=2026-09-29T16:11:46.545Z severity_hint=autonomous occurrence_count=10 fingerprint_hex=<fingerprint>
pulse-log digest.runtime.cadence_tick: 12
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:11:33.730Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:11:46.559Z mode=tier1 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:12:33.729Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:13:33.720Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:14:33.719Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:15:33.719Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:16:33.723Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:17:33.723Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:18:33.721Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:19:33.720Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:20:33.718Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:21:33.724Z mode=tier3 cue_present=false
pulse-log digest.assemble.request: 12
pulse-log   digest.assemble.request t=2026-09-29T16:11:33.730Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:11:46.559Z mode=tier1 cue_kind=retry_storm cue_priority_tier=autonomous
pulse-log   digest.assemble.request t=2026-09-29T16:12:33.729Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:13:33.720Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:14:33.719Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:15:33.719Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:16:33.723Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:17:33.723Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:18:33.721Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:19:33.720Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:20:33.718Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:21:33.724Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log digest.corpus.retrieve: 12
pulse-log   digest.corpus.retrieve t=2026-09-29T16:11:33.740Z query_id=incidents_for_workspace_since row_count_returned=2 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T16:11:46.573Z query_id=incidents_for_workspace_since row_count_returned=2 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T16:12:33.741Z query_id=incidents_for_workspace_since row_count_returned=2 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T16:13:33.728Z query_id=incidents_for_workspace_since row_count_returned=2 duration_ms=3
pulse-log   digest.corpus.retrieve t=2026-09-29T16:14:33.733Z query_id=incidents_for_workspace_since row_count_returned=2 duration_ms=9
pulse-log   digest.corpus.retrieve t=2026-09-29T16:15:33.734Z query_id=incidents_for_workspace_since row_count_returned=2 duration_ms=10
pulse-log   digest.corpus.retrieve t=2026-09-29T16:16:33.737Z query_id=incidents_for_workspace_since row_count_returned=2 duration_ms=9
pulse-log   digest.corpus.retrieve t=2026-09-29T16:17:33.728Z query_id=incidents_for_workspace_since row_count_returned=2 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T16:18:33.728Z query_id=incidents_for_workspace_since row_count_returned=2 duration_ms=3
pulse-log   digest.corpus.retrieve t=2026-09-29T16:19:33.727Z query_id=incidents_for_workspace_since row_count_returned=2 duration_ms=2
pulse-log   digest.corpus.retrieve t=2026-09-29T16:20:33.725Z query_id=incidents_for_workspace_since row_count_returned=2 duration_ms=1
pulse-log   digest.corpus.retrieve t=2026-09-29T16:21:33.731Z query_id=incidents_for_workspace_since row_count_returned=2 duration_ms=2
pulse-log heartbeat ingest.tick (15 s): 44
creating digest prompt_version: unknown
creating digest corpus retrieval rows: unknown
canary: dismissed t=2026-09-29T16:11:46.559Z cue_kind=retry_storm cue_priority_tier=autonomous parse=ok created=none deduped=none
canary other cue-bearing digests: 0 ()
"######,
    },
    Drive {
        label: "a2",
        file: "rm-capture-a2.txt",
        block: r######"real-model capture
run_id: 2026-09-29T16-24-08-168
emission_instant_ms: 1790699099338
trace: spans=db.insert_run,emit.batch,fault.silence,report.generate,scenario.run,timeline.execute,verify.readback.call_tool,verify.readback.connect,verify.readback.connect_command,verify.readback.list_tools,verify.readback.observe,verify.readback.preflight wire_shape_lines=51 retrieve_report_witness=false
envelope: {"fingerprints":[],"journal_emitted_at":"2026-09-29T16:24:59Z","latency_ms":136339,"p_ids":["P-018","P-031","P-033","P-034","P-044"],"read_back_observed_at":"2026-09-29T16:27:15Z","run_id":"2026-09-29T16-24-08-168","scenario":"real-model-interpretation","seed":4317033,"slo_tier":"<90s","state":"KnownResidual","verdict":null}
envelope fingerprints: 0, det- prefixed: 0
scenario cue fingerprint: <fingerprint>
polls: 61 over 600 s
incident: incident_id=1 opened_at_unix_nano=1790150921903387300 carries_scenario_fingerprint=false seen_active=false
incident: incident_id=2 opened_at_unix_nano=1790152780899282900 carries_scenario_fingerprint=false seen_active=false
incident: incident_id=3 opened_at_unix_nano=1790699093290706400 carries_scenario_fingerprint=false seen_active=false
attribution: none within 600s
pulse-log file: agent-latest.jsonl.2026-09-29
pulse-log inference_mode: real
pulse-log workspace basename carries conductor: false
pulse-log bootstrap_window.override (whole file): 0
pulse-log uptime at emission: 1744595 ms (the log's first line to Conductor's emission instant)
pulse-log window: 244162 lines since the leg's first self-obs line
pulse-log interpretation.prompt.assemble: 6
pulse-log   interpretation.prompt.assemble t=2026-09-29T16:24:33.740Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T16:24:53.290Z prompt_version=v2.2 token_count=6312 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T16:25:04.458Z prompt_version=v2.2 token_count=6312 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T16:25:33.736Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T16:26:33.737Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T16:28:33.729Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log interpretation.json.parse: 6
pulse-log   interpretation.json.parse t=2026-09-29T16:24:37.177Z parse_outcome=ok output_bytes=427 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T16:24:58.346Z parse_outcome=ok output_bytes=1598 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T16:25:08.338Z parse_outcome=ok output_bytes=553 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T16:25:38.538Z parse_outcome=ok output_bytes=1416 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T16:26:37.600Z parse_outcome=ok output_bytes=479 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T16:28:37.780Z parse_outcome=ok output_bytes=859 duration_ms=0
pulse-log interpretation.incident.created: 1
pulse-log   interpretation.incident.created t=2026-09-29T16:24:58.369Z created=true deduped=false severity=info priority_tier=curious
pulse-log interpretation.inference.error: 0
pulse-log interpretation.inference.skipped: 0
pulse-log triage.pattern.storm.detected: 4
pulse-log   triage.pattern.storm.detected t=2026-09-29T16:24:53.267Z severity_hint=suggested occurrence_count=5 fingerprint_hex=<fingerprint>
pulse-log   triage.pattern.storm.detected t=2026-09-29T16:24:53.273Z severity_hint=autonomous occurrence_count=10 fingerprint_hex=<fingerprint>
pulse-log   triage.pattern.storm.detected t=2026-09-29T16:25:01.895Z severity_hint=suggested occurrence_count=5 fingerprint_hex=<fingerprint>
pulse-log   triage.pattern.storm.detected t=2026-09-29T16:25:04.438Z severity_hint=autonomous occurrence_count=10 fingerprint_hex=<fingerprint>
pulse-log digest.runtime.cadence_tick: 15
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:24:33.728Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:24:53.281Z mode=tier1 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:25:04.448Z mode=tier1 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:25:33.725Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:26:33.727Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:27:33.725Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:28:33.723Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:29:33.727Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:30:33.728Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:31:33.727Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:32:33.726Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:33:33.721Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:34:33.723Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:35:33.731Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:36:33.729Z mode=tier3 cue_present=false
pulse-log digest.assemble.request: 15
pulse-log   digest.assemble.request t=2026-09-29T16:24:33.728Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:24:53.281Z mode=tier1 cue_kind=retry_storm cue_priority_tier=autonomous
pulse-log   digest.assemble.request t=2026-09-29T16:25:04.448Z mode=tier1 cue_kind=retry_storm cue_priority_tier=autonomous
pulse-log   digest.assemble.request t=2026-09-29T16:25:33.725Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:26:33.727Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:27:33.725Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:28:33.723Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:29:33.727Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:30:33.728Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:31:33.727Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:32:33.726Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:33:33.721Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:34:33.723Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:35:33.731Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:36:33.729Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log digest.corpus.retrieve: 15
pulse-log   digest.corpus.retrieve t=2026-09-29T16:24:33.739Z query_id=incidents_for_workspace_since row_count_returned=2 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T16:24:53.290Z query_id=incidents_for_workspace_since row_count_returned=2 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T16:25:04.458Z query_id=incidents_for_workspace_since row_count_returned=3 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T16:25:33.735Z query_id=incidents_for_workspace_since row_count_returned=3 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T16:26:33.737Z query_id=incidents_for_workspace_since row_count_returned=3 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T16:27:33.733Z query_id=incidents_for_workspace_since row_count_returned=3 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T16:28:33.729Z query_id=incidents_for_workspace_since row_count_returned=3 duration_ms=1
pulse-log   digest.corpus.retrieve t=2026-09-29T16:29:33.732Z query_id=incidents_for_workspace_since row_count_returned=3 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T16:30:33.735Z query_id=incidents_for_workspace_since row_count_returned=3 duration_ms=2
pulse-log   digest.corpus.retrieve t=2026-09-29T16:31:33.733Z query_id=incidents_for_workspace_since row_count_returned=3 duration_ms=1
pulse-log   digest.corpus.retrieve t=2026-09-29T16:32:33.732Z query_id=incidents_for_workspace_since row_count_returned=3 duration_ms=1
pulse-log   digest.corpus.retrieve t=2026-09-29T16:33:33.727Z query_id=incidents_for_workspace_since row_count_returned=3 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T16:34:33.730Z query_id=incidents_for_workspace_since row_count_returned=3 duration_ms=1
pulse-log   digest.corpus.retrieve t=2026-09-29T16:35:33.740Z query_id=incidents_for_workspace_since row_count_returned=3 duration_ms=3
pulse-log   digest.corpus.retrieve t=2026-09-29T16:36:33.735Z query_id=incidents_for_workspace_since row_count_returned=3 duration_ms=0
pulse-log heartbeat ingest.tick (15 s): 52
creating digest prompt_version: unknown
creating digest corpus retrieval rows: unknown
canary: surfaced t=2026-09-29T16:24:53.281Z cue_kind=retry_storm cue_priority_tier=autonomous parse=ok created=true deduped=false
canary other cue-bearing digests: 0 ()
"######,
    },
    Drive {
        label: "a3",
        file: "rm-capture-a3.txt",
        block: r######"real-model capture
run_id: 2026-09-29T16-38-27-074
emission_instant_ms: 1790699957194
trace: spans=db.insert_run,emit.batch,fault.silence,report.generate,scenario.run,timeline.execute,verify.readback.call_tool,verify.readback.connect,verify.readback.connect_command,verify.readback.list_tools,verify.readback.observe,verify.readback.preflight wire_shape_lines=51 retrieve_report_witness=true
envelope: {"fingerprints":["<model-text>","<model-text>","<model-text>"],"journal_emitted_at":"2026-09-29T16:39:17Z","latency_ms":136345,"p_ids":["P-018","P-031","P-033","P-034","P-044"],"read_back_observed_at":"2026-09-29T16:41:33Z","run_id":"2026-09-29T16-38-27-074","scenario":"real-model-interpretation","seed":4317033,"slo_tier":"<90s","state":"ManualCheck","verdict":null}
envelope fingerprints: 3, det- prefixed: 0
scenario cue fingerprint: <fingerprint>
polls: 61 over 600 s
incident: incident_id=6 opened_at_unix_nano=1790700037970451100 carries_scenario_fingerprint=false seen_active=true
attribution: none within 600s
pulse-log file: agent-latest.jsonl.2026-09-29
pulse-log inference_mode: real
pulse-log workspace basename carries conductor: false
pulse-log bootstrap_window.override (whole file): 0
pulse-log uptime at emission: 2602451 ms (the log's first line to Conductor's emission instant)
pulse-log window: 243922 lines since the leg's first self-obs line
pulse-log interpretation.prompt.assemble: 7
pulse-log   interpretation.prompt.assemble t=2026-09-29T16:38:33.740Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T16:39:12.179Z prompt_version=v2.2 token_count=6312 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T16:39:22.319Z prompt_version=v2.2 token_count=6312 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T16:39:33.741Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T16:40:33.741Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T16:40:37.970Z prompt_version=v1.1-reflection token_count=6860 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T16:42:33.738Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log interpretation.json.parse: 7
pulse-log   interpretation.json.parse t=2026-09-29T16:38:38.537Z parse_outcome=ok output_bytes=1524 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T16:39:16.725Z parse_outcome=ok output_bytes=1261 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T16:39:26.804Z parse_outcome=ok output_bytes=1160 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T16:39:38.156Z parse_outcome=ok output_bytes=975 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T16:40:37.970Z parse_outcome=ok output_bytes=940 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T16:40:42.464Z parse_outcome=ok output_bytes=1231 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T16:42:38.061Z parse_outcome=ok output_bytes=1069 duration_ms=0
pulse-log interpretation.incident.created: 3
pulse-log   interpretation.incident.created t=2026-09-29T16:39:16.747Z created=true deduped=false severity=info priority_tier=curious
pulse-log   interpretation.incident.created t=2026-09-29T16:39:26.825Z created=true deduped=false severity=error priority_tier=autonomous
pulse-log   interpretation.incident.created t=2026-09-29T16:40:42.485Z created=true deduped=false severity=info priority_tier=curious
pulse-log interpretation.inference.error: 0
pulse-log interpretation.inference.skipped: 0
pulse-log triage.pattern.storm.detected: 4
pulse-log   triage.pattern.storm.detected t=2026-09-29T16:39:12.155Z severity_hint=suggested occurrence_count=5 fingerprint_hex=<fingerprint>
pulse-log   triage.pattern.storm.detected t=2026-09-29T16:39:12.161Z severity_hint=autonomous occurrence_count=10 fingerprint_hex=<fingerprint>
pulse-log   triage.pattern.storm.detected t=2026-09-29T16:39:19.746Z severity_hint=suggested occurrence_count=5 fingerprint_hex=<fingerprint>
pulse-log   triage.pattern.storm.detected t=2026-09-29T16:39:22.299Z severity_hint=autonomous occurrence_count=10 fingerprint_hex=<fingerprint>
pulse-log digest.runtime.cadence_tick: 17
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:38:33.733Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:39:12.170Z mode=tier1 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:39:22.308Z mode=tier1 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:39:33.730Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:40:33.725Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:40:33.741Z mode=reflection cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:41:33.738Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:42:33.724Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:43:33.723Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:44:33.723Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:45:33.721Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:46:33.728Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:47:33.722Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:48:33.721Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:49:33.720Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:50:33.730Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T16:51:33.722Z mode=tier3 cue_present=false
pulse-log digest.assemble.request: 17
pulse-log   digest.assemble.request t=2026-09-29T16:38:33.733Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:39:12.170Z mode=tier1 cue_kind=retry_storm cue_priority_tier=autonomous
pulse-log   digest.assemble.request t=2026-09-29T16:39:22.308Z mode=tier1 cue_kind=retry_storm cue_priority_tier=autonomous
pulse-log   digest.assemble.request t=2026-09-29T16:39:33.730Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:40:33.725Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:40:33.741Z mode=reflection cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:41:33.738Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:42:33.724Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:43:33.723Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:44:33.723Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:45:33.721Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:46:33.728Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:47:33.722Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:48:33.721Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:49:33.720Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:50:33.730Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T16:51:33.722Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log digest.corpus.retrieve: 17
pulse-log   digest.corpus.retrieve t=2026-09-29T16:38:33.740Z query_id=incidents_for_workspace_since row_count_returned=3 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T16:39:12.178Z query_id=incidents_for_workspace_since row_count_returned=3 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T16:39:22.318Z query_id=incidents_for_workspace_since row_count_returned=4 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T16:39:33.740Z query_id=incidents_for_workspace_since row_count_returned=5 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T16:40:33.740Z query_id=incidents_for_workspace_since row_count_returned=5 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T16:40:33.752Z query_id=incidents_for_workspace_since row_count_returned=5 duration_ms=3
pulse-log   digest.corpus.retrieve t=2026-09-29T16:41:33.747Z query_id=incidents_for_workspace_since row_count_returned=6 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T16:42:33.738Z query_id=incidents_for_workspace_since row_count_returned=6 duration_ms=9
pulse-log   digest.corpus.retrieve t=2026-09-29T16:43:33.739Z query_id=incidents_for_workspace_since row_count_returned=6 duration_ms=11
pulse-log   digest.corpus.retrieve t=2026-09-29T16:44:33.738Z query_id=incidents_for_workspace_since row_count_returned=6 duration_ms=9
pulse-log   digest.corpus.retrieve t=2026-09-29T16:45:33.728Z query_id=incidents_for_workspace_since row_count_returned=6 duration_ms=3
pulse-log   digest.corpus.retrieve t=2026-09-29T16:46:33.735Z query_id=incidents_for_workspace_since row_count_returned=6 duration_ms=1
pulse-log   digest.corpus.retrieve t=2026-09-29T16:47:33.728Z query_id=incidents_for_workspace_since row_count_returned=6 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T16:48:33.727Z query_id=incidents_for_workspace_since row_count_returned=6 duration_ms=1
pulse-log   digest.corpus.retrieve t=2026-09-29T16:49:33.727Z query_id=incidents_for_workspace_since row_count_returned=6 duration_ms=2
pulse-log   digest.corpus.retrieve t=2026-09-29T16:50:33.735Z query_id=incidents_for_workspace_since row_count_returned=6 duration_ms=1
pulse-log   digest.corpus.retrieve t=2026-09-29T16:51:33.730Z query_id=incidents_for_workspace_since row_count_returned=6 duration_ms=2
pulse-log heartbeat ingest.tick (15 s): 53
creating digest prompt_version: v2.2
creating digest corpus retrieval rows: 4
canary: surfaced t=2026-09-29T16:39:12.170Z cue_kind=retry_storm cue_priority_tier=autonomous parse=ok created=true deduped=false
canary other cue-bearing digests: 0 ()
"######,
    },
    Drive {
        label: "b1",
        file: "rm-capture-b1.txt",
        block: r######"real-model capture
run_id: 2026-09-29T17-01-28-150
emission_instant_ms: 1790701513340
trace: spans=db.insert_run,emit.batch,fault.silence,report.generate,scenario.run,timeline.execute,verify.readback.call_tool,verify.readback.connect,verify.readback.connect_command,verify.readback.list_tools,verify.readback.observe,verify.readback.preflight wire_shape_lines=75 retrieve_report_witness=true
envelope: {"fingerprints":["<model-text>"],"journal_emitted_at":"2026-09-29T17:05:13Z","latency_ms":136455,"p_ids":["P-018","P-031","P-033","P-034","P-044"],"read_back_observed_at":"2026-09-29T17:07:29Z","run_id":"2026-09-29T17-01-28-150","scenario":"real-model-interpretation","seed":4317033,"slo_tier":"<90s","state":"ManualCheck","verdict":null}
envelope fingerprints: 1, det- prefixed: 0
scenario cue fingerprint: <fingerprint>
polls: 61 over 600 s
incident: incident_id=1 opened_at_unix_nano=1790150921903387300 carries_scenario_fingerprint=false seen_active=false
incident: incident_id=2 opened_at_unix_nano=1790152780899282900 carries_scenario_fingerprint=false seen_active=false
incident: incident_id=3 opened_at_unix_nano=1790699093290706400 carries_scenario_fingerprint=false seen_active=false
incident: incident_id=4 opened_at_unix_nano=1790699952179194500 carries_scenario_fingerprint=false seen_active=false
incident: incident_id=5 opened_at_unix_nano=1790699962319180900 carries_scenario_fingerprint=true seen_active=false
incident: incident_id=6 opened_at_unix_nano=1790700037970451100 carries_scenario_fingerprint=false seen_active=false
incident: incident_id=7 opened_at_unix_nano=1790701333296857100 carries_scenario_fingerprint=false seen_active=false
incident: incident_id=8 opened_at_unix_nano=1790701394234165100 carries_scenario_fingerprint=false seen_active=false
incident: incident_id=9 opened_at_unix_nano=1790701513365269200 carries_scenario_fingerprint=false seen_active=false
incident: incident_id=10 opened_at_unix_nano=1790701522102287300 carries_scenario_fingerprint=false seen_active=true
incident: incident_id=11 opened_at_unix_nano=1790701959232476200 carries_scenario_fingerprint=false seen_active=true
incident: incident_id=12 opened_at_unix_nano=1790702122228054000 carries_scenario_fingerprint=false seen_active=true
attribution: none within 600s
pulse-log file: agent-latest.jsonl.2026-09-29
pulse-log inference_mode: real
pulse-log workspace basename carries conductor: false
pulse-log bootstrap_window.override (whole file): 0
pulse-log uptime at emission: 4158597 ms (the log's first line to Conductor's emission instant)
pulse-log window: 298934 lines since the leg's first self-obs line
pulse-log interpretation.prompt.assemble: 24
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:01:50.242Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:02:13.296Z prompt_version=v2.2 token_count=6312 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:02:18.058Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:02:50.244Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:03:14.234Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:03:43.311Z prompt_version=v2.2 token_count=6312 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:03:50.249Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:04:15.238Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:04:50.241Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:05:13.365Z prompt_version=v2.2 token_count=6312 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:05:22.102Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:05:30.929Z prompt_version=v2.2 token_count=6312 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:05:50.240Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:06:15.243Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:06:50.242Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:08:17.236Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:08:50.229Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:09:17.239Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:09:41.248Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:12:39.232Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:12:50.238Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:13:20.232Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:15:22.228Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:16:22.236Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log interpretation.json.parse: 24
pulse-log   interpretation.json.parse t=2026-09-29T17:01:54.488Z parse_outcome=ok output_bytes=910 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:02:18.035Z parse_outcome=ok output_bytes=1250 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:02:21.654Z parse_outcome=ok output_bytes=480 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:02:55.616Z parse_outcome=ok output_bytes=2011 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:03:19.455Z parse_outcome=ok output_bytes=1778 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:03:47.076Z parse_outcome=ok output_bytes=410 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:03:54.987Z parse_outcome=ok output_bytes=1287 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:04:19.350Z parse_outcome=ok output_bytes=787 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:04:55.164Z parse_outcome=ok output_bytes=385 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:05:22.079Z parse_outcome=ok output_bytes=1033 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:05:30.906Z parse_outcome=ok output_bytes=1293 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:05:35.722Z parse_outcome=ok output_bytes=368 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:05:54.859Z parse_outcome=ok output_bytes=1105 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:06:19.661Z parse_outcome=ok output_bytes=1060 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:06:54.330Z parse_outcome=ok output_bytes=738 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:08:20.891Z parse_outcome=ok output_bytes=442 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:08:53.886Z parse_outcome=ok output_bytes=443 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:09:27.337Z parse_outcome=ok output_bytes=1596 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:09:48.757Z parse_outcome=ok output_bytes=1110 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:12:43.808Z parse_outcome=ok output_bytes=982 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:12:54.194Z parse_outcome=ok output_bytes=731 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:13:23.868Z parse_outcome=ok output_bytes=433 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:15:27.336Z parse_outcome=ok output_bytes=1588 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:16:29.352Z parse_outcome=ok output_bytes=1416 duration_ms=0
pulse-log interpretation.incident.created: 7
pulse-log   interpretation.incident.created t=2026-09-29T17:02:18.058Z created=true deduped=false severity=error priority_tier=autonomous
pulse-log   interpretation.incident.created t=2026-09-29T17:03:19.477Z created=true deduped=false severity=error priority_tier=autonomous
pulse-log   interpretation.incident.created t=2026-09-29T17:05:22.102Z created=true deduped=false severity=error priority_tier=autonomous
pulse-log   interpretation.incident.created t=2026-09-29T17:05:30.929Z created=true deduped=false severity=error priority_tier=autonomous
pulse-log   interpretation.incident.created t=2026-09-29T17:06:19.672Z created=false deduped=true severity=error priority_tier=autonomous
pulse-log   interpretation.incident.created t=2026-09-29T17:12:43.829Z created=true deduped=false severity=error priority_tier=autonomous
pulse-log   interpretation.incident.created t=2026-09-29T17:15:27.357Z created=true deduped=false severity=error priority_tier=autonomous
pulse-log interpretation.inference.error: 0
pulse-log interpretation.inference.skipped: 0
pulse-log triage.pattern.storm.detected: 8
pulse-log   triage.pattern.storm.detected t=2026-09-29T17:02:13.264Z severity_hint=suggested occurrence_count=5 fingerprint_hex=<fingerprint>
pulse-log   triage.pattern.storm.detected t=2026-09-29T17:02:13.272Z severity_hint=autonomous occurrence_count=10 fingerprint_hex=<fingerprint>
pulse-log   triage.pattern.storm.detected t=2026-09-29T17:03:43.280Z severity_hint=suggested occurrence_count=5 fingerprint_hex=<fingerprint>
pulse-log   triage.pattern.storm.detected t=2026-09-29T17:03:43.287Z severity_hint=autonomous occurrence_count=10 fingerprint_hex=<fingerprint>
pulse-log   triage.pattern.storm.detected t=2026-09-29T17:05:13.313Z severity_hint=suggested occurrence_count=5 fingerprint_hex=<fingerprint>
pulse-log   triage.pattern.storm.detected t=2026-09-29T17:05:13.327Z severity_hint=autonomous occurrence_count=10 fingerprint_hex=<fingerprint>
pulse-log   triage.pattern.storm.detected t=2026-09-29T17:05:15.972Z severity_hint=suggested occurrence_count=5 fingerprint_hex=<fingerprint>
pulse-log   triage.pattern.storm.detected t=2026-09-29T17:05:18.545Z severity_hint=autonomous occurrence_count=10 fingerprint_hex=<fingerprint>
pulse-log digest.runtime.cadence_tick: 44
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:01:50.229Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:02:13.285Z mode=tier1 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:02:14.232Z mode=tier2 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:02:50.232Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:03:14.228Z mode=tier2 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:03:43.300Z mode=tier1 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:03:50.239Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:04:15.227Z mode=tier2 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:04:50.227Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:05:13.348Z mode=tier1 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:05:15.251Z mode=tier2 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:05:18.577Z mode=tier1 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:05:50.231Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:06:15.234Z mode=tier2 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:06:50.232Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:07:16.238Z mode=tier2 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:07:50.225Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:08:17.228Z mode=tier2 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:08:50.222Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:09:17.230Z mode=tier2 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:09:41.234Z mode=tier2 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:09:50.238Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:10:18.223Z mode=tier2 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:10:41.232Z mode=tier2 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:10:50.229Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:11:19.221Z mode=tier2 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:11:42.226Z mode=tier2 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:11:50.221Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:12:19.228Z mode=tier2 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:12:39.227Z mode=tier1 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:12:50.224Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:13:20.227Z mode=tier2 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:13:40.226Z mode=tier1 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:13:50.224Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:14:21.234Z mode=tier2 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:14:41.229Z mode=tier1 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:14:50.222Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:15:22.222Z mode=tier2 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:15:41.232Z mode=tier1 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:15:50.223Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:16:22.230Z mode=tier2 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:16:41.245Z mode=tier1 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:16:50.231Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:17:23.242Z mode=tier2 cue_present=true
pulse-log digest.assemble.request: 44
pulse-log   digest.assemble.request t=2026-09-29T17:01:50.229Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T17:02:13.285Z mode=tier1 cue_kind=retry_storm cue_priority_tier=autonomous
pulse-log   digest.assemble.request t=2026-09-29T17:02:14.232Z mode=tier2 cue_kind=error_rate_spike cue_priority_tier=suggested
pulse-log   digest.assemble.request t=2026-09-29T17:02:50.232Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T17:03:14.228Z mode=tier2 cue_kind=error_rate_spike cue_priority_tier=suggested
pulse-log   digest.assemble.request t=2026-09-29T17:03:43.300Z mode=tier1 cue_kind=retry_storm cue_priority_tier=autonomous
pulse-log   digest.assemble.request t=2026-09-29T17:03:50.239Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T17:04:15.227Z mode=tier2 cue_kind=error_rate_spike cue_priority_tier=suggested
pulse-log   digest.assemble.request t=2026-09-29T17:04:50.227Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T17:05:13.348Z mode=tier1 cue_kind=retry_storm cue_priority_tier=autonomous
pulse-log   digest.assemble.request t=2026-09-29T17:05:15.251Z mode=tier2 cue_kind=error_rate_spike cue_priority_tier=suggested
pulse-log   digest.assemble.request t=2026-09-29T17:05:18.577Z mode=tier1 cue_kind=retry_storm cue_priority_tier=autonomous
pulse-log   digest.assemble.request t=2026-09-29T17:05:50.231Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T17:06:15.234Z mode=tier2 cue_kind=error_rate_spike cue_priority_tier=suggested
pulse-log   digest.assemble.request t=2026-09-29T17:06:50.232Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T17:07:16.238Z mode=tier2 cue_kind=error_rate_spike cue_priority_tier=suggested
pulse-log   digest.assemble.request t=2026-09-29T17:07:50.225Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T17:08:17.228Z mode=tier2 cue_kind=error_rate_spike cue_priority_tier=suggested
pulse-log   digest.assemble.request t=2026-09-29T17:08:50.222Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T17:09:17.230Z mode=tier2 cue_kind=error_rate_spike cue_priority_tier=suggested
pulse-log   digest.assemble.request t=2026-09-29T17:09:41.234Z mode=tier2 cue_kind=service_went_silent cue_priority_tier=suggested
pulse-log   digest.assemble.request t=2026-09-29T17:09:50.238Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T17:10:18.223Z mode=tier2 cue_kind=error_rate_spike cue_priority_tier=suggested
pulse-log   digest.assemble.request t=2026-09-29T17:10:41.233Z mode=tier2 cue_kind=service_went_silent cue_priority_tier=suggested
pulse-log   digest.assemble.request t=2026-09-29T17:10:50.229Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T17:11:19.221Z mode=tier2 cue_kind=error_rate_spike cue_priority_tier=suggested
pulse-log   digest.assemble.request t=2026-09-29T17:11:42.226Z mode=tier2 cue_kind=service_went_silent cue_priority_tier=suggested
pulse-log   digest.assemble.request t=2026-09-29T17:11:50.221Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T17:12:19.228Z mode=tier2 cue_kind=error_rate_spike cue_priority_tier=suggested
pulse-log   digest.assemble.request t=2026-09-29T17:12:39.227Z mode=tier1 cue_kind=service_went_silent cue_priority_tier=autonomous
pulse-log   digest.assemble.request t=2026-09-29T17:12:50.224Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T17:13:20.227Z mode=tier2 cue_kind=error_rate_spike cue_priority_tier=suggested
pulse-log   digest.assemble.request t=2026-09-29T17:13:40.226Z mode=tier1 cue_kind=service_went_silent cue_priority_tier=autonomous
pulse-log   digest.assemble.request t=2026-09-29T17:13:50.224Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T17:14:21.234Z mode=tier2 cue_kind=error_rate_spike cue_priority_tier=suggested
pulse-log   digest.assemble.request t=2026-09-29T17:14:41.229Z mode=tier1 cue_kind=service_went_silent cue_priority_tier=autonomous
pulse-log   digest.assemble.request t=2026-09-29T17:14:50.222Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T17:15:22.222Z mode=tier2 cue_kind=error_rate_spike cue_priority_tier=suggested
pulse-log   digest.assemble.request t=2026-09-29T17:15:41.232Z mode=tier1 cue_kind=service_went_silent cue_priority_tier=autonomous
pulse-log   digest.assemble.request t=2026-09-29T17:15:50.223Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T17:16:22.230Z mode=tier2 cue_kind=error_rate_spike cue_priority_tier=suggested
pulse-log   digest.assemble.request t=2026-09-29T17:16:41.245Z mode=tier1 cue_kind=service_went_silent cue_priority_tier=autonomous
pulse-log   digest.assemble.request t=2026-09-29T17:16:50.231Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T17:17:23.242Z mode=tier2 cue_kind=error_rate_spike cue_priority_tier=suggested
pulse-log digest.corpus.retrieve: 44
pulse-log   digest.corpus.retrieve t=2026-09-29T17:01:50.240Z query_id=incidents_for_workspace_since row_count_returned=6 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:02:13.296Z query_id=incidents_for_workspace_since row_count_returned=6 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:02:14.246Z query_id=incidents_for_workspace_since row_count_returned=6 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:02:50.244Z query_id=incidents_for_workspace_since row_count_returned=7 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:03:14.233Z query_id=incidents_for_workspace_since row_count_returned=7 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:03:43.310Z query_id=incidents_for_workspace_since row_count_returned=8 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:03:50.248Z query_id=incidents_for_workspace_since row_count_returned=8 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:04:15.237Z query_id=incidents_for_workspace_since row_count_returned=8 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:04:50.240Z query_id=incidents_for_workspace_since row_count_returned=8 duration_ms=4
pulse-log   digest.corpus.retrieve t=2026-09-29T17:05:13.364Z query_id=incidents_for_workspace_since row_count_returned=8 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:05:15.317Z query_id=incidents_for_workspace_since row_count_returned=8 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:05:18.615Z query_id=incidents_for_workspace_since row_count_returned=8 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:05:50.239Z query_id=incidents_for_workspace_since row_count_returned=10 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:06:15.243Z query_id=incidents_for_workspace_since row_count_returned=10 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:06:50.242Z query_id=incidents_for_workspace_since row_count_returned=10 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:07:16.247Z query_id=incidents_for_workspace_since row_count_returned=10 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:07:50.233Z query_id=incidents_for_workspace_since row_count_returned=10 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:08:17.235Z query_id=incidents_for_workspace_since row_count_returned=10 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:08:50.229Z query_id=incidents_for_workspace_since row_count_returned=10 duration_ms=2
pulse-log   digest.corpus.retrieve t=2026-09-29T17:09:17.239Z query_id=incidents_for_workspace_since row_count_returned=10 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:09:41.247Z query_id=incidents_for_workspace_since row_count_returned=10 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:09:50.245Z query_id=incidents_for_workspace_since row_count_returned=10 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:10:18.228Z query_id=incidents_for_workspace_since row_count_returned=10 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:10:41.237Z query_id=incidents_for_workspace_since row_count_returned=10 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:10:50.235Z query_id=incidents_for_workspace_since row_count_returned=10 duration_ms=1
pulse-log   digest.corpus.retrieve t=2026-09-29T17:11:19.226Z query_id=incidents_for_workspace_since row_count_returned=10 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:11:42.232Z query_id=incidents_for_workspace_since row_count_returned=10 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:11:50.227Z query_id=incidents_for_workspace_since row_count_returned=10 duration_ms=1
pulse-log   digest.corpus.retrieve t=2026-09-29T17:12:19.233Z query_id=incidents_for_workspace_since row_count_returned=10 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:12:39.232Z query_id=incidents_for_workspace_since row_count_returned=10 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:12:50.238Z query_id=incidents_for_workspace_since row_count_returned=11 duration_ms=8
pulse-log   digest.corpus.retrieve t=2026-09-29T17:13:20.232Z query_id=incidents_for_workspace_since row_count_returned=11 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:13:40.230Z query_id=incidents_for_workspace_since row_count_returned=11 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:13:50.230Z query_id=incidents_for_workspace_since row_count_returned=11 duration_ms=2
pulse-log   digest.corpus.retrieve t=2026-09-29T17:14:21.239Z query_id=incidents_for_workspace_since row_count_returned=11 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:14:41.233Z query_id=incidents_for_workspace_since row_count_returned=11 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:14:50.228Z query_id=incidents_for_workspace_since row_count_returned=11 duration_ms=1
pulse-log   digest.corpus.retrieve t=2026-09-29T17:15:22.227Z query_id=incidents_for_workspace_since row_count_returned=11 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:15:41.237Z query_id=incidents_for_workspace_since row_count_returned=12 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:15:50.231Z query_id=incidents_for_workspace_since row_count_returned=12 duration_ms=1
pulse-log   digest.corpus.retrieve t=2026-09-29T17:16:22.235Z query_id=incidents_for_workspace_since row_count_returned=12 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:16:41.263Z query_id=incidents_for_workspace_since row_count_returned=12 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:16:50.249Z query_id=incidents_for_workspace_since row_count_returned=12 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:17:23.252Z query_id=incidents_for_workspace_since row_count_returned=12 duration_ms=0
pulse-log heartbeat ingest.tick (15 s): 64
creating digest prompt_version: v2.2
creating digest corpus retrieval rows: 8
canary: pipeline-fault t=2026-09-29T17:02:13.285Z cue_kind=retry_storm cue_priority_tier=autonomous parse=none created=none deduped=none
canary: dismissed t=2026-09-29T17:03:43.300Z cue_kind=retry_storm cue_priority_tier=autonomous parse=ok created=none deduped=none
canary other cue-bearing digests: 3 (error_rate_spike,error_rate_spike,error_rate_spike)
"######,
    },
    Drive {
        label: "b2",
        file: "rm-capture-b2.txt",
        block: r######"real-model capture
run_id: 2026-09-29T17-19-35-933
emission_instant_ms: 1790702601159
trace: spans=db.insert_run,emit.batch,fault.silence,report.generate,scenario.run,timeline.execute,verify.readback.call_tool,verify.readback.connect,verify.readback.connect_command,verify.readback.list_tools,verify.readback.observe,verify.readback.preflight wire_shape_lines=75 retrieve_report_witness=true
envelope: {"fingerprints":["<model-text>","<model-text>"],"journal_emitted_at":"2026-09-29T17:23:21Z","latency_ms":136349,"p_ids":["P-018","P-031","P-033","P-034","P-044"],"read_back_observed_at":"2026-09-29T17:25:37Z","run_id":"2026-09-29T17-19-35-933","scenario":"real-model-interpretation","seed":4317033,"slo_tier":"<90s","state":"ManualCheck","verdict":null}
envelope fingerprints: 2, det- prefixed: 0
scenario cue fingerprint: <fingerprint>
polls: 1 over 0 s
incident: incident_id=1 opened_at_unix_nano=1790150921903387300 carries_scenario_fingerprint=false seen_active=false
incident: incident_id=2 opened_at_unix_nano=1790152780899282900 carries_scenario_fingerprint=false seen_active=false
incident: incident_id=3 opened_at_unix_nano=1790699093290706400 carries_scenario_fingerprint=false seen_active=false
incident: incident_id=4 opened_at_unix_nano=1790699952179194500 carries_scenario_fingerprint=false seen_active=false
incident: incident_id=5 opened_at_unix_nano=1790699962319180900 carries_scenario_fingerprint=true seen_active=false
incident: incident_id=6 opened_at_unix_nano=1790700037970451100 carries_scenario_fingerprint=false seen_active=false
incident: incident_id=7 opened_at_unix_nano=1790701333296857100 carries_scenario_fingerprint=false seen_active=false
incident: incident_id=8 opened_at_unix_nano=1790701394234165100 carries_scenario_fingerprint=false seen_active=false
incident: incident_id=9 opened_at_unix_nano=1790701513365269200 carries_scenario_fingerprint=false seen_active=false
incident: incident_id=10 opened_at_unix_nano=1790701522102287300 carries_scenario_fingerprint=false seen_active=false
incident: incident_id=11 opened_at_unix_nano=1790701959232476200 carries_scenario_fingerprint=false seen_active=false
incident: incident_id=12 opened_at_unix_nano=1790702122228054000 carries_scenario_fingerprint=false seen_active=false
incident: incident_id=13 opened_at_unix_nano=1790702517928854500 carries_scenario_fingerprint=false seen_active=true
incident: incident_id=14 opened_at_unix_nano=1790702545284808200 carries_scenario_fingerprint=false seen_active=false
incident: incident_id=15 opened_at_unix_nano=1790702606754859000 carries_scenario_fingerprint=true seen_active=true
attributed: incident_id=15 opened_at_unix_nano=1790702606754859000 degraded_mode=false pickup_ms=5595
## Symptom

Credit card data was leaked from a user's account.

## Timeline

09/23/2026 09:38:40 - Credit card data leaked from user's account. 09/23/2026 09:38:42 - Initial alert sent to developers.

## Hypotheses

- **(high)** The credit card data was leaked due to a security vulnerability in the application.
  - The vulnerability was identified in the application's code and is a known issue in the current version of the software.
- **(medium)** The credit card data was leaked due to a misconfiguration of the application's security settings.
  - The misconfiguration was identified in the application's configuration files and is a possible cause of the data leak.
- **(low)** The credit card data was leaked due to a user error.
  - The user error was identified in the application's logs and is a possible cause of the data leak, but is less likely than the other two hypotheses.

## Investigation Steps

1. Run a security audit of the application to identify the source of the data leak.
   - _Expected yield:_ A list of vulnerabilities found in the application's code and configuration files.
2. Verify the security settings of the application to ensure they are properly configured.
   - _Expected yield:_ A confirmation that the security settings are properly configured.
3. Review the application's logs to identify any user errors that may have contributed to the data leak.
   - _Expected yield:_ A list of user errors identified in the application's logs.

## Evidence

- `<fingerprint>`

## Project Context

workspace=[redacted: credit_card]

## Previously Seen

- incident #5 @ 1790699962319180900 — Credit Card Issue ([redacted: credit_card])

-- end of report sections --
pickup: 5595 ms (Pulse opened_at, ns, less Conductor's emission instant, ms): digest pickup, inference excluded
pulse-log file: agent-latest.jsonl.2026-09-29
pulse-log inference_mode: real
pulse-log workspace basename carries conductor: false
pulse-log bootstrap_window.override (whole file): 0
pulse-log uptime at emission: 5246416 ms (the log's first line to Conductor's emission instant)
pulse-log window: 112678 lines since the leg's first self-obs line
pulse-log interpretation.prompt.assemble: 13
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:19:50.270Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:20:21.182Z prompt_version=v2.2 token_count=6312 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:20:28.180Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:20:50.239Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:21:24.242Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:21:50.255Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:21:57.928Z prompt_version=v2.2 token_count=6312 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:22:25.284Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:22:50.262Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:23:21.186Z prompt_version=v2.2 token_count=6312 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:23:26.754Z prompt_version=v2.2 token_count=6312 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:23:50.266Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-29T17:24:50.260Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log interpretation.json.parse: 13
pulse-log   interpretation.json.parse t=2026-09-29T17:19:59.825Z parse_outcome=ok output_bytes=555 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:20:28.180Z parse_outcome=ok output_bytes=374 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:20:35.510Z parse_outcome=ok output_bytes=663 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:20:54.439Z parse_outcome=ok output_bytes=596 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:21:30.700Z parse_outcome=ok output_bytes=428 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:21:57.928Z parse_outcome=ok output_bytes=1039 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:22:07.374Z parse_outcome=ok output_bytes=1801 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:22:34.792Z parse_outcome=ok output_bytes=1212 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:22:57.376Z parse_outcome=ok output_bytes=639 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:23:26.742Z parse_outcome=ok output_bytes=1941 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:23:32.975Z parse_outcome=ok output_bytes=2206 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:24:01.506Z parse_outcome=ok output_bytes=1311 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-29T17:24:57.902Z parse_outcome=ok output_bytes=1225 duration_ms=0
pulse-log interpretation.incident.created: 4
pulse-log   interpretation.incident.created t=2026-09-29T17:22:07.402Z created=true deduped=false severity=warn priority_tier=suggested
pulse-log   interpretation.incident.created t=2026-09-29T17:22:34.817Z created=true deduped=false severity=error priority_tier=autonomous
pulse-log   interpretation.incident.created t=2026-09-29T17:23:26.754Z created=false deduped=true severity=error priority_tier=autonomous
pulse-log   interpretation.incident.created t=2026-09-29T17:23:32.999Z created=true deduped=false severity=error priority_tier=autonomous
pulse-log interpretation.inference.error: 0
pulse-log interpretation.inference.skipped: 0
pulse-log triage.pattern.storm.detected: 8
pulse-log   triage.pattern.storm.detected t=2026-09-29T17:20:21.111Z severity_hint=suggested occurrence_count=5 fingerprint_hex=<fingerprint>
pulse-log   triage.pattern.storm.detected t=2026-09-29T17:20:21.130Z severity_hint=autonomous occurrence_count=10 fingerprint_hex=<fingerprint>
pulse-log   triage.pattern.storm.detected t=2026-09-29T17:21:51.140Z severity_hint=suggested occurrence_count=5 fingerprint_hex=<fingerprint>
pulse-log   triage.pattern.storm.detected t=2026-09-29T17:21:51.158Z severity_hint=autonomous occurrence_count=10 fingerprint_hex=<fingerprint>
pulse-log   triage.pattern.storm.detected t=2026-09-29T17:23:21.156Z severity_hint=suggested occurrence_count=5 fingerprint_hex=<fingerprint>
pulse-log   triage.pattern.storm.detected t=2026-09-29T17:23:21.164Z severity_hint=autonomous occurrence_count=10 fingerprint_hex=<fingerprint>
pulse-log   triage.pattern.storm.detected t=2026-09-29T17:23:23.720Z severity_hint=suggested occurrence_count=5 fingerprint_hex=<fingerprint>
pulse-log   triage.pattern.storm.detected t=2026-09-29T17:23:26.268Z severity_hint=autonomous occurrence_count=10 fingerprint_hex=<fingerprint>
pulse-log digest.runtime.cadence_tick: 13
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:19:50.244Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:20:21.152Z mode=tier1 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:20:24.244Z mode=tier2 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:20:50.229Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:21:24.230Z mode=tier2 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:21:50.240Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:21:51.174Z mode=tier1 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:22:25.252Z mode=tier2 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:22:50.240Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:23:21.172Z mode=tier1 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:23:26.277Z mode=tier1 cue_present=true
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:23:50.240Z mode=tier3 cue_present=false
pulse-log   digest.runtime.cadence_tick t=2026-09-29T17:24:50.238Z mode=tier3 cue_present=false
pulse-log digest.assemble.request: 13
pulse-log   digest.assemble.request t=2026-09-29T17:19:50.244Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T17:20:21.152Z mode=tier1 cue_kind=retry_storm cue_priority_tier=autonomous
pulse-log   digest.assemble.request t=2026-09-29T17:20:24.244Z mode=tier2 cue_kind=error_rate_spike cue_priority_tier=suggested
pulse-log   digest.assemble.request t=2026-09-29T17:20:50.229Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T17:21:24.230Z mode=tier2 cue_kind=error_rate_spike cue_priority_tier=suggested
pulse-log   digest.assemble.request t=2026-09-29T17:21:50.240Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T17:21:51.174Z mode=tier1 cue_kind=retry_storm cue_priority_tier=autonomous
pulse-log   digest.assemble.request t=2026-09-29T17:22:25.252Z mode=tier2 cue_kind=error_rate_spike cue_priority_tier=suggested
pulse-log   digest.assemble.request t=2026-09-29T17:22:50.240Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T17:23:21.172Z mode=tier1 cue_kind=retry_storm cue_priority_tier=autonomous
pulse-log   digest.assemble.request t=2026-09-29T17:23:26.277Z mode=tier1 cue_kind=retry_storm cue_priority_tier=autonomous
pulse-log   digest.assemble.request t=2026-09-29T17:23:50.240Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log   digest.assemble.request t=2026-09-29T17:24:50.238Z mode=tier3 cue_kind= cue_priority_tier=
pulse-log digest.corpus.retrieve: 13
pulse-log   digest.corpus.retrieve t=2026-09-29T17:19:50.269Z query_id=incidents_for_workspace_since row_count_returned=12 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:20:21.181Z query_id=incidents_for_workspace_since row_count_returned=12 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:20:24.264Z query_id=incidents_for_workspace_since row_count_returned=12 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:20:50.238Z query_id=incidents_for_workspace_since row_count_returned=12 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:21:24.241Z query_id=incidents_for_workspace_since row_count_returned=12 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:21:50.255Z query_id=incidents_for_workspace_since row_count_returned=12 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:21:51.190Z query_id=incidents_for_workspace_since row_count_returned=12 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:22:25.283Z query_id=incidents_for_workspace_since row_count_returned=13 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:22:50.261Z query_id=incidents_for_workspace_since row_count_returned=14 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:23:21.185Z query_id=incidents_for_workspace_since row_count_returned=14 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:23:26.286Z query_id=incidents_for_workspace_since row_count_returned=14 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:23:50.264Z query_id=incidents_for_workspace_since row_count_returned=15 duration_ms=0
pulse-log   digest.corpus.retrieve t=2026-09-29T17:24:50.259Z query_id=incidents_for_workspace_since row_count_returned=15 duration_ms=0
pulse-log heartbeat ingest.tick (15 s): 24
creating digest prompt_version: v2.2
creating digest corpus retrieval rows: 14
canary: pipeline-fault t=2026-09-29T17:20:21.152Z cue_kind=retry_storm cue_priority_tier=autonomous parse=none created=none deduped=none
canary: surfaced t=2026-09-29T17:21:51.174Z cue_kind=retry_storm cue_priority_tier=autonomous parse=ok created=true deduped=false
canary other cue-bearing digests: 3 (error_rate_spike,error_rate_spike,error_rate_spike)
pickup check: pickup_ms >= 0: true
"######,
    },
];
