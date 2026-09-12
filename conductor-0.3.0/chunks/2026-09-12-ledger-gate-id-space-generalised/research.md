# Codebase Research — 2026-09-12-ledger-gate-id-space-generalised

## Scope
- **Depth:** moderate · **Reads:** 6 · **Globs/Greps:** 9 · **Graph queries:** 2 (rust plane)
- **Harness rules consulted:** none — no live leg in this chunk. The gate is a static CI-runnable
  target over committed artifacts (no Pulse, no stub, no `runs.db`), so `verification-harness.md`'s
  firing-form recipes do not apply.

## Files inspected
- `crates/conductor-report/tests/matrix_ledger_gate.rs` (full, 267 lines) — the entire modify-set.
  Three `#[test]` arms plus `the_gate_discriminates`; five helper fns; `repo_root()` resolves from
  `CARGO_MANIFEST_DIR`.
- `crates/conductor-report/tests/coverage_gate.rs` (1-60 of 87) — the sibling gate all four in-domain
  extracts name as the pattern precedent. Confirms the shared idiom and the negative-arm discipline.
- `crates/conductor-report/Cargo.toml` (`[dev-dependencies]`) — **only** `assert_fs.workspace = true`.
- `Cargo.toml` (workspace root, `:81`) — `rstest = "0.26"` is declared at the workspace level.
- `.gitattributes` (full) — one rule, scoped to `coverage-matrix.md`.
- `conductor-0.{1,2,3}.0/requirements.md` (structural: declaring-form lines only).

## Graph impact (rust plane)

Trace: `.andromeda/runs/2026-09-12T10-52-00-phase/tree-query-2026-09-12-ledger-gate-id-space-generalised.json`
(`rows: 7`) and `…-impact.json` (`rows: 25`) — both `db_state: fresh`, plane `rust`. The counts below are
the trace's own `rows` field, not a count off a printed view.

- **All five helpers are indexed and all are file-local.** `requirement_ids`, `is_pulse_p_id`,
  `ledger_dirs`, `set_difference`, `id_positions` each resolve to exactly one `fn` symbol in
  `crates/conductor-report/tests/matrix_ledger_gate.rs`, crate `conductor-report`. (`Ledger#requirement_ids`
  and `Ledger#id_positions` also appear as `term` — the struct's fields, same file.)
- **25 call edges, every one inside that file.** Zero cross-file and zero cross-crate callers. The
  existence probe answers the "is this a query-pattern miss or a real leaf" question in the affirmative
  direction: the symbols ARE indexed, and the caller set is genuinely file-local.
- `requirement_ids` — 4 call sites (graph lines `134` ×2, `146`, `217`, `220`; graph is 0-indexed, so
  file lines 135, 147, 218, 221). Two are the production path (`ledgers` → the set-equality arm), two are
  `the_gate_discriminates` fixtures. **This is the function the repair changes, and its entire caller set
  is those four sites.**
- `is_pulse_p_id` — 5 sites, `set_difference` — 3, `ledgers` — 3, `ledger_dirs` — 1. None changes shape.

**Consequence for the modify-set:** the caller-threading enumeration closes at one file. No registration
site, no re-export, no crate-local companion test elsewhere pins this data.

## Patterns detected
- **`CARGO_MANIFEST_DIR`-rooted repo resolution** (`matrix_ledger_gate.rs:25-27`, identical at
  `coverage_gate.rs:25-27`): `Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")`. The gate already
  follows the sibling's idiom; the security extract's not-operator-steerable question is answered —
  there is no `CONDUCTOR_*` handle and no CWD-relative default anywhere in the file.
- **Anti-vacuity assertion per arm** (`:105-109` for dirs, `:151-154` for required ids, `:155-158` for
  matrix ids): each arm refuses to pass on an empty subject. `:151-154` is the assertion currently firing.
- **Negative arm that names what it dropped** (`coverage_gate.rs:46-60`): removes each real row IN TURN
  and asserts the error names the dropped id. The ledger gate's analogue is `the_gate_discriminates`.
- **Both-directions set equality** (`set_difference`, `:76-88`) returning `(missing, extra)` so the
  failure message says which direction diverged.
- **One shared basis** — arch asked whether the anti-vacuity assertion and the set-equality arm consult
  ONE predicate. **They do**: both read `Ledger.requirement_ids`, populated by the single
  `requirement_ids()` call at `:135`. There is one filter site to change, not two spellings.

## Conventions to follow
- **Plain `#[test]` fns, snake_case, no fixture framework** (`matrix_ledger_gate.rs:142,176,197,215`;
  `coverage_gate.rs:34,46`): every test in both gates is a bare `#[test]`, and `the_gate_discriminates`
  already carries a multi-case predicate matrix as inline assertions (`:250-252`).
- **Assertion messages name the version-dir label + the file, never a path** (`:119`, `:123`, `:128`,
  `:153`, `:157`, `:166`): the `{label}` is `dir.file_name()`, e.g. `conductor-0.3.0`.
- **Doc-header provenance citation** (`:1-19`): the module header states the gate's purpose, its
  capability provenance, and — at `:13-16` — the de-hardcoding rationale for the directory scan. That
  paragraph is the natural home for the same rationale on the id-space axis.

## Files to modify
- `crates/conductor-report/tests/matrix_ledger_gate.rs` — the whole change. Sites, re-derived by bare
  token sweep (13 `v2-` lines, against the 11 an anchored `"v2-` pattern returns):
  - `:49` the filter — the defect.
  - `:153`, `:208` assertion messages naming `v2-NN`.
  - `:218`/`:219`, `:221` fixtures exercising `requirement_ids` — must discriminate the new predicate.
  - `:228`, `:250`, `:256`, `:257`, `:259`, `:260` fixtures for id-space-agnostic helpers — may stay.
  - `:2` provenance citation — unchanged.
- No manifest change required by the recommended shape (see the rstest finding below).

## Findings that bear on the P4 decision

1. **`rstest` would cost a manifest + lockfile change.** It is a workspace dependency (`Cargo.toml:81`)
   but `conductor-report`'s `[dev-dependencies]` carries only `assert_fs`. Adding `rstest.workspace = true`
   moves `Cargo.lock` (a member's dependency list gains a line) while the package COUNT holds at 562 —
   the exact shape the standing rule says to state as a package-count basis, not byte-identity. The
   tests extract offers rstest as the plan's fixture pattern for a predicate matrix; the file's own
   established idiom is an inline assertion block, which costs nothing. **Decisive lean: inline, no
   rstest** — it keeps the no-dependency-delta property the scope, arch and security all assert.
2. **The `.gitattributes` LF question does not bind.** The one rule is scoped to `coverage-matrix.md`
   because that gate byte-compares a rendered artifact. This gate parses line-wise: `str::lines()`
   strips a trailing `\r`, and the id is extracted by `strip_prefix("- **")` + `split_once("**")`
   before end-of-line, so a CRLF checkout cannot contaminate an id. Both committed `requirements.md`
   files are LF-only on disk today. **No `.gitattributes` addition is needed.**
3. **Supply chain is green at HEAD, verified not assumed.** `$CARGO_HOME/advisory-db` porcelain prints
   0 lines (the local-currency check the corrected rule requires BEFORE any red is classified);
   `cargo audit` exit 0 (7 allowed warnings); `cargo deny check advisories bans licenses sources`
   exit 0 (one `advisory-not-detected` note on a now-inert ignore). `Cargo.lock` un-drifted, 562 packages.
4. **The red reproduces identically under BOTH runners** — `cargo nextest run -p conductor-report
   --test matrix_ledger_gate --profile ci` exit **100**, `cargo test -p conductor-report --test
   matrix_ledger_gate` exit **101**; both report 3 passed / 1 failed, same failing test, same message
   (`conductor-0.3.0/requirements.md declares no v2-NN capability — the gate would pass vacuously`).
   No runner-dependence to chase, and the RED baseline the plan's Test Commands need is established
   on both.
5. **The three other arms are already green** — `matrix_ids_are_unique`, `the_gate_discriminates` and
   `no_pulse_p_id_occupies_an_id_position` all PASS at HEAD. Only
   `every_requirement_capability_has_exactly_one_matrix_entry` fails. Notably `the_gate_discriminates`
   passes *because* its fixtures are `v2-`-shaped — it is green for a reason the repair removes, which
   is precisely why `:218`/`:219`/`:221` are in the modify-set rather than optional.
6. **No capability is claimable.** The unclaimed pool is 10 (`v3-02` … `v3-11`); read in full, none
   names the ledger gate — they cover the a11y terminal, keyboard/focus ownership, scenario-assertion
   hygiene, the P-025 contract, the live legs, diagnostic quality and secret scanning. This chunk links
   nothing.
7. **A latent, unreachable host-path render exists at `:116`** — `unwrap_or_else(|| dir.display().to_string())`
   is the `label` fallback when `file_name()` returns `None`. For a directory yielded by `read_dir` that
   branch is not reachable in practice. It is the only `display()` in the file; every reachable message
   renders `{label}` = the directory's own name. Recorded as an observation, **not** taken into scope:
   the chunk's boundary is the id-space predicate and the messages it re-words, and security's question
   ("do the current messages render host-path-free origins") is answered yes for every reachable path.

8. **The zero-unlogged-panics gate does NOT see this red** — obs raised whether that gate's stderr scope
   reaches the nextest job. It does not: `ci.yml:170` greps `logs/agent-latest.jsonl` plus
   `logs/producer-stderr.log`, both artifacts of a harness `agent-run` leg, never the nextest job's own
   stderr. A failing Rust assertion's `thread … panicked` line goes to the nextest job's stderr, which
   that step does not read. So the standing red is a nextest-stage failure only, and repairing it neither
   satisfies nor disturbs the panic gate. No plan step follows from this; recorded so the signal is not
   left dangling.

## Scope premise closure
Both `[inferred]` bullets in `scope.md` are **VERIFIED** and their tags dropped (scope.md amended in
this step):
- *No dependency delta* — verified at HEAD by package count (562), un-drifted lock, and green
  audit/deny. Refined rather than falsified: the property holds **only if** P4 declines rstest, so the
  bullet now names that as the constraint it imposes rather than a free expectation.
- *Capability targeted: none* — verified against the full unclaimed pool of 10.

No mechanism claim in the folded freight required re-derivation: the entry's `BLOCKING` annotation is a
sequencing fact, not a causal claim, and the defect mechanism it states (vacuous-then-red) was reproduced
directly above.

## Open questions
- none — the two forks research could have left open (predicate shape, fixture framework) are both
  closed by decisive material leans recorded above and carried into P4.
