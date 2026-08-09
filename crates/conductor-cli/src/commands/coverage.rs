//! `conductor coverage [--write]` — render the capability coverage matrix; `--write` regenerates
//! `coverage-matrix.md` at the repo root (the definition-of-done artifact).

use std::path::Path;
use std::process::ExitCode;

use anyhow::Context as _;
use conductor_report::CoverageMatrix;

use crate::render;

pub fn coverage(write: bool) -> anyhow::Result<ExitCode> {
    println!("{}", render::coverage_table());
    println!("{}", render::coverage_summary());
    if write {
        CoverageMatrix::write(Path::new("coverage-matrix.md")).context("write coverage-matrix.md")?;
    }
    Ok(ExitCode::SUCCESS)
}
