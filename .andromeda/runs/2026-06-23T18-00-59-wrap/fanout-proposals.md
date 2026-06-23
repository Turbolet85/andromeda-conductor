# Fan-out proposals + validation — 2026-06-23-conductor-run-suite-report-verbs

## arch (2 proposals → APPLY, consolidated)
- D-arch-resources: add `clap 4 (derive)` row to §Stack table. **APPLY (routine)** — genuinely new dep; §Stack is the dep registry (lists tokio/serde/garde/thiserror/anyhow/tracing). Not the symbol/file/token over-reach.
- D-arch-decisions: note clap in §Inherited Defaults. **APPLY (routine, folded)** — keep §Inherited Defaults synced with §Stack. Both → one sidecar entry.

## security (3 → all FALSE-POSITIVE)
- D-security-deps (tauri ≥2.10.3): **DISMISS** — playbook line 31-33 (not-added dep already at floor). tauri untouched, pinned 2.10.3; `cargo audit` exit 0 (would catch a tauri CVE). The real new dep `clap` is audit-clean.
- D-security-input (canonicalize CONDUCTOR_* paths): **ESCALATE→recommend dismiss** — the impl uses `conductor_core::resolve_under` for all three handles (report states it); resolve_under IS the mandated `canonicalize` + `..`/absolute/symlink-escape guard (config_path.rs). §Input Validation satisfied. No exact playbook rule.
- D-security-subprocess (spawn pattern): **ESCALATE→recommend dismiss** — the chunk does NOT modify the spawn; it calls the EXISTING hardened `ReadbackClient::connect` (conductor-verify, shipped mcp-read-back-client; build_command/resolve_data_dir are crate-private + unchanged). Hermetic test exercises the metachar-reject. No new boundary added. No exact playbook rule.

## obs (2)
- D-obs-redaction: **ESCALATE→recommend dismiss** — the CLI's structured logs flow through `init_observability`'s JsonObsLayer (field-allowlist `is_allowlisted` + `redact_value` scrub, obs.rs); artifacts flow through conductor-report's hygiene + the path-free 11-field RunRecord envelope. No new unredacted log/artifact path added. §11 invariant holds. No exact playbook rule.
- D-obs-instrumentation (scenario.run root span): **DISMISS doc-edit; carry code follow-up** — obs §4 is target-state + correct (names scenario.run); playbook line 57 forbids "not yet" caveats in §4. This chunk IS the driver, so the missing CLI root span is a genuine CODE follow-up (carried to handoff), not doc drift.

## design / a11y / tests
- design D-design-tokens: `proposals: []` (CLI ASCII Lamp prefixes, never color-alone — satisfied).
- a11y D-a11y-surface / D-a11y-obs-schema: `proposals: []` (CLI not-assertable; envelope unchanged).
- tests D-tests-coverage / D-tests-framework: clean (assert_cmd + nextest match §4). D-tests-obs-harness: **DISMISS** — playbook line 46-48 (pre-existing §3↔§3 dual-record-shape gap, chunk changed neither bound section; carried follow-up).

## layouts (1 → APPLY)
- D-layout-surface: add a brief `conductor report <run_id>` output note to layout-templates §CLI (run/suite documented, report not). **APPLY (routine)** — real CLI output surface.

## Pattern → proposed playbook rule
The 3 escalate false-positives share a root: an escalate-severity security/obs detector demands re-confirmation of a hardening/validation invariant the chunk CONSUMES from already-shipped, unchanged infrastructure (`resolve_under` / `ReadbackClient::connect` / `init_observability`'s redaction layer) rather than adding a new boundary. Pre-empts re-fire on ch2–ch5 + Epoch-9.
