# Fan-out results — 2026-08-08-dependency-advisory-remediation

7 Explore doc-agents, one parallel batch. **1 proposal total · 6 clean.**

| doc | verdict | detectors evaluated |
|---|---|---|
| arch | **1 proposal** (D-arch-decisions, warning) | D-arch-resources ✓ clean · D-arch-decisions → proposal |
| security-plan | `proposals: []` | D-security-input · D-security-subprocess · D-security-deps |
| design-system | `proposals: []` | D-design-tokens |
| layout-templates | `proposals: []` | D-layout-surface |
| test-plan | `proposals: []` | D-tests-coverage · D-tests-framework · D-tests-obs-harness |
| obs-plan | `proposals: []` | D-obs-instrumentation · D-obs-stack · D-obs-redaction |
| a11y-plan | `proposals: []` | D-a11y-surface · D-a11y-obs-schema |

Raw twin retained: `.raw-fanout-arch.md` (carried proposals). The six clean returns are recorded here.

## The one proposal (applied — routine)

- **detector:** D-arch-decisions · **severity:** warning
- **section:** §Stack and Technologies (Error handling row) + §Established Decisions [Error Handling] + §Inherited Defaults (Error handling)
- **change:** `anyhow 1.0.102` → `anyhow 1.0.104` at architecture.md:31, :52, :234; thiserror 2.0.18 and the verdict/error-wall prose untouched.
- **rationale:** the report's Changes → Dependencies lists anyhow 1.0.102 → 1.0.104 as both a lock bump and a `[workspace.dependencies]` manifest-floor raise, naming it "the one arch-registered version string this chunk moves".
- **validation:** playbook:28-30 (spec-wording → sound-impl reconciliation; the decision's invariants preserved, only the value refreshed) → **routine**. Not playbook:91-93 (decision reversal → escalate) — no decision changes. Intent-consistent: the chunk plan's Implementation notes predicted this amendment and named these three sites.

## Absence evidence (why the six returned clean)

Each cited the report's Changes bullets rather than inferring: `Symbols / APIs: none — zero Rust source delta`, `Crates / modules: none`, `Schema / config: none`, and `Coverage of new surfaces: none`. Specifically —

- **security-plan** independently confirmed §Dependency Security carries **no anyhow version string**, matching the operator's P5 correction; all its gates hold (audit exit 0, deny exit 0, `deny.toml` untouched so no new exception, lock committed + un-drifted, toolchain unchanged, `tauri 2.11.3` listed Unchanged and already ≥2.10.3 floor). It also correctly applied the routine dismiss to the pre-existing `deny.toml` 17/8-vs-one-of-each gap.
- **obs-plan** confirmed no logging/OTel package moved; `opentelemetry-proto` + `tonic` are in the Unchanged list, so §3's dormant-transitive note still holds. It read the empty-message `ERROR conductor_verify::client` lines as the designed field-allowlist drop, consistent with §11 — not a leak.
- **test-plan** confirmed every command in the report's Outcome is on-spec (§3 / §4 / §9), and that `ci.yml` / `.config/nextest.toml` / `deny.toml` are byte-unchanged so the §3 ↔ obs §3 harness bind is untouched.
- **design-system / layout-templates / a11y-plan** each keyed on `Coverage of new surfaces: none` — no UI element, surface, region, or violation-schema change to cover.

## Pre-existing gaps surfaced but deliberately NOT proposed (→ handoff follow-ups)

The arch agent applied the changed-only-if-listed rule (and playbook:31-33) to two version mismatches that appear ONLY in the report's *Unchanged (asserted)* list, so they are not this chunk's drift:

1. **`tauri` 2.11.3 resolved vs arch's "Tauri 2 (bundler v2.10.x, latest 2.10.1)"** — arch §Stack.
2. **`tokio` 1.52.3 resolved vs arch's "tokio 1.48.x"** — arch §Stack + §Established Decisions + §Inherited Defaults.

These join the third pre-existing item the report already carries (`deny.toml`'s 17 ignores / 8 allows vs security-plan §Accepted exceptions naming one of each). All three belong to a dedicated doc-reconcile pass, per playbook:46.
