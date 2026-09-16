# Fan-out results — 2026-09-16-scenario-assertion-audit-gate

7 Explore doc-agents, one parallel batch. **14 proposals across 3 docs; 4 docs clean.**
Entity-decode applied to every return (`&gt;-` → `>-`, `&amp;` → `&`, `&lt;`/`-&gt;` → `<`/`->`);
`entities=0` on every stored copy.

| doc | verdict |
|---|---|
| architecture | 10 proposals (3 primaries + 7 dependents) — raw twin `.raw-fanout-arch.md` |
| security-plan | 1 proposal (escalate) — raw twin `.raw-fanout-security-plan.md` |
| test-plan | 3 proposals (1 primary + 2 dependents) — raw twin `.raw-fanout-test-plan.md` |
| design-system | `proposals: []` — clean |
| layout-templates | `proposals: []` — clean |
| obs-plan | `proposals: []` — clean |
| a11y-plan | `proposals: []` — clean |

The four clean returns each reasoned from the report rather than merely returning empty, and two
independently corroborated the report's own `979` sweep (design-system and layout-templates both
re-ran it against their own doc and found 0). obs-plan additionally noted that minting a
`scenario_audit.*` span would itself have violated §11's bounded span-name set, so emitting no span
is the conforming choice — the same precedent §4 records for the coverage-completeness gate.

---

## Validation (orchestrator)

**Line profiles read before dispositioning any hit** (check 4's multi-KB clause):
`architecture.md` — 278 lines, longest 10 724 c (line 70), **13 lines over 2 000 c**;
`test-plan.md` — 616 lines, longest 9 079 c (line 307), **13 lines over 2 000 c**.
Every arch hit on a multi-KB line was resolved BY OFFSET with a bounded window, never from the grep
view. `architecture.md:53` is 2 862 c and carries two of the claims below.

### architecture — 8 apply · 2 dismiss

| # | section | disposition | basis |
|---|---|---|---|
| A | §Occupied Resources — On-disk artifacts | **APPLY** | `grep -c 'scenario-audit-ledger'` → **0**; the five sibling `contracts/` artifacts are individually registered at `:175-180`, so a new committed artifact earns a row. Rule 49/51 (per-file content in a registered directory) does NOT govern — `contracts/` members are registered individually, not at directory grain. |
| A1 | §Infrastructure Patterns — directory tree, `contracts/` comment | **APPLY** (dependent of A) | `:235` enumerates the `contracts/` members; left alone it asserts a five-member directory. |
| B | §Occupied Resources — Crate names (the 6 public API symbols + `CoreError` variant) | **DISMISS** | **playbook :37-39** — registering a public library API symbol in ANY arch registry section is over-reach; arch tracks ports/sockets/endpoints/IPC/events/env-vars/crates, not per-crate API surface. Precedent: 8 `conductor-emit` chunks registered none. |
| B1 | §Established Decisions [Accepted Capability Set] — the integrity-gate pair | **DISMISS** | **False hit, on its own merits and not merely by parent rejection.** `:53` @offset 1655 reads "A second integrity gate sits beside it on a different axis" and names `check_sut_drift` (classified?) / `check_scenario_backing` (backed?) — the pair is scoped to the ACCEPTED-CAPABILITY-SET axis. `check_load_envelope`, an existing static gate of identical shape returning a `CoreError`, appears ONLY at `:177` and is **not** in that pair (measured: `'check_load_envelope' in line 53` → False). So the sentence is not an enumeration of all integrity gates and is not stale; `check_scenario_audit` belongs where `check_load_envelope` sits — its artifact's §Occupied Resources row, which is proposal A. |
| B2 | §Established Decisions [Accepted Capability Set] — the load-site enumeration | **APPLY — orchestrator-raised, decoupled from B** | `:53` @offset 525 reads "applied by `Scenario::from_toml_str_with(&str, &CapabilityManifest)` at the three binary-edge load sites (`conductor-cli::paths`, `conductor-tauri::commands`, `list_scenarios(dir, &manifest)`)". `load_catalog` calls `from_toml_str_with`, so the literal "three" is stale. **Governed by playbook :127 directly** (a stale literal enumerating a SET the code owns → routine, apply as SET-NAMING, never a fresh count). See the escalation note below — it arrived as a dependent of a DISMISSED primary, which the atomic rule would reject. |
| C | §Established Decisions [CI/CD] (`:60`) | **APPLY** | A new CI gate step; the 2026-09-09 fmt-gate precedent established this five-site fan-out. |
| C1 | §Stack and Technologies — CI/CD row (`:37`) | **APPLY** (dependent of C) | Verified present: the row's gate list. |
| C2 | §Infrastructure Patterns — CI/CD approach (`:243`) | **APPLY** (dependent of C) | Verified present. |
| C3 | §Infrastructure Patterns — directory tree, `.github/workflows/` comment (`:240`) | **APPLY** (dependent of C) | Verified present; carries no `cargo fmt` token, so a token-keyed sweep would miss it. |
| C4 | §Inherited Defaults — CI/CD (`:273`) | **APPLY** (dependent of C) | Verified present; downstream scopes inherit it. |

**Finding surfaced while verifying C1–C4:** all five CI enumerations are ALREADY stale by one gate —
**none names the coverage-completeness gate**, which shipped as its own named CI step on 2026-09-06.
Measured across `:37`, `:60`, `:240`, `:243`, `:273`: every one reads "fmt + build + test + clippy +
the a11y webview e2e gate". Under playbook `:127` the remedy is SET-naming, so the applied text names
the static-gates-over-committed-data SET (coverage-completeness · scenario-assertion audit) rather
than either a fresh count or this chunk's gate alone — writing an enumeration I have just measured
incomplete would author a false body.

### test-plan — 3 apply

| # | section | disposition | basis |
|---|---|---|---|
| T1 | §6 — `#### Scenario: Coverage-matrix completeness gate (static)` (`:359`) | **APPLY** | The plan's Expected amendment #3. Recorded as a sibling leg INSIDE the existing static-gate scenario — never an eighth `####` scenario, so §6's "(7 scenarios — the test-scope Section 4 maximum.)" qualifier holds — and no §9 stage-table row, per the `2026-09-06-coverage-completeness-gate` operator resolution. |
| T2 | §4 — Scenario catalog + coverage-matrix bullet (`:250`, 542 c) | **APPLY** (dependent of T1) | Verified: the bullet names `conductor-report/tests/coverage_gate.rs` as the single crate-local `tests/`-tier gate over committed data. |
| T3 | §7 — Self-bootstrapping requirement (`:400`, 921 c) | **APPLY** (dependent of T1) | Verified: the bullet's committed-fixture sample names two trees ("today `crates/conductor-run/tests/fixtures/` … and `crates/conductor-tauri/tests/fixtures/scenarios/`"); this chunk adds a third. Applied as SET-naming per `:127`. |

### security-plan — 1 ESCALATE

See the escalation record below.

---

## Validation checks 2–6

2. **Cross-contradiction** — none. No two proposals edit the same section, and none opposes another.
3. **Intent-consistency** — aligned. The report matches the chunk's working-route entry and its plan
   acceptance criteria; no divergence to classify.
4. **Absence needs evidence** — every absence claim carries its search, and every hit on a multi-KB
   line was read by offset (above). No hit was dispositioned from a clipped view.
5. **Expected-amendments reconciliation** — the plan lists THREE entries; all three were proposed by
   detectors, none needed an orchestrator raise: `architecture.md` §Occupied Resources → A/A1 + C..C4;
   `security-plan.md` §Input Validation → the escalated proposal; `test-plan.md` §6 → T1. Floor met.
6. **Disproved-claims disposition** — the report's `Spec claims disproved by measurement` bullet is
   **none**, so nothing to dispose.
