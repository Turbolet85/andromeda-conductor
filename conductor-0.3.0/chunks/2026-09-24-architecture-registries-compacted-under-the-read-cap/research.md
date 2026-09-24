# Codebase Research — 2026-09-24-architecture-registries-compacted-under-the-read-cap

## Scope
- **Depth:** moderate (a spec-body chunk; the "codebase" is the spec corpus, its readers and the pipeline contract
  that decides who may write it) · **Reads:** 9 (architecture.md whole-file Read; plan-template.md §Discipline;
  amendment-flow.md §5; wrap SKILL.md P7; gate-contract.md; implement SKILL.md role/constraints; a prior
  spec-centric plan (`2026-09-17-keyboard-and-focus-order-coverage-ownership/plan.md:1-120`); `scripts/mutation-gate.py:1-60`;
  `architecture-amendments.md:1-4`) · **Globs/Greps:** 11
- **Harness rules consulted:** none — no live leg in this chunk (no scenario/harness run against a real process).
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry (Setup 5a read HEAD `f0d92bd`
  all three check-runs `success`; nothing folded).

## Files inspected
- `.andromeda/architecture.md` (whole-file Read, then python section/line measurements) — the subject. Section
  bytes via `len(text.encode())` heading-to-next-`## ` (`scratchpad/sections.py`, HEAD `f0d92bd`): §Established
  Decisions **49 133 B** (lines 41-72), §Occupied Resources **48 858 B** (lines 143-207); file 148 508 B / 283 LF
  lines. Longest lines: `:60` [CI/CD] 12 099 B, `:70` 11 969 B, `:178` (`pulse-run-contract.toml` row) 9 561 B.
- `.andromeda/architecture-amendments.md:1-4` — preamble, verbatim: "_Append-only changelog of amendments to
  `architecture.md` (the body holds only current truth; history lives here + in git). Written by
  /andromeda-wrap-session P2._" File 179 822 B / 806 lines.
- `~/.claude/skills/andromeda-phase/references/plan-template.md:113` — "**Spec masters are NEVER touchpoints** —
  `Files to modify` must not list any of the seven `.andromeda/` spec masters … A chunk whose work makes a spec
  stale … records it under `## Implementation notes` as `Expected amendments (wrap): {doc} §{section} — {direction}`".
- `~/.claude/skills/andromeda-implement/SKILL.md:15-45` — implement touches "no route / master / specs"; "Specs stay
  read-only EVEN IF `plan.md` lists one under Files to modify — that is a plan defect".
- `~/.claude/skills/andromeda-wrap-session/SKILL.md:256-275` — P7.1 light gate re-runs the plan's `## Test Commands`
  AFTER P2 applied the amendments and BEFORE the P7 commit.
- `~/.claude/skills/andromeda-wrap-session/references/amendment-flow.md:127-131` — step 5: every `Expected amendments
  (wrap)` entry no detector proposed "the orchestrator raises itself"; the plan's list "is this chunk's coverage floor".
- `~/.claude/skills/andromeda-phase/references/gate-contract.md` — twelve closed keys; no wrap-only / implement-only
  gate key exists; `expect = []` is the report-only form (`recorded`, never counted green).
- `conductor-0.3.0/chunks/2026-09-17-keyboard-and-focus-order-coverage-ownership/plan.md:25-27, 85-90` — the in-repo
  precedent: "the enumeration is /implement's, the master's prose is wrap's"; `.andromeda/a11y-plan.md` listed as
  "NOT a touchpoint".
- `scripts/mutation-gate.py:1-60` — the committed operator-instrument convention: module docstring stating the
  gate's reading rule + `Usage: python -X utf8 scripts/…`, `ROOT = Path(__file__).resolve().parent.parent`, stdlib
  only, typed functions, `UNPARSED:` lines for unreadable input.

## Graph impact
- **Not queried — no existing code symbol is in the modify-set.** The chunk adds one stdlib Python script and
  chunk-folder drafts; it changes no Rust or TS. Basis for "no code reader parses architecture.md":
  `grep -rlE 'architecture\.md' crates scripts .github` → 2 files, both COMMENTS citing a section by name
  (`crates/conductor-run/tests/composition_root.rs:5` §Standard Contracts; `scripts/webview2-cause-probe.ps1:6,113`
  §Established Decisions [CI/CD] / §Occupied Resources). Neither reads the file. The graph is not consulted because
  there is no symbol to key on, not because a query returned 0.

## Measurements (the chunk's own premises, re-derived at HEAD)

### M1 — What a Read past the cap does (relay hypothesis, second half)
A whole-file `Read` of `.andromeda/architecture.md` (this session, 2026-09-24) did **not** refuse. It returned
lines 1-72 whole and a banner: `PARTIAL view … showing lines 1-72 of 284 total (58443 tokens, cap 25000). Call Read
with offset=73 …`. So "returns page 1 without refusing" is **verified**, with one sharpening: it is not SILENT —
the banner names the total and the cap — but a consumer that does not act on the banner reads page 1 only.
- **Bytes per token on this file:** 148 508 B / 58 443 tokens = **2.541 B/token** (the tool's own count). The cap in
  bytes for this corpus is therefore 25 000 × 2.541 ≈ **63 525 B** — the relay's "~64 KB" estimate holds, and
  health-criteria check 4's "≈60 KB at 2.56 B/token" is the same figure measured on rule files.
- **Page 1 ends at line 72 — the last line of §Established Decisions.** Any whole-file reader of architecture.md
  (a distiller, a doc-agent) sees §Established Decisions whole and §Occupied Resources (line 143+) not at all.
- **No per-line truncation:** lines `:60` (12 099 B) and `:70` (11 969 B) were returned whole inside that page.

### M2 — Each section is UNDER the cap today (changes the acceptance's footing)
§Established Decisions 49 133 B ≈ 19 336 tokens; §Occupied Resources 48 858 B ≈ 19 228 tokens (at 2.541 B/token)
— each ≈ 77 % of the 25 000-token cap. An offset-bounded Read of either section returns it whole **today**. The
entry's outcome "each readable whole in one Read" therefore already holds at HEAD; what the chunk buys is
**headroom** against the growth measured in M3. The acceptance must be a headroom threshold, not "readable whole",
or it is satisfied by doing nothing.

### M3 — Growth rate (relay hypothesis, first half)
Per-commit section sizes over `git log -- .andromeda/architecture.md` (82 commits; `scratchpad/growth.py`):

| date | sha | §Established Decisions | §Occupied Resources |
|---|---|---|---|
| 2026-09-01 | `f1584b1` | 27 539 | 19 538 |
| 2026-09-15 | `dc757b3` | 40 600 | 42 360 |
| 2026-09-23 | `edc0af8` | 49 133 | 48 858 |

Last eight days: +8 533 B (≈1.07 KB/day) and +6 498 B (≈0.81 KB/day). Since 2026-09-01: ≈0.98 and ≈1.33 KB/day. At
those rates §Established Decisions reaches ≈63.5 KB in ≈13-14 days and §Occupied Resources in ≈11-18 days —
"about two weeks" is **verified**. (The relay's "79.8 KB … on 2026-09-15" re-derives to 82 960 B = 81.0 KiB at
`dc757b3`; a small difference in a prior date's figure, not load-bearing.) Every one of those commits is a wrap
(the body is written only by wrap P2), so growth happens only at wrap.

### M4 — The body carries its amendment log
Counted over the two sections at HEAD (`scratchpad/inventory.py`):
- §Established Decisions: 62 ISO-date mentions, 28 sha-like tokens, 12 CI run ids, `measured` ×38, `RETIRED` ×6,
  `no longer` ×7.
- §Occupied Resources: 70 ISO-date mentions, 7 sha-like tokens, 1 CI run id, `measured` ×30.

This is against the sidecar's own preamble ("the body holds only current truth") and wrap SKILL.md ("a body never
holds an amendment log"). So the compaction is mostly a **return to an existing rule**, not a new policy. Much of
that narrative is already in the sidecar: the entries that added it are there. So moved text need not be
re-appended where the sidecar already carries it.

### M5 — The registry inventory the "no fact lost" check must conserve
Derived at HEAD from the two sections (`scratchpad/inventory.py`; re-derived by the checker at run time, never
pinned):
- 22 `[Bracket]` decision labels.
- 7 bold sub-registry labels in §Occupied Resources: `Ports` · `Interface routes / surfaces (no HTTP routes)` ·
  `Service / process names` · `Crate names (workspace members)` · `Frontend asset subtree` · `On-disk artifacts /
  database` · `Environment variables`.
- Distinct backtick spans: 356 (§ED) and 406 (§OR).
- Ports: `127.0.0.1:4317` · `127.0.0.1:4444` · `127.0.0.1:4445` · `:4317` · `:4318` · `:5173`.
- The arch extract's "49 bullets / 24 handles / 14 artifacts" is a distiller count and is **not** used as a
  literal. The checker derives its sets from HEAD itself.

### M6 — Live line-number readers of architecture.md (scope premise 4)
A sweep over every LIVE (non-history) consumer. Pattern
`architecture(\.md)?`?:[0-9]+|arch(itecture)?(\.md)? ?`?:[0-9]{2,3}\b`; files covered: CLAUDE.md, `.claude/rules/*.md`,
`.claude/docs/*.md`, the seven masters, `drift-base.md`, `playbook.md`, `residuals.md`, the working route,
`contracts/*`, `scenarios/*.toml`. Result: **1 hit**.
- `.andromeda/residuals.md:15`, offset 1126: "its unmet-floor outcome already recorded accepted at
  `architecture.md:69`". Read by offset. **Already stale at HEAD**: `:69` is a blank line. The fact it means sits
  in `:70`'s degraded-route passage.

Every other live consumer cites by section name or `[Bracket]` label, never by line:
- `.claude/docs/gotchas.md:14,20,26,50,72`, `.claude/docs/stack.md:53`
- `drift-base.md:29,35-36`, `playbook.md:37-234`
- `CLAUDE.md:52`
- obs-plan / security-plan / a11y-plan / test-plan bodies (per their extracts)

Those citations survive compaction as long as the labels do.

History citations of `architecture.md:{N}` are append-only records. They are not live readers, and none is
rewritten (`grep -rnoE 'architecture\.md:[0-9]+'` repo-wide → 127 files). Those files are the sidecars, chunk
reports/plans/scopes and run dirs.

### M7 — Byte counts are host-stable
`.gitattributes` `* text=auto eol=lf`; `git ls-files --eol .andromeda/architecture.md` → `i/lf w/lf`, and no
historical blob carries CRLF (`growth.py`'s crlf column, 82 commits, all False). A UTF-8 byte count is the same
on every checkout.

## Patterns detected
- **Spec work = implement-staged artifact + wrap-applied amendment** (`2026-09-17-keyboard…/plan.md:25-27`): the
  master's prose is an `Expected amendments (wrap)` entry; /implement produces the artifact the amendment rests on.
- **Committed stdlib operator instrument** (`scripts/mutation-gate.py:1-37`): docstring states the reading rule,
  `ROOT` from `__file__`, typed stdlib Python, run as `python -X utf8 scripts/{name}.py …`, no CI step. The
  mutation-gate footing is also the precedent (test-plan §9 "Scoped mutation audit") for an instrument outside CI.
- **Printed verdict as the last line** (`2026-09-17-keyboard…/plan.md:52-53, 117`): a checker's gate asserts
  `last line {verdict}` rather than the bare exit.
- **Derived, never literal, counts** (test-plan §6 Coverage-matrix Verification signal, per the tests extract): a
  gate derives its expected set from the source, never pins a count.

## Conventions to follow
- **Cite by label, not line** — every live reader already does (M6); the drafts keep all 22 labels and 7
  sub-labels byte-identical so no reader moves.
- **Host-path hygiene** — the checker prints section names, byte/token figures and missing NAMES, never an absolute
  path (security-plan §Security Anti-Patterns → Logging; obs-plan §9).
- **Spawn discipline** — the checker's only subprocess is `git show {rev}:{path}` as an argv LIST with a
  validated rev and a hard-coded path (security-plan §Security Anti-Patterns → Code Patterns rule (b); a fixed
  toolchain binary, the 2026-09-09 class ruling per the security extract).

## New files to create
- `scripts/arch-registry-check.py` — the instrument (measure + conserve + selftest).
- `conductor-0.3.0/chunks/{marker}/compaction/established-decisions.md` — the drafted replacement for
  §Established Decisions, heading included.
- `conductor-0.3.0/chunks/{marker}/compaction/occupied-resources.md` — the drafted replacement for §Occupied
  Resources, heading included.
- `conductor-0.3.0/chunks/{marker}/compaction/moved-history.md` — the sidecar text the wrap appends: only passages
  whose names the sidecar does not already carry.
- `conductor-0.3.0/chunks/{marker}/evidence/` — the checker's recorded verdicts.

## Files to modify
- None in the code tree. The two master edits (`.andromeda/architecture.md`, `.andromeda/architecture-amendments.md`)
  are `Expected amendments (wrap)`, never touchpoints (plan-template.md:113).
- **`.andromeda/residuals.md:15`** — the stale `architecture.md:69` citation (M6). It is not a master, and the
  handoff records residuals.md as having "no sanctioned writer yet" (its `:11` carry). **No change by this chunk**:
  it is surfaced for the wrap's sweep beside the existing carry, not repaired here.
- **`scripts/webview2-cause-probe.ps1:6`** — its comment says [CI/CD] "holds OPEN" three candidates. That is
  already stale at HEAD: the cause was established 2026-09-12 per `:60` itself. It is a diagnostic script's
  historical comment. **No change** (out of scope; the script is not a reader).

## Open questions
- **The target figure** — how far below the ≈63.5 KB cap each section must land. M2 makes "readable whole" already
  true, so this number IS the acceptance. → blocks: plan-decision (P4 fork).
- **Where the durable check lives** — a wrap-time drift-base detector (arch changes only at wrap, M3) vs a CI step
  as well. → blocks: plan-decision (P4; research leans wrap-time, see scope).
