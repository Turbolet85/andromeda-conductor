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
    /// The isatty gate: an interactive `inquire` prompt only when BOTH stdin and stdout are a
    /// terminal (the prompt reads stdin, draws on stdout); otherwise the headless never-block
    /// default — the same `IsTerminal` primitive `render::stdout_color()` uses.
    pub fn select(spinner: Option<ProgressBar>) -> Self {
        if std::io::stdin().is_terminal() && std::io::stdout().is_terminal() {
            CliResolver::Interactive(PromptResolver { spinner })
        } else {
            CliResolver::Headless(HeadlessResolver::proceed())
        }
    }
}

impl PauseResolver for CliResolver {
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
        }
    }

    #[test]
    fn select_off_tty_is_headless() {
        // The test harness captures stdin/stdout (not a terminal) ⇒ the never-block default — the
        // headless invariant the agent path depends on.
        assert!(matches!(CliResolver::select(None), CliResolver::Headless(_)));
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
