//! Live-leg evidence harvest for the four delegated timing budgets (P-025, P-027, P-037, P-045).
//!
//! Pulse delegated these bounds to Conductor and, at SUT HEAD `f0c38f5`, emits each one as a
//! `tracing` event behind its OWN exact allowlist leaf. None of them reaches an MCP read-back
//! surface — Pulse's 8 tools read the corpus and the in-memory buffer, never its own self-obs
//! stream — so, exactly as the storm / baseline / restart / pii / connection / severity harvests do,
//! the bounds grade HERE, over a live-leg capture of `{data_dir}/logs/agent-latest.jsonl.<date>`.
//!
//! Two measured facts shape everything below.
//!
//!   * The FIELD NAME IS NOT UNIFORM. The three leaves added for this delegation carry the duration
//!     in `duration_ms`; the pre-existing `metric.report.render_ms` carries it in `value`. A reader
//!     that assumes one field reads three of four and silently records the fourth as absent — which
//!     is indistinguishable from a leg where it never fired. `bounds()` pins the field per target
//!     and `render_ms_is_not_carried_in_duration_ms` is the negative test that keeps it honest.
//!   * `budget_ms` IS THE WRONG INSTRUMENT and is deliberately not used. It narrows the deadline on
//!     `read_back_observed_at − journal_emitted_at` (`conductor-verify/src/slo.rs`), i.e. Conductor's
//!     own MCP round-trip. These four budgets bound a duration measured INSIDE Pulse, so grading one
//!     against the other would measure Conductor's read-back speed and label it Pulse's render time.
//!
//! ABSENCE IS NEVER A PASS: a bound with no observation grades `Err`, never a satisfied budget —
//! the degrade direction the read-back freshness work settled.
//!
//! ALL FOUR bounds grade HARD here, each by the same worst-observation `grade()`. Three were
//! MEASURED 2026-08-21 over four live legs (fresh data dir + `pulse-app` restart per leg, window
//! open) against Pulse HEAD `f0c38f5`, and are met:
//!
//!   * P-027 constellation discovery — 702.4ms against 5000ms, at `discovered_count: 3` (the
//!     scenario's own three-service topology, which is what attributes it away from the canary).
//!   * P-037 report render — 0-1ms against 2000ms across four samples, every one `degraded_mode`.
//!   * P-045 counter refresh — 269 samples in leg D's window alone (median 1.9ms, max 7.0ms), zero
//!     over the 1000ms budget; ~1000 more across the other three legs, likewise none over.
//!
//! P-025 hue update grades at Pulse HEAD `226554a` (which carries the P-025 fix `e98d838`) under the
//! NEW observable: `duration_ms` is now the paint instant minus the service's tier-effective instant,
//! one sample per changed service, witnessed only. It is graded over the leg window by
//! `grade_in_window` under the rule `contracts/pulse-p025-measurement-contract.md` §The grading rule
//! states, with each rise sample anchored to its incident's creation line.
//!
//! The file also keeps the RETIRED instrument's record. At Pulse HEAD `83d4060` the same leaf
//! reported `t_sample - t_last_refreshing_tick` (TICK QUANTIZATION, U(0, 15s), independent of the
//! dispatch rate), and the 2026-08-21 and 2026-09-07 legs read 35581ms, 36705ms and 14525.9ms. Those
//! readings are true measurements of what that leaf emitted then; the contract and obs-plan §4 cite
//! them, so `p025_hue_update_is_recorded_over_budget_never_asserted_as_a_pass` and
//! `p025_the_re_driven_leg_measures_tick_quantization_not_update_latency` still pin them.
//!
//! TEST-ONLY affordance (the storm-harvest precedent): nothing here is wired into the run path and
//! nothing harvested reaches a Conductor artifact.

mod evidence_pin;

mod delegated_timing_grading;
use delegated_timing_grading::*;

#[cfg(test)]
mod tests {
    use super::*;

    /// The derivation reproduces the 2026-09-29 leg's hand-derived window from that leg's own
    /// committed self-obs — the known positive the round's window rests on.
    #[test]
    fn round_the_p025_window_derives_from_frozen_self_obs() {
        let selfobs = evidence_pin::committed(
            "conductor-0.3.0/chunks/2026-09-29-hue-shift-budget-graded-hard/evidence/h.jsonl",
        );
        assert_eq!(
            p025_window(&selfobs),
            Some((1_790_716_226_860, 1_790_716_380_720))
        );
    }

    #[test]
    fn round_a_self_obs_without_one_start_and_one_close_yields_no_window() {
        let start = r#"{"span":"timeline.execute","span_event":"new","timestamp_ms":1000}"#;
        let child = r#"{"parent":"timeline.execute","span":"emit.batch","span_event":"new","timestamp_ms":1500}"#;
        let close = r#"{"span":"scenario.run","span_event":"close","timestamp_ms":90000}"#;
        assert_eq!(
            p025_window(&[start, child, close].join("\n")),
            Some((31_000, 90_000))
        );
        assert_eq!(p025_window(&[child, close].join("\n")), None);
        assert_eq!(p025_window(&[start, start, close].join("\n")), None);
    }

    // ---- the P-075 round, 2026-10-02, Pulse S `03ec944` (round-request assertions 3-6) ----
    // One `pulse-app` launch, deterministic L4, fresh data dir, the compact widget visible and no
    // desktop input. Each slice is Pulse's log between its leg's pre-leg count and the next leg's,
    // filtered by target; run ids, counts and the censuses are in `evidence/round-ledger.md`.

    const ROUND_EVIDENCE: &str =
        "conductor-0.3.0/chunks/2026-10-02-p-075-assert-round-against-pulse/evidence/";

    fn round_lines(name: &str, sha256: &str) -> Vec<String> {
        evidence_pin::pinned(&format!("{ROUND_EVIDENCE}{name}"), sha256)
            .lines()
            .map(str::to_owned)
            .collect()
    }

    /// ROUND-REQUEST ASSERTION 3 (P-025), AS MEASURED: leg H's window, derived from its frozen
    /// self-obs by the contract's §The grading rule, holds a 438.24 ms rise (`autonomous`) and a
    /// 478.56 ms fall (`none`); the worst is 478.56 ms against 2 000 ms. The rise's start instant lands
    /// 38.24 ms from its incident's creation line.
    #[test]
    fn p075_round_assertion_3_p025_hue_update() {
        let selfobs = evidence_pin::pinned(
            &format!("{ROUND_EVIDENCE}h.jsonl"),
            "21c6520a5a0d10014d871a8a4a6120ab8054514d5d4097244d6909d7b3824332",
        );
        let window = p025_window(&selfobs).expect("one start and one close");
        assert_eq!(window, (1_790_918_907_230, 1_790_919_061_263));
        let lines = round_lines(
            "pulse-h.jsonl",
            "62791cd31ff2cdc40e2c15dfe801365c120cf5c9c75bb9d0a619603750942851",
        );
        let bound = bounds()[0];
        assert_eq!(
            grade_in_window(&lines, &bound, window),
            Ok(478.557_861_328_125)
        );
        let rises = tiered_hue_samples_in_window(&lines, &bound, window);
        assert_eq!(rises.len(), 1, "one rise in the window: {rises:?}");
        let error = incident_anchor_error_ms(&rises[0], &lines).expect("a fresh incident precedes");
        assert!(error <= P025_ANCHOR_TOLERANCE_MS, "anchor error {error}ms");
        assert!(
            (error - 38.236).abs() < 0.01,
            "the measured anchor error: {error}"
        );
    }

    /// ROUND-REQUEST ASSERTION 4 (P-027), AS MEASURED: leg D's slice holds two first-sighting
    /// discovery samples (489.07 ms at `discovered_count: 2`, 605.26 ms at 1); worst 605.26 ms
    /// against 5 000 ms.
    #[test]
    fn p075_round_assertion_4_p027_discovery() {
        let lines = round_lines(
            "pulse-d.jsonl",
            "a5ffacd9df730a91d9853c620bbcf96587aade04e16c2ed22948cd7162f86c7f",
        );
        assert_eq!(grade(&lines, &bounds()[1]), Ok(605.262_207_031_25));
    }

    /// ROUND-REQUEST ASSERTION 5 (P-037), AS MEASURED: leg R's slice holds one render sample, 0 ms in
    /// `value`, carrying `degraded_mode: false` — fired by the Report webview's own selection, no click.
    #[test]
    fn p075_round_assertion_5_p037_report_render() {
        let lines = round_lines(
            "pulse-r.jsonl",
            "75e8de2942418d5bffb9ea4b806edc338fc185223ba164b88252f8feb2849576",
        );
        assert_eq!(grade(&lines, &bounds()[2]), Ok(0.0));
    }

    /// ROUND-REQUEST ASSERTION 6 (P-045), AS MEASURED: leg F's slice holds 163 counter-refresh
    /// samples; the worst is 5.0 ms against 1 000 ms.
    #[test]
    fn p075_round_assertion_6_p045_counter_refresh() {
        let lines = round_lines(
            "pulse-f.jsonl",
            "57776de49989020ec58d61eacb3eac680f2fac747f515b58dbc1945327275565",
        );
        assert_eq!(observations(&lines, &bounds()[3]).len(), 163);
        assert_eq!(grade(&lines, &bounds()[3]), Ok(5.0));
    }

    #[test]
    fn p075_round_slices_digest_pin_fails_on_a_tampered_byte() {
        let name = format!("{ROUND_EVIDENCE}pulse-r.jsonl");
        let text = evidence_pin::committed(&name);
        let pin = "75e8de2942418d5bffb9ea4b806edc338fc185223ba164b88252f8feb2849576";
        assert!(evidence_pin::check_digest(&name, &text, pin).is_ok());
        let tampered = text.replacen("\"value\":0", "\"value\":9", 1);
        assert_ne!(tampered, text, "the tamper landed");
        assert!(evidence_pin::check_digest(&name, &tampered, pin).is_err());
    }

    // ---- the P-075 re-round, 2026-10-03, Pulse S2 `cdb6c1e` (round-request assertions 3-6) ----
    // One `pulse-app` launch on the Linux host (deterministic L4, `WEBKIT_DISABLE_DMABUF_RENDERER=1`,
    // fresh data dir), the compact widget visible and no desktop input; slices cut exactly as the
    // prior round's, and the run ids, counts and censuses are in that chunk's `evidence/round-ledger.md`.

    const REROUND_EVIDENCE: &str =
        "conductor-0.3.0/chunks/2026-10-03-p-075-re-round-on-incident-events/evidence/";

    fn reround_lines(name: &str, sha256: &str) -> Vec<String> {
        evidence_pin::pinned(&format!("{REROUND_EVIDENCE}{name}"), sha256)
            .lines()
            .map(str::to_owned)
            .collect()
    }

    /// ROUND-REQUEST ASSERTION 3 (P-025), AS MEASURED AT S2: leg H's window holds one sample, a
    /// 128.45 ms rise (`autonomous`) whose start instant lands 0.55 ms from its incident's creation
    /// line; the leg's 930.82 ms fall is stamped 13 s after `scenario.run` closed, outside the window
    /// the contract's grading rule defines.
    #[test]
    fn p075_reround_assertion_3_p025_hue_update() {
        let selfobs = evidence_pin::pinned(
            &format!("{REROUND_EVIDENCE}h.jsonl"),
            "5eea3c19012ccb05b83775de6119b429f79b924f1ce80b473c9f76044fedf01d",
        );
        let window = p025_window(&selfobs).expect("one start and one close");
        assert_eq!(window, (1_791_068_892_902, 1_791_069_043_736));
        let lines = reround_lines(
            "pulse-h.jsonl",
            "cdc2ecc3dc1170fa617878e309ebdcee9007609dca9ba7868719cb1b05e0f2a9",
        );
        let bound = bounds()[0];
        assert_eq!(
            grade_in_window(&lines, &bound, window),
            Ok(128.450_195_312_5)
        );
        let rises = tiered_hue_samples_in_window(&lines, &bound, window);
        assert_eq!(rises.len(), 1, "one rise in the window: {rises:?}");
        let error = incident_anchor_error_ms(&rises[0], &lines).expect("a fresh incident precedes");
        assert!(error <= P025_ANCHOR_TOLERANCE_MS, "anchor error {error}ms");
        assert!(
            (error - 0.550).abs() < 0.01,
            "the measured anchor error: {error}"
        );
    }

    /// ROUND-REQUEST ASSERTION 4 (P-027), AS MEASURED AT S2: leg D's slice holds two first-sighting
    /// discovery samples (672.27 ms at `discovered_count: 2`, 715.54 ms at 1); worst 715.54 ms
    /// against 5 000 ms.
    #[test]
    fn p075_reround_assertion_4_p027_discovery() {
        let lines = reround_lines(
            "pulse-d.jsonl",
            "18e443b4f9f3a51b623788dd130de011abe5ce648e27a9ea30c21327949ce6e8",
        );
        assert_eq!(observations(&lines, &bounds()[1]).len(), 2);
        assert_eq!(grade(&lines, &bounds()[1]), Ok(715.539_306_640_625));
    }

    /// ROUND-REQUEST ASSERTION 5 (P-037), AS MEASURED AT S2: leg R's slice holds one render sample,
    /// 0 ms in `value`, carrying `degraded_mode: false` — fired by the Report webview's own
    /// selection, no click.
    #[test]
    fn p075_reround_assertion_5_p037_report_render() {
        let lines = reround_lines(
            "pulse-r.jsonl",
            "3f6c13c19f8d8100fdb137e2b8b8c2dbbe14c510a2f25e5cd6427147d86954d5",
        );
        assert_eq!(observations(&lines, &bounds()[2]).len(), 1);
        assert_eq!(grade(&lines, &bounds()[2]), Ok(0.0));
    }

    /// ROUND-REQUEST ASSERTION 6 (P-045), AS MEASURED AT S2: leg F's slice holds 118 counter-refresh
    /// samples; the worst is 1.0 ms against 1 000 ms.
    #[test]
    fn p075_reround_assertion_6_p045_counter_refresh() {
        let lines = reround_lines(
            "pulse-f.jsonl",
            "498496d70b80311c10ba742fe524a68cad61f5113e58c845b0ba8e7e8c5b5c03",
        );
        assert_eq!(observations(&lines, &bounds()[3]).len(), 118);
        assert_eq!(grade(&lines, &bounds()[3]), Ok(1.000_000_000_232_830_6));
    }

    #[test]
    fn p075_reround_slices_digest_pin_fails_on_a_tampered_byte() {
        let name = format!("{REROUND_EVIDENCE}pulse-r.jsonl");
        let text = evidence_pin::committed(&name);
        let pin = "3f6c13c19f8d8100fdb137e2b8b8c2dbbe14c510a2f25e5cd6427147d86954d5";
        assert!(evidence_pin::check_digest(&name, &text, pin).is_ok());
        let tampered = text.replacen("\"value\":0", "\"value\":9", 1);
        assert_ne!(tampered, text, "the tamper landed");
        assert!(evidence_pin::check_digest(&name, &tampered, pin).is_err());
    }

    #[test]
    fn round_a_tampered_byte_fails_the_digest_pin() {
        let text = "{\"target\":\"metric.findings.counter_refresh_ms\"}\n";
        let pin = evidence_pin::sha256_hex(text);
        assert!(evidence_pin::check_digest("synthetic", text, &pin).is_ok());
        assert!(evidence_pin::check_digest("synthetic", &text.replace("ms", "MS"), &pin).is_err());
    }

    // ---- Live-leg evidence, 2026-08-21 ----
    // Four legs, each on a fresh data dir with `pulse-app` restarted and its window open, against
    // Pulse at HEAD `f0c38f5`. Every line below is VERBATIM from that leg's capture, sliced to the
    // leg window by a pre-leg line count taken after readiness and before the first dispatch.

    /// VERBATIM from the leg B capture (run `2026-08-21T18-42-31-734`, pre-leg 6747). The
    /// `discovered_count: 3` is what ATTRIBUTES this sample to `service-constellation-discovery`'s
    /// own second phase — its three-service topology — rather than to the preflight canary, whose
    /// discovery line in the same window carries `discovered_count: 1` at 703.98ms.
    fn leg_b_discovery_line() -> String {
        r#"{"fields":{"deployment.environment":"production","discovered_count":3,"duration_ms":702.4326171875,"service.name":"com.andromeda.pulse","service.version":"0.1.0"},"level":"INFO","message":"constellation discovery latency recorded","target":"metric.constellation.discovery_ms","timestamp":"2026-08-21T18:43:26.650Z"}"#.to_owned()
    }

    #[test]
    fn p027_constellation_discovery_meets_its_five_second_budget() {
        let bound = bounds()[1];
        assert_eq!(grade(&[leg_b_discovery_line()], &bound), Ok(702.4326171875));
    }

    /// VERBATIM from the leg C captures (runs `2026-08-21T18-45-01-029` and
    /// `2026-08-21T18-50-22-415`). Both carry `degraded_mode: true` because deterministic L4 leaves
    /// the interpretation pending, so the six-section report built here is the DEGRADED form — a
    /// cheaper build than a fully-populated one. At 0-1ms against a 2000ms budget the caveat cannot
    /// change the verdict, but it is what the samples measure.
    fn leg_c_render_lines() -> Vec<String> {
        [
            r#"{"fields":{"degraded_mode":true,"deployment.environment":"production","section_count":6,"service.name":"com.andromeda.pulse","service.version":"0.1.0","value":1},"level":"INFO","message":"report render latency sample","target":"metric.report.render_ms","timestamp":"2026-08-21T18:45:46.431Z"}"#,
            r#"{"fields":{"degraded_mode":true,"deployment.environment":"production","section_count":6,"service.name":"com.andromeda.pulse","service.version":"0.1.0","value":0},"level":"INFO","message":"report render latency sample","target":"metric.report.render_ms","timestamp":"2026-08-21T18:51:08.429Z"}"#,
        ]
        .iter()
        .map(|s| (*s).to_owned())
        .collect()
    }

    #[test]
    fn p037_report_render_meets_its_two_second_budget() {
        let bound = bounds()[2];
        assert_eq!(grade(&leg_c_render_lines(), &bound), Ok(1.0));
    }

    /// VERBATIM from the leg D capture (run `2026-08-21T18-53-35-135`, pre-leg 3549) — the SLOWEST
    /// of the 269 samples in that leg's window (n=269, min 1.3ms, median 1.9ms, max 7.0ms, zero
    /// samples over the budget). Pinning the worst sample is what makes the assertion meaningful:
    /// the budget is broken by the worst observation, never rescued by a fast neighbour.
    fn leg_d_counter_worst_line() -> String {
        r#"{"fields":{"deployment.environment":"production","duration_ms":7.0,"service.name":"com.andromeda.pulse","service.version":"0.1.0"},"level":"INFO","message":"findings counter refresh latency recorded","target":"metric.findings.counter_refresh_ms","timestamp":"2026-08-21T18:55:03.299Z"}"#.to_owned()
    }

    #[test]
    fn p045_counter_refresh_meets_its_one_second_budget() {
        let bound = bounds()[3];
        assert_eq!(grade(&[leg_d_counter_worst_line()], &bound), Ok(7.0));
    }

    /// VERBATIM hue samples from legs A and B (runs `2026-08-21T18-38-48-967` and
    /// `2026-08-21T18-42-31-734`), emitted by the RETIRED instrument. They are RECORDED here, not
    /// asserted green: they are true readings of what the leaf emitted before Pulse's P-025 change,
    /// and the contract cites the raw `36704.983642578125` from this fixture.
    fn hue_lines() -> Vec<String> {
        [
            r#"{"fields":{"deployment.environment":"production","duration_ms":35581.440673828125,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity_tier":"autonomous"},"level":"INFO","message":"constellation hue update latency recorded","target":"metric.constellation.hue_update_ms","timestamp":"2026-08-21T18:39:35.309Z"}"#,
            r#"{"fields":{"deployment.environment":"production","duration_ms":36704.983642578125,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity_tier":"autonomous"},"level":"INFO","message":"constellation hue update latency recorded","target":"metric.constellation.hue_update_ms","timestamp":"2026-08-21T18:43:17.652Z"}"#,
        ]
        .iter()
        .map(|s| (*s).to_owned())
        .collect()
    }

    /// The RETIRED instrument's first record (Pulse HEAD `f0c38f5`; the mechanism measured at
    /// `83d4060`) — and the ORIGIN of the mechanism the successor test pins. It does not grade P-025
    /// today: the current grade is `grade_in_window` over the 2026-09-29 leg, under the new observable.
    ///
    /// Both legs measured ~36s against a 2s budget, and neither sample was the scenario's: at that
    /// time `halo-hue-encoding.toml` declared ZERO `[phases.emission]` blocks, so it drove no stream
    /// and caused no tier change of its own — both samples carry `severity_tier: "autonomous"` and
    /// follow the preflight canary's incident by under a second. (The scenario has since been
    /// re-driven; these lines stay as the record of what the un-driven shape measured.)
    ///
    /// THREE fire-site terms explain it, and only the first was known when this test was written.
    /// (1) STALENESS: the site computes `now - item.last_seen_unix_nano`, i.e. how stale the
    /// service's telemetry was when its hue changed, not how long the update took.
    /// (2) SLOWEST-WINS: it loops every dot whose tier changed in the render pass and emits the
    /// MAXIMUM staleness with THAT dot's tier, so a same-pass canary change reports the canary.
    /// (3) QUANTIZATION: `last_seen_unix_nano` has no ingest-path writer — steady-state it is
    /// written only by the 15s lifecycle tick, and only when the service was seen <1s before it. So
    /// the observable reports `t_sample - t_last_refreshing_tick`, U(0, 15s) at a randomly-timed
    /// flip and INDEPENDENT of the dispatch rate.
    ///
    /// Term (3) is why no arm over these lines asserts a sample UNDER 2000ms: at 15s quantization a
    /// sub-2s reading was a tick coincidence (~13% of flips), never attainment, so the retired
    /// instrument could not grade the bound. Pulse's `e98d838` removed all three terms from the hue
    /// path (`contracts/pulse-p025-measurement-contract.md`).
    #[test]
    fn p025_hue_update_is_recorded_over_budget_never_asserted_as_a_pass() {
        let bound = bounds()[0];
        let err = grade(&hue_lines(), &bound).expect_err("both legs measured far over the budget");
        assert!(
            err.contains("35581") || err.contains("36704"),
            "the reason carries a measured value: {err}"
        );

        let worst = observations(&hue_lines(), &bound)
            .into_iter()
            .fold(0.0_f64, f64::max);
        assert!(
            worst > 17.0 * bound.budget_ms,
            "measured {worst}ms against a {}ms budget",
            bound.budget_ms
        );
    }

    // ---- Live-leg evidence, 2026-09-07 (the re-driven leg, the RETIRED instrument) ----
    // Leg H of the operator-gated `run --live` suite, run `2026-09-07T07-42-45-582`, pre-leg 50243,
    // against Pulse at HEAD `83d4060` with the compact-widget window open. `halo-hue-encoding` drove
    // 360 dispatches at 2/s across two phases (`emission_count: 360`, `timeline.execute` spanning
    // 183.8s), so its service was emitting CONTINUOUSLY through the tier flip. These lines are the
    // record of what the pre-`e98d838` leaf emitted; they do not grade P-025 today.

    /// VERBATIM — the ONLY hue sample inside leg H's window (phase-2 start `1788767041678` through
    /// `scenario.run` close `1788767195445`). The capture held 7 hue samples after the pre-leg
    /// baseline; the other 6 fall outside the window and belong to the canary or to post-leg tier
    /// changes.
    fn leg_h_hue_line() -> String {
        r#"{"fields":{"deployment.environment":"production","duration_ms":14525.947021484377,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity_tier":"autonomous"},"level":"INFO","message":"constellation hue update latency recorded","target":"metric.constellation.hue_update_ms","timestamp":"2026-09-07T07:44:07.658Z"}"#.to_owned()
    }

    /// VERBATIM — the lifecycle tick immediately preceding that sample. Tick spacing across the
    /// capture measured n=76, min 14986ms, median 15000ms, max 15013ms.
    fn leg_h_preceding_tick_line() -> String {
        r#"{"fields":{"deployment.environment":"production","service.name":"com.andromeda.pulse","service.version":"0.1.0","services_active":0,"services_archived":0,"services_bootstrapping":2,"services_dormant":0,"services_quiet":0,"services_silent":0,"services_unknown":0,"tracked_services_total":2},"level":"INFO","message":"lifecycle heartbeat tick","target":"triage.lifecycle.tick","timestamp":"2026-09-07T07:43:53.131Z"}"#.to_owned()
    }

    /// THE RE-DRIVEN MEASUREMENT OF THE RETIRED INSTRUMENT (Pulse HEAD `83d4060`) — and the
    /// disproof completed on the SUBJECT's own sample. A true measurement of what that leaf emitted
    /// then, kept because the contract and obs-plan §4 cite it.
    ///
    /// The 2026-08-21 legs could be dismissed as never having driven anything: the scenario declared
    /// no emission, so both samples were the canary's and the service had gone quiet. This leg
    /// removes that explanation. `halo-hue-encoding` emitted 2 dispatches per second THROUGH the
    /// tier flip, so at the moment its dot changed tier the service had been seen ~0.5s earlier —
    /// and the observable still reported **14525.9ms** against a 2000ms budget.
    ///
    /// The reason is the quantization, and this is what pins it: the sample's duration equals its
    /// offset to the PRECEDING lifecycle tick (14527ms) to within 1.1ms. `last_seen_unix_nano` is
    /// stamped by that 15s tick, never by ingest, so a flip landing 14.5s after a tick reported
    /// 14.5s no matter how recently a span arrived. Meeting 2000ms required the flip to land inside
    /// the first 2s of a 15s window — a ~13% coincidence, which is why NO arm here asserts a pass.
    #[test]
    fn p025_the_re_driven_leg_measures_tick_quantization_not_update_latency() {
        let bound = bounds()[0];
        let lines = vec![leg_h_preceding_tick_line(), leg_h_hue_line()];
        let window = (1_788_767_041_678_i64, 1_788_767_195_445_i64);

        let samples = hue_samples_in_window(&lines, &bound, window);
        assert_eq!(
            samples.len(),
            1,
            "leg H's own sample, canary excluded: {samples:?}"
        );
        let sample = samples[0];

        let ticks = tick_times_ms(&lines);
        let offset = tick_offset_ms(&sample, &ticks).expect("a tick precedes the sample");

        // THE MECHANISM: the reported duration IS the offset to the tick that stamped `last_seen`.
        assert!(
            (sample.duration_ms - offset).abs() <= TICK_OFFSET_TOLERANCE_MS,
            "reported {}ms vs {offset}ms since the preceding tick",
            sample.duration_ms
        );
        assert_eq!(offset, 14_527.0, "the measured tick offset, pinned");

        // RECORDED, never asserted as a pass: the subject's own sample is over budget, on a leg
        // where the subject never stopped emitting.
        assert!(
            sample.duration_ms > bound.budget_ms,
            "the re-driven sample is recorded over budget: {}ms vs {}ms",
            sample.duration_ms,
            bound.budget_ms
        );
        assert!((sample.duration_ms - 14_525.947_021_484_377).abs() < 1e-6);
    }

    // ---- Live-leg evidence, 2026-09-29 (the P-025 graded leg, the NEW observable) ----
    // One `conductor run halo-hue-encoding --agent-mode`, run `2026-09-29T21-09-10-754`, pre-leg
    // 34573, against a fresh deterministic-L4 `pulse-app` on a fresh data dir with the compact widget
    // visible. Pulse checkout HEAD `4502d5d`, whose `pulse-app` / MCP-crate source equals `226554a`
    // and carries `e98d838`; the binary holds `tier_effective_at_unix_nano` by content. The graded
    // rule is `contracts/pulse-p025-measurement-contract.md` §The grading rule, recorded by sha256
    // before the drive.

    /// VERBATIM — every `metric.constellation.hue_update_ms`, `interpretation.incident.created` and
    /// `triage.incident.auto_resolve.tick` (`resolved_count` ≥ 1) line after the pre-leg count, in
    /// capture order. The first two are the preflight canary's rise, before phase-2 start.
    fn leg_2026_09_29_lines() -> Vec<String> {
        [
            r#"{"fields":{"created":true,"deduped":false,"deployment.environment":"production","priority_tier":"autonomous","service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"error"},"level":"INFO","message":"incident producer outcome","target":"interpretation.incident.created","timestamp":"2026-09-29T21:09:55.909Z"}"#,
            r#"{"fields":{"deployment.environment":"production","duration_ms":438.1103515625,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity_tier":"autonomous"},"level":"INFO","message":"constellation hue update latency recorded","target":"metric.constellation.hue_update_ms","timestamp":"2026-09-29T21:09:56.316Z"}"#,
            r#"{"fields":{"created":true,"deduped":false,"deployment.environment":"production","priority_tier":"autonomous","service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"error"},"level":"INFO","message":"incident producer outcome","target":"interpretation.incident.created","timestamp":"2026-09-29T21:10:32.660Z"}"#,
            r#"{"fields":{"deployment.environment":"production","duration_ms":684.976318359375,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity_tier":"autonomous"},"level":"INFO","message":"constellation hue update latency recorded","target":"metric.constellation.hue_update_ms","timestamp":"2026-09-29T21:10:33.315Z"}"#,
            r#"{"fields":{"created":true,"deduped":false,"deployment.environment":"production","priority_tier":"autonomous","service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"error"},"level":"INFO","message":"incident producer outcome","target":"interpretation.incident.created","timestamp":"2026-09-29T21:10:33.636Z"}"#,
            r#"{"fields":{"created":false,"deduped":true,"deployment.environment":"production","priority_tier":"autonomous","service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"error"},"level":"INFO","message":"incident producer outcome","target":"interpretation.incident.created","timestamp":"2026-09-29T21:10:43.633Z"}"#,
            r#"{"fields":{"deployment.environment":"production","duration_ms":11,"evaluated_count":1,"resolved_count":1,"service.name":"com.andromeda.pulse","service.version":"0.1.0"},"level":"INFO","message":"incident auto-resolution tick","target":"triage.incident.auto_resolve.tick","timestamp":"2026-09-29T21:11:57.883Z"}"#,
            r#"{"fields":{"deployment.environment":"production","duration_ms":44,"evaluated_count":2,"resolved_count":2,"service.name":"com.andromeda.pulse","service.version":"0.1.0"},"level":"INFO","message":"incident auto-resolution tick","target":"triage.incident.auto_resolve.tick","timestamp":"2026-09-29T21:12:57.928Z"}"#,
            r#"{"fields":{"deployment.environment":"production","duration_ms":430.78955078125,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity_tier":"none"},"level":"INFO","message":"constellation hue update latency recorded","target":"metric.constellation.hue_update_ms","timestamp":"2026-09-29T21:12:58.315Z"}"#,
        ]
        .iter()
        .map(|s| (*s).to_owned())
        .collect()
    }

    /// THE P-025 HARD GRADE, at its real measured value. The window is derived from the frozen
    /// self-obs per the rule: the one `timeline.execute` `new` at `1790716196860` plus the
    /// `healthy-baseline` phase's 30000ms, through `scenario.run` close at `1790716380720`.
    ///
    /// Two samples fall in it: the rise (684.98ms, `autonomous`) and a fall to `none` (430.79ms). The
    /// fall landed in-window, not after the dot hid as the rule's prose forecast: the scenario's
    /// incident auto-resolved at the storm's end while its dot was still live. It is graded like
    /// every in-window sample. The worst is 684.98ms against 2000ms, so the grade is a PASS.
    #[test]
    fn p025_the_graded_leg_meets_its_two_second_budget_at_its_real_value() {
        let bound = bounds()[0];
        let lines = leg_2026_09_29_lines();
        let window = (1_790_716_226_860_i64, 1_790_716_380_720_i64);

        assert_eq!(
            grade_in_window(&lines, &bound, window),
            Ok(684.976_318_359_375)
        );

        let samples = hue_samples_in_window(&lines, &bound, window);
        let durations: Vec<f64> = samples.iter().map(|s| s.duration_ms).collect();
        assert_eq!(
            durations,
            vec![684.976_318_359_375, 430.789_550_781_25],
            "the canary's 438.11ms rise precedes phase-2 start and is excluded"
        );

        // Mechanism corroboration: each rise's start instant lands on its incident's opening.
        let rises = tiered_hue_samples_in_window(&lines, &bound, window);
        assert_eq!(rises.len(), 1, "one rise in the window: {rises:?}");
        let error = incident_anchor_error_ms(&rises[0], &lines).expect("a fresh incident precedes");
        assert!(
            error <= P025_ANCHOR_TOLERANCE_MS,
            "anchor error {error}ms over the {P025_ANCHOR_TOLERANCE_MS}ms tolerance"
        );
        assert!(
            (error - 29.976).abs() < 0.01,
            "the measured anchor error, pinned: {error}"
        );
    }
}
