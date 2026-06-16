//! Toolchain exemplars — prove the rstest / proptest / insta wiring compiles and runs green
//! against `conductor-core`'s public surface. Smoke for the tools, not domain coverage; the
//! per-seam tests land with their epochs (test-plan §4).

use conductor_core::{redact_value, ReportState, Verdict};
use proptest::prelude::*;
use rstest::rstest;

#[rstest]
#[case(Verdict::Pass, "[PASS]")]
#[case(Verdict::Fail, "[FAIL]")]
#[case(Verdict::CalibrationRegion, "[HOLD]")]
fn verdict_status_prefix(#[case] verdict: Verdict, #[case] expected: &str) {
    assert_eq!(verdict.status_prefix(), expected);
}

#[rstest]
#[case(ReportState::Pass, "[PASS]")]
#[case(ReportState::ManualCheck, "[MANUAL]")]
#[case(ReportState::Blocked, "[BLOCKED]")]
fn report_state_status_prefix(#[case] state: ReportState, #[case] expected: &str) {
    assert_eq!(state.status_prefix(), expected);
}

proptest! {
    /// Scrubbing already-scrubbed text changes nothing — host-path tokens become `<redacted>`,
    /// which carries no host-path marker, so a second pass is a no-op.
    #[test]
    fn redact_value_is_idempotent(raw in ".*") {
        let once = redact_value(&raw).into_owned();
        let twice = redact_value(&once).into_owned();
        prop_assert_eq!(once, twice);
    }
}

#[test]
fn verdict_prefixes_snapshot() {
    let line = format!(
        "{} {} {}",
        Verdict::Pass.status_prefix(),
        Verdict::Fail.status_prefix(),
        Verdict::CalibrationRegion.status_prefix(),
    );
    insta::assert_snapshot!(line, @"[PASS] [FAIL] [HOLD]");
}
