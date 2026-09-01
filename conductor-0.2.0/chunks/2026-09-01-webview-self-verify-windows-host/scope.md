# Scope — 2026-09-01-webview-self-verify-windows-host

**Working-route entry (verbatim title):** _Webview self-verify on the Windows host — agent-driven affordance
proof through the real Tauri window; operator legs reserved for judgment items_

**Epoch:** 5 — Verification surfaces · **Version:** conductor-0.2.0

---

## What this chunk is

The chunk that answers one measured question and then acts on the answer: **can an agent drive the real
Tauri window on THIS host?** Every downstream verification surface in Epochs 5 and 6 — the a11y sweep, the
live verdict lamps, the screen-reader spec, the CI gate — is currently written against a Linux+xvfb runner
that does not exist and is not planned. The 2026-08-22 boundary adaptation ruled this host Windows-only and
agent-driven GUI verification the DEFAULT, which leaves those surfaces aimed at a platform nobody will run.

So this chunk owns the **probe** (does a Windows webview driver actually drive the shipped bundle?) and the
consequence of whichever way it lands. The specs, the wdio config and the driver dependencies already ship —
what has never been established is a working driver on this host.

## What it builds

- **The Windows driver probe, measured.** Whether `tauri-driver` + a WebView2 driver (`msedgedriver`) can
  attach to the built `target/release/conductor-tauri.exe` and return a live WebDriver session. This is the
  chunk's gating measurement, and it is a real run, never a documentation reading.
- **If the probe succeeds:** the agent-driven self-verify path — the shipped display-gated specs made
  runnable on Windows under agent drive, with the platform gate re-expressed as a *driver-availability* gate
  rather than a Linux assumption. Per the entry: _"Once a driver exists the pipeline's own rules take over:
  'headful self-verify available' makes the ui-smoke a Test Commands gate."_
- **Affordance proof through real controls** — the interaction legs an agent can honestly perform (a real
  click / keypress on a named affordance through the driver), which is what separates affordance-level
  verification from a process-proxy. The operator's eyes stay reserved for the ManualCheck judgment class.
- **If the probe fails:** the measurement recorded with its mechanism, the affected surfaces' premises
  corrected, and the work routed forward — never a quietly relaxed claim and never a silent Linux re-aim.
- **The CARRY amendment** (below) and the **PREREQ probe** (below), both owed at this chunk's wrap.

## Boundaries — what this chunk does NOT do

- **Does not run the a11y sweep as its deliverable.** _Desktop a11y sweep_ (v2-22) is its own Epoch-5 entry
  two lines down; this chunk makes the running POSSIBLE and does not consume that entry's scope.
  **VERIFIED** — `working-route.md:85` is a separate markerless entry.
- **Does not build the a11y CI gate** (v2-24) — an Epoch-6 entry annotated `BLOCKED-ON: a Linux runner`.
  Whether this chunk's measurement changes that entry's blocked status is an observation to record, not a
  re-scope to perform here. **VERIFIED** — `working-route.md:104` carries that exact annotation.
- **Does not author new specs.** v2-22's requirement says the real-webview specs are *shipped* and
  display-gated ("without re-authoring"). **VERIFIED** — `test/a11y/accessibility.e2e.ts` is 66 lines with
  five `it()` blocks already driving real affordances (`browser.keys('Escape')`, `browser.keys('Space')`,
  `AxeBuilder(...).analyze()`, focus-trap and token-contrast assertions) across the four accessible paths.
- **Does not build the verdict-lamp data path** (v2-30, the next Epoch-5 entry).
- **Does not change the operator's role.** Judgment items (the ManualCheck class) and the occasional holistic
  pass stay the operator's; this chunk removes routine affordance proof from that list, nothing more.
- **Claims no verification-matrix capability by default.** The 11 unclaimed ids are each owned by a later
  entry or a different axis; P5's matrix step re-checks whether this chunk makes any of them FULLY
  verifiable, and a claim is a deliberate P5 act, not an assumption here.
- **Does not install host software as a hidden side effect.** If the probe needs a driver binary the host
  lacks, that acquisition is an explicit, recorded step — an operator-visible decision.

## Surfaces and contracts touched

| Surface | Expected involvement |
|---|---|
| `crates/conductor-tauri/ui/wdio.conf.ts` | the display gate (`:3-6`) + the bare-name `tauri-driver` spawn (`:41-43`) — the two places a Windows path is decided. **VERIFIED** |
| `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts` | the shipped 5-spec suite the driver would execute. **VERIFIED** — read, unchanged by this chunk |
| `crates/conductor-tauri/ui/package.json` | the `a11y` script (`wdio run wdio.conf.ts`); any Windows-path script or driver dependency |
| `scripts/agent-run.{sh,ps1}` | **corrected at P3:** `--e2e` ALREADY exists in both (`agent-run.sh:86` · `agent-run.ps1:96`) but maps to `cargo nextest run -p conductor-cli` and — unlike `--unit`/`--integration` — does NOT call `ensure_frontend`. Wiring the webview leg therefore changes what `--e2e` DOES, rather than adding a first stage flag |
| `.andromeda/test-plan.md` §2 / §9 · `a11y-plan.md` §3 | the Linux-only framing vs a measured Windows path — a **wrap amendment**, never a phase edit |
| `.andromeda/architecture.md` §Read-Back Dependency Posture | the CARRY amendment below — wrap-owned |
| `verification-matrix.json` | at most a `notes` narrative unless P5 finds a cap genuinely satisfied |

## Measured premises (folded freight re-verified at promotion)

The working entry's named coordinates were re-verified against the artifacts themselves before shaping this
scope (`promotion.md` — annotations fold as HYPOTHESES). All confirmed:

| Claim | Verdict |
|---|---|
| `test-plan.md` §2 (:54) names tauri-driver as the desktop-webview driver | **confirmed** — "Driver: tauri-driver (headless)" |
| `test-plan.md` (:465) justification is macOS-specific, Windows never named | **confirmed** — "separate `ubuntu-latest` + `xvfb` job only — no WKWebView…" |
| `package.json` carries `@crabnebula/tauri-driver ^2.0.9` · `@axe-core/webdriverio` · wdio 9 | **confirmed** (`:24`, `:25`, `:30-42`) |
| `tauri-driver` and `msedgedriver` absent from PATH | **confirmed** — both unresolvable as commands |
| `architecture.md:67` carries the over-scoped claim | **confirmed** — exact string at that line |
| `lifecycle_harvest.rs:23` carries it in a module doc | **confirmed** |
| `master-route.md:107` + chunk `report.md:79` carry it (known-stale) | **confirmed** |
| Pulse predicate at `pulse-app/src/inference_runtime.rs:811`, HEAD `83d4060` | **confirmed** — `.find(\|inc\| inc.kind == kind && inc.scope == scope && inc.scope_id == scope_id)` over `registry.list_active(&digest.workspace)`; Pulse HEAD is `83d4060` |
| `crates/conductor-run/Cargo.toml` declares an EMPTY `[features] live-pulse = []` | **confirmed** — no dependency, so zero package nodes |

**The entry's central premise is FALSIFIED — measured at P3.**
`[premise-corrected: node_modules/@crabnebula/tauri-driver-win32-x64-msvc/tauri-driver.win32-x64-msvc.node —
a 4.03 MB Windows x64 MSVC prebuilt, installed 2026-06-27; `node cli.js --help` runs at exit 0]`
The entry states _"NO driver binary exists on this host — `tauri-driver` and `msedgedriver` are both absent
from PATH, and the installed npm package ships Rust source rather than a binary."_ Both halves of the
EVIDENCE are true (PATH resolves neither; the top-level package does ship `src/lib.rs`), but the CONCLUSION
is false: `@crabnebula/tauri-driver` declares platform prebuilts as `optionalDependencies`, and the win32-x64
one **is installed** and **executes**, advertising `--native-driver PATH` for the underlying WebDriver. A
working Windows tauri-driver has been in this tree since the harness was set up.

**What is genuinely missing is narrower and different:** the NATIVE WebDriver. `msedgedriver` is on no PATH,
Edge does not bundle one (two Edge versions installed: 151.0.4129.78 · 152.0.4191.53), and the transitively
installed `edgedriver@^6.1.2` npm package has downloaded no binary yet — it fetches on first use. Plus
`target/release/conductor-tauri.exe` does not currently exist (`ui/dist` IS built). So the probe's remaining
distance is **two acquisition/build steps, not a platform impossibility.**

### Mechanism claims carried in verbatim (re-derived at P3)

- **VERIFIED** _"The 'Linux+xvfb only' framing is a CI-runner assumption, not a Windows verdict: test-plan's
  own justification (:465) is macOS-specific ('no WKWebView WebDriver on macOS') and Windows is never named
  in the E2E job."_ — citation confirmed at source, and the measurement above independently supports the
  inference: the Windows driver layer works.
- **VERIFIED** _"Tauri documents Windows support via msedgedriver — NOT yet measured here; this entry's
  research owns that probe."_ — measured: the driver's own `--native-driver` option is that support, and
  `a11y-plan.md` §3 (Configuration) already names `msedgedriver` as tauri-driver's Windows backend.
- **VERIFIED** `wdio.conf.ts`'s own header asserts it "does NOT run on the Windows dev host (no display)" —
  present verbatim at `:3-6`; its `onPrepare` (`:41-43`) spawns the bare name `tauri-driver`, which PATH
  cannot resolve. Both are now measured-stale rather than suspected-stale.

## Folded annotations (from the working entry)

### CONTEXT — the 2026-08-22 boundary adaptation (two operator rulings)

1. **This host is Windows-only and no Linux runner exists or is planned**, so platform-foreign legs moved to
   the Epoch-6 tail.
2. **Agent-driven GUI verification is the DEFAULT**, the operator's eyes reserved for judgment items (the
   ManualCheck class) and an occasional holistic pass, **never routine affordance proof**.

These are this entry's premise, not findings to re-derive.

### CARRY — the over-scoped claim, amendment owed at THIS chunk's wrap

_(from `2026-08-31-p-075-assert-round`, operator correction post-commit)_

`architecture.md` §Read-Back Dependency Posture states **"At most ONE incident is active per workspace"** —
WRONG. Pulse's dedupe key is the `(kind, scope, scope_id)` **TUPLE**: the workspace scopes the CANDIDATE SET,
the tuple is the key (measured at `inference_runtime.rs:811`, HEAD `83d4060` — re-verified above). The
originating leg saw workspace-level behaviour only because its storms shared one tuple; **the leg evidence
stays valid, the generalisation does not.**

Three consequences to fold into the wrap amendment:
- **(a)** re-scope the claim to the tuple and cite the predicate;
- **(b)** "spared-control unattainable" narrows to **WITHIN-tuple** — a CROSS-SCOPE two-incident control is
  possible and untested, so record it as a **weighable route option, never a retirement** (the liveness
  attribution stands as delivered);
- **(c)** the freshness-caveat retirement narrows the same way — a concurrent unrelated incident cannot arise
  within the tuple but **can across scopes**.

Two sites are amendable (`architecture.md:67` · the `lifecycle_harvest.rs:23` module doc) and two are
known-stale with no sanctioned channel (`master-route.md:107`, `@import`ed into every session, and the
originating chunk's `report.md:79`). **Site list swept 2026-09-01 — RE-SWEEP at the wrap rather than trusting
it.**

### PREREQ — `cargo audit` re-check, **41st consecutive**, FULL form

Standing deferral since `2026-08-08-sut-capability-manifest`, ratified at the
`2026-08-10-workspace-key-divergence-probe` wrap. Re-pinned in FULL form because the **BASIS CHANGED**: the
40th was discharged at the `2026-08-31-p-075-assert-round` wrap and the signature reproduced byte-identically
— `cargo audit` true exit **1**, first diagnostic `duplicate advisory ID: RUSTSEC-2026-0244`; `cargo deny
check advisories bans licenses sources` true exit **0** — but that chunk TOUCHED a manifest
(`crates/conductor-run/Cargo.toml`, +6 lines), so the prior basis "every manifest byte-untouched" no longer
holds and the compact form is not available.

The CONCLUSION is unchanged and re-verified: the delta declares an EMPTY cargo feature
(`[features] live-pulse = []`) with no dependency, so it admits **ZERO package nodes**, `Cargo.lock` is
byte-unchanged, and `cargo deny`'s coverage set is identical — deny was run and **VERIFIED** green over that
state, not assumed (playbook 2026-08-20: the pin's form keys on whether the delta **ADMITS A PACKAGE**, not
on whether a manifest moved). The advisory-DATABASE fault therefore remains the sole cause and the
audit↔deny overlap stays the coverage.

Discharge terms: PROBE-AUTO-SATISFY signature unchanged; **capture the audit exit code BEFORE any pipe**
(`$?` after `| head` reports the pipeline's last stage); ANY further deviation restores the full form again;
remedy stays the bounded wait — **no floor raise, no `deny.toml` ignore, no CI edit**; close the moment it
parses. Full rationale:
`conductor-0.2.0/chunks/2026-08-14-canary-fingerprint-feed-capture/report.md`.
