# Session Handoff

**Last Updated:** 2026-09-16T17:05:00Z
**Branch:** `build/conductor-0.3.0` · **0 ahead of `origin/build/conductor-0.3.0` at Setup** (the operator
pushed `38d21d1` mid-session); this wrap's commit leaves it **1**. The operator pushes.
**Status:** clean — drift 0, all escalations resolved, `v3-02` un-claimed by operator ruling.
**Last Commit:** `feat(2026-09-16-a11y-ci-gate-at-an-honest-terminal)` — see below.

## Position
- Done: **`2026-09-16-a11y-ci-gate-at-an-honest-terminal`** — the terminal is **UNDECIDED**, and that is the
  honest outcome. What the chunk shipped is a CAUSE established at the project's bar and two routes closed
  by MECHANISM rather than exhaustion.
- Next: **`Medium-integrity launch for the a11y routine arm`** — `conductor-0.3.0/working-route.md:35`, head
  of the markerless tail, Epoch 3. Added at this wrap's P5 trajectory halt, operator-directed.
- Coverage **5/11 verified · 6 unclaimed** — `v3-02` returned to the pool (was claimed to this chunk).

## The finding

**Integrity level, not the administrator role, decides whether WebView2 grants a debugging endpoint.**
Established by variation with a control on both sides, three dev-host legs at runtime 153:

| leg | role | integrity | session |
|---|---|---|---|
| A (operator, elevated shell) | admin | High | **no** — `DevToolsActivePort`, 2:00 |
| B (operator, `runas /trustlevel`) | False | High | **no** — 2:00 |
| C (overseer, normal shell) | False | **Medium** | **yes** — `[webview2 153…]` ×35, 0:06 |

A→B varies the role with integrity held (no change ⇒ role exonerated); B→C varies integrity with the role
held (the session appears). **Integrity buys the SESSION, not a green arm** — leg C still failed 0/1/2
because `:384`/`:397` are broken by runtime 153. Keep the two questions apart.

Both routes this chunk built are retired BY MECHANISM: `RunLevel Limited` cannot lower the label (the
runner's account is the built-in Administrator, RID 500, `FilterAdministratorToken` absent — no filtered
token exists); `runas /trustlevel` cannot either (strips the group, leaves the label). Remedy for the
successor: `CreateRestrictedToken` + `SetTokenInformation` + `CreateProcessAsUser`.

## Work done
`.github/workflows/ci.yml` (+20/−4) · `scripts/a11y-limited-token-launch.ps1` (new) ·
`scripts/a11y-token-witness.ps1` (new) · **8 evidence records**. No Rust/TS/crate surface, no dependency,
no lockfile delta. Four CI probes driven inside the fix-loop under the operator's ci-probe directive — no
operator commits, `HEAD 38d21d1` throughout, probe branch deleted.

## Drift resolved
**7 fan-out proposals applied across 3 masters** (arch 5 · security-plan 1 · test-plan 1; four docs clean),
**plus 3 duplicates the fan-out missed and the cascade sweep caught** (`test-plan.md:470` restating the
retired remedy claim, `a11y-plan.md:516` pinning the very pair that regressed, `a11y-plan.md:471`), **plus
1 distillation** (`.claude/rules/security.md` SIX→SEVEN, invisible to the literal sweep because it is
bolded `**SIX**`). One escalation resolved: the seventh governed harness-spawn form, operator-ratified
under playbook `:124` — and registered as the mechanism that SHIPS (`runas`), not the scheduled task the
chunk's own drafted text described.

## Notes
- **`v3-02` un-claimed, not excluded.** The requirement is right and the world is wrong. An exclusion was
  explicitly NOT ratified: after leg C it would record a falsehood, since a probable path to green is now
  measured.
- **Dated advance warning:** `:384`/`:397` break on WebView2 153 and go red in CI the day the runner image
  moves. CI runs 152 and is unaffected today. Separate defect, own owner, carried on the next entry.
- **ONE hygiene defect, THREE surfaces** (carried): orphaned `msedgewebview2` hold inherited handles (the
  ~15-min CI tail), `CENSUS_NAMES` at `parse-nvda-log.ts:130-137` omits the webview host (SR territory),
  and they accumulate across runs. The reap went 7 → 17 and is **not** done (6 in-window survivors).
- **The Evergreen float charged a cost before its gate lit** — an unprompted overnight bump broke the arm's
  two newest assertions with no repo input changing. The pin-or-retire disposition returns to the pool.
- **Owed to the operator:** `.claude/rules/testing.md` 82.7 KB and `verification-harness.md` 69.8 KB, both
  past the Read cap. `host-win32.md` grew to 17.9 KB this wrap (2 entries, 984 B and 755 B — above the
  ~600 B always-loaded bar, detail pushed to Tier 3 with a pointer). Always-loaded rule total **43.7 KB**.
- **Still open from prior sessions:** the n=1 deferred escalation class; the audit-debt chunk's discarded
  wrap `gates` evolve record; the `quantile` 14-vs-11 correction for `code-metrics.ndjson`.
- **`v3-08` stays BLOCKED** — unchanged.
- **Last failed command:** none.

## Session End Status
Wrapped at 2026-09-16 — context ran to 90%, wrap completed without a split.
