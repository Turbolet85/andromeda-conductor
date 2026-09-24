# Scope — Architecture registries compacted under the read cap

**Marker:** `2026-09-24-architecture-registries-compacted-under-the-read-cap` · **Version:** conductor-0.3.0 ·
**Epoch:** Epoch 4 — Live proof against a real Pulse · **Working entry:** `working-route.md:48`

## Intent (the working entry, verbatim title + hint)
Architecture registries compacted under the read cap — Established Decisions and Occupied Resources each readable
whole in one Read, no registered fact lost.

## What this chunk builds
- `.andromeda/architecture.md` `## Established Decisions` and `## Occupied Resources` each brought well below the
  Read cap, measured in bytes against a stated threshold. [premise-corrected: each section is ALREADY under the
  cap at HEAD — ≈19.3k and ≈19.2k tokens of 25 000 (research M2) — so "readable whole" holds today and the chunk's
  measurable outcome is HEADROOM below the cap, at a target figure fixed in P4]
- **No registered fact lost:** every decision, port, surface, process, crate, artifact and env-var handle registered
  today is still registered after compaction — as the current-truth statement in the body. History, dated
  corrections and extension narratives that the body carries leave the body.
- A durable, re-runnable measurement of the two sections' sizes. [premise-corrected: its form is a committed stdlib
  instrument `scripts/arch-registry-check.py` (the `scripts/mutation-gate.py` footing), with the standing
  enforcement proposed at wrap as a drift-base detector — the body grows only at wrap (research M3), so a wrap-time
  check sits at the only point growth happens; the CI-step alternative is a P4 question]

## Measured baseline (2026-09-24, HEAD `f0d92bd`, python `len(text.encode())` per section)
- `architecture.md` 148 508 B, 283 lines. `## Established Decisions` (lines 41-73) 49 133 B; `## Occupied Resources`
  (lines 143-208) 48 858 B — together 66 %. Byte-identical to the relay's `edc0af8` figures (the file was last
  written 2026-09-23 11:34; the capture-path chunk did not touch it).
- Largest single lines: Established Decisions `:60` ([CI/CD]) 12 099 B and `:70` (the fingerprint-derivation
  paragraph) 11 969 B; `:64` 3 004 B; `:53` 3 177 B. Occupied Resources `:178` (`pulse-run-contract.toml` row)
  9 561 B; `:177` 3 875 B.
- `architecture-amendments.md` is itself 179 822 B / 806 lines — already past any single-Read cap. Whatever moves
  out of the body lands in a sidecar that is append-only history, so moving text there does not make the sidecar
  "readable whole" and is not required to. [verified: the sidecar preamble names itself "Append-only changelog …
  the body holds only current truth; history lives here + in git" (`architecture-amendments.md:3`); moved text is
  appended only where the sidecar does not already carry it (research M4)]

## Folded annotations (from the working entry, re-verified)
- **CONTEXT (operator relay CE-3, 2026-09-23 0-pending adaptation)** — sizes re-measured above; they hold exactly.
  The relay's 2026-09-15 figure (79.8 KB, 64 %) is a prior date's measurement, not a claim about today (re-derived
  at `dc757b3`: 82 960 B = 81.0 KiB; research M3).
- **hypothesis (verbatim from the relay):** "at the last eight days' pace the larger section reaches the ~64 KB Read
  cap (itself an estimate) in about two weeks, and a Read past its cap returns page 1 without refusing." —
  [verified, both halves: (1) 25 000 tokens × 2.541 B/token measured on this file = ≈63.5 KB; §Established
  Decisions grows ≈1.07 KB/day and reaches it in ≈13-14 days (research M1, M3); (2) a whole-file Read returned a
  PARTIAL view of lines 1-72 of 284 with a banner naming "58443 tokens, cap 25000" — no refusal, though not silent
  (research M1). Page 1 ends exactly at the last line of §Established Decisions, so a whole-file reader never sees
  §Occupied Resources]
- **"How to compact — what moves to `architecture-amendments.md`, what the decisions log's contract allows — is this
  chunk's research."** — closed in research M4-M6 and the premise closures below.
- **CARRY:** propose a `D-arch-collision` detector at this chunk's wrap (propose → operator approves → append to
  `.andromeda/drift-base.md`). Re-verified: `drift-base.md:27-31` is `D-arch-resources`, invariant "every new IPC
  method / endpoint / event / socket / port / env var / workspace crate the chunk lands is registered in arch
  §Occupied Resources / §Standard Contracts / §Inherited Defaults workspace crates" — registration only; nothing
  there checks for a SECOND owner of an already-registered resource. The proposal is a wrap-time act; this chunk
  carries it to its wrap.

## Boundaries
- In: the two registry sections of `architecture.md`; their history's destination; the size measurement; the
  CARRY's detector proposal text.
- Out: the other nine sections of `architecture.md` (Standard Contracts 15 486 B, Infrastructure Patterns
  12 641 B are under the cap); the other six spec masters; CLAUDE.md / rule-file size (the health warnings on
  `testing.md` / `verification-harness.md` / T1 bullets are a separate operator call). No Rust / TS code change.
  [verified: the modify-set is one new stdlib Python script plus chunk-folder drafts; no code reader parses
  architecture.md — `grep -rlE 'architecture\.md' crates scripts .github` → 2 files, both comments (research
  §Graph impact)]
- No fact is re-decided: compaction changes WHERE and HOW LONG a fact is stated, never WHAT is decided.

## Premises closed in P3
1. **Ownership of the edit.** [verified, with the route found: /implement treats the seven masters as read-only,
   and plan-template.md:113 bars them from `Files to modify` — "A chunk whose work makes a spec stale … records it
   under `## Implementation notes` as `Expected amendments (wrap)`". The in-repo precedent
   (`2026-09-17-keyboard-and-focus-order-coverage-ownership/plan.md:25-27`) splits the work: the artifact is
   /implement's, the master's prose is wrap's. Here /implement drafts the two replacement sections and the moved
   history into the chunk folder and proves them with the instrument; wrap P2 applies them as Expected amendments;
   the wrap P7 light gate (which runs AFTER P2 applies and BEFORE the commit — wrap SKILL.md P7.1) re-runs the same
   gates]
2. **What is history vs current truth.** [verified: the body carries its amendment log against the sidecar's own
   rule — 62 + 70 ISO-date mentions, 12 CI run ids and 35 sha-like tokens across the two sections (research M4).
   The compaction is largely a return to the existing "body holds only current truth" rule]
3. **Does the Read tool truncate a long single line?** [premise-corrected: no — lines `:60` (12 099 B) and `:70`
   (11 969 B) were returned whole inside the partial page; the only truncation is the whole-read token cap, applied
   at line granularity and announced by a banner (research M1)]
4. **Readers that key on line numbers.** [premise-corrected: live readers cite by section name or `[Bracket]` label,
   not by line — one live line citation exists, `.andromeda/residuals.md:15` → `architecture.md:69`, and it is
   ALREADY stale at HEAD (`:69` is a blank line). History citations (127 files: sidecars, chunk artifacts, run dirs)
   are append-only records and are not rewritten (research M6). Keeping every label byte-identical is what keeps
   the live readers resolving]

## CI verdict (Setup 5a)
HEAD `f0d92bdc` — all three check-runs `completed · success` (Frontend gate, Rust gate, A11y gate). Nothing to fold.
