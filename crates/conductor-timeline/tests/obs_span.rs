//! `timeline.execute` must carry `phase_count` AND `emission_count` on a real emitted record
//! (obs-plan §4 Critical Path 1 + §3).
//!
//! Asserted against the JSONL the subscriber actually writes, never against the `fields(...)` we
//! wrote in the macro — the previous chunk found three spans that looked instrumented and emitted
//! nothing, and `JsonObsLayer` still implements no `on_record`, so a post-open `Span::record` would
//! vanish silently here too. Reading the line back is what proves the attribute survived.

use std::time::Duration;

use conductor_core::{ObsSink, init_observability};
use conductor_timeline::{Phase, PhaseTimeline, run_timeline};
use serde_json::{Map, Value};

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn timeline_execute_carries_phase_count_and_emission_count_on_the_emitted_line() {
    let dir = std::env::temp_dir().join(format!("conductor-timeline-obs-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let path = dir.join("agent-latest.jsonl");
    init_observability(
        "conductor",
        Some("RUN-TIMELINE".to_string()),
        ObsSink::File(path.clone()),
    );

    let timeline = PhaseTimeline::new(
        vec![
            Phase::emitting("storm", Duration::from_secs(2), 6),
            Phase::emitting("quiet", Duration::from_secs(1), 0),
            Phase::emitting("resume", Duration::from_secs(1), 4),
        ],
        Duration::ZERO,
    );
    assert_eq!(timeline.total_emissions(), 10);
    run_timeline(&timeline, 424_242)
        .await
        .expect("non-empty timeline");

    let body = std::fs::read_to_string(&path).expect("self-obs log written");
    let lines: Vec<Value> = body
        .lines()
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect();
    let new_record: &Map<String, Value> = lines
        .iter()
        .filter_map(Value::as_object)
        .find(|o| {
            o.get("span") == Some(&Value::from("timeline.execute"))
                && o.get("span_event") == Some(&Value::from("new"))
        })
        .expect("a timeline.execute `new` record reached the sink");

    assert_eq!(new_record.get("phase_count"), Some(&Value::from(3)));
    assert_eq!(
        new_record.get("emission_count"),
        Some(&Value::from(10)),
        "the declared total rides the span it is computed on: {new_record:?}"
    );
    assert_eq!(new_record.get("run_id"), Some(&Value::from("RUN-TIMELINE")));

    let _ = std::fs::remove_dir_all(&dir);
}
