//! Operator-pause orchestration — the runtime-agnostic hold/resume mechanism.
//!
//! A [`HoldPoint`] describes a *non-Conductor action* — one Conductor deliberately will not perform
//! itself (a Pulse restart for P-015, a manual observation of a visual claim on the [`crate::ReportState`]
//! `ManualCheck` path, any go/no-go gate before a committed timeline step) — and [`resolve_hold`]
//! awaits a [`Decision`] through a [`PauseResolver`], yielding a recorded [`HoldResolution`]. The
//! resolution is a VALUE, never a `Result::Err` (the verdict/error wall): a no-go is an operator
//! decision, not a harness fault.
//!
//! The abstraction lives in the core so both thin shells drive the *same* mechanism: the headless
//! [`HeadlessResolver`] (shipped here) never blocks — it answers immediately and the run records the
//! decision (the agent path is never gated on a prompt); the CLI `inquire` prompt (Epoch 8) and the
//! Tauri go/no-go dialog (Epoch 9) implement [`PauseResolver`] later. The pause is wall-clock time
//! *outside* the seeded virtual clock, so it never perturbs the emission-stream shape (architecture
//! §Design Philosophy: determinism under a seed).

use std::future::Future;

use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::redact_value;
use crate::scenario::{ChecklistItem, PId};

/// An operator's go/no-go answer to a [`HoldPoint`]. Closed set; serializes to its canonical
/// PascalCase name. A [`Decision`] is always a value (the verdict/error wall) — a [`Decision::NoGo`]
/// is the operator declining, never an error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Decision {
    /// Proceed — resume the run from the hold point.
    Go,
    /// Decline — the operator withheld confirmation.
    NoGo,
}

impl Decision {
    /// Human-facing status label — pairs with a glyph/color downstream so status is never
    /// color-alone (CLAUDE.md universal invariant); the `[HOLD]` amber phase-line is the shells'
    /// concern (Epoch 8/9).
    pub fn label(&self) -> &'static str {
        match self {
            Decision::Go => "Go",
            Decision::NoGo => "No-Go",
        }
    }
}

/// One operator hold: the non-Conductor action to perform/observe, the scenario/P-ID/step it gates,
/// and whether a no-go is permitted.
///
/// `#[derive(Validate)]` rejects empty text at load (`CoreError::Validation`); the `p_id` is checked
/// via the existing [`PId`] rule (`dive`). The `prompt` is redacted only at the artifact edge when a
/// [`HoldResolution`] is recorded — never here, so the in-memory hold keeps the operator's full text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
pub struct HoldPoint {
    /// The scenario this hold belongs to (e.g. `"restart-suppression"`).
    #[garde(length(min = 1))]
    pub scenario: String,
    /// The Pulse capability the hold gates.
    #[garde(dive)]
    pub p_id: PId,
    /// A short, stable step label (e.g. `"restart-pulse"`) — the report key for this hold.
    #[garde(length(min = 1))]
    pub step: String,
    /// The human-readable non-Conductor action to perform or observe before answering.
    #[garde(length(min = 1))]
    pub prompt: String,
    /// Whether a [`Decision::NoGo`] is an offered outcome (a go-only confirmation sets this false).
    #[garde(skip)]
    pub allow_no_go: bool,
    /// The scenario's declared operator-checklist items, each an induced state paired with the
    /// observation to confirm. Empty when the scenario declares none — the hold then carries only
    /// `prompt`, which is the shape every pre-checklist scenario keeps.
    #[serde(default)]
    #[garde(dive)]
    pub checklist: Vec<ChecklistItem>,
}

/// The recorded outcome of resolving a [`HoldPoint`] — a value the Epoch-6 run-report writers
/// consume (the verify classifier's `Assessment` is the sibling shape). `prompt` is pre-redacted
/// ([`redact_value`]) so no absolute host path reaches a log line or the run-report artifact
/// (security-plan §Error Handling).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HoldResolution {
    /// The operator's (or headless default's) decision.
    pub decision: Decision,
    /// Which resolver answered ([`PauseResolver::kind`]) — what makes a recorded resolution able to
    /// distinguish a real attended activation from the headless default.
    pub resolver_kind: &'static str,
    /// The scenario the hold belonged to.
    pub scenario: String,
    /// The Pulse capability the hold gated.
    pub p_id: PId,
    /// The hold's prompt, redacted for the artifact edge.
    pub prompt: String,
}

/// How a [`HoldPoint`] is answered — the seam between the core hold mechanism and a shell's go/no-go
/// surface, so the headless path, the CLI prompt, and the Tauri dialog all drive the one
/// [`resolve_hold`].
///
/// Declared with `-> impl Future` rather than `async fn` so the public trait does not trip the
/// `async_fn_in_trait` lint; implementors may write `async fn`.
pub trait PauseResolver {
    /// A short, stable label for HOW this resolver answers — the witness that separates a real
    /// attended activation from the headless default when a resolution is recorded.
    ///
    /// Hand-chosen, never a type name: an internal struct name must not reach a log line
    /// (security-plan §Error Handling), which rules out `std::any::type_name`.
    fn kind(&self) -> &'static str;

    /// Answer `hold` with a [`Decision`].
    fn resolve(&self, hold: &HoldPoint) -> impl Future<Output = Decision>;
}

/// The headless / agent-mode resolver: it NEVER blocks on operator input — it answers every hold
/// with its configured [`Decision`] immediately and the run records that decision (the headless
/// invariant: the source-of-truth path is never gated on an interactive prompt). The outcome is
/// configurable so a run can default to proceeding while tests exercise both branches.
#[derive(Debug, Clone, Copy)]
pub struct HeadlessResolver {
    default: Decision,
}

impl HeadlessResolver {
    /// A resolver that answers every hold with `default`.
    pub fn new(default: Decision) -> Self {
        Self { default }
    }

    /// A resolver that always proceeds ([`Decision::Go`]) — the agent-run default.
    pub fn proceed() -> Self {
        Self { default: Decision::Go }
    }

    /// A resolver that always declines ([`Decision::NoGo`]).
    pub fn abort() -> Self {
        Self { default: Decision::NoGo }
    }
}

impl PauseResolver for HeadlessResolver {
    fn kind(&self) -> &'static str {
        "headless"
    }

    async fn resolve(&self, _hold: &HoldPoint) -> Decision {
        self.default
    }
}

/// Resolve `hold` through `resolver`, returning the recorded [`HoldResolution`].
///
/// Infallible — the outcome is a value, never `Result::Err` (the verdict/error wall). Generic over
/// the resolver (no `dyn`, no boxing): the headless default and the later interactive shells all
/// drive this one path. The `prompt` is redacted into the resolution at the artifact edge.
pub async fn resolve_hold<R: PauseResolver>(resolver: &R, hold: &HoldPoint) -> HoldResolution {
    let decision = resolver.resolve(hold).await;
    HoldResolution {
        decision,
        resolver_kind: resolver.kind(),
        scenario: hold.scenario.clone(),
        p_id: hold.p_id.clone(),
        prompt: redact_value(&hold.prompt).into_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hold(prompt: &str) -> HoldPoint {
        HoldPoint {
            scenario: "restart-suppression".to_string(),
            p_id: PId("P-015".to_string()),
            step: "restart-pulse".to_string(),
            prompt: prompt.to_string(),
            allow_no_go: true,
            checklist: Vec::new(),
        }
    }

    #[test]
    fn decision_serializes_to_canonical_names() {
        assert_eq!(serde_json::to_string(&Decision::Go).unwrap(), "\"Go\"");
        assert_eq!(serde_json::to_string(&Decision::NoGo).unwrap(), "\"NoGo\"");
    }

    #[test]
    fn decision_round_trips_through_json() {
        for d in [Decision::Go, Decision::NoGo] {
            let json = serde_json::to_string(&d).unwrap();
            let back: Decision = serde_json::from_str(&json).unwrap();
            assert_eq!(d, back);
        }
    }

    #[test]
    fn decision_labels_are_stable() {
        assert_eq!(Decision::Go.label(), "Go");
        assert_eq!(Decision::NoGo.label(), "No-Go");
    }

    #[test]
    fn well_formed_hold_point_validates() {
        assert!(hold("Restart the Pulse process, then confirm").validate().is_ok());
    }

    #[test]
    fn empty_prompt_is_rejected() {
        assert!(hold("").validate().is_err());
    }

    #[test]
    fn empty_scenario_and_step_are_rejected() {
        let mut h = hold("do the thing");
        h.scenario = String::new();
        assert!(h.validate().is_err());

        let mut h = hold("do the thing");
        h.step = String::new();
        assert!(h.validate().is_err());
    }

    #[test]
    fn malformed_p_id_is_rejected_via_dive() {
        let mut h = hold("do the thing");
        // Shape-malformed, not merely absent from the SUT set: garde validates the P-NNN shape and
        // the capability manifest owns membership, so only a bad shape fails here.
        h.p_id = PId("P-99".to_string());
        assert!(h.validate().is_err());
    }

    #[test]
    fn hold_point_round_trips_through_json() {
        let h = hold("Observe the desktop toast");
        let json = serde_json::to_string(&h).unwrap();
        let back: HoldPoint = serde_json::from_str(&json).unwrap();
        assert_eq!(h, back);
    }

    #[test]
    fn headless_constructors_carry_decision() {
        assert_eq!(HeadlessResolver::proceed().default, Decision::Go);
        assert_eq!(HeadlessResolver::abort().default, Decision::NoGo);
        assert_eq!(HeadlessResolver::new(Decision::NoGo).default, Decision::NoGo);
    }
}
