//! B1, the launch witness and the routes' trace witness sets, over synthetic captures.

use crate::*;

// ---- B1 and the launch witness --------------------------------------------------------------------

#[test]
fn a_canned_capture_is_never_the_real_model() {
    let real = rank1("The conductor service is in a retry storm.");
    assert!(real_model_witnessed(&real));

    let canned_mode = real.replace(
        &format!("{INFERENCE_MODE}real"),
        &format!("{INFERENCE_MODE}deterministic"),
    );
    assert!(!real_model_witnessed(&canned_mode));

    // The deterministic fixture's evidence refs (andromeda-pulse deterministic_inference.rs:68-72).
    let canned_evidence = real.replace("- `ref-1`", "- `det-span-9f2c4a7e1b6d0358`");
    assert!(!real_model_witnessed(&canned_evidence));
}

#[test]
fn a_launch_whose_basename_carries_conductor_is_not_clear() {
    let clear = rank1("The conductor service is in a retry storm.");
    assert!(launch_cwd_clear(&clear));
    let fouled = clear.replace(&format!("{LAUNCH_CWD}false"), &format!("{LAUNCH_CWD}true"));
    assert!(!launch_cwd_clear(&fouled));
}

// ---- routes ---------------------------------------------------------------------------------------

#[test]
fn each_route_carries_its_own_trace_witness_set() {
    let read_back = rank1("The conductor service is in a retry storm.");
    assert_eq!(route(&read_back), Route::ReadBack);
    assert!(trace_conforms(&read_back));

    let no_read_back = read_back.replace(
        "retrieve_report_witness=true",
        "retrieve_report_witness=false",
    );
    assert_eq!(route(&no_read_back), Route::EmittedNoReadBack);
    assert!(trace_conforms(&no_read_back));

    let blocked = format!(
        "{NO_EMISSION}\n{TRACE}spans=scenario.run,verify.readback.preflight wire_shape_lines=12 \
         retrieve_report_witness=false\n"
    );
    assert_eq!(route(&blocked), Route::PreflightBlocked);
    assert!(trace_conforms(&blocked));
}

#[test]
fn a_read_back_route_missing_its_emission_span_does_not_conform() {
    let broken =
        rank1("The conductor service is in a retry storm.").replace("spans=emit.batch,", "spans=");
    assert_eq!(route(&broken), Route::ReadBack);
    assert!(!trace_conforms(&broken));
}
