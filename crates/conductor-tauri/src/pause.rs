//! The Tauri interactive operator-pause resolver — the GUI half of the core hold mechanism.
//!
//! [`conductor_core::resolve_hold`] awaits a [`Decision`] through a [`PauseResolver`]; this seam
//! supplies the answer from the webview go/no-go dialog. The run drives on a background thread
//! ([`crate::commands`]), so the resolver bridges thread ↔ webview without blocking Tauri's event
//! loop: [`TauriResolver::resolve`] arms the shared [`HoldGate`] with a [`oneshot::Sender`], pushes
//! the [`HoldPrompt`] over the IPC [`Channel`] to open the dialog, and awaits the reply the
//! [`resolve_operator_hold`] command delivers when the operator answers. A dropped sender (the run
//! aborted, or the webview went away) collapses to [`Decision::NoGo`] — the abort-safe default, as in
//! the CLI `PromptResolver`. `PauseResolver::resolve` returns `impl Future` (not object-safe), so the
//! Tauri resolver is a concrete type, never a trait object.

use std::sync::{Arc, Mutex};

use conductor_core::{ChecklistItem, Decision, HoldPoint, PauseResolver};
use serde::Serialize;
use tauri::ipc::Channel;
use tokio::sync::oneshot;

/// The webview-facing projection of a [`HoldPoint`] — the dialog renders these fields, never
/// re-authoring the hold. Carries no host path (the prompt is operator-authored instructional text).
#[derive(Debug, Clone, Serialize)]
pub struct HoldPrompt {
    /// The dialog title — the gated capability + step (e.g. `P-025 — observe-hue`).
    pub title: String,
    /// The non-Conductor action to perform/observe before answering.
    pub body: String,
    /// Whether a no-go (Abort) is an offered outcome.
    pub allow_no_go: bool,
    /// The scenario's declared checklist items — each an induced state paired with the observation
    /// to confirm. Empty when the scenario declares none, and the dialog then renders `body` alone.
    pub checklist: Vec<ChecklistItem>,
}

impl HoldPrompt {
    fn from_hold(hold: &HoldPoint) -> Self {
        Self {
            title: format!("{} — {}", hold.p_id.0, hold.step),
            body: hold.prompt.clone(),
            allow_no_go: hold.allow_no_go,
            checklist: hold.checklist.clone(),
        }
    }
}

/// The single in-flight hold's reply slot, shared between the background run thread (which `arm`s it
/// when a hold is raised) and the [`resolve_operator_hold`] command (which `deliver`s the operator's
/// [`Decision`]). The run driver awaits each hold before the next scenario, so one slot suffices — no
/// queue, no correlation id.
#[derive(Default, Clone)]
pub struct HoldGate {
    slot: Arc<Mutex<Option<oneshot::Sender<Decision>>>>,
}

impl HoldGate {
    /// Register the reply channel for the in-flight hold (the resolver side).
    fn arm(&self, tx: oneshot::Sender<Decision>) {
        if let Ok(mut slot) = self.slot.lock() {
            *slot = Some(tx);
        }
    }

    /// Deliver the operator's decision to the awaiting hold; `true` iff one was pending and still
    /// connected (the command side). Idempotent — a second call finds the slot empty.
    fn deliver(&self, decision: Decision) -> bool {
        let sender = self.slot.lock().ok().and_then(|mut slot| slot.take());
        sender.is_some_and(|tx| tx.send(decision).is_ok())
    }
}

/// The GUI resolver: surfaces a hold to the webview and awaits the operator's go/no-go.
pub struct TauriResolver {
    gate: HoldGate,
    on_hold: Channel<HoldPrompt>,
}

impl TauriResolver {
    pub fn new(gate: HoldGate, on_hold: Channel<HoldPrompt>) -> Self {
        Self { gate, on_hold }
    }
}

impl PauseResolver for TauriResolver {
    fn kind(&self) -> &'static str {
        "tauri-dialog"
    }

    async fn resolve(&self, hold: &HoldPoint) -> Decision {
        let (tx, rx) = oneshot::channel();
        self.gate.arm(tx);
        if self.on_hold.send(HoldPrompt::from_hold(hold)).is_err() {
            return Decision::NoGo;
        }
        rx.await.unwrap_or(Decision::NoGo)
    }
}

/// Deliver the operator's go/no-go to the awaiting hold. A no-pending-hold call (the dialog already
/// resolved, or a stray invoke) is a no-op, never an error — the resolution is a value, not a fault.
#[tauri::command]
pub fn resolve_operator_hold(
    decision: Decision,
    gate: tauri::State<'_, HoldGate>,
) -> Result<(), String> {
    let _span = tracing::info_span!("tauri.command.resolve_operator_hold").entered();
    if gate.deliver(decision) {
        tracing::info!("operator hold resolved ({})", decision.label());
    } else {
        tracing::info!("operator hold resolve: no pending hold");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use conductor_core::PId;
    use tauri::ipc::{CallbackFn, InvokeBody};
    use tauri::test::{INVOKE_KEY, get_ipc_response, mock_builder, mock_context, noop_assets};
    use tauri::webview::InvokeRequest;
    use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

    /// How long an armed hold may wait for its decision before the test calls it undelivered.
    /// Generous enough never to flake on a loaded host, short enough that a mutant which stops the
    /// oneshot ever being sent FAILS instead of timing the binary out (the survivor class this
    /// bound converts from timeout to caught).
    const AWAIT_BOUND: std::time::Duration = std::time::Duration::from_secs(5);

    fn hold() -> HoldPoint {
        HoldPoint {
            scenario: "constellation-hue".to_string(),
            p_id: PId("P-025".to_string()),
            step: "observe-hue".to_string(),
            prompt: "Observe the constellation hue for this scenario".to_string(),
            allow_no_go: true,
            checklist: vec![ChecklistItem {
                induced: "error-pressure stream driven through phase 2".to_string(),
                observation: "hue shifted toward burgundy under error pressure?".to_string(),
            }],
        }
    }

    #[test]
    fn hold_prompt_projects_the_hold_point() {
        let prompt = HoldPrompt::from_hold(&hold());
        assert_eq!(prompt.title, "P-025 — observe-hue");
        assert_eq!(
            prompt.body,
            "Observe the constellation hue for this scenario"
        );
        assert!(prompt.allow_no_go);
        // The projection carries the declared items verbatim — it never re-authors the hold, so a
        // dialog row's text is the scenario's own declaration (design-system §Component Patterns 7).
        assert_eq!(prompt.checklist, hold().checklist);
        assert_eq!(
            prompt.checklist[0].induced,
            "error-pressure stream driven through phase 2"
        );
        assert_eq!(
            prompt.checklist[0].observation,
            "hue shifted toward burgundy under error pressure?"
        );
    }

    #[test]
    fn hold_prompt_serializes_the_checklist_for_the_webview() {
        // The items reach the dialog over the existing Channel<HoldPrompt>, so the SERIALIZED shape
        // is the real contract the webview reads — a field the derive dropped would be invisible to
        // every Rust-side assertion above.
        let json = serde_json::to_value(HoldPrompt::from_hold(&hold())).expect("prompt serializes");
        let items = json["checklist"]
            .as_array()
            .expect("checklist serializes as an array");
        assert_eq!(items.len(), 1);
        assert_eq!(
            items[0]["induced"],
            "error-pressure stream driven through phase 2"
        );
        assert_eq!(
            items[0]["observation"],
            "hue shifted toward burgundy under error pressure?"
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn gate_delivers_the_decision_to_the_awaiting_hold() {
        // The resolver-await half + the command-deliver half meet at the gate's oneshot.
        let gate = HoldGate::default();
        let (tx, rx) = oneshot::channel();
        gate.arm(tx);
        assert!(
            gate.deliver(Decision::Go),
            "a pending, connected hold accepts the decision"
        );
        // BOUNDED: a `deliver` that reports success without sending would otherwise block here
        // forever and time the whole binary out, which reports as a timeout rather than a failure.
        let decision = tokio::time::timeout(AWAIT_BOUND, rx)
            .await
            .expect("deliver reported success but never sent the decision")
            .expect("the armed sender stayed connected");
        assert_eq!(decision, Decision::Go);
    }

    #[test]
    fn deliver_is_a_no_op_without_a_pending_hold() {
        assert!(!HoldGate::default().deliver(Decision::Go));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn a_dropped_sender_is_abort_safe_no_go() {
        // The run aborts / the webview closes: the gate's sender drops unanswered ⇒ the resolver's
        // `rx.await.unwrap_or(NoGo)` yields the abort-safe default (mirrors the CLI cancel → NoGo).
        let gate = HoldGate::default();
        let (tx, rx) = oneshot::channel::<Decision>();
        gate.arm(tx);
        drop(gate.slot.lock().unwrap().take());
        assert_eq!(rx.await.unwrap_or(Decision::NoGo), Decision::NoGo);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn resolve_operator_hold_command_delivers_the_decision() {
        // The ch8 deferred leg: the `resolve_operator_hold` COMMAND through the real IPC dispatch +
        // managed-`State` extraction (the bridge core `HoldGate::{arm,deliver}` is covered above). A
        // pre-armed managed gate receives the operator's `Go` via the dispatched command (test-plan §5).
        let app = mock_builder()
            .manage(HoldGate::default())
            .invoke_handler(tauri::generate_handler![resolve_operator_hold])
            .build(mock_context(noop_assets()))
            .expect("mock app builds");
        let window = WebviewWindowBuilder::new(&app, "main", WebviewUrl::default())
            .build()
            .expect("mock webview builds");

        let (tx, rx) = oneshot::channel();
        app.state::<HoldGate>().arm(tx);

        get_ipc_response(
            &window,
            InvokeRequest {
                cmd: "resolve_operator_hold".into(),
                callback: CallbackFn(0),
                error: CallbackFn(1),
                url: "http://tauri.localhost".parse().unwrap(),
                body: InvokeBody::Json(serde_json::json!({ "decision": "Go" })),
                headers: Default::default(),
                invoke_key: INVOKE_KEY.to_string(),
            },
        )
        .expect("resolve_operator_hold dispatches");

        // BOUNDED for the same reason as the gate round-trip above: a command that never reaches
        // `deliver` must fail fast and name itself, not hang the test binary.
        let decision = tokio::time::timeout(AWAIT_BOUND, rx)
            .await
            .expect("the dispatched command never delivered a decision to the armed hold")
            .expect("the armed sender stayed connected");
        assert_eq!(
            decision,
            Decision::Go,
            "the dispatched command delivered the operator's Go"
        );
    }

    #[test]
    fn the_tauri_resolver_identifies_itself_as_the_dialog_resolver() {
        // `kind` is the resolver's identity in the hold record; nothing else asserted it, which is
        // why both its mutants survived. Treated as ONE disposition unit: the `""` mutant is a stable
        // miss and the `"xyzzy"` sibling flips missed/unviable across runs on an identical tree.
        let resolver = TauriResolver::new(HoldGate::default(), Channel::new(|_| Ok(())));
        assert_eq!(resolver.kind(), "tauri-dialog");
    }
}
