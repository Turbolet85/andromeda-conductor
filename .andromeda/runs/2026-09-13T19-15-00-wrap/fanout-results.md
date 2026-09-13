# Fan-out results — 2026-09-13-audit-debt-retired-before-epoch-1-closes

7 Explore doc-agents, one per spec source, one parallel batch. Entity-decode applied to every return
(`&amp;` appeared in the arch and test-plan payloads; `&lt;`/`&gt;` in the a11y return). `entities=0`
on every saved body.

**Raw twins:** `.raw-fanout-arch.md` and `.raw-fanout-test-plan.md` (the two returns carrying proposals).
The other five returned `proposals: []`; their prose commentary was stripped and their verdicts are
recorded below — this file is their sanctioned audit artifact.

## Verdicts

| doc | proposals | outcome |
|---|---|---|
| arch | 5 | 4 apply · 1 dismiss |
| security-plan | 0 | clean |
| design-system | 0 | clean |
| layout-templates | 0 | clean |
| test-plan | 11 | 10 apply · 1 dismiss |
| obs-plan | 0 | clean |
| a11y-plan | 0 | clean |

**Totals: 16 proposals · 14 applied · 2 dismissed · 3 escalations resolved with the operator.**

## Clean returns — what each checked

- **security-plan** — `proposals: []`. The chunk REMOVED a dependency edge and added none, so
  §Dependency Security's new-dependency invariant has no subject; no sidecar spawn / data-dir / preflight
  surface was touched.
- **design-system** — `proposals: []`. All four new surfaces carry `tokens n/a`; no UI rendered. Verified
  none of the four moved counts is baked into a palette row, ANSI-map entry or reuse tally (the doc's `6`s
  are the comfy-table columns and the six-lamp set — different subjects).
- **layout-templates** — `proposals: []`. No new surface or region; the `conductor cleanup` entry at `:189`
  is re-asserted MET. The doc's `14`s are the HOLD wireframe's step index, not the survivor tally.
- **obs-plan** — `proposals: []`. No new must-trace op, no span, no `tracing` call site; obs-plan nowhere
  asserts the removed `conductor-emit → conductor-core` edge (redaction is a subscriber layer, not a
  call-site dep). The `C:/Program Files/Git/etc/gitconfig` citation is wrap-report basis, not one of §11's
  three governed sinks.
- **a11y-plan** — `proposals: []`. No interactive element added; a11y §3's reproduced envelope and its
  13/15-key presence-not-exclusivity note both remain accurate, since only the key list's code HOME moved.

## Applied (14)

**arch — 4**
1. `D-arch-decisions` §Infrastructure Patterns (Build system, `:205`) — retire "the repo's sole
   `.gitattributes` rule" and "no repo-wide attribute policy is asserted".
2. `D-arch-decisions` §Infrastructure Patterns (directory tree, `:217`) — the `.gitattributes` comment,
   `dependent-of` #1.
3. `D-arch-resources` §Occupied Resources (Crate names, `:162`) — `conductor-core`'s "every other crate
   depends on" universal, now false for `conductor-emit`.
4. `D-arch-decisions` §Stack — register the host-Python instruments. **ESCALATED and resolved:** the
   proposal's premise ("the chunk introduces a runtime the stack does not yet allow") is FALSE —
   `scripts/code-graph.py` and `scripts/scip_pb2.py` predate this chunk. Operator chose *register
   accurately, no novelty implied*; the applied text is re-derived to name Python as the runtime it already
   was, with the code-graph pipeline as precedent.

**test-plan — 10**
5–9. `D-tests-derived-count` §12 ×5 (one primary + four `dependent-of`) — retire `file:line:col` member
   identity across all five roster members; key each by its cargo-mutants mutation DESCRIPTION joined to
   `scripts/mutation-roster.toml`'s `member`. **ESCALATED and resolved:** no playbook rule governs —
   `:127` requires BOTH de-literalization qualifiers to fail and only one does (prior wraps DID substitute
   fresh literals that re-staled, 2026-09-04 and 2026-09-05). Operator chose *retire coordinates*.
10. `D-tests-derived-count` §10 (Mutation-survivor disposition) — name the roster's executable form beside
   §12's citation home, joined by `member`.
11. `D-tests-derived-count` §1 (Coverage scope, `conductor-core` entity) — the same "every other crate
   depends on" universal, cited from arch. The cascade's lateral-citation class: arch owns the wording,
   test-plan cites it, so both move in this pass.
12. `D-tests-framework` §4 (Mutation instrument) — record the committed gate form.
13. `D-tests-framework` §9 (Scoped mutation audit) — point the invocation at it, `dependent-of` #12.
14. `D-tests-framework` §10 (Coverage thresholds) — replace the enumerated exclusion list with the shipped
   `--ignore-filename-regex '[\\/]tests[\\/]'`, noting it is broader for any `tests/` dir and narrower for
   the src-side `stub_pulse_mcp`.

## Dismissed (2)

- **arch `D-arch-resources` §Standard Contracts — register `conductor_core::ENVELOPE_KEYS_SORTED`.**
  Playbook `:37` GOVERNS; every qualifying clause holds — the proposal registers a public library API
  symbol (a `const`) in an arch registry section, and the chunk's actual occupied resources (ports,
  sockets, endpoints, IPC, events, env vars, workspace crates) are already registered, the report stating
  "No new IPC method, endpoint, port, socket or env var. No new `CONDUCTOR_*` handle." The rule's verdict
  is *routine — dismiss as over-reach*: arch tracks occupied resources and contract SHAPES, not per-crate
  public API surface. Precedent cited in the rule: the eight `conductor-emit` chunks registered none of
  their many public symbols.
- **test-plan `D-tests-coverage` §4 — a verification-duty clause for committed non-Rust instruments.**
  **ESCALATED and resolved:** this mints new policy rather than recording a fact the chunk produced; its
  sibling (#12) already records the instrument's committed form. Operator chose *drop it*.

## Near-misses the detectors checked and correctly rejected

- `test-plan.md:363` names `default_path()`, but that is `CapabilityManifest::default_path()`
  (`conductor-core/src/capability_manifest.rs:31`), which still exists — NOT the removed
  `ContractManifest::default_path()` in `conductor-verify`.
- `test-plan.md:80`'s `.gitattributes` LF-pinning sentence stays TRUE: `coverage-matrix.md` is still
  LF-pinned, now by an explicit rule that still wins (`git check-attr` → `text: set` / `eol: lf`).
- §12's historical "25 standing survivors → 8" is `conductor-run`'s 2026-09-03 score, not the workspace
  survivor count this chunk moved 25 → 20.

## Playbook / drift-base

No new rule proposed this pass. The three escalations were each settled by the operator on their own
coordinates; none established a recurring CLASS yet (the §12 coordinate retirement is a one-time form
change, and the Python registration was a premise error in a single proposal).
