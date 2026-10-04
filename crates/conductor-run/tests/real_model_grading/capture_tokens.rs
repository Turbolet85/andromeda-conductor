//! Fingerprint elision, and producer/grader agreement between the capture and the rule.

use crate::*;

use super::capture_population::un_elided_keyed_values;

#[test]
fn a_fingerprint_is_elided_and_a_stamp_a_seed_and_a_det_prefix_are_not() {
    let fp = ["12dcd67b", "34e41302", "ed9cd723", "dd1e28cf"].concat();
    assert_eq!(
        elide_fingerprints(&format!("- `{fp}` and fingerprint_hex={}", &fp[..8])),
        "- `<fingerprint>` and fingerprint_hex=<fingerprint>"
    );
    for kept in [
        "opened_at_unix_nano=1790702606754859000",
        "\"seed\":4317033",
        "run_id: 2026-01-01T00-00-00-000",
        "a deadbee word",
        "prefix12dcd67b",
    ] {
        assert_eq!(elide_fingerprints(kept), kept);
    }
    assert_eq!(
        elide_fingerprints("- `det-span-9f2c4a7e1b6d0358`"),
        "- `det-span-<fingerprint>`",
        "the canned-evidence prefix survives, so the witness still reads it"
    );
    let digits = ["1357", "2468"].concat();
    assert_eq!(
        elide_fingerprints(&format!("storm fingerprint_hex={digits} count=12")),
        "storm fingerprint_hex=<fingerprint> count=12",
        "a keyed value is a fingerprint whatever its characters"
    );
    for kept in [
        "storm fingerprint_hex=<fingerprint> count=12",
        "storm fingerprint_hex= count=12",
        "fingerprint_hex=",
    ] {
        assert_eq!(elide_fingerprints(kept), kept);
    }
}

#[test]
fn un_elided_keyed_values_counts_a_planted_value() {
    let digits = ["1357", "2468"].concat();
    let line = |value: &str| format!("storm severity_hint=autonomous fingerprint_hex={value}\n");
    assert_eq!(un_elided_keyed_values(&line(&digits)), 1);
    assert_eq!(un_elided_keyed_values(&line("<fingerprint>")), 0);
    assert_eq!(un_elided_keyed_values(&line("")), 0);
}

// ---- producer/grader agreement --------------------------------------------------------------------

#[test]
fn the_capture_prints_every_token_the_rule_reads() {
    // The capture is feature-gated, so this default-suite target cannot call it; it can read its
    // source — its own file and the shared module whose canary pairing it prints through. Each grammar
    // literal the rule keys on must appear there verbatim.
    let capture = [
        include_str!("../real_model_live.rs"),
        include_str!("../real_model_common/mod.rs"),
    ]
    .concat();
    for token in [
        EMISSION_INSTANT,
        NO_EMISSION,
        TRACE,
        ATTRIBUTED,
        AMBIGUOUS,
        READ_BACK_FAILED,
        END_OF_SECTIONS,
        INFERENCE_MODE,
        LAUNCH_CWD,
        CORPUS_ROWS,
        CANARY,
        SURFACED,
        DISMISSED,
        PIPELINE_FAULT,
    ] {
        assert!(
            capture.contains(&format!("\"{token}\"")),
            "the capture prints {token:?}"
        );
    }
    for key in [
        "opened_at_unix_nano=",
        "spans=",
        "wire_shape_lines=",
        "retrieve_report_witness=",
    ] {
        assert!(capture.contains(key), "the capture prints the {key} field");
    }
}

#[test]
fn the_capture_computes_the_dispatcher_s_own_exception() {
    // Attribution keys on the scenario's cue fingerprint, which the capture computes from a
    // transcription of the dispatcher's private base exception. Drift in either copy would attribute
    // nothing; both must carry the same four literals.
    let capture = include_str!("../real_model_live.rs");
    let dispatch = include_str!("../../src/dispatch.rs");
    for literal in [
        r#""ValueError","#,
        r#""conductor synthetic exception","#,
        r#"Frame::new("conductor::worker::handle", "src/worker.rs", 42)"#,
        r#"Frame::new("conductor::worker::parse", "src/worker.rs", 17)"#,
    ] {
        assert!(dispatch.contains(literal), "dispatch.rs carries {literal}");
        assert!(capture.contains(literal), "the capture carries {literal}");
    }
}
