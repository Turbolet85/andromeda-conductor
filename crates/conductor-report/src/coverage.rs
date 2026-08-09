//! The coverage-matrix Markdown artifact.
//!
//! Renders the `conductor-core` [`coverage_matrix`] classification (every capability the SUT capability
//! manifest accepts) into `coverage-matrix.md` — Conductor's definition-of-done artifact. The render is a pure function of
//! the committed classification (no clock, no IO), so the artifact is byte-identical across runs
//! (exact-string golden-testable, like [`RunReport`](crate::RunReport)). Unlike the per-run report
//! (run_id-stemmed, `create_new`, never-overwrite), `coverage-matrix.md` is a single **regenerated**
//! definition-of-done file: [`CoverageMatrix::write`] is an atomic deterministic OVERWRITE.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use conductor_core::{CapabilityRow, CoverageMode, coverage_matrix};

use crate::ReportError;

/// The coverage-matrix generator.
pub struct CoverageMatrix;

impl CoverageMatrix {
    /// Render the coverage matrix to a Markdown string — a pure function of the committed
    /// classification (no clock, no IO), so the output is byte-identical for the fixed table.
    pub fn render() -> String {
        let rows = coverage_matrix();
        let mut out = String::new();
        let _ = writeln!(out, "# Coverage matrix");
        let _ = writeln!(out);
        let _ = writeln!(out, "{}", summary_line(rows));
        let _ = writeln!(out);
        let _ = writeln!(out, "| P-ID | Title | Category | Mode |");
        let _ = writeln!(out, "|---|---|---|---|");
        for r in rows {
            let _ =
                writeln!(out, "| `{}` | {} | {} | {} |", r.p_id, r.title, r.category, r.mode.label());
        }
        out
    }

    /// Render + write the coverage matrix to `out_path`, returning the written path.
    ///
    /// `coverage-matrix.md` is a single regenerated definition-of-done artifact, so the write is an
    /// **atomic deterministic overwrite** (`.tmp` → rename) — NOT the run-report's `create_new`:
    /// regeneration must always succeed, and a fixed classification yields a byte-identical file. A
    /// write fault is a harness fault ([`ReportError`] → `Err`), never a verification outcome.
    pub fn write(out_path: &Path) -> Result<PathBuf, ReportError> {
        if let Some(parent) = out_path.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent)?;
        }
        let mut tmp = out_path.as_os_str().to_owned();
        tmp.push(".tmp");
        let tmp = PathBuf::from(tmp);
        fs::write(&tmp, Self::render())?;
        fs::rename(&tmp, out_path)?;
        Ok(out_path.to_path_buf())
    }
}

/// The per-mode tally in fixed [`CoverageMode::ALL`] order (deterministic — no map iteration).
fn summary_line(rows: &[CapabilityRow]) -> String {
    let mut line = format!("**Capabilities** {}", rows.len());
    for mode in CoverageMode::ALL {
        let n = rows.iter().filter(|r| r.mode == mode).count();
        let _ = write!(line, " · {n} {}", mode.label());
    }
    line
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::TempDir;

    #[test]
    fn render_is_deterministic() {
        assert_eq!(CoverageMatrix::render(), CoverageMatrix::render());
    }

    /// The summary line's FORMAT, locked exactly over a fixed synthetic set — so the shape stays
    /// golden-tested without baking the live capability count (which the manifest owns).
    #[test]
    fn summary_line_format_is_exact() {
        let rows = [
            CapabilityRow { p_id: "P-001", title: "t", category: "c", mode: CoverageMode::Auto },
            CapabilityRow { p_id: "P-002", title: "t", category: "c", mode: CoverageMode::Auto },
            CapabilityRow {
                p_id: "P-003",
                title: "t",
                category: "c",
                mode: CoverageMode::DriveObserve,
            },
            CapabilityRow {
                p_id: "P-004",
                title: "t",
                category: "c",
                mode: CoverageMode::NotConductors,
            },
        ];
        assert_eq!(
            summary_line(&rows),
            "**Capabilities** 4 · 2 auto · 1 drive+observe · 0 static-only · 1 not-conductors"
        );
    }

    #[test]
    fn renders_title_summary_and_table_header() {
        let rows = coverage_matrix();
        let md = CoverageMatrix::render();
        let mut lines = md.lines();
        assert_eq!(lines.next(), Some("# Coverage matrix"));
        assert_eq!(lines.next(), Some(""));
        assert_eq!(lines.next(), Some(summary_line(rows).as_str()));
        assert_eq!(lines.next(), Some(""));
        assert_eq!(lines.next(), Some("| P-ID | Title | Category | Mode |"));
        assert_eq!(lines.next(), Some("|---|---|---|---|"));
    }

    /// The per-mode counts must account for every row — no capability silently uncounted.
    #[test]
    fn summary_counts_sum_to_the_row_count() {
        let rows = coverage_matrix();
        let summed: usize =
            CoverageMode::ALL.iter().map(|m| rows.iter().filter(|r| r.mode == *m).count()).sum();
        assert_eq!(summed, rows.len());
        assert!(summary_line(rows).starts_with(&format!("**Capabilities** {}", rows.len())));
    }

    #[test]
    fn every_capability_renders_as_an_exact_row() {
        let md = CoverageMatrix::render();
        for r in coverage_matrix() {
            let expected =
                format!("| `{}` | {} | {} | {} |", r.p_id, r.title, r.category, r.mode.label());
            assert!(md.contains(&expected), "missing exact row: {expected}");
        }
        assert_eq!(
            md.lines().filter(|l| l.starts_with("| `P-")).count(),
            coverage_matrix().len()
        );
    }

    #[test]
    fn no_host_paths_or_struct_names_leak() {
        let md = CoverageMatrix::render();
        for leak in [
            "C:\\",
            "/Users/",
            "/home/",
            "CoverageMatrix",
            "CapabilityRow",
            "CoverageMode",
            "coverage_matrix",
            "NotConductors",
        ] {
            assert!(!md.contains(leak), "leaked {leak:?}");
        }
    }

    #[test]
    fn write_overwrites_and_round_trips() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("coverage-matrix.md");
        let written = CoverageMatrix::write(&path).unwrap();
        assert_eq!(written, path);
        assert_eq!(std::fs::read_to_string(&path).unwrap(), CoverageMatrix::render());
        // A second write is a clean OVERWRITE (not create_new) — regeneration must succeed.
        CoverageMatrix::write(&path).unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), CoverageMatrix::render());
        // The temp sidecar is consumed by the rename.
        assert!(!path.with_file_name("coverage-matrix.md.tmp").exists());
    }
}
