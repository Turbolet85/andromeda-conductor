//! Line-oriented terminal rendering for the `conductor` verbs.
//!
//! Colorized status lines (owo-colors), a live run spinner (indicatif), and the results / coverage
//! tables (comfy-table) over the already-typed outcomes. [`Lamp`] is the single status-truth source
//! (conductor-core `lamp.rs`), reused verbatim — this seam only adds presentation. Status is never
//! color-alone: the ASCII `Lamp::status_prefix()` is always present and color is a tty-gated overlay
//! (CLAUDE.md universal invariant; design-system §Surface: cli). Color, the spinner, and table
//! styling share ONE `stdout_color()` / tty decision, so piped / `NO_COLOR` / `TERM=dumb` output
//! carries no escape bytes (the agent-mode log format is ch5; the operator-pause spinner-freeze is ch4).
//!
//! The public renders delegate to `*_styled(…, color)` cores so tests assert both the plain and
//! colored paths deterministically, independent of how the test harness wires stdout.

use std::io::IsTerminal;
use std::time::Duration;

use comfy_table::{Cell, Color, ContentArrangement, Table, presets};
use conductor_core::{HoldPoint, Lamp, RunRecord, coverage_matrix};
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use owo_colors::{OwoColorize, XtermColors};

/// The render of a measurement cell a blocked row never carries (the blocked-row null rule, mirrored
/// from the Markdown report).
const ABSENT: &str = "—";

/// The reserved mono ID-cyan (xterm 117) for identity fields (P-IDs, run_id, latency) — never
/// decoration (design-system §Surface: cli / Tokens).
pub const ID_CYAN: u8 = 117;

/// The muted grey (xterm 246) for the de-emphasized `hint:` label on the error edge.
const HINT_GREY: u8 = 246;

/// A check's lamp → its xterm-256 color (design-system §Surface: cli / Tokens; layout-templates
/// §Multi-surface coordination).
pub fn lamp_code(lamp: Lamp) -> u8 {
    match lamp {
        Lamp::Pass => 114,
        Lamp::Fail => 203,
        Lamp::Hold => 179,
        Lamp::Manual => 146,
        Lamp::Residual => 246,
        Lamp::Blocked => 60,
    }
}

/// Paint `text` in xterm-256 `code` when stdout color is enabled, else return it plain — the ASCII
/// content always stands alone (status is never color-alone).
pub fn paint(text: &str, code: u8) -> String {
    paint_styled(text, code, stdout_color())
}

/// The sanitized operator error edge: `error: <short>` + `hint: <fix>` on two lines (design-system
/// §cli "Error output"). Color is a tty-gated overlay on the `error:`/`hint:` labels (gated on
/// stderr, where the edge is printed); piped / `NO_COLOR` / agent stderr is plain ASCII — the labels
/// are the color-independent signal.
pub fn error_block(error_msg: &str, hint: &str) -> String {
    error_block_styled(error_msg, hint, stderr_color())
}

/// One colored status line — the verdict-first lamp prefix + the scenario name (the `run` per-check line).
pub fn status_line(record: &RunRecord) -> String {
    status_line_styled(record, stdout_color())
}

/// The operator-pause hold phase-line — the `[HOLD]` signature (amber overlay, prefix always
/// present) with the hold's scenario / P-ID / step. Rendered above the interactive prompt (the CLI
/// mirror of the desktop titlebar hold signature).
pub fn hold_line(hold: &HoldPoint) -> String {
    hold_line_styled(hold, stdout_color())
}

/// The per-run results table — one row per check, the blocked row em-dashing its never-measured cells.
pub fn results_table(records: &[RunRecord]) -> String {
    results_table_styled(records, stdout_color())
}

/// The static coverage matrix — all sixty P-IDs by mode (the definition-of-done classification; carries
/// no per-run lamp).
pub fn coverage_table() -> String {
    coverage_table_styled(stdout_color())
}

/// A live per-scenario progress spinner on stderr, hidden when stderr is not a terminal (so piped /
/// agent runs stay clean; the operator-pause freeze is ch4).
pub fn spinner(len: usize) -> ProgressBar {
    let target = if std::io::stderr().is_terminal() {
        ProgressDrawTarget::stderr()
    } else {
        ProgressDrawTarget::hidden()
    };
    let pb = ProgressBar::with_draw_target(Some(len as u64), target);
    if let Ok(style) = ProgressStyle::with_template("{spinner} {pos}/{len} scenarios") {
        pb.set_style(style);
    }
    pb.enable_steady_tick(Duration::from_millis(120));
    pb
}

/// `true` when stdout should carry color: a terminal, `NO_COLOR` unset, `TERM` not `dumb`
/// (design-system §Surface: cli / Platform-Specific Notes).
fn stdout_color() -> bool {
    std::io::stdout().is_terminal()
        && std::env::var_os("NO_COLOR").is_none()
        && std::env::var("TERM").map_or(true, |t| t != "dumb")
}

/// `true` when stderr should carry color — the error edge is printed to stderr, so it gates on
/// stderr's tty (mirrors [`stdout_color`] for the other stream).
fn stderr_color() -> bool {
    std::io::stderr().is_terminal()
        && std::env::var_os("NO_COLOR").is_none()
        && std::env::var("TERM").map_or(true, |t| t != "dumb")
}

fn paint_styled(text: &str, code: u8, color: bool) -> String {
    if color { text.color(XtermColors::from(code)).to_string() } else { text.to_string() }
}

fn error_block_styled(error_msg: &str, hint: &str, color: bool) -> String {
    let error_label = paint_styled("error:", lamp_code(Lamp::Fail), color);
    let hint_label = paint_styled("hint:", HINT_GREY, color);
    format!("{error_label} {error_msg}\n{hint_label} {hint}")
}

fn status_line_styled(record: &RunRecord, color: bool) -> String {
    let lamp = Lamp::for_record(record);
    format!("{} {}", paint_styled(lamp.status_prefix(), lamp_code(lamp), color), record.scenario)
}

fn hold_line_styled(hold: &HoldPoint, color: bool) -> String {
    let prefix = paint_styled(Lamp::Hold.status_prefix(), lamp_code(Lamp::Hold), color);
    format!("{prefix} — operator pause · {} · {} · {}", hold.scenario, hold.p_id.0, hold.step)
}

fn results_table_styled(records: &[RunRecord], color: bool) -> String {
    let mut table = Table::new();
    table
        .load_preset(presets::UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Disabled)
        .set_header(vec!["P-IDs", "Scenario", "State", "SLO", "Latency", "Fingerprints"]);
    for rec in records {
        let lamp = Lamp::for_record(rec);
        let p_ids = rec.p_ids.iter().map(|p| p.0.as_str()).collect::<Vec<_>>().join(" ");
        table.add_row(vec![
            tint(Cell::new(p_ids), ID_CYAN, color),
            Cell::new(&rec.scenario),
            tint(Cell::new(lamp.status_prefix()), lamp_code(lamp), color),
            Cell::new(wire(serde_json::to_value(rec.slo_tier))),
            Cell::new(latency(rec.latency_ms)),
            Cell::new(fingerprints(&rec.fingerprints)),
        ]);
    }
    table.to_string()
}

fn coverage_table_styled(color: bool) -> String {
    let mut table = Table::new();
    table
        .load_preset(presets::UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Disabled)
        .set_header(vec!["P-ID", "Title", "Category", "Mode"]);
    for r in coverage_matrix() {
        table.add_row(vec![
            tint(Cell::new(r.p_id), ID_CYAN, color),
            Cell::new(r.title),
            Cell::new(r.category),
            Cell::new(r.mode.label()),
        ]);
    }
    table.to_string()
}

/// Apply an xterm-256 foreground to a cell when color is enabled (comfy-table's own styling, so widths
/// stay correct), else leave it plain.
fn tint(cell: Cell, code: u8, color: bool) -> Cell {
    if color { cell.fg(Color::AnsiValue(code)) } else { cell }
}

/// An enum's bare serde wire spelling (`SloTier` → `<5s`), matching the Markdown report / JSONL form
/// (the `#[serde(rename)]` is the single source of truth) — mirrors `conductor-report`'s `wire`.
fn wire(value: serde_json::Result<serde_json::Value>) -> String {
    match value {
        Ok(serde_json::Value::String(s)) => s,
        Ok(other) => other.to_string(),
        Err(_) => ABSENT.to_string(),
    }
}

/// Latency in ms, or the em-dash for a never-measured (blocked) row.
fn latency(ms: Option<i64>) -> String {
    ms.map_or_else(|| ABSENT.to_string(), |n| n.to_string())
}

/// The fingerprint list: em-dash when never measured (`None`), `(none)` when measured-empty, else joined.
fn fingerprints(fps: &Option<Vec<String>>) -> String {
    match fps {
        None => ABSENT.to_string(),
        Some(v) if v.is_empty() => "(none)".to_string(),
        Some(v) => v.join(" "),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use conductor_core::{HoldPoint, PId, ReportState, SloTier, Verdict};

    fn measured(scenario: &str, verdict: Verdict, state: ReportState) -> RunRecord {
        RunRecord::measured(
            "2026-06-23T19-48-21-abc",
            424242,
            scenario,
            vec![PId("P-009".to_string())],
            verdict,
            state,
            "2026-06-23T19:48:21Z",
            "2026-06-23T19:48:22Z",
            1840,
            SloTier::Tier5s,
            vec!["fp-1".to_string()],
        )
    }

    fn blocked(scenario: &str) -> RunRecord {
        RunRecord::blocked(
            "2026-06-23T19-48-21-abc",
            424242,
            scenario,
            vec![PId("P-003".to_string())],
            SloTier::Tier20s,
        )
    }

    #[test]
    fn status_line_plain_keeps_prefix_without_escapes() {
        let line = status_line_styled(&measured("ok", Verdict::Pass, ReportState::Pass), false);
        assert!(line.starts_with("[PASS] ok"), "{line}");
        assert!(!line.contains('\u{1b}'), "plain line must carry no escape bytes: {line:?}");
    }

    #[test]
    fn status_line_colored_overlays_escapes_on_the_prefix() {
        let line = status_line_styled(&measured("ok", Verdict::Pass, ReportState::Pass), true);
        // the ASCII prefix survives under color (never color-alone) and color was applied
        assert!(line.contains("[PASS]"), "{line}");
        assert!(line.contains('\u{1b}'), "colored line must carry an escape: {line:?}");
    }

    #[test]
    fn error_block_plain_has_labels_without_escapes() {
        let block = error_block_styled("no scenario matches \"x\"", "pass a P-ID like P-009", false);
        assert!(block.starts_with("error: no scenario matches"), "{block}");
        assert!(block.contains("\nhint: pass a P-ID"), "{block}");
        assert!(!block.contains('\u{1b}'), "piped error edge must carry no escape bytes: {block:?}");
    }

    #[test]
    fn error_block_colored_overlays_escapes_on_the_labels() {
        let block = error_block_styled("boom", "try --debug", true);
        // the ASCII labels survive under color (never color-alone) and color was applied
        assert!(block.contains("error:") && block.contains("hint:"), "{block}");
        assert!(block.contains('\u{1b}'), "colored error edge must carry an escape: {block:?}");
    }

    #[test]
    fn results_table_plain_shows_lamps_and_em_dashes_blocked() {
        let table =
            results_table_styled(&[measured("ok", Verdict::Pass, ReportState::Pass), blocked("blk")], false);
        assert!(table.contains("[PASS]"), "{table}");
        assert!(table.contains("[BLOCKED]"), "{table}");
        assert!(table.contains("ok") && table.contains("blk"), "{table}");
        // the blocked row em-dashes its never-measured cells (latency, fingerprints)
        assert!(table.contains(ABSENT), "{table}");
        assert!(!table.contains('\u{1b}'), "piped table must carry no escape bytes");
    }

    #[test]
    fn calibration_region_renders_hold_not_manual() {
        let table = results_table_styled(
            &[measured("sev", Verdict::CalibrationRegion, ReportState::ManualCheck)],
            false,
        );
        assert!(table.contains("[HOLD]"), "{table}");
        assert!(!table.contains("[MANUAL]"), "{table}");
    }

    #[test]
    fn coverage_table_renders_all_sixty_pids() {
        let table = coverage_table_styled(false);
        for n in 1..=60 {
            assert!(table.contains(&format!("P-{n:03}")), "missing P-{n:03}");
        }
        assert!(!table.contains('\u{1b}'));
    }

    #[test]
    fn renders_leak_no_host_paths_or_struct_names() {
        let out = format!(
            "{}{}",
            results_table_styled(&[measured("ok", Verdict::Pass, ReportState::Pass), blocked("blk")], false),
            coverage_table_styled(false),
        );
        for leak in ["C:\\", "/Users/", "/home/", "RunRecord", "Lamp", "CapabilityRow", "RunsDb"] {
            assert!(!out.contains(leak), "leaked {leak:?}");
        }
    }

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
    fn hold_line_plain_keeps_prefix_without_escapes() {
        let line = hold_line_styled(&hold(), false);
        assert!(line.starts_with("[HOLD] "), "{line}");
        assert!(line.contains("restart-suppression") && line.contains("P-015"), "{line}");
        assert!(!line.contains('\u{1b}'), "plain hold line must carry no escape bytes: {line:?}");
    }

    #[test]
    fn hold_line_colored_overlays_escapes_on_the_prefix() {
        let line = hold_line_styled(&hold(), true);
        assert!(line.contains("[HOLD]"), "{line}");
        assert!(line.contains('\u{1b}'), "colored hold line must carry an escape: {line:?}");
    }
}
