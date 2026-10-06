//! The real-model harvest's grading families and dated series, as CHILD modules of
//! `real_model_harvest.rs`. A child sees the rule's private items through `use crate::*;`, so the rule
//! gains no `pub` and its pinned bytes never move. A `tests/` subdirectory module, so it is never a test
//! target of its own.
//!
//! Below the declarations sits the one shape every dated series grades through.
#![allow(dead_code)]

mod canary_pairing;
mod capture_population;
mod capture_tokens;
mod scrub_and_sweep;
pub(crate) mod series_2026_09_29;
pub(crate) mod series_2026_09_30;
pub(crate) mod series_2026_10_01;
pub(crate) mod series_2026_10_06;
mod witnesses;
mod workspace_mask;

use crate::*;

/// What the rule measured on one drive: its route, its rank-1 grade, the three further grades and the
/// canary tokens the capture printed.
pub(crate) type Measured = (Route, Grade, [Outcome; 3], CanaryAttempts);

/// One dated series: its drives, the directory their committed captures sit in, and what the rule
/// measured on each.
pub(crate) struct Series {
    pub(crate) drives: &'static [Drive],
    pub(crate) evidence: &'static str,
    pub(crate) measured: fn(&str) -> Measured,
}

impl Series {
    fn name(&self, drive: &Drive) -> String {
        format!("{}/{}", self.evidence, drive.file)
    }

    /// A drive's committed capture, digest-checked.
    pub(crate) fn capture(&self, drive: &Drive) -> String {
        pinned(&self.name(drive), drive.sha256)
    }

    /// Every drive's committed capture matches its pinned digest.
    pub(crate) fn captures_match_their_pins(&self) {
        for drive in self.drives {
            let name = self.name(drive);
            let text = committed(&name);
            assert_eq!(
                check_digest(&name, &text, drive.sha256),
                Ok(()),
                "{}",
                drive.label
            );
        }
    }

    /// Every drive's pre-leg rule record equals the rule this target carries now.
    pub(crate) fn drives_recorded_the_current_rule(&self) {
        let current = rule_section(include_str!("../real_model_harvest.rs"))
            .expect("this file carries its rule");
        for drive in self.drives {
            let recorded =
                rule_section(&self.capture(drive)).expect("the capture opens with the rule record");
            assert_eq!(
                recorded, current,
                "{}: the rule moved after the drive",
                drive.label
            );
        }
    }

    /// The capture's own elision has nothing left to do on any drive, and no drive carries `key`.
    pub(crate) fn captures_carry_no_fingerprint(&self, key: Option<&str>) {
        for drive in self.drives {
            let committed = self.capture(drive);
            assert_eq!(elide_fingerprints(&committed), committed, "{}", drive.label);
            if let Some(key) = key {
                assert!(
                    !committed.contains(key),
                    "{}: the workspace key",
                    drive.label
                );
            }
        }
    }

    /// Every drive grades as the series' ledger records, with its route's trace witness set.
    pub(crate) fn drives_grade_as_the_ledger_records(&self) {
        for drive in self.drives {
            let (route_, grade_, further, tokens) = (self.measured)(drive.label);
            let committed = self.capture(drive);
            let block = capture_block(&committed);
            assert_eq!(route(block), route_, "{}", drive.label);
            assert_eq!(grade(block), grade_, "{}", drive.label);
            assert_eq!(
                [structure(block), steps(block), retrieval(block)],
                further,
                "{}",
                drive.label
            );
            assert_eq!(canary_attempts(block), tokens, "{}", drive.label);
            assert!(
                trace_conforms(block),
                "{}: the trace witness set",
                drive.label
            );
        }
    }

    /// B1 and the launch witness hold on every drive.
    pub(crate) fn drives_witness_the_real_model_and_a_clear_launch(&self) {
        for drive in self.drives {
            let committed = self.capture(drive);
            let block = capture_block(&committed);
            assert!(real_model_witnessed(block), "{}", drive.label);
            assert!(launch_cwd_clear(block), "{}", drive.label);
        }
    }

    /// The drives v3-09's pass condition grades — those that attributed an incident on the read-back
    /// route — each with its rank-1 grade.
    pub(crate) fn graded(&self) -> Vec<(&'static str, Grade)> {
        self.drives
            .iter()
            .filter_map(|drive| {
                let committed = self.capture(drive);
                let block = capture_block(&committed);
                (route(block) == Route::ReadBack && attributed_report(block).is_ok())
                    .then(|| (drive.label, grade(block)))
            })
            .collect()
    }
}

/// The pass condition (posture contract, The drive series (a)): met only if at least one drive is graded
/// AND every graded drive reads Identified.
pub(crate) fn v3_09_met(graded: &[(&str, Grade)]) -> bool {
    !graded.is_empty() && graded.iter().all(|(_, g)| *g == Grade::Identified)
}

/// A contract section, LF-normalized: its heading line up to the next `## ` heading.
pub(crate) fn contract_section(text: &str, heading: &str) -> Option<String> {
    let start = text.find(&format!("\n{heading}\n"))? + 1;
    let rest = &text[start..];
    let end = rest[1..].find("\n## ")? + 2;
    Some(rest[..end].to_string())
}

/// A series' contract section was fixed before `d1`: its digest now, the digest its attempt ledger in
/// `evidence` recorded before the first drive, and the pin `sha256` agree.
pub(crate) fn pre_registered(evidence: &str, heading: &str, sha256: &str) {
    let contract = committed("contracts/pulse-real-model-leg-posture.md");
    let section = contract_section(&contract, heading).expect("the section exists");
    let recorded = committed(&format!("{evidence}/attempt-ledger.md"))
        .lines()
        .find_map(|l| {
            l.strip_prefix("pre-registration sha256: ")
                .map(str::to_owned)
        })
        .expect("the ledger recorded the pre-registration digest");
    assert_eq!(recorded, sha256);
    assert_eq!(
        check_digest(
            "contracts/pulse-real-model-leg-posture.md",
            &section,
            sha256
        ),
        Ok(())
    );
}
