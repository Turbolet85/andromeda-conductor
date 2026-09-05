# Scope — Sidecar spawn without a console window

**Marker:** `2026-09-04-sidecar-spawn-without-a-console-window`
**Working entry (verbatim surface):** Sidecar spawn without a console window — the GUI-launched MCP
sidecar stops raising a foreground terminal pane that steals focus and shows its path.

---

## What this chunk builds

The MCP sidecar spawn suppresses the child's console window, so starting a run no longer raises a
terminal pane that (a) seizes the OS foreground away from the app and (b) publishes the sidecar's
absolute exe path as that pane's title — a host-path disclosure channel sitting outside every
redaction edge Conductor owns.

The defect is a **product defect in shipped Rust**, not a harness artifact: it fires on every
`ReadBackClient` connect, which means every preflight, every live leg, and every GUI-initiated run.

### The single site

Verified 2026-09-04 against the source, not inherited from the annotation:

- `crates/conductor-verify/src/spawn.rs:80` — `pub(crate) fn build_command(data_dir: &Path) -> Command`
  whose body at `:81` is `Command::new(PULSE_MCP_PROGRAM)` followed only by `.env(DATA_DIR_ENV, …)`.
  **No creation flags are set.**
- This is the **only** `Command::new` in any shipped crate's `src/` (grep across `crates/*/src/`,
  test-cfg excluded — exactly one hit).
- It has **one** production caller: `crates/conductor-verify/src/client.rs:48`,
  `Self::connect_command(spawn::build_command(&dir))`.
- The type is **`tokio::process::Command`** (`spawn.rs:11`), not `std::process::Command`.
  **VERIFIED (P3):** `tokio::process::Command::creation_flags(u32)` exists at
  `tokio-1.52.3/src/process/mod.rs:675` inside `cfg_windows! { }`, delegating to std's
  `CommandExt::creation_flags`, and its own docs state the flags "will always be ORed with
  `CREATE_UNICODE_ENVIRONMENT`". The fix is a one-line builder call under `#[cfg(windows)]` with
  **no new dependency** — the flag is a `u32` literal.
- **The actual spawn point is `connect_command`** (`client.rs:54-57`), which pipes stdin/stdout,
  sets `kill_on_drop(true)` and calls `.spawn()`. Piped stdio does NOT suppress console allocation
  for a console-subsystem child, which is why the defect survives the existing piping. The flag
  belongs at `build_command` (the mandate's subject) rather than `connect_command` (which also
  serves the stub-binary test path).
- **[premise-corrected: `std::process::Command` exposes only `get_program` / `get_args` /
  `get_envs` / `get_current_dir` / `get_env_clear` — no creation-flags getter (rustc 1.95.0)]**
  "The flag is set on the Command" is **not assertable as observable behaviour** through
  `as_std()`. The flag VALUE can be pinned by a pure const/helper unit test; its APPLICATION is a
  test-plan §1 untestable zone whose real measurement is the live `sr` re-run — never a manual
  "developer confirms no pane appears" step, which test-plan §11 → Universal bans.

### Boundaries

- **In scope:** the creation-flag suppression at `build_command`, its test coverage, and the
  doc-comment corrections the fix invalidates.
- **Out of scope:** the spawn's other hardening properties. The fixed-program-name resolution, the
  `.env(...)`-only data-dir pass, and the injection-metacharacter rejection are all correct today
  and are not touched — this chunk ADDS a creation flag, it does not restructure the spawn.
- **Out of scope:** `sidecar_resolves_on_path` / `resolves_on` (the non-spawning sibling). Its
  directory-walk design stands on its own merit — it answers the presence question with no process
  at all — and survives the fix unchanged. Only its doc comment's *rationale sentence*, which cites
  the flagless spawn as a reason not to spawn, becomes stale and is corrected.
- **Out of scope:** any harness spawn. The four governed harness spawn forms (vendored driver,
  fixed-argv PowerShell activation, guarded `CONDUCTOR_NVDA`, fixture-seed `cargo test`) are
  security-plan §Anti-Patterns (b) and are not this defect.

---

## Surfaces and contracts touched

| Surface | Obligation |
|---|---|
| `conductor-verify` spawn seam | The creation flag; `build_command`'s existing unit test extended, not replaced |
| security-plan §Anti-Patterns (a) | Records this fix as **route-owned, "not yet shipped"** — shipping it makes that clause stale (a wrap amendment, not a phase edit) |
| obs-plan §11 (+ §11's boundary enumeration) | Same: records the gap as a measured host-path channel outside the three application sites, fix "not shipped" |
| `sr` screen-reader suite | The leg's `reactivateWindow()` workaround becomes retire-able (see the mechanism bullet below) |
| Windows-only construction | **VERIFIED (P3), with a correction.** The `#[cfg(windows)]` / `#[cfg(not(windows))]` block precedent is in this same file (`platform_default()`, `spawn.rs:44`), alongside `cfg!(windows)` at `:129`/`:179` and **`#[cfg(windows)]`-gated unit tests** at `:273`/`:288` — the direct shape for testing a Windows-only helper. **[premise-corrected: no gate would catch a cfg mistake — `.github/workflows/ci.yml` runs BOTH jobs on `windows-latest` (`:18`, `:164`), so no non-Windows target is ever compiled.]** Correct cfg-gating is a review obligation here, not a verified one. |

---

## Folded freight

Every coordinate below was **re-verified against the artifact** on 2026-09-04 before it shaped this
scope. Verdicts are stated per item; the annotations arrived as hypotheses, not facts.

### CONTEXT (operator WRAP directive 2026-09-02, item 1) — VERIFIED

> a Conductor spawn DEFECT, `CREATE_NO_WINDOW` class, a route candidate of its own

- `spawn.rs:80` coordinate: **verified exact.**
- security-plan §Anti-Patterns (a) at `:354` and obs-plan `:613` / `:638`: **verified** — all three
  cite `spawn.rs:80` and all three say the fix is route-owned and **not yet shipped**.
- **VERIFIED (P3), and the count was low.** The source file *itself* already documents the defect:
  `sidecar_resolves_on_path`'s doc comment (`spawn.rs:92`) reads "[`build_command`] above sets no
  creation flags, so starting the sidecar to test its presence would raise a console pane titled
  with its absolute path" — a **fourth** site the annotation did not name.
  **[premise-corrected: a FIFTH stale in-code claim sits at `spawn.rs:79`]** — `build_command`'s own
  doc comment ends "The caller hands the result to `TokioChildProcess`", and that type has been
  absent from the code since the 2026-06-27 rmcp removal (grep across `crates/**/*.rs`: this doc
  comment is its only occurrence). It sits on the very function this chunk edits. Arch registers the
  same retired type at TWO of its own sections (§Occupied Resources — Service/process names,
  §Cross-cutting Patterns — Trust boundary), which are wrap amendments, not phase edits.

**Mechanism claim, preserved verbatim from the freight** (measured-marked by the entry;
**RE-DERIVED AT HEAD by P3** — see the closure note below the quote):

> measured at `2026-09-02-screen-reader-manual-spec` (SR pass row S1-01, `security_finding`) —
> `conductor-verify/src/spawn.rs:80` builds `Command::new(PULSE_MCP_PROGRAM)` with no creation
> flags, so a Windows Terminal pane titled with the sidecar's absolute exe path takes the OS
> foreground ~200 ms after `Start`; any keyboard or screen-reader user loses focus to it, and the
> title is a host-path disclosure outside every redaction edge

Corroborated at the evidence: `nvda-pass.json` finding S1-01 reads "a host path was HEARD in this
row window — either a foreign window title NVDA announced (a console, a terminal pane) or text
Conductor rendered; only the latter is a security finding against the sanitize_…". Note the
evidence itself is **disjunctive** about which of the two it was; the masters resolve it to the
pane.

**Mechanism re-derivation (P3, at HEAD) — the claim holds, on two sources independent of the
evidence file.** The concern above was that `nvda-pass.json`'s S1-01 text is disjunctive ("a foreign
window title … or text Conductor rendered"), so the pane attribution could not rest on it alone.
Two in-repo sources resolve the disjunction to the pane:

1. `.claude/rules/verification-harness.md` entry 59, item (4) — "Anything that takes the foreground
   mid-leg detaches it silently: **the sidecar's console pane did, ~200 ms after `Start`**, and
   every later focus row went quiet until the leg re-activated the window."
2. `screen-reader.e2e.ts:420-422` — the call-site comment: "The run start spawns the sidecar, whose
   console window takes the OS foreground (a finding the evidence records as the foreign-window
   speech inside S1-01)."

The causal chain also re-derives structurally at HEAD: `build_command` sets no creation flags
(`spawn.rs:81`), `connect_command` pipes stdio and spawns (`client.rs:54-57`), and piped stdio does
not suppress console allocation for a console-subsystem child. **Timing (~200 ms) is inherited from
the 2026-09-02 measurement and is not re-measured here** — the live re-run is what re-measures it.

**Freight correction — the retire target is HALF of what the entry implies.** The entry says
"retire the leg's re-activation step." Measured: `screen-reader.e2e.ts` calls the activation script
at **two** sites, and only one is this workaround.

- `bringToForeground()` — defined `:68`, called at **`:335`, `:546`, `:593` (three sites)** —
  **STAYS.** Its rationale is independent of the sidecar: "a window launched by a background process
  never takes the foreground on its own (measured 2026-09-02: NVDA read the Pulse window while the
  driven app's DOM focus moved unannounced)". No spawn fix changes that.
- `reactivateWindow()` — defined `:120`, called at **`:424` (one site)** — **the retire target.** Its
  own doc comment names this exact defect: "for the moments the app itself hands the OS foreground to
  another window (measured 2026-09-02: starting a run raised a console window for the spawned
  sidecar, and NVDA followed it, so every later focus row went silent)."

**VERIFIED (P3).** `reactivateWindow` has exactly **one** caller — `screen-reader.e2e.ts:424`,
immediately after row S1-01, carrying a comment that names this defect ("The run start spawns the
sidecar, whose console window takes the OS foreground") and writing an `@reactivate` timeline stamp.
The harness rule records the same mechanism independently at HEAD
(`.claude/rules/verification-harness.md` entry 59, item (4): "the sidecar's console pane did, ~200 ms
after `Start`, and every later focus row went quiet until the leg re-activated the window").

Retiring it is therefore **conditional on the live re-run proving the foreground no longer moves**,
not a mechanical deletion. If the re-run still shows a foreground handoff, the workaround stays and
the finding is recorded — this chunk does not delete a guard on a prediction. Note the same rule
entry generalises the workaround ("re-activate after any action that spawns a process"), so a
successful retire makes that clause a wrap amendment too.

### CARRY 1 — `commands.rs` doc comments (from `2026-09-03-conductor-tauri-survivors-dispositioned`) — VERIFIED

Both doc comments carry the unqualified "never an error" claim:

- `crates/conductor-tauri/src/commands.rs:161-165` (`run_report`): "An absent/empty runs dir yields
  an empty list (the webview renders "No run yet"), **never an error**."
- `crates/conductor-tauri/src/commands.rs:217-221` (`run_envelope`): "A run that recorded no
  envelope row, or an absent runs dir, yields `None` — the banner is simply absent, **never an
  error**."

The narrowing premise is verified: `conductor-core/src/run_journal.rs:12-16` — `latest_run_id`
returns `Ok(None)` on a `read_dir` failure (`let Ok(entries) = … else { return Ok(None) }`), so the
claim holds for `run_id = None`. For a **supplied** id the path reaches `resolve_under` against a
base that need not exist and can error. Both commands behave correctly; **only the comments read
wider than the code.** Correct the two comments while editing this file.

**VERIFIED (P3): the condition does not fire on its own.** The fix lives in `conductor-verify`, so
this chunk touches no `conductor-tauri/src` file for its own sake. The comment correction is cheap
and self-contained, so it stays in scope as an **independent deliverable**, not as a side-effect of
an edit that will not happen.

### CARRY 2 — the rustfmt EDITION mismatch (from `2026-09-03-live-pulse-preconditions-probed`) — VERIFIED

Every structural claim re-measured:

- No `rustfmt.toml` or `.rustfmt.toml` anywhere in the tree. **Confirmed absent.**
- `.claude/settings.json:36` PostToolUse hook runs `rustfmt "$f" 2>/dev/null || true` — **bare, no
  `--edition`.** Confirmed.
- Workspace `Cargo.toml:17` is `edition = "2024"`. Confirmed.
- `cargo fmt` / `fmt --check` appears in **neither** `.github/workflows/*.yml` nor
  `scripts/agent-run.sh` nor `scripts/agent-run.ps1`. Confirmed — it is not a gate.

Instance remedy: a one-file `rustfmt.toml` carrying `edition = "2024"`, which both bare `rustfmt`
and `cargo fmt` honour and a setup re-render cannot overwrite. **The seeded hook ROW is a pipeline
defect the overseer encodes — do NOT edit it.**

The freight's own bound is kept verbatim: the fix "bounds FUTURE churn rather than promising a clean
tree" (~745 pre-existing spots workspace-wide).

**VERIFIED (P3).** This is the highest-leverage item for *this* chunk specifically, not only for the
future: the chunk edits Rust in `spawn.rs` — one of the three files the prior chunk measured being
re-wrapped (182+/6−) — so every hook pass during implement re-wraps imports 2015-style unless
`rustfmt.toml` lands first. P4 orders it **first**.

### CARRY 3 — SR finding 1, initial focus (from `2026-09-04-sr-findings-remediation`) — VERIFIED, with a live tension

`tabs_to_start` measured **live 3 · empty 6 · error 5** — verified exactly against
`chunks/2026-09-04-sr-findings-remediation/evidence/nvda-pass.json` (`subjects.live.foreground
.tabs_to_start = 3`, `empty = 6`, `error = 5`; `initial_focus` is `"BODY"` on all three). The
defect is **CONFIRMED, not fixed**.

The freight states, verbatim: "Mechanism unidentified and nothing guessed: cmdk 1.1.1 has no
`autofocus` of any spelling, both its `.focus()` calls are guarded on focus already being inside
cmdk, and no `src/` file focuses at mount." `cmdk ^1.1.1` confirmed in `ui/package.json:23`.

**VERIFIED (P3) — both claims stand, and the conflict is unresolved.** This is now an Open question
in `research.md` blocking implementation-scope. **A competing mechanism claim already exists in the
leg's own source** and the freight
does not mention it. `screen-reader.e2e.ts:78-84` comments: "Chromium keeps a
sequential-focus-navigation starting point apart from activeElement: a control that held focus and
lost it (**the picker input at mount, measured 2026-09-02**) leaves the next Tab continuing from it,
and neither `blur()` nor a programmatic selection collapse moves that point." The two are
reconcilable only if something outside `src/` and outside cmdk's guarded calls focuses the input.
P3 must settle this before any fix — the leg's `tabsToStart` loop is currently a *workaround* that
would mask the fix's effect if left in place.

### CARRY 4 — SR finding 4, the load-time alert (from `2026-09-04-sr-findings-remediation`) — VERIFIED

Fix shipped, proof blocked. Verified: `App.tsx` mounts **four** `role="alert"` regions (`:245`,
`:272`, `:285`, `:309`), and `:241` carries the intent comment. The discriminator problem is
verified in the spec row itself — `nvda-pass-spec.md:181` (R0-01) reads "focus warm-up (Tab), then
reload the document — **the leg still reloads, so this row does NOT yet discriminate** a first-load
announcement from a post-reload one", and `screen-reader.e2e.ts:599` still performs that reload.

Remedy per the freight: drop the reload from that action to discriminate it.

### PREREQ (standing, external decay) — the 51st re-pin

Re-check `cargo audit`. Standing deferral since 2026-08-09, ratified at the 2026-08-10 wrap.
Basis: the RustSec advisory DB itself will not parse — a **DATABASE** fault, not a tool fault, so no
floor raise exists to make. Overlap: `cargo deny check advisories bans licenses sources` runs green
every chunk.

**SIGNATURE:** `cargo audit` exit **1** with first diagnostic
`error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244`; overlap
`cargo deny` exit **0**. Reproduced byte-identically at the
`2026-09-04-preconditions-probe-reads-path-handles-by-presence` wrap (the 50th, package count 564
unchanged); **this is the 51st.**

Read the exit **DIRECTLY, never through a pipe.** ANY deviation restores the FULL form and is
reported.

Per security-plan §Session Additions (2026-09-03): **both** probes — the audit (expected RED, its
exit captured before any pipe, disposition naming the deferral's origin and the re-pin ordinal) and
the deny overlap — go in the plan's `## Test Commands` unconditionally, or the wrap cannot re-pin
them.

---

## What "done" looks like

1. The sidecar spawn sets a console-suppressing creation flag; no terminal pane appears on connect.
2. The **five** in-repo sites asserting the flagless spawn (security-plan §Anti-Patterns (a),
   obs-plan ×2, and `spawn.rs`'s own doc comments at `:92` and `:79`) no longer state a shipped
   falsehood — the two in-code ones corrected here, the master ones surfaced to wrap. `spawn.rs:79`
   additionally retires the `TokioChildProcess` name, which arch restates at two of its own sections.
3. Live re-run of the `sr` subject measures whether the foreground still moves; `reactivateWindow()`
   retires only if it does not.
4. The four CARRYs discharged or explicitly recorded with their reason.
5. The audit PREREQ re-pinned as the 51st, signature byte-matched or the full form restored.

`[inferred]` No verification-matrix capability is an obvious claim for this chunk — it is a defect
remediation, and the six unclaimed caps (`v2-04`, `v2-21`, `v2-24`, `v2-26`, `v2-27`, `v2-32`) are
not on its face about spawn hygiene. P5 decides; the default is to link nothing rather than
manufacture a claim.
