# Adaptation Record — operator-requested route adaptation (0-pending wrap)

**When:** 2026-08-22T21:17:21Z · **Branch:** `build/conductor-0.2.0` · **HEAD at entry:** `b8f3332`
**Path:** wrap Setup step 6 — 0 pending, tree dirty only with contract-documented bookkeeping
(`friction-log.ndjson` · `code-metrics.ndjson` · `session-handoff.md`) + two untracked
`.andromeda/runs/` dirs → counts as git-CLEAN → no-op path **with** an operator route-adaptation
request, so P5 route-resolve ran (P1/P2/P4/P7 do not run on this path).

**Scope touched:** `conductor-0.2.0/working-route.md` markerless tail only. 33 frozen `[{marker}]`
lines verified **byte-identical** before and after (set equality, both directions).

---

## Standing rulings recorded (the context correction this adaptation encodes)

Two operator rulings landed at the 2026-08-22 boundary that the route did not know:

1. **This host is Windows-only.** No Linux runner exists or is planned. Platform-foreign legs move
   to the tail rather than sitting in the active epoch.
2. **Agent-driven GUI verification is the DEFAULT.** The operator's eyes are for judgment items (the
   ManualCheck class) and an occasional holistic pass — never routine affordance proof.

---

## Items and dispositions

### 1 — NEW ENTRY (trajectory → dialogue → approved as recommended)

Added as the **first entry of Epoch 5** (line 80):

> Webview self-verify on the Windows host — agent-driven affordance proof through the real Tauri
> window; operator legs reserved for judgment items

WHAT-level, 20 words, no implementation verbs; the driver name lives in its `CONTEXT` annotation per
the route register. Placed first because every other Epoch-5 entry (lamps, a11y sweep, parity)
becomes an agent leg through it.

**Basis verified at this wrap** (the relay's citations, each checked against the artifact):

| Claim | Verdict | Evidence |
|---|---|---|
| test-plan §2 names tauri-driver as the desktop-webview driver | ✓ holds | `.andromeda/test-plan.md:54` — "**Driver:** tauri-driver (headless)" |
| the ui package carries the wdio + axe + tauri-driver stack | ✓ holds | `crates/conductor-tauri/ui/package.json:24,25,30-35,42` — `@axe-core/webdriverio ^4.12.1` · `@crabnebula/tauri-driver ^2.0.9` · wdio 9.29.1 |
| no driver binary exists on this host | ✓ holds | `tauri-driver` and `msedgedriver` both **absent from PATH**; the installed npm package ships Rust source (`Cargo.toml`, `build.rs`, `cli.js`), not a binary |
| the project's framing is "Linux+xvfb only" | ✓ holds | test-plan `:304` "headless, Linux + `xvfb`" · `:456` "(Linux only)" · `:465` "separate `ubuntu-latest` + `xvfb` job only" |
| Windows support via msedgedriver | **not measured** | owned by this entry's research, as the relay states |

**One refinement found, and it strengthens the premise.** test-plan's own justification for the
Linux-only E2E job (`:465`) is **macOS-specific** — *"no WKWebView WebDriver on macOS"*. Windows is
never named in that job, and no Windows-specific exclusion appears anywhere in the plan. The
"Linux+xvfb only" framing is therefore a CI-runner assumption that was never a Windows verdict.

### 2 — PIN MIGRATION (factual → auto)

The **39th** `cargo audit` PREREQ migrated from the lamps entry onto the new first entry (the
2026-08-20 precedent: the pin follows whichever entry is first). Moved by **index slice, never
re-typed**, and post-checked **byte-identical** — 1347 chars in, 1347 chars out, with the origin
(`2026-08-08-sut-capability-manifest`), the ratification marker, the PROBE-AUTO-SATISFY signature and
the `**39th consecutive**` ordinal all intact. The ordinal counts probes, not entries, so it stays 39.

**Basis re-verified (route-resolve requires it each pin):** unchanged and still true. This wrap has
**zero source delta** — nothing has changed since `b8f3332` except bookkeeping — so the recorded
basis ("admitted ZERO packages; `Cargo.toml`, `Cargo.lock`, `package-lock.json` byte-untouched")
holds without re-authoring.

**Forward note for the absorbing chunk:** item 1's entry will install a webdriver. `cargo install
tauri-driver` writes to `~/.cargo/bin` and an msedgedriver is a standalone binary — neither touches
the project lockfiles — so no package admission is *expected*. That is a prediction, not a
measurement: the absorbing wrap must re-verify the zero-delta basis rather than inherit this line.

**Formatting normalization.** The pin was separated from the preceding CARRY by a **single space**
on the old entry; it is re-attached with the conventional **three-space** separator. Beyond
consistency this matters mechanically: flip-compaction's strip rule requires `\s{2,}` or ` · ` before
an annotation introducer, so at a single space the pin would have survived compaction as stale-truth
freight once the entry froze.

### 3 — TAIL MOVE (trajectory → dialogue → approved, placement refined)

`Webview E2E harness leg` and `A11y CI gate` moved from Epoch 5 to the end of Epoch 6, each annotated:

> `BLOCKED-ON: a Linux runner (none exists; Windows-only host by operator ruling 2026-08-22) —
> evidence: .andromeda/runs/2026-08-22T21-17-21Z-wrap/adaptation-record.md. The trajectory question
> is dispositioned; /andromeda-phase Setup refuses promotion until the block clears, which is
> intended.`

**Placement refined in dialogue:** the relay said "END of Epoch 6", whose literal reading puts two
permanently-blocked entries *after* `Release build and bundle`. Operator chose to place them
**before** the release entry so the version's terminal entry stays the release. Mechanism is
unaffected either way; at version end an open `BLOCKED-ON` rides `.andromeda/residuals.md` like an
open gate deferral.

**The BLOCKED-ON trajectory question HALTs once** (route-resolve, mirror of the age trigger's
ratify-once). This dialogue **is** that halt; the disposition is recorded here. Route-resolve stays
silent on it while the block stands, and clears the annotation when a Linux runner ships.

### 4 — REWRITE (trajectory → dialogue → approved, **after a correction**)

`Desktop a11y sweep` rewritten to:

> Desktop a11y sweep — operator judgment pass over the accessible paths, with axe and routine
> affordance proof agent-driven

with a `CONTEXT` annotation splitting the six paths: axe, contrast, not-color-alone, keyboard-trap
escape, focus restoration and reduced-motion become agent legs through the item-1 entry; the operator
keeps the judgment class.

**Correction — the relay mis-attributed the CARRY it names.** There is no "Epoch-5 OPC CARRY about
the unresolved display gate". The display-gate text is a **clause inside the
`2026-08-09-sut-load-envelope` CARRY on the LAMPS entry**, not an OPC annotation, and not on the
a11y-sweep entry (which carried no annotation at all). The OPC wrap recorded the display gate only as
a *handoff Note*, never as a route CARRY. Left uncorrected, a rewrite aimed at the sweep entry would
have left the real clause standing untouched.

**Disposition (operator, option A):** rewrite the sweep **and** repoint the lamps clause, so neither
entry re-raises the gate per chunk. The lamps CARRY now reads
"…display-gated to Linux+xvfb; **resolving that gate is owned by the Webview self-verify entry above,
not by this chunk**)". Deliberately *not* struck: the clause is still literally true today — no
Windows driver has been measured — so the edit names an owner rather than asserting a resolution that
has not happened.

### 5 — CARRY (factual → auto; owner decided in dialogue)

Appended to the Epoch-6 `Coverage completeness gate` entry (its fifth CARRY), the operator's lean and
the dialogue's choice: an eighth P-047 category is a coverage fact about the **current SUT set**,
which is exactly what that entry gates.

**Cross-project citation verified at the SUT repo** (`D:\dev\projects\andromeda-pulse`), per the
standing rule that a dictated cross-project citation is verified when the SUT is on disk:

- Pulse HEAD **is** `70344d5` — *"feat(2026-08-22-pii-scrubber-recall): a bare provider key no longer
  reaches stored fields"*. ✓
- It adds `provider_key` to `crates/security/src/scrubber.rs`, anchored on published issuer prefixes
  (`[sr]k_(live|test)_…` · `sk-…` · `gh[pousr]_…` · `AKIA…` · `xox[baprs]-…` · `AIza…`) and placed
  after the keyed arms so `api_key=…` keeps its own category. ✓
- That module's own doc changed from "the 7 P-047 categories" to "the **8**". ✓
- Conductor's `crates/conductor-emit/src/pii.rs:32` still documents *"The seven P-047 PII categories
  Pulse's scrubber must detect and redact"* over a seven-variant `PiiCategory`. ✓ — with
  `all() -> [PiiCategory; 7]` fixing the **arity** and `PiiCorpus` storing in discriminant order, so
  an eighth arm moves the enum, the array type and the corpus, not just a doc line.

**Pedigree worth keeping:** this closes a loop Conductor opened. `2026-08-19-pii-scrub-live-proof`
recorded the bare `sk_live_` recall gap as **SUT intake**; Pulse has now discharged it.

**`v2-14` stays VERIFIED.** It measured a state that genuinely held against the then-current SUT, and
a SUT change invalidates only FUTURE legs. This is owned debt, never a re-opened chunk and never an
un-verified ledger entry.

---

## Anchor-only compliance

Nothing else in the tail moved. Entry count 45 → 46 (one added; two relocated within the tail).
Frozen lines 33 → 33, byte-identical.

## Observation recorded, not fixed (out of scope)

`working-route.md` line 41 carries a **dangling `   ↓` separator** at the end of Epoch 2 — it follows
the last frozen Epoch-2 entry and precedes the blank + `### Epoch 3` header, with no entry after it.
Verified **pre-existing at `b8f3332`** (identical in `git show HEAD:` and in the edited file), so this
adaptation neither caused it nor touched it. It sits in the frozen region, outside route-resolve's
markerless-tail scope, and is inert: cursor derivation keys on markerless **entry** lines and the
grammar excludes `   ↓`. Recorded so a future reader does not mistake it for damage from this edit.
