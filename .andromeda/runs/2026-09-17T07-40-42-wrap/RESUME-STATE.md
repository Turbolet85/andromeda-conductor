# RESUME STATE — wrap of 2026-09-16-medium-integrity-launch-for-the-a11y-routine-arm

Written at the 85 % context alarm, mid-P2. **Nothing is committed. HEAD is `139bbb1`. The chunk record is
still `pending`.** The working tree is exactly as implement left it plus this wrap's P1/P2 artifacts.

## Where the wrap is

| phase | state |
|---|---|
| Setup | DONE — 1 pending chunk, branch `build/conductor-0.3.0`, 1 ahead, run dir `.andromeda/runs/2026-09-17T07-40-42-wrap` |
| P1 report | **DONE** — `conductor-0.3.0/chunks/.../report.md`, evolve record appended |
| P2 fan-out | **DONE** — 7 doc-agents, 29 proposals, consolidated in `fanout-results.md` beside this file |
| P2 validate | **PARTIAL** — check 1 (re-derivation tell) cleared; checks 2–6 NOT run |
| P2 escalate | **BLOCKED — four operator rulings outstanding** |
| P2 apply + cascade | NOT STARTED — must not start before the rulings |
| P3 curation · P4 graph · P5 route · P6 state · P7 gates/commit | NOT STARTED |

The code-graph refresh was fired in the background at Setup; check `.andromeda/cache/.refresh-done` /
`.refresh-stale` at P4.

## The four rulings that gate everything

1. **`≥152` floor falsified as a posture.** Not necessary (coherent 131/131 works), not sufficient
   (coherent 152/152 fails, run 34654076633), and it DESTROYED the working configuration (run 35185153012).
   Measured subject is driver↔runtime major COHERENCE. Proposed: replace the floor with a coherence
   assertion. Sites: `security-plan.md:185` + `:86` + `:87`, `architecture.md:148`.
2. **Spec↔reality gap:** the float's exit condition names a mechanism that does not exist — the Standalone
   Installer is Evergreen and takes no version; only Fixed Version is versioned (>250 MB,
   `WEBVIEW2_BROWSER_EXECUTABLE_FOLDER`, latest/second-latest majors only, so 131 unobtainable). Site:
   `security-plan.md:185`.
3. **The seventh governed spawn form** no longer describes what ships — retired, or retained scoped to the
   dev-only path? Sites: `security-plan.md:363`, `a11y-plan.md:471`, `architecture.md:225`/`:246`.
4. **A second non-loopback egress** (`msedgedriver.microsoft.com`). Sites: `architecture.md:148` + `:254`,
   `security-plan.md:86`.

## THE HARD GATE — four unratified surfaces in ci.yml

These must NOT ride the commit on their own authority. Either rulings 1/3/4 land and the comments are
rewritten to cite them, or the code comes out before P7.5:

- `runs-on: windows-2025` → `windows-2022`
- the `≥152` floor bypassed on `win22`
- the driver-pin step fetching from `msedgedriver.microsoft.com`
- **the launcher's removal from the asserting step** — this chunk's own deliverable, and the one that
  contradicts its own acceptance criterion

(The wrap directive named two of these; the diff has four.)

## Directive items still owed

- **P5:** mint the successor NOW, not a carry — subject: correct `:384`'s counting basis (visits vs
  distinct) so the assertion measures reachability rather than one environment's wrap behaviour. It owns
  `v3-02`.
- **P7.1:** the red's verdict word — `red — not this chunk's: :384 SC 2.1.1 counting-basis defect, its own
  failure message showing the property holding (identical bracket lists, 6 expected vs 12 received) →
  owner: the successor minted at P5`.
- **P7.3:** `v3-02` cannot flip — **un-claim** it (`matrix.py unclaim`), do NOT refine the acceptance
  downward, and add a `notes` line recording that the exclusion arm is DEAD on measurement.
- **P2 apply:** `residuals.md`'s entry is premise-corrected — wrap P5 owns that rewrite, tagged
  `[premise-corrected: …]` at the line's end.

## Facts to preserve across the compact

- Terminal NOT reached. Run **35192876641** opened the first hosted-runner WebView2 session in this saga
  and ran the arm: **11 passing / 1 failing / 2 skipped**; `:397` PASSES on 131.
- The single red `:384` is a **counting-basis** defect: expected and received bracket lists identical to
  the character, only the prefix differs — **"12 visits / 6 distinct"**, never a bare count.
- Working configuration: windows-2022 · native runtime 131.0.2903.86 · driver pinned 131.0.2903.86 ·
  **High integrity, no launcher**. Integrity's SIGN is configuration-bound — the predecessor's legs A/B/C
  are BOUNDED, not retired.
- Seven ci-probe refs created, all seven deleted — `git ls-remote --heads origin 'ci-probe/*'` empty.
- Six `msedgewebview2` on the dev host with `StartTime 18:33:31` predate this session — the predecessor's
  carried survivors (CARRY 3), not this run's.
