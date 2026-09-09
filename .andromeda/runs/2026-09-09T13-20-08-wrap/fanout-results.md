# Fan-out results — 2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate

7 Explore doc-agents, one batch. Entity-decode probe: `entities=0` on all seven returns (the test-plan
return carried `&lt;crate&gt;` in its no-hit notes only, outside the YAML payload).

## Per-doc verdicts

| doc | verdict | detectors evaluated |
|---|---|---|
| arch | **6 proposals** | D-arch-resources (no hit) · D-arch-decisions (1 primary + 4 `dependent-of`) · D-platform-claim (1) |
| security-plan | `proposals: []` | D-security-input · D-security-subprocess · D-security-deps · D-platform-claim — all no hit |
| design-system | `proposals: []` | D-design-tokens · D-design-derived-count · D-platform-claim — all no hit |
| layout-templates | `proposals: []` | D-layout-surface · D-layout-derived-count · D-platform-claim — all no hit |
| test-plan | **4 proposals** | D-tests-coverage/-framework/-obs-harness/-derived-count (no hit) · D-platform-claim (2 primaries + 2 `dependent-of`) |
| obs-plan | `proposals: []` | D-obs-instrumentation · D-obs-stack · D-obs-redaction · D-platform-claim — bare return, no evaluation notes |
| a11y-plan | `proposals: []` | D-a11y-surface · D-a11y-obs-schema · D-platform-claim — all no hit |

**Raw twins:** `.raw-fanout-arch.md` and `.raw-fanout-test-plan.md` (the two returns carrying proposals).
The five empty-and-clean returns are recorded here, which is their sanctioned audit artifact.

## Orchestrator raises (Validate checks 4–6) — what the detectors could not see

The per-doc detectors are scoped by invariant, and the claim *"what CI runs"* belongs to no single
detector's invariant. The cascade's cross-master sweep found it in **four** masters where only arch was
proposed. Sweep pattern set, derived after the last proposal was read (a sweep run before that is partial
by construction), with a known-positive control fired through each:

- `grep -nE 'cargo build.*(nextest|clippy)|(nextest|cargo test).*clippy'` over all seven masters —
  control: `architecture.md:37` fires. Hits beyond the arch proposals: `security-plan.md:87`,
  `obs-plan.md:42`, `a11y-plan.md:115`, `a11y-plan.md:280`.
- Wider pattern for the same claim WITHOUT the `cargo build` token (`build \+ test \+ clippy|GitHub
  Actions:`) — catches `architecture.md:235`, which the plan's own `CI/CD\|ci\.yml` grep missed because
  the tree comment carries none of those tokens. (The arch detector found it independently.)
- `obs-plan.md:526` — the §9 Lint/typecheck row naming only `cargo clippy`. This is the plan's own
  `Expected amendments (wrap)` entry; **no detector proposed it**, so it is raised here under Validate
  check 5, routine because the report substantiates it.

## Anticipation checked and found half wrong

The prior wrap's handoff recorded that *"test-plan §6 (Mode cell) / §9 (Matrix builds) **and a11y-plan §1 /
§11** extend the measured runtime × driver PAIR SET with this run's member."* Measured here:

- **test-plan — confirmed.** Pair-set literals at `:56` (§1 Surfaces under test), `:307` (§6 Drivers per
  surface, the `151 × 151` Failing entry) and `:469` (§9 Matrix builds). The detector found all three.
- **a11y-plan — the disposition stands, but my first basis for it was WRONG and is corrected here.**
  I initially recorded that a11y-plan carries no runtime/driver literals. It does: WebView2
  **151.0.4129.107** (§1), **152.0.4191.53 under a 151 msedgedriver** — a11y-plan's own words, *"a measured
  unsupported-but-working cross-major pair"* (§1) — `msedgedriver` at `:217`/`:218`,
  `CONDUCTOR_MSEDGEDRIVER` at `:299`, and the 152.0.4191.53 pass record at `:333`/`:424`. My grep returned
  those hits and I mischaracterised them as colour-token pairs without reading them; the operator caught it.
  **The correct basis:** every one is a DATED DEV-HOST measurement, not a provisional claim about the hosted
  image, so this falsification retires none of them. What the handoff anticipated — a runtime × driver
  **PAIR SET** in a11y-plan to extend — does not exist there; test-plan alone carries pair-set literals. So
  the a11y detector's `proposals: []` on that axis was correct, and the handoff's anticipation was
  over-broad — but on the measured basis above, not on an absence claim the file contradicts.
- **`a11y-plan:218` ("CI is TARGETED at a Linux+`xvfb` runner alongside tests' webview E2E") — routed, not
  amended.** Cascade step-2 test, since arch's [CI/CD] sentence is in this pass's amended set: does arch
  state that targeting, making `:218` a citation that rides this pass? `grep -nE 'xvfb|ubuntu-latest'
  architecture.md` → **0 hits**, control firing elsewhere (a11y-plan 13, test-plan 10). So it is a11y-plan's
  OWN statement, not a citation of arch → routes to its own channel as pre-existing drift, outside this
  pass. Reading recorded rather than hand-waved: the same sentence self-corrects (*"a CI a11y job EXISTS
  since 2026-09-07 — job `a11y` on `windows-2025`"*) and a11y-plan elsewhere marks `ubuntu-latest`+`xvfb`
  explicitly as "the CI ARRANGEMENT" and "absent and target-state", so that channel may find nothing to fix.

## Validation outcome

- **Check 1 (playbook):** Class A + B match rule @28 (spec-illustration → sound-impl alignment, invariant
  holds) and rule @88 (a chunk operationalizing a spec'd CI gate for the first time reconciles that spec's
  own stale description) → **routine**. Class C matches **no rule** → escalated.
- **Check 2 (cross-contradiction):** none. No two proposals edit the same section in opposing directions.
  Two lines (`architecture.md:59`, `test-plan.md:469`) each carry TWO distinct claims taking two distinct
  edits — the intra-line duplicate hazard, handled as two edits per line, not one.
- **Check 3 (intent-consistency):** the report matches the chunk's working-route entry and plan acceptance
  criteria; the one UNMET criterion (runner portability) is a noted deferral with a named owner.
- **Check 4 (absence needs evidence):** every "none" in the report's Changes carries its derivation; the
  a11y pair-set absence above is established by a named grep with its result, not inferred.
- **Check 5 (expected-amendments reconciliation):** both plan entries dispositioned — `architecture.md`
  §Established Decisions [CI/CD] by proposal, `obs-plan.md` §9 by orchestrator raise.
- **Check 6 (disproved-claims disposition):** both report entries DISPOSED — `test-plan:469`'s two-job
  claim by proposal (Class B); the runtime-major falsification by Class C plus the P7.3 `matrix.py defer`.

## Applied — 16 sites, 5 masters

**Class A, the gate-set enumeration gains `cargo fmt --all --check` (11 sites):** `arch:37` `:59` `:235`
`:238` `:268` (detector) · `security-plan:87` · `obs-plan:42` · `obs-plan:526` · `a11y-plan:115` `:280`
`:471` (orchestrator raises). Playbook rules @28 + @88 → routine.

**Class B, the CI arrangement (1 site):** `test-plan:469` two-job claim → the measured three-job set named
by job rather than line coordinate. Rule @88 → routine.

**Class C, the runtime-152 falsification (4 sites):** `arch:59` · `test-plan:469` (second claim on the same
line) · `test-plan:56` · `test-plan:307`. No rule matched → escalated → operator-approved, with the
accompanying playbook rule minted.

**The `a11y:471` catch.** The post-amendment sweep ran a known-positive control — `grep -c 'build + test
gating'` in a11y-plan returned **3** where only 2 had been edited — exposing a THIRD verbatim citation of
arch §Stack's rationale cell. Without the control it would have survived a two-site apply. Final state: 0
surviving occurrences of the old `"build + test gating` citation across all seven masters; 4 of the new
wording (arch:37 + a11y ×3).

**Operator correction to Class C's shape (`test-plan:307`).** The detector proposed extending the measured
runtime set. That would have left the table implying the pair still discriminates at major granularity.
Measured: a **152 × 151** combination both PASSES on the dev host (152.0.4191.53 × 151.0.4129.101) and
FAILS on the hosted image (152.0.4191.66 × 151.0.4129.101) — same majors, opposite outcomes. So the
applied text says the PAIR is not the discriminator, names the runtime PATCH (53 vs 66) as unmeasured-as-a-
cause, and puts the open variable on the hosted image itself.

**Cascade leaves:** one edit — `.claude/docs/stack.md:42` (the §Stack CI row's leaf). CLAUDE.md's GENERATED
tier states no CI gate set and no specialist summary carries a retired claim, both verified by grep.
`CLAUDE.md:132` sits inside `USER:session-learnings` (preserve-verbatim); it is not staled by these
amendments, so nothing routes to curation.

**One finding recorded, not amended:** the fmt step does **not** move security-plan's "six governed spawn
forms" enumeration. A plain `run:` step invoking a fixed toolchain binary with fixed argv is the class of
the existing `cargo build` / `cargo audit` steps, none of which are counted among the six (those are
constructed-argv spawns). No seventh crossing, so no boundary-widening ratification is owed — which the
prior chunk explicitly warned would escalate again.
