# Wrap decisions — written BEFORE the wrap, so a context split loses nothing

Operator-directed 2026-09-16. If the wrap resumes in a fresh window it resumes from THIS file plus the
other `evidence/` records; anything living only in the session conversation is already lost.

## 1. Un-claim `v3-02` — ruled, with the precedent named

The capability is claimed to this marker at `status: planned` / `ref: null`, so the P7.3 coverage gate
halts as `coverage-incomplete: v3-02`. The disposition is the letter's own release valve:

```
matrix.py unclaim --dir conductor-0.3.0 --id v3-02 --run-dir {run_dir} --marker 2026-09-16-a11y-ci-gate-at-an-honest-terminal
```

**Why un-claim and not refine:** the requirement is RIGHT and the world is wrong. The runner is High
integrity and no shipped mechanism lowers the label. This applies the 2026-09-15 precedent rather than
excepting it — a partial advance leaves `chunk:null`, cites the cap in provenance, and the remainder
becomes the immediately-following chunk.

**Do NOT ratify an exclusion.** An hour before leg C it was weakly grounded; after leg C it would record
a falsehood, because a probable path to green has now been measured.

## 2. The next route entry — a trajectory edit, P5 will HALT, take the halt

**Subject:** a medium-integrity launch (`CreateRestrictedToken` + `SetTokenInformation` setting the
mandatory label to the Medium SID, then `CreateProcessAsUser`), on the measured premise that the
integrity LABEL — not the administrator role — is the discriminator.

**Freight that must travel with it:**
- the three-leg table below;
- **integrity buys the SESSION, not a green arm** — the two questions stay separate;
- `:384` / `:397` must ADDITIONALLY hold at the runner's runtime 152 for CI to go green;
- **window station: a NAMED, unvaried risk** — no longer last-standing, not excluded as a second
  contributor on the runner, where the session context differs from the dev host's.

## 3. The hygiene defect — ONE defect, THREE surfaces

Not three notes. Measured, not noticed: 19 `msedgewebview2` orphans alive on the dev host at wrap time,
after a reap improved 7 → 17 and explicitly **not** called done.

| Surface | Where |
|---|---|
| ~15-minute CI tail after a dead leg | orphans hold the inherited redirect handles, pinning the parent |
| `CENSUS_NAMES` gap | `parse-nvda-log.ts:130-137` — the webview host is the one name absent |
| cross-run accumulation | any host running the leg repeatedly |

The `CENSUS_NAMES` surface stays a **CARRY in SR territory** — both its call sites are screen-reader-gated
(`wdio.conf.ts:396`, `:450`, plus `:240` in `stopNvda`), so it is not this chunk's modify-set.

## 4. Outcome basis — the legs are not implement's own output

Legs **A** and **B** were run by the **operator** from one elevated shell; leg **C** by the **overseer**
from a non-elevated one. None of the three is /implement's output, and the report's outcome-basis line
exists for exactly this.

## 5. The established cause — the three-leg table

One host, WebView2 runtime 153 throughout:

| Leg | Role | Integrity | Session | Elapsed |
|---|---|---|---|---|
| A — direct, elevated | admin | **High** | **NO** — `DevToolsActivePort` | 2:00 |
| B — under `runas /trustlevel` | False | **High** | **NO** — `DevToolsActivePort` | 2:00 |
| C — normal shell | False | **MEDIUM** | **YES** — `[webview2 153.0.4234.32 windows]` ×35 | 0:06 |

A→B varies the ROLE with integrity held High: no change ⇒ role exonerated.
B→C varies INTEGRITY with the role held False: the session appears.

## 6. What the chunk shipped — a result, not an absence

- **The cause established at the project's bar** — variation with a control on both sides.
- **Two routes closed by MECHANISM, not exhaustion:** `RunLevel Limited` cannot lower the label (RID 500
  has no split token); `runas /trustlevel` cannot either (strips the group, keeps the label). Both were
  coherent with the cause and structurally unable to reach it.
- **The hygiene defect measured** (above).
- **The ci-probe instrument proven in anger** — four CI iterations inside the fix-loop, no operator
  commits, build branch and working tree untouched throughout (`HEAD` `38d21d1` across all of them).
- **A worry retired:** runtime 153 did NOT break session creation — leg C came up on 153 — so 153's
  damage is confined to `:384`/`:397` and the dev-host control is alive again for the endpoint question.

**The terminal is undecided. That is the honest outcome, not a failure of the chunk.**

## 7. Corrections made in-session (curation raw material)

- `IsInRole(Administrator)` returns False for a **deny-only** membership — "not elevated now" is not
  "has no admin rights". The same conflation, one layer down, as `IsElevatedAdmin` itself.
- **Clipped views, three instances in one session across two readers** — a per-path grep quoted from a
  clipped combined search; a site enumeration narrower than the graph query that had already answered
  it; a `:397`-only failure reading that nearly became "alignment made it worse".
- **Inherited-handle mechanism:** `$output = & cmd *>&1` holds the pipeline open until every process
  inheriting the stdout handle exits. `Start-Process -Wait` has the same defect (it drains redirection).
  Waiting on the process object does not.
- **An instrument's self-report can under-count its own subject:** `OrphansLeft` measures at the instant
  the last reap pass runs, while the tree is still settling.
- **A probe validated outside the context it runs in proves nothing:** the UAC block passed locally in a
  bare shell and threw in CI under `Set-StrictMode -Version Latest`.
