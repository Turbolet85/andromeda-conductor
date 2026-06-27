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

use conductor_core::{Decision, HoldPoint, PauseResolver};
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
}

impl HoldPrompt {
    fn from_hold(hold: &HoldPoint) -> Self {
        Self {
            title: format!("{} — {}", hold.p_id.0, hold.step),
            body: hold.prompt.clone(),
            allow_no_go: hold.allow_no_go,
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
    use tauri::test::{get_ipc_response, mock_builder, mock_context, noop_assets, INVOKE_KEY};
    use tauri::webview::InvokeRequest;
    use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

    fn hold() -> HoldPoint {
        HoldPoint {
            scenario: "constellation-hue".to_string(),
            p_id: PId("P-025".to_string()),
            step: "observe-hue".to_string(),
            prompt: "Observe the constellation hue for this scenario".to_string(),
            allow_no_go: true,
        }
    }

    #[test]
    fn hold_prompt_projects_the_hold_point() {
        let prompt = HoldPrompt::from_hold(&hold());
        assert_eq!(prompt.title, "P-025 — observe-hue");
        assert_eq!(prompt.body, "Observe the constellation hue for this scenario");
        assert!(prompt.allow_no_go);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn gate_delivers_the_decision_to_the_awaiting_hold() {
        // The resolver-await half + the command-deliver half meet at the gate's oneshot.
        let gate = HoldGate::default();
        let (tx, rx) = oneshot::channel();
        gate.arm(tx);
        assert!(gate.deliver(Decision::Go), "a pending, connected hold accepts the decision");
        assert_eq!(rx.await.unwrap(), Decision::Go);
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

        assert_eq!(rx.await.unwrap(), Decision::Go, "the dispatched command delivered the operator's Go");
    }
}
