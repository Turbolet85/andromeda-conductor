# Session Handoff

**Last Updated:** 2026-08-08T14:37:12Z
**Branch:** build/conductor-0.2.0
**Status:** clean
**Last Commit:** 2026-08-08-sut-capability-manifest — SUT capability manifest: the accepted P-ID set is data, not a compile-time constant

## Position
- Done: **2026-08-08-sut-capability-manifest** — `contracts/pulse-capabilities.toml` (82 ids, `P-001`..`P-082`) replaces garde's `(1..=60)` bound; validation split into shape (garde `pid_format`) + membership (`Scenario::check_capabilities` via `from_toml_str_with`), wired at the three binary-edge load sites. Proven live: P-074 loads, P-083 is rejected naming the manifest, and appending P-083 to the TOML made it load **with the same binary** — a Pulse release is now a data update.
- Next: **SUT-drift check** — loud failure when Pulse's ledger advances past the set Conductor knows. `/andromeda-phase` to promote + plan it. It carries a CARRY: build the check over the shipped `CapabilityManifest::load` / `accepts`, don't re-source the set.

## Work done
9 files (2 new: the manifest artifact + `conductor-core/src/capability_manifest.rs`). Workspace nextest **428/428** (was 420), clippy clean, doctests ok, `agent-run.sh run` exit 0, `Cargo.lock` un-drifted with no new dependency. `v2-01` verified.

## Drift resolved
7 detector proposals: **3 applied routine** (arch §Occupied Resources artifact registration · security-plan §Input Validation boundary row · test-plan §4 coverage bullet), **2 escalated and resolved with the operator**, **2 rejected**.
- **Escalation 1 — the 60→82 reversal, ratified.** Resolved by **de-hardcoding rather than substituting 82**: every "all 60 P-IDs" / "P-001..P-060" claim across arch, security-plan, test-plan, obs-plan, design-system and layout-templates now names the manifest's accepted set. A future Pulse release needs **zero doc edits** — which is why no playbook rule was proposed: the de-hardcoding removed the recurrence surface.
- **Escalation 2 — the red audit gate.** No spec amendment (playbook rule 7: not this chunk's drift); carried forward as a route entry instead (below).
- **Rejected:** `D-obs-instrumentation` proposed a `run_id`-may-be-null carve-out for pre-run lines — **false**, verified by running the binary: the manifest-load line carries the full §3 base schema including `run_id` (minted at obs init). `D-layout-surface`'s "document the cli error region" half — the `error:`/`hint:` region already exists and was reused with zero new tokens.
- Cascade: CLAUDE.md (overview · pointer-table · warnings), `commands.md`, `conventions.md`, `tests-summary.md`. 6 sidecars appended.

## Notes
- **⚠ Carried blocker — supply-chain gate is RED, and it is not this chunk's doing.** `cargo audit` / `cargo deny check advisories` fail on **RUSTSEC-2026-0190 / -0194 / -0195 / -0204** (`crossbeam-epoch 0.9.18`, `quick-xml 0.39.4`; two rated **7.5 high**). All published 2026-06-29 → 2026-07-06, during the 41-day pause. `Cargo.lock` is byte-identical to HEAD and this chunk added no dependency — **the gate decays with wall-clock time, not with the diff**. Fixes are transitive bumps through the Tauri tree (`quick-xml` 0.39 → 0.41 is a **major**). Deliberately NOT excepted in `deny.toml` — these are actionable vulnerabilities with fixes, not the unmaintained-transitive class. Now a markerless route entry: *Dependency advisory remediation*.
- **Decisions this chunk:** malformed/absent manifest is a `CoreError` **harness fault**, never `Blocked` (a blocked row needs identity fields that do not exist at load time) · the manifest lives in `contracts/` (arch's runtime-read root), not `.andromeda/refs/` · the three load edges were wired rather than embedding via `include_str!` · **no `CONDUCTOR_*` override handle** (one was written and reverted).
- **Context for the next chunks (below the curation confidence bar, kept here because it is load-bearing):**
  - garde's `#[garde(dive)]` **propagates Context** — giving `Scenario` a capability-set context would force it onto `PId`/`PhaseSpec`/`EmissionSpec`/`ExpectedCheck` and change `PId`'s public API. That is why membership lives outside garde.
  - `coverage.rs` is a `&'static str` `[CapabilityRow; 60]` documented "no runtime IO" — manifest-sourcing it is a **representation change**, not a re-point (pinned as a CARRY on the classification entry).
  - A CLI E2E rooted at a `TempDir` must carry every repo-shaped artifact the binary reads; `cli_smoke.rs` now copies `contracts/` via `copy_capability_manifest`.
- **Scope note:** the operator approved widening research's Files-to-modify 3 → 6 (the three load edges); two further files (`pause.rs`, `cli_smoke.rs`) were fix-loop consequences judged in-scope, so 8 total.
- **Curation:** T1 ×1 (code-graph reading discipline) · T2 ×0 · T3 ×0 · filters: 2 duplicate, 3 below confidence.
- **Last failed command:** none.
