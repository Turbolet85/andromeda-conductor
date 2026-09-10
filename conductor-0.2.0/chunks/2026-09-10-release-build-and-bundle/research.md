# Codebase Research — 2026-09-10-release-build-and-bundle

## Scope
- **Depth:** moderate-deep · **Reads:** 14 · **Globs/Greps:** ~22
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read IN FULL as a structural
  extraction (header + `## Session Additions` indexed by per-entry introducer, then offset-bounded reads
  covering every indexed span, `:40`–`:64`; the file is 66 KB over 64 lines, multi-KB per line).
  `.claude/rules/host-win32.md` and `.claude/rules/security.md` load unconditionally and were already in
  context. **This chunk WILL name a live leg, so this read is load-bearing, not optional.**

## Files inspected
- `crates/conductor-core/src/obs.rs` (`:45-60`, `:155-195`, `:395-440`, `:525-600`) — CARRY A's two racing
  tests, the private helpers they use, and the `service.version` derivation.
- `crates/conductor-run/tests/baseline_harvest.rs` (`:196-215`, `:358-372`) — the two `service.name":"conductor"`
  version literals; both are INPUT fixtures, not assertions (decisive for the version-stamp question).
- `crates/conductor-run/tests/canary_obs_witness.rs` (`:58-80`) — asserts self-obs base-field PRESENCE, not value.
- `crates/conductor-report/tests/coverage_gate.rs` (`:1-45`, 87 lines) — the committed-artifact set-equality
  gate precedent `v2-21` should follow.
- `crates/conductor-tauri/tauri.conf.json` (full) — `bundle.active: true`, `targets: "all"`, 3 icons, `version 0.1.0`.
- `crates/conductor-tauri/ui/package.json` (full) — the 10 npm scripts; **no Tauri CLI**.
- `Cargo.toml` + `crates/*/Cargo.toml` — workspace `version = "0.1.0"`, all members `version.workspace = true`.
- `scripts/agent-run.sh` (`:52-56`, `:105`, `:137-138`, `:228-256`) — `ensure_frontend` + the release build form.
- `.andromeda/architecture.md` (`:58`, `:148`, `:204`, `:206`) · `.andromeda/test-plan.md` (`:538`) ·
  `.andromeda/residuals.md` (full) · `conductor-0.2.0/requirements.md` · `conductor-0.2.0/verification-matrix.json`.

## Graph impact (from the code-graph query, rust plane; `probe_hits` all > 0, so the empties are real)
Editor lines = the graph's 0-indexed `line` + 1.

- **`log_panic`** — exactly **3** call sites: `install_panic_hook` @`obs.rs:172` (production) and the two
  racing tests @`:536` / `:583`. No other caller anywhere.
- **`build_subscriber`** — **5** call sites: `init_observability` @`:82`, the test helper `capture` @`:435`,
  the two racing tests @`:530` / `:577`, and `file_sink_writes_redacted_json_to_the_file` @`:609`.
- **`install_panic_hook`** — **1** caller, `init_observability` @`:84`.
- **Meaning for CARRY A:** the race's blast radius is **entirely inside `obs.rs`'s `tests` module and is
  exactly the two named tests**. `file_sink_writes_redacted_json_to_the_file` shares `build_subscriber` but
  never touches the panic hook (`grep take_hook` returns exactly 2 sites, `:535`/`:582`), so it is not a
  third racer. Nothing outside `conductor-core` is impacted.

## Patterns detected
- **Take-and-restore, not first-install-wins** (`obs.rs:535-540`, `:582-587`): both tests do
  `take_hook` → `set_hook(log_panic)` → `with_default(subscriber, …)` → `set_hook(prev)`. They use a
  SCOPED subscriber (`tracing::subscriber::with_default`), never the global `init_observability`.
- **Crate-private helper set** (`obs.rs:157`, `:171`, `:175`): `build_subscriber`, `install_panic_hook` and
  `log_panic` are bare `fn` — no `pub`. `lib.rs:48` re-exports only
  `ObsSink, ServiceIdentity, init_observability, mint_run_id, now_rfc3339`. `fixed_identity` (`:424`) and
  `capture` (`:433`) live inside the `tests` module.
- **Committed-artifact gate shape** (`coverage_gate.rs:19-45`): `repo_root()` from `env!("CARGO_MANIFEST_DIR")`
  (never CWD-relative — "a cargo test binary runs with its CWD at the package root"), an explicit
  non-vacuity guard ("the gate would pass vacuously"), then SET equality — never a literal count.
- **Release build form already in the shells** (`agent-run.sh:137-138`, `:239-240`):
  `ensure_frontend` then `cargo build --release -p conductor-tauri --features tauri/custom-protocol`.

## Conventions to follow
- **Version identity is compile-time derived, at two sites**: `obs.rs:51`
  `service_version: env!("CARGO_PKG_VERSION")` and `conductor-verify/src/client.rs:93`
  `"clientInfo": { "name": "conductor", "version": env!("CARGO_PKG_VERSION") }`. A workspace stamp change
  therefore reaches every self-obs line AND the MCP `initialize` handshake wire.
- **Gate homes follow the dependency edge** (`coverage_gate.rs:11-14`): the gate lives in the crate that can
  reach both sides of what it compares.
- **`.gitattributes`** pins `coverage-matrix.md text eol=lf` because that gate byte-compares a `writeln!`
  render; any new committed-artifact byte-comparison inherits the same requirement.

## Scope premise closure

| scope bullet | verdict |
|---|---|
| `v2-21` claim-reachability | **VERIFIED** — see below; tag dropped |
| Version stamp | **VERIFIED as unmandated**; new facts below; stays an operator fork, not a spec question |
| What "a final SLO verification pass" names | **VERIFIED** — obs-plan defines it; new binding constraint below |
| Bundle targets on this host | **[premise-corrected]** — the live obstacle is not targets, it is that **no Tauri CLI exists in this repo or on this host** |
| CARRY A mechanism (`hypothesis`-grade, unmarked prose) | **RE-DERIVED TRUE** — coordinates, privacy and the 2-test blast radius all confirmed at HEAD |
| CARRY C `hypothesis:` correlation | **WEAKENED** at P1 and unchanged here — `run2` remains the counter-example |

**`v2-21` is already TRUE at HEAD and mechanically checkable.** `requirements.md` declares 32 `v2-NN`
capabilities (`v2-01`…`v2-32`); `verification-matrix.json` holds 32 entries; the id sets are equal, all ids
unique, and zero ids match `^P-\d+$`. So the cap needs an ENUMERATION `/implement` can write — which
satisfies the claim-reachability rule (its decisive artifact is not wrap-authored). Design point the plan
must settle: the matrix path is version-scoped (`conductor-0.2.0/…`), so a gate hard-coding that literal
rots at 0.3.0 — resolve the active version rather than baking it.

**The version stamp is unmandated but observable.** `architecture.md` registers no workspace-package or
`tauri.conf.json` version policy, and its amendment sidecar is explicitly empty on that axis. **No test pins
the live value:** the only two `"service.name":"conductor"` + `"service.version":"0.1.0"` literals are
`baseline_harvest.rs:206` (a synthetic `format!` fixture — "Verbatim shape of Conductor's flat self-obs
line") and `:369` (a deliberately verbatim 2026-08-18 live capture); every other `0.1.0` hit is
`"service.name":"com.andromeda.pulse"`, i.e. the SUT's own lines, which a Conductor bump cannot touch. And
`canary_obs_witness.rs:70` asserts the KEY `"service.version"`, never its value. **A bump therefore breaks
no test** — it changes only what a new run emits, at the two `env!` sites above.

**The SLO pass has a hard, already-ledgered exception.** obs-plan defines the pass as a JSON field
assertion — `latency_ms <= threshold(slo_tier)` against 5000/20000/90000 ms at report-generation time, never
an instrument. The catalog is **36 scenarios** (~9 `<5s`, ~11 `<20s`, ~16 `<90s`). But
`.andromeda/residuals.md` records `constellation-severity-live-wiring`'s `slo_tier = "<20s"` as
**UNATTAINABLE BY CONSTRUCTION** (its own phases sum to 30 s; measured `latency_ms 30212` vs
`deadline_ms 20000`), `open`, target `next`, and explicitly **"neither blocking the release."** So a pass
asserting every scenario's tier CANNOT be green over the whole catalog, and the honest shape records that
residual as a named, ledgered exception rather than descoping the pass or letting it read as a failure.
A graded `latency_ms` also requires a **live Pulse** — without one every row is `Blocked` with null
measurement fields, which asserts nothing.

## Files to modify
- `crates/conductor-core/src/obs.rs` — CARRY A: one shared in-code guard covering both tests' take/set/restore
  window. Caller threading: none escapes the file (graph impact above); no signature changes; no `lib.rs`
  re-export needed, which is the point.
- `Cargo.toml` (workspace `version`) **and** `crates/conductor-tauri/tauri.conf.json` (`version`) — only if the
  operator takes the stamp bump; the two must move together or the binary and the bundle disagree.
- A new committed-artifact gate for `v2-21` — home to be set by the dependency edge, `conductor-report`
  being the precedent's crate (`crates/conductor-report/tests/coverage_gate.rs` is the template).
- `crates/conductor-tauri/ui/package.json` + `package-lock.json` — ONLY if the bundle CLI is taken from npm
  rather than `cargo install`; see the open question.

## New files to create
- `conductor-0.2.0/chunks/2026-09-10-release-build-and-bundle/evidence/` — the recorded SLO pass, the
  supply-chain gate output and the bundle manifest, committed durably because `/runs/` is gitignored
  (the precedent this version already uses).

## Open questions
1. **No Tauri CLI exists to produce the bundle** → blocks: **plan-decision**. `cargo-tauri` is not on PATH,
   `Cargo.toml` declares only `tauri-build = "2"` (a build-dependency, not the CLI), and `@tauri-apps/cli`
   appears **0 times** in both `ui/package.json` and `ui/package-lock.json`. `architecture.md:204` states
   plainly that "a bare `cargo build` does NOT run `beforeBuildCommand` — only the Tauri CLI does", and
   `:206` names the bundler as producing "an optional ~3 MB GUI installer". So `v2-27`'s bundle half cannot
   be produced at HEAD without acquiring the CLI. **Lean (decisive, stated in the plan, not asked):**
   `cargo install tauri-cli` at the arch-registered 2.11.3 line — this is the established host-dev-tool
   pattern (cargo-audit / cargo-deny / cargo-llvm-cov / cargo-mutants are all `cargo install`ed FLOORS,
   never lockfile-pinnable), and the security extract already places "a bundler verb" outside spawn rule
   (b)'s six governed forms. The npm alternative would pull the CLI into the frontend lock and the
   `npm audit` arm for no gain.
2. **The SLO pass's leg set** → blocks: **plan-decision**. A graded pass needs a live Pulse, and the choice
   between a bounded in-lane set (last chunk's shape) and the composed `run --live` suite has real
   stability cost — test-plan records the `--live` auto-resolve leg as **not run-stable even under the
   stretched bootstrap posture**. Put to the operator at P4.
3. **CARRY C ownership** → blocks: **plan-decision**. Carried from scope; the route entry itself defers it
   to the operator at promotion. New input from research: **a11y-plan makes the SR leg supplemental, never
   sole**, so no release WCAG claim rests on `S3-04` either way — the fork is about where the work sits, not
   about whether the release can be claimed.
