//! Operator-pause orchestration — go/no-go resolution, headless never-block, determinism, redaction.

use conductor_core::{Decision, HeadlessResolver, HoldPoint, PId, resolve_hold};

fn hold(prompt: &str) -> HoldPoint {
    HoldPoint {
        scenario: "restart-suppression".to_string(),
        p_id: PId("P-015".to_string()),
        step: "restart-pulse".to_string(),
        prompt: prompt.to_string(),
        allow_no_go: true,
    }
}

#[tokio::test]
async fn headless_proceed_resolves_to_go() {
    let r = resolve_hold(&HeadlessResolver::proceed(), &hold("Restart Pulse, then confirm")).await;
    assert_eq!(r.decision, Decision::Go);
}

#[tokio::test]
async fn headless_abort_resolves_to_no_go() {
    let r = resolve_hold(&HeadlessResolver::abort(), &hold("Restart Pulse, then confirm")).await;
    assert_eq!(r.decision, Decision::NoGo);
}

#[tokio::test]
async fn resolver_choice_drives_the_decision() {
    let h = hold("Observe the desktop toast");
    let go = resolve_hold(&HeadlessResolver::proceed(), &h).await;
    let no_go = resolve_hold(&HeadlessResolver::abort(), &h).await;
    assert_ne!(go.decision, no_go.decision);
}

#[tokio::test]
async fn resolution_is_deterministic() {
    let h = hold("Restart Pulse, then confirm");
    let a = resolve_hold(&HeadlessResolver::proceed(), &h).await;
    let b = resolve_hold(&HeadlessResolver::proceed(), &h).await;
    assert_eq!(a, b);
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn headless_never_blocks_under_paused_clock() {
    // Under a paused virtual clock a resolver that awaited a timer would deadlock the test;
    // completing without any `tokio::time::advance` proves the hold consumes no virtual time —
    // the operator pause is wall-clock-only, outside the seeded clock.
    let r = resolve_hold(&HeadlessResolver::proceed(), &hold("Confirm to proceed")).await;
    assert_eq!(r.decision, Decision::Go);
}

#[tokio::test]
async fn prompt_is_redacted_in_the_resolution() {
    let r = resolve_hold(
        &HeadlessResolver::proceed(),
        &hold("open C:\\Users\\turbo\\corpus.db then confirm"),
    )
    .await;
    assert!(!r.prompt.contains("C:\\Users"), "host path leaked: {}", r.prompt);
    assert!(r.prompt.contains("<redacted>"));
}
