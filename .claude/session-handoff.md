# Session Handoff

**Last Updated:** 2026-09-08T16:10:43Z
**Branch:** build/conductor-0.2.0 (tracks `origin/build/conductor-0.2.0`; **0 ahead at wrap start** — the
operator pushed before this session, which is what produced the build-branch baseline run. This wrap's chunk
commit makes it **1 ahead and unpushed**. The push is the operator's act, and it will NOT turn CI green: the
`a11y` job is correctly wired and correctly RED, now with a MEASURED cause.)
**Status:** clean
**Last Commit:** `feat(2026-09-08-hosted-runner-webview2-session): …` (this wrap)

## Position
- Done: **`2026-09-08-hosted-runner-webview2-session`** — diagnose-only chunk; the a11y job's failure cause is
  now measured and localized.
- Next: **`/andromeda-phase`** on the first markerless head — **_WebView2 runtime 152+ installed in-job — the
  one variable every passing case shares and the failing case lacks (v2-24 claimed or deferred)_**
  (`working-route.md:127`). It carries `PREREQ: close rust gate deferral (deferred since
  2026-09-08-hosted-runner-webview2-session)` — that chunk MUST run `cargo nextest run --workspace` +
  `clippy` as mandatory gates. No annotation-position `BLOCKED-ON`, so phase will not halt. The sibling is
  *Release build and bundle* (`:129`).
- Coverage **28/32 verified · 4 unclaimed** (`v2-04`, `v2-21`, `v2-24`, `v2-27`) — unchanged. `v2-24`
  deliberately stays `planned` and pooled per the wrap directive: a `deferred` status is terminal until the
  next version's intake, so it waits for the follow-up probe.
- **Evolve:** Epoch 6b at **11 chunks** (9 frozen + 2 markerless) — past the ~10 split threshold, surfaced
  again this wrap. The split remains the operator's call.

## Work done
Shipped **zero source delta**. The only code change is `.github/workflows/ci.yml` (+71/−4), extending the
`WebView2 session isolation (diagnostic)` step with three measurements, then one probe push (run
34234558853, ref deleted after reading).

**The finding:** on the hosted `windows-2025` runner the WebView2 runtime **never opens its remote-debugging
endpoint**, and this reproduces with **no driver in the picture at all**. Probe (a) — a bare launch with
`WEBVIEW2_USER_DATA_FOLDER` honoured (`EBWebView` created under it) and the app alive throughout — never sees
`DevToolsActivePort` within 90 s, where the identical form on the dev host produces it in 1 s. Everything
downstream follows, which is why the failure was byte-identical through the intermediary and direct. **The
driver was never the discriminator.**

H1, H2, H3-as-driver-skew and H5 are all retired on evidence. **H3 re-opens on the RUNTIME axis:** every
passing case runs runtime 152, the single failing case runs 151 — and the runtime is the one variable this
probe did not vary. That is the minted follow-up entry.

Two probe defects were caught by the plan's dev-host validation step BEFORE the push, both of which would
have failed **silently** in CI under `continue-on-error` and burned the single budgeted push: `Receive-Job`
has no `-Timeout` parameter, and `Select-Object -First 1` over `scoped_dir*` could not discriminate
(msedgedriver makes two siblings per session, prior runs leave residue) — it reported `EBWebView=False` on a
session that had returned HTTP 200.

## Drift resolved
**10 amendments across 3 masters, 2 escalations resolved, 1 proposal rejected. Drift = 0 on exit.**
- `architecture.md` §Occupied Resources — registered `WEBVIEW2_USER_DATA_FOLDER` /
  `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS` as a diagnostic-scoped pair. **Escalated** (no playbook rule
  governed an external handle a shipped artifact SETS but never READS — `:137` and `:143` each failed a
  qualifying clause in opposite directions); operator-ratified, and playbook rule **44** minted for the class.
- `test-plan.md` — 7 edits across 5 sections: `:307`'s driver↔runtime claim re-stated as a PAIR SET, five
  host-capability sites bounded to the measured runtime, and the `userDataDir: ""` mechanism recorded.
  **Escalated** on SCOPE and resolved: the retirement names the hosted image AS SHIPPED (runtime
  151.0.4129.101) rather than a flat incapacity, so it does not pre-judge the follow-up probe.
- `a11y-plan.md` — 3 sites saying the a11y job's first push-triggered run was still pending, corrected to run
  34209940695 (`event: push`, RED). **Raised by the orchestrator** under Validate check 5, not by a detector:
  the report legitimately does not carry that fact in a Changes bullet.
- **Rejected:** a11y-plan `:115`'s D-platform-claim proposal would have retired a TRUE claim. Its premise
  conflated two chunks; that chunk's own `nvda-pass.json` timestamps its subjects 10:41–10:44Z, before the
  12:45Z driver refresh, so its cross-major pairing is true as written and stays untouched. This is why the
  `:307` amendment SPLITS the claim rather than discarding it.
- Cascade: 5 leaf edits (`a11y-summary` ×3, `tests-summary` ×1, `rules/a11y.md` ×1). CLAUDE.md carries none
  of the amended tokens, so no `GENERATED:setup:*` block was owed.

## Notes
- **My report's first draft carried three wrong counts, all caught by re-deriving before the fan-out.** The
  worst: "no other master states a driver-version pairing" — `a11y-plan:115` does, and the amendment set was
  about to be authored one master short. Also measured: the `msedgedriver` TOKEN sweep returns 7 sites across
  3 masters while the VERSION-literal sweep returns 3 across 2 — the token is a poor proxy for the claim.
- **The 46-banner figure in the directive re-derives to 24** in the current `runs/a11y-e2e.log`; that log
  truncates per `--e2e` invocation, so the two are different moments and the load-bearing fact — every banner
  is runtime 152 — holds on both.
- **Test Command 5 was authored with the un-anchored drive-letter pattern** and returned 11 false positives
  unscoped (all read: prose about the tokens, the pattern itself, `p://` in URLs). See Deferred learnings.
- **Last failed command:** none.

## Deferred learnings
1 finding was not applied as a new entry:
- **`recurrence-despite-learning`:** `.claude/rules/testing.md:80` (dated 2026-09-03) already prescribes the
  WORD-ANCHORED `\b[A-Za-z]:[\\/]` over the bare form, names the exact `p:/` inside `http://127.0.0.1:4317`
  collision, and records it verified over seven cases. I authored this chunk's plan Test Command 5 with the
  bare form anyway, and testing.md auto-loads on the paths these gates touch. No rule edit is owed and a
  third entry is not the remedy — the entry is correct and did not prevent its own recurrence. **Third
  consecutive wrap carrying one** (the prior two: the diagnostic-form rule, the nextest-filter rule); logged
  to the friction stream as `recall.corpus-recurrence` for the pipeline's owners.
