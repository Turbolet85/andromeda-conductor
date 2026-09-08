# Session Handoff

**Last Updated:** 2026-09-08T21:02:00Z
**Branch:** build/conductor-0.2.0 (tracks `origin/build/conductor-0.2.0`; **0 ahead at wrap start** — the
operator pushed mid-session, so the 2-ahead figure in the previous handoff is stale. This wrap's chunk
commit makes it **1 ahead and unpushed**. The push is the operator's act — and this time it is
**load-bearing, not bookkeeping**: the chunk's whole verdict is a CI run that cannot happen until the push
lands.)
**Status:** clean
**Last Commit:** `feat(2026-09-08-webview2-runtime-152-installed-in-job)` (this wrap)

## Position
- Done: **`2026-09-08-webview2-runtime-152-installed-in-job`** — flipped `complete` at this wrap.
- Next: **`/andromeda-phase`** on the first markerless head — **_Workspace formatting pass and a fmt CI
  gate_** (`working-route.md:129`), minted at this wrap on the operator's directive and **placed ahead of
  *Release build and bundle* by the operator at P5**. No `BLOCKED-ON`, so phase will not halt. The sibling
  behind it is *Release build and bundle* (`:131`).
- Coverage **28/32 verified · 4 unclaimed** (`v2-04`, `v2-21`, `v2-24`, `v2-27`) — unchanged. This chunk
  claimed nothing, deliberately.
- **Evolve:** Epoch 6b at **12 chunks** (10 frozen + 2 markerless). Surfaced at P5; operator ruled **no
  split** — 6b is one or two chunks from its natural end, so a boundary now would fragment the cadence
  rather than restore it. Re-surfaces next wrap.

## THE OPEN ITEM — `v2-24`, and it has no route owner
`v2-24` stays **`planned` and pooled** (operator directive item 2). The probe this chunk ships answers a
question only a CI run can answer, and that run follows the push — so the **claim-or-defer fork belongs to
the follow-up entry that reads run (a)**, and **no such entry exists on the route yet**.

Read the run's `[diag] (a) bare-app DevToolsActivePort first seen:` line. Then:
- **A number of seconds** ⇒ the runtime major was the cause. `v2-24` stays `planned`; mint the follow-up
  entry that claims it on a green `a11y` job.
- **`never within 90s`** ⇒ the runtime-major hypothesis is **FALSIFIED**. Record it with its mechanism, and
  `deferred` becomes the honest status (terminal until the next version's intake), since no route forward
  would remain inside this version. Surviving unmeasured candidates, from the prior chunk's verdict: no
  interactive desktop session on a hosted runner; a Session-0 / service-account restriction on the WebView2
  browser process; an image policy on remote debugging.

Also owed on that run, and likewise not yet due: **test-plan §6 (Mode cell) / §9 (Matrix builds) and
a11y-plan §1 / §11** extend the measured runtime × driver **PAIR SET** with this run's member — never a
fresh literal substituted for the set. Both detectors independently classed it *owed-but-not-yet-due*.

## Work done
One chunk, **one source file**: `.github/workflows/ci.yml`, the `a11y` job only. A new **gate** step
`Install WebView2 Evergreen runtime 152+ (gate)` (HTTPS fetch → `Get-AuthenticodeSignature` gate → fixed
array-form `Start-Process … '/silent','/install'` → post-install major ≥ 152 assertion; no
`continue-on-error`, no `if:`), plus one guarded path staging `runs/logs/conductor-tauri.jsonl` — an
app-liveness witness independent of `DevToolsActivePort`, which is the reading the verdict turns on.

Step placement verified structurally: 12 steps, the gate at index 5, `continue-on-error` still confined to
indices 6/8/9.

**Dev-host validation (the installer deliberately NOT run** — this host is at 152.0.4191.66 and CI at
151.0.4129.101, so a local install exercises install-over-newer, a no-op, and cannot measure the CI upgrade
path): registry read ✓, HTTPS download ✓ (1 783 000 bytes), signature PASS branch ✓ (`Valid`,
`O=Microsoft Corporation`), signature **FAIL** branch ✓ (a byte-corrupted copy returned `UnknownError` and
was rejected — the gate catches tampering, not merely passes on a good file), version-major assertion ✓
(**151 → exit 1**, 152/153 → pass, which is exactly the no-op detection the design turns on).

## Gates
Green: porcelain probe (0 lines, advisory-db HEAD `8a1eb4f9`) · `cargo audit` 0 · `cargo deny check
advisories bans licenses sources` 0 · `npm run build` 0 · `cargo clippy --workspace --all-targets -D
warnings` 0 · `cargo nextest run --workspace --profile ci` 0 (**902 tests, 902 passed**) · YAML parse + 4
structural greps.

**The absorbed PREREQ is discharged** — the prior chunk deferred exactly `nextest --workspace` and
`clippy`; both ran green, closing the chain opened at `2026-09-08-hosted-runner-webview2-session`.

**Red, and NOT this chunk's:** `cargo fmt --check` exits 1 — 282 sites across ~40 files, no `.rs` in the
porcelain, so it reproduces at HEAD; `grep -n 'fmt' ci.yml` returns 0. Treated under the playbook's
external-decay rule (`playbook.md:93`) per operator directive item 1: no in-diff cause ⇒ not the
discovering chunk's drift, never blocks it, always produces an owner. **The owner is now the route entry at
`:129`.** `test-plan.md:455` is a target-state row left deliberately standing — the entry makes it true
rather than retiring it.

## Drift resolved
**10 amendments across 2 masters · 1 escalation resolved · 0 open.**

All 9 fan-out proposals routed to `playbook.md:124` (boundary widening — "always a human's call; never mint
a routine rule for this class"), collapsing to ONE operator decision, ratified: **the CI-fetched WebView2
runtime is Evergreen now, pinned once the a11y job actually GATES.** No playbook rule was minted, by design
— a seventh crossing escalates again.

- **architecture.md ×5** — §Ports registers the Evergreen HTTPS egress as CI-JOB-SCOPED (the project's only
  non-loopback outbound target); §Trust boundary's second enumeration gains it; §Environment variables
  registers `RUNNER_TEMP` and records that the install gate sets no handle; §Established Decisions [CI/CD]
  records the in-job provisioning **and states the endpoint outcome is UNMEASURED**; §Build system now
  distinguishes the still-pinned Edge **driver** from the deliberately floated **runtime**.
- **security-plan.md ×5** — spawn rule (b) **5 → 6 governed forms** across a third locus (`ci.yml`), the
  sixth being the first whose program is neither repo-derived nor a fixed OS binary; a **THIRD dependency
  class** no lockfile gate can see; the signed-artifact **SKIP** scoped to *produced* artifacts;
  §Threat-Model CI/CD's "supply-chain steps are unchanged" qualified; and **§Networking `:86`** — found not
  by the detector but by the cascade's own sweep, its "no public/VPN networking" clause false as written.
- **Cascade:** 4 leaves re-derived — CLAUDE.md's warnings block (its trust-boundary **ban** would otherwise
  have read as violated by the shipped change), `.claude/rules/security.md` ×2 sections,
  `.claude/docs/security-summary.md` ×2 sites. `docs/gotchas.md` examined and deliberately **not** edited:
  its claim is about *inbound* listeners.
- Five detectors returned `proposals: []`; **D-platform-claim fired in none of the seven** — the report's
  explicit "do not propose retiring `test-plan.md:455`" guard held everywhere.

## Notes
- **Curation: T1 0 · T2 1 · T3 1** (filtered 4, all duplicates; 0 conflicts, 0 deferred).
  - **T2 → `host-win32.md`**: `grep -E` accepts PCRE syntax **silently** — a `(?!…)` guard matches nothing
    and exits **0**, so it passes unconditionally. Measured: exactly that line shipped as a security gate
    asserting "no plaintext `http://`" and would have passed whatever the file held. Write the POSITIVE
    probe and assert hit count + identity.
  - **T3 → `session-learnings.md`**: read a matrix entry through `matrix.py show --id`, not a JSON load —
    the tool prints `title`/`observed_gap`/`requirement`, and `requirement` was **stricter** than the
    `acceptance` the decline decision rested on.
  - Two curated rules were **consulted and held**: the census rule (attribution by `Get-Process StartTime`,
    not the name pattern — six `msedgewebview2` processes proved 26 days old and not this run's) and the
    2026-09-07 rule that a chunk shipping a CI gate cannot prove it at its own wrap (which is *why* `v2-24`
    was declined).
- **Two self-authored gate defects, both caught only by running them** — at phase P4 a gate that could
  never **fail** (the `grep -E` lookahead); at implement P2 one that could never **pass** (`cargo fmt
  --check`). Both sat in the same Test Commands block. The P5 mechanical checks verify a command's presence
  and shape, never its satisfiability.
- **Last failed command:** none.

## Deferred learnings
None — 2 applied, under the cap of 3. No `recurrence-despite-learning`: every Filter-1 duplicate deduped
against an entry that was consulted and correctly applied this session, and one deduped against a line
**this wrap's own cascade had just written** (Filter 1's generated-body carve-out working as designed).
