# Fan-out results — 2026-08-09-out-of-scope-classification-treatment

7 doc-agents, one batch. 15 detectors evaluated against `report.md` alone.

| doc | detectors | verdict |
|---|---|---|
| arch | D-arch-resources · D-arch-decisions | **clean** — `proposals: []` |
| security-plan | D-security-input · D-security-subprocess · D-security-deps | **clean** — `proposals: []` |
| design-system | D-design-tokens | **1 proposal** (warning) |
| layout-templates | D-layout-surface | **1 proposal** (warning) |
| test-plan | D-tests-coverage · D-tests-framework · D-tests-obs-harness | **clean** — `proposals: []` |
| obs-plan | D-obs-instrumentation · D-obs-stack · D-obs-redaction | **1 proposal** (warning, from D-obs-instrumentation) |
| a11y-plan | D-a11y-surface · D-a11y-obs-schema | **1 proposal** (warning, from D-a11y-surface) |

All 7 returned parseable YAML with no preamble requiring a strip; the three clean returns are recorded here
(the sanctioned audit artifact for them). Raw twins saved for the four docs carrying proposals.

## Clean returns — why each held

- **arch** — Changes states no new IPC/endpoint/event/socket/port/env-var/crate and no dependency delta; the
  added symbols are intra-crate fns/consts inside three already-registered crates, below the altitude of
  arch's registries. `coverage-matrix.md` is already registered and its registered claim is unchanged by a
  presentational roll-up. Deviation 1 (`enforce_styling`) *upholds* the locked tty-gated-never-color-alone
  decision rather than contradicting it.
- **security-plan** — no new external-input surface (all three new surfaces record `validation n/a`), nothing
  touches the sidecar spawn / preflight / data-dir, and Dependencies is empty with both lockfiles un-drifted.
  The `cargo audit` red is external decay with zero dependency delta, not a dependency this chunk adds.
- **test-plan** — every new path carries a unit-tier test; all commands are on-spec (§3/§4); the single shape
  change is a rendered string, not the JSONL journal / `status` read / 5-command harness, so §3 ↔ obs §3 stay
  consistent on both sides.

## Proposals (4)

1. **D-design-tokens** → `design-system.md` §Surface: cli / Tokens (the ANSI-246 map entry, `:297`).
   Record the out-of-scope Mode cell as a **second non-lamp reuse** of the `--status-residual` ↔ ANSI 246
   pair (the first being `hint:` at `:314`), note the label always carries the signal, and note Markdown's
   emphasis as the surface-adapted counterpart. *Matches the report's Expected amendment 3.*
2. **D-layout-surface** → `layout-templates.md` §Surface: cli / Primary screens (the
   `conductor coverage [--write]` entry, `:178`). Document the **newly gained roll-up line** and its in-scope
   denominator form. *Not among the report's four Expected amendments — a genuine additional find.*
3. **D-obs-instrumentation** → `obs-plan.md` §4 (coverage-matrix completeness gate). Record that
   `coverage_percent`'s denominator is the manifest-derived **in-scope** count (distinct from
   `p_id_count_expected`, the full manifest count), so the Epoch-6 gate and the shipped roll-up cannot
   diverge. *Matches the report's Expected amendment 2.*
4. **D-a11y-surface** → `a11y-plan.md` §6 State color tokens. Record the non-lamp reuse of
   `--status-residual` and that the tinted Mode cell is **not a seventh lamp state** — the six-label
   not-color-alone assertion (§10-gated) keys on lamp display labels, and the Mode cell's signal is its own
   `not-conductors` text. *Not among the report's four Expected amendments — a genuine additional find.*

## Absence claims — verified by main before validation

- `layout-templates.md:178` describes only the comfy-table for `conductor coverage`; **no summary line named** ✓
- `a11y-plan.md:155` + `:433` bind `--status-residual` solely to the `Residual` lamp; the not-color-alone
  assertion enumerates **exactly six** display labels ✓
- `design-system.md:297` scopes ANSI 246 to `KnownResidual` only; `:314` is the `hint:` reuse ✓

## Expected amendments the fan-out did NOT surface (escalated)

The report named four Expected amendments. Detectors found #2 (obs) and #3 (design) and added two of their
own. **Not proposed by any detector:**

- **Expected amendment 1** — `layout-templates.md:126` Mode-cell entry, EXTEND with the out-of-scope
  treatment. D-layout-surface fired on the roll-up (`:178`) instead and treated `:126` as already-documented
  (it does document the four values — but not their treatment, which is exactly what this chunk shipped).
- **Expected amendment 4** — the `layout-templates.md:246` / `design-system.md:310` "coverage-matrix / SLO
  table" 6-column conflation. The design-system agent explicitly declined it as outside D-design-tokens'
  invariant; no layout detector raised it.

Both escalated to the operator (see the wrap console), with the playbook-rule tension on #4 named.
