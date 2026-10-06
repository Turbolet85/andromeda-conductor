//! The workspace-key mask (the capture's fourth scrub stage).

use crate::*;

// ---- the workspace-key mask (the capture's fourth scrub stage) -------------------------------------
// Synthetic; the key is a letters-only leaf like the series' data dir.

pub(super) const KEY: &str = "rm-clean-series";

#[test]
fn the_mask_replaces_a_raw_key_in_a_body_line() {
    assert_eq!(
        mask_workspace_key("the dir rm-clean-series holds the corpus\n", Some(KEY)),
        "the dir <workspace-key> holds the corpus\n"
    );
}

#[test]
fn the_mask_replaces_the_key_in_a_previously_seen_suffix() {
    assert_eq!(
        mask_workspace_key(
            "## Previously Seen\n\n- incident #3 @ 1790699962319180900 — Disk Pressure (rm-clean-series)\n",
            Some(KEY)
        ),
        "## Previously Seen\n\n- incident #3 @ 1790699962319180900 — Disk Pressure (<workspace-key>)\n"
    );
}

#[test]
fn the_mask_replaces_a_scrubber_rendered_workspace_value() {
    assert_eq!(
        mask_workspace_key(
            "## Project Context\n\nworkspace=[redacted: credit_card]\n",
            Some(KEY)
        ),
        "## Project Context\n\nworkspace=<workspace-key>\n"
    );
}

#[test]
fn the_mask_replaces_the_workspace_line_without_a_key() {
    assert_eq!(
        mask_workspace_key("workspace=rm-clean-series\r\n  workspace=anything\n", None),
        "workspace=<workspace-key>\r\n  workspace=<workspace-key>\n"
    );
    assert_eq!(
        mask_workspace_key("rm-clean-series stays\n", None),
        "rm-clean-series stays\n"
    );
}

#[test]
fn the_mask_only_replaces_a_neighbour_bounded_key() {
    assert_eq!(
        mask_workspace_key(
            "alphabet alpha alpha-beta alpha.txt beta_alpha (alpha)",
            Some("alpha")
        ),
        "alphabet <workspace-key> alpha-beta alpha.txt beta_alpha (<workspace-key>)"
    );
}

#[test]
fn the_mask_is_idempotent() {
    let text = "## Project Context\n\nworkspace=rm-clean-series\n\n## Previously Seen\n\n- incident #2 @ 1 — Disk Pressure (rm-clean-series)\n";
    let once = mask_workspace_key(text, Some(KEY));
    assert!(!once.contains(KEY));
    assert_eq!(mask_workspace_key(&once, Some(KEY)), once);
    assert_eq!(mask_workspace_key(&once, None), once);
}

#[test]
fn the_later_scrub_stages_leave_the_placeholder_intact() {
    let masked = mask_workspace_key(
        "workspace=rm-clean-series\n- seen (rm-clean-series)\n",
        Some(KEY),
    );
    assert_eq!(elide_fingerprints(&masked), masked);
    let piped = elide_fingerprints(&mask_host_paths(&conductor_core::redact_value(&masked)));
    assert_eq!(
        piped.matches(WORKSPACE_KEY_PLACEHOLDER).count(),
        2,
        "{piped}"
    );
}

#[test]
fn a_path_valued_workspace_leaks_neither_its_key_nor_its_path() {
    // Pulse stamps the workspace as a path, so a Previously Seen suffix carries the whole path: the
    // mask takes the leaf, and the host-path stage takes the rest.
    let text = "workspace=\\\\?\\X:\\tmp\\pulse-legs\\rm-clean-series\n- incident #2 @ 1 — Disk Pressure (\\\\?\\X:\\tmp\\pulse-legs\\rm-clean-series)\n";
    let piped = elide_fingerprints(&mask_host_paths(&conductor_core::redact_value(
        &mask_workspace_key(text, Some(KEY)),
    )));
    assert!(!piped.contains(KEY), "{piped}");
    assert!(!piped.contains("pulse-legs"), "{piped}");
    assert!(piped.starts_with("workspace=<workspace-key>\n"), "{piped}");
}

#[test]
fn a_posix_path_valued_workspace_leaks_neither_its_key_nor_its_path() {
    // The same claim on a POSIX host, home-rooted and temp-rooted, through the capture's chain in its
    // order. The roots are built, never spelled.
    for root in [
        format!("/{}/someone/.cache", "home"),
        format!("/{}", "tmp"),
        format!("/{}/{}", "var", "tmp"),
    ] {
        let path = format!("{root}/pulse-legs/{KEY}");
        let text = format!("workspace={path}\n- incident #2 @ 1 — Disk Pressure ({path}) again\n");
        let piped = elide_fingerprints(&mask_host_paths(&conductor_core::redact_value(
            &mask_workspace_key(&text, Some(KEY)),
        )));
        assert!(!piped.contains(KEY), "{root}: {piped}");
        for segment in ["pulse-legs", "someone", ".cache", "tmp", "var", "home"] {
            assert!(!piped.contains(segment), "{root}: {segment}: {piped}");
        }
        assert!(piped.starts_with("workspace=<workspace-key>\n"), "{piped}");
        assert!(piped.ends_with(" again\n"), "{piped}");
    }
}

#[test]
fn the_rendering_witness_classifies_without_the_value() {
    let context = |value: &str| format!("## Project Context\n\nworkspace={value}\n");
    assert_eq!(workspace_rendering(&context(KEY), Some(KEY)), "verbatim");
    // Pulse stamps a path; its leaf is the key.
    assert_eq!(
        workspace_rendering(&context("/tmp/pulse-legs/rm-clean-series"), Some(KEY)),
        "verbatim"
    );
    assert_eq!(
        workspace_rendering(
            &context(r"\\?\X:\tmp\pulse-legs\rm-clean-series"),
            Some(KEY)
        ),
        "verbatim"
    );
    assert_eq!(
        workspace_rendering(&context("/tmp/rm-clean-series/logs"), Some(KEY)),
        "scrubbed"
    );
    assert_eq!(
        workspace_rendering(&context("[redacted: credit_card]"), Some(KEY)),
        "scrubbed"
    );
    assert_eq!(workspace_rendering(&context(KEY), None), "unknown-key");
    assert_eq!(
        workspace_rendering("## Evidence\n\n- none\n", Some(KEY)),
        "absent"
    );
}
