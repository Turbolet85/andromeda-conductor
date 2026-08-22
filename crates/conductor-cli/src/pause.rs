//! The CLI interactive operator-pause resolver — the tty-gated half of the core hold mechanism.
//!
//! [`conductor_core::resolve_hold`] awaits a [`Decision`] through a [`PauseResolver`]; this seam
//! supplies the interactive answer. [`CliResolver::select`] is the isatty gate: an interactive
//! terminal drives the [`PromptResolver`] (`inquire` confirm), while a piped / agent / redirected
//! session falls back to the core [`HeadlessResolver`] that NEVER blocks (the headless invariant —
//! the source-of-truth path is never gated on a prompt). The `inquire` prompt is synchronous
//! wall-clock time outside the seeded virtual clock, so it never perturbs the emission-stream shape.
//!
//! `PauseResolver::resolve` returns `impl Future` (not object-safe), so dispatch is a match over the
//! [`CliResolver`] enum, not a trait object. Status is never color-alone: the `[HOLD]` phase-line
//! (`render::hold_line`) carries its ASCII prefix with color as a tty-gated overlay.

use std::io::IsTerminal;

use conductor_core::{Decision, HeadlessResolver, HoldPoint, PauseResolver};
use indicatif::ProgressBar;
use inquire::Confirm;

use crate::render;

/// The interactive resolver: prints the `[HOLD]` phase-line above an `inquire` confirm and maps the
/// operator's answer to a [`Decision`]. Holds the live run spinner (when one exists) so the heartbeat
/// freezes at its current count for the duration of the prompt.
pub struct PromptResolver {
    spinner: Option<ProgressBar>,
}

impl PauseResolver for PromptResolver {
    fn kind(&self) -> &'static str {
        "cli-interactive"
    }

    async fn resolve(&self, hold: &HoldPoint) -> Decision {
        let render_and_prompt = || {
            eprintln!("{}", render::hold_line(hold));
            confirm(hold)
        };
        match &self.spinner {
            Some(bar) => bar.suspend(render_and_prompt),
            None => render_and_prompt(),
        }
    }
}

/// Ask the operator to go/no-go on `hold`. `resolve` is infallible, so an `inquire` error (Esc /
/// Ctrl-C) collapses to a [`Decision`]: a cancelled prompt is the operator backing out — `NoGo` when
/// a no-go is offered, otherwise `Go` (the proceed-only hold has no decline outcome).
fn confirm(hold: &HoldPoint) -> Decision {
    let message = if hold.allow_no_go {
        format!("{} — proceed?", hold.prompt)
    } else {
        format!("{} — acknowledge to proceed", hold.prompt)
    };
    let answer = Confirm::new(&message).with_default(true).prompt();
    match (hold.allow_no_go, answer) {
        (false, _) => Decision::Go,
        (true, Ok(true)) => Decision::Go,
        (true, Ok(false)) => Decision::NoGo,
        (true, Err(_)) => Decision::NoGo,
    }
}

/// How the CLI answers an operator hold — interactive when attended, headless when not. The trait is
/// not object-safe, so the two resolvers are an enum dispatched by match.
pub enum CliResolver {
    Interactive(PromptResolver),
    Headless(HeadlessResolver),
}

impl CliResolver {
    /// The agent-mode + isatty gate: an interactive `inquire` prompt only when NOT in agent mode AND
    /// BOTH stdin and stdout are a terminal (the prompt reads stdin, draws on stdout); otherwise the
    /// headless never-block default. `--agent-mode` forces Headless regardless of the tty (the
    /// release-gate-never-blocks invariant) — an OR-override on the same `IsTerminal` primitive
    /// `render::stdout_color()` uses.
    pub fn select(spinner: Option<ProgressBar>, agent_mode: bool) -> Self {
        match resolve_kind(agent_mode, std::io::stdin().is_terminal(), std::io::stdout().is_terminal()) {
            Kind::Interactive => CliResolver::Interactive(PromptResolver { spinner }),
            Kind::Headless => CliResolver::Headless(HeadlessResolver::proceed()),
        }
    }
}

/// Which resolver [`CliResolver::select`] picks — factored out as a pure decision so both arms test
/// deterministically without a pty (the `render::*_styled(color)` testable-core pattern).
enum Kind {
    Interactive,
    Headless,
}

/// `Headless` when agent mode is forced OR either stream is not a terminal; `Interactive` only when
/// attended on both streams.
fn resolve_kind(agent_mode: bool, stdin_tty: bool, stdout_tty: bool) -> Kind {
    if agent_mode || !stdin_tty || !stdout_tty {
        Kind::Headless
    } else {
        Kind::Interactive
    }
}

impl PauseResolver for CliResolver {
    fn kind(&self) -> &'static str {
        match self {
            CliResolver::Interactive(r) => r.kind(),
            CliResolver::Headless(r) => r.kind(),
        }
    }

    async fn resolve(&self, hold: &HoldPoint) -> Decision {
        match self {
            CliResolver::Interactive(r) => r.resolve(hold).await,
            CliResolver::Headless(r) => r.resolve(hold).await,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use conductor_core::{PId, resolve_hold};

    fn hold() -> HoldPoint {
        HoldPoint {
            scenario: "restart-suppression".to_string(),
            p_id: PId("P-015".to_string()),
            step: "restart-pulse".to_string(),
            prompt: "Restart the Pulse process, then confirm".to_string(),
            allow_no_go: true,
            checklist: Vec::new(),
        }
    }

    #[test]
    fn select_off_tty_is_headless() {
        // The test harness captures stdin/stdout (not a terminal) ⇒ the never-block default — the
        // headless invariant the agent path depends on.
        assert!(matches!(CliResolver::select(None, false), CliResolver::Headless(_)));
    }

    #[test]
    fn select_agent_mode_is_headless() {
        // `--agent-mode` forces the never-block default — the release-gate-never-blocks invariant.
        assert!(matches!(CliResolver::select(None, true), CliResolver::Headless(_)));
    }

    #[test]
    fn resolve_kind_agent_mode_overrides_an_attended_tty() {
        // The override that matters: even with BOTH streams a terminal, agent mode → Headless.
        assert!(matches!(resolve_kind(true, true, true), Kind::Headless));
        assert!(matches!(resolve_kind(false, true, true), Kind::Interactive));
        assert!(matches!(resolve_kind(false, false, true), Kind::Headless));
        assert!(matches!(resolve_kind(false, true, false), Kind::Headless));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn headless_variant_proceeds() {
        let resolver = CliResolver::Headless(HeadlessResolver::proceed());
        assert_eq!(resolve_hold(&resolver, &hold()).await.decision, Decision::Go);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn headless_variant_can_decline() {
        let resolver = CliResolver::Headless(HeadlessResolver::abort());
        assert_eq!(resolve_hold(&resolver, &hold()).await.decision, Decision::NoGo);
    }
}
