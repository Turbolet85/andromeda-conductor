# Fan-out drift results — 2026-06-21-runs-db-index wrap

7 doc-detectors, report-only inputs. **0 escalations.**

| doc | detector | result | resolution |
|---|---|---|---|
| arch | D-arch-resources | clean | runs.db + conductor-report already registered; `RunsDb`/`RunsDbError` are library API (over-reach per playbook §library-symbol rule) |
| arch | D-arch-decisions | **2 proposals (warning)** | (a) §Stack/§Inherited-Defaults: `libsqlite3-sys 0.36.0` / SQLite 3.50.4 → **applied** (routine doc-alignment, audit-green); (b) §Data-model/§Standard-Contracts: timestamps TEXT RFC-3339 → **applied** (routine, P4 user-ratified; SLO-math invariant preserved via `latency_ms` INTEGER) |
| security-plan | D-security-input | clean | no new external-input surface (consumes typed `RunRecord`; path resolved at cli edge) |
| security-plan | D-security-subprocess | clean | no sidecar/preflight/data-dir touch |
| security-plan | D-security-deps (escalate sev) | **1 proposal (warning)** | §Infrastructure/Database: SQLite 3.50.4 → **applied**. Did NOT escalate — dep arch-locked + audit-green (cargo-audit 0 / cargo-deny ok) + Cargo.lock committed |
| design-system | D-design-tokens | clean (`proposals: []`) | no UI |
| layout-templates | D-layout-surface | clean (`proposals: []`) | no UI surface |
| test-plan | D-tests-coverage / -framework / -obs-harness | clean (`proposals: []`) | 10 tests at unit/integ tier; nextest; envelope shape preserved (§3↔obs§3 agree) |
| obs-plan | D-obs-instrumentation | **1 proposal (warning)** | `db.insert_run` deferral note → **dismissed** per the any-seam-primitive playbook rule (the must-trace OP is the Epoch-8 live caller; obs §4 already correct). Playbook rule enumeration extended to `conductor-report [db.insert_run]` + confirming instance appended |
| obs-plan | D-obs-stack | clean | no OTel SDK / exporter added |
| obs-plan | D-obs-redaction (escalate sev) | clean | hygiene test proves no host-path/struct-name in any runs.db cell |
| a11y-plan | D-a11y-surface / -obs-schema | clean (`proposals: []`) | no UI; envelope schema unchanged |

**Applied:** 3 amendments — arch ×2 (versions, timestamps) + security-plan ×1 (version). Cascade: `docs/stack.md` version row updated; CLAUDE.md / security-summary / rules-security / timestamp-detail = grep-confirmed no-ops. Playbook: 1 rule enumeration extended (verdict-neutral upkeep). Sidecars: architecture-amendments.md (+2), security-plan-amendments.md (+1).
