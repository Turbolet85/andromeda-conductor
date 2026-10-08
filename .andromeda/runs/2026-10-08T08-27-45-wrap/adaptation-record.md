# 0-pending wrap — route adaptation on the founder's two rulings (2026-10-08)

**Path:** Setup step 6, the no-op path, an operator-requested route adaptation. 163 master records, all
`complete`; 0 pending, 0 gated (`route.py cursor` and an anchored status-word grep agree). The tree at entry was
dirty only with bookkeeping: `friction-log.ndjson` (one record), the handoff's session-end stamp, and the prior
wrap's evolve trail (a tracked run-dir file the trailing friction append extends after its commit).

**The request,** as this wrap's invocation arguments give it (2026-10-08), with the pc overseer's relay
`conductor-wrap-0pending-pairing-2026-10-08.md` read first as they direct: the founder ruled by dialog today.
The capture pairing-window race is fixed in 0.3.0 as a separate small chunk before the version close; the model
statement in the frozen capture-run report stands as is. Mint ONE entry before the version close for the fix
and move that CARRY into it; resolve the model-statement CARRY on his word; leave the other two CARRYs; change
nothing else.

**Authority,** in the form of `gate-contract.md` §Scope, *Whose word*:
- word: "Починить в 0.3.0" — the founder, 2026-10-08, relayed verbatim by the pc overseer (the relay's §1; the
  option he picked: a separate small chunk before the close, the harness change and its tests, no GPU).
- word: "Оставить как есть" — the founder, 2026-10-08, relayed verbatim by the pc overseer (the relay's §1).

Both are his own picks passed on whole, so neither is provisional. The direction names the entry and its
disposition, which satisfies the trajectory gate: no halt, no dialogue round.

## The relay's state claims, re-measured here

| Claim | Reading at this wrap |
|---|---|
| HEAD equals the upstream, `d54751f` | `git rev-list --left-right --count @{u}...HEAD` reads `0 0` after a fetch; HEAD `d54751f` |
| CI run `37745625899` green on it, 3 of 3 jobs | `gh run view`: `completed`, `success`, head `d54751f`, three jobs each `success` |
| the ledger reads 11 of 11 `verified` | `matrix.py coverage`: `verified 11/11 · deferred 0 · planned 0 · implemented 0 · unclaimed 0` |
| the only entry left is the version close | `route.py markerless`: 1 of 43 entries, `working-route.md:100`, 4 freight blocks |

## The facts the new entry carries, each read at its source

| Fact | Source read |
|---|---|
| the classifier pairs only lines stamped before the emission instant | `crates/conductor-run/tests/real_model_live.rs:723-727`, the filter `at < emitted` feeding `canary_attempts` (`tests/real_model_common/mod.rs:73`) |
| d1 and d2: ticks 4 ms and 2 ms before the instant, prompt assemblies 1 ms and 2 ms after it | the sixth series' `evidence/attempt-ledger.md:258-259` (d1 tick `.351`, instant `.355`, prompt `.356`; d2 tick `.851`, instant `.853`, prompt `.855`) |
| each storm parsed `ok` and deduped, no inference error, no skip | the same table, and `:246` |
| d3: tick 11 ms after the instant, two `canary:` lines and no third | the ledger's `:260`; `rm-capture-d3.txt` holds 2 lines opening `canary:` (d1 and d2 hold 3 each) |
| the contract carries one earlier correction of this pairing | `contracts/pulse-real-model-leg-posture.md:316`, `[corrected 2026-09-29 …]`, inside the bullet "The canary classification" (`:306`) |

One reading for whoever builds a fixture from the captures: a bare count of the token `pipeline-fault` over a
capture is not a count of printed `canary:` tokens. Each capture also prints the test source's own constant
line (d3 reads 1 hit of the token and 0 `canary:` lines carrying it).

## Items and dispositions

| # | Subject | Disposition |
|---|---|---|
| 1 | a new entry for the pairing-window fix | Minted at `working-route.md:100`, markerless, ahead of the version close (now `:102`). |
| 2 | the pairing-window `CARRY` on the version-close entry | Moved onto the new entry, re-authored with the ruling and its bounds. Off the version-close entry. |
| 3 | the model-statement `CARRY` on the version-close entry | Rewritten to the ruled state: the close states one known exception. No quote. Dated ledger note on `v3-09`. |
| 4 | the two other `CARRY` blocks (the script comments; architecture's byte threshold) | Byte-identical, compared against `HEAD`'s line. |

### 1 and 2 — the new entry

- **Text:** `Capture canary pairing-window corrective — a canary storm ticking milliseconds before the emission
  instant prints what Pulse's own log bears out, never a false pipeline-fault`. 24 words, an outcome, no
  implementation verb. One `CARRY:` block of 2091 chars follows it.
- **The block** keeps the measured mechanism with its pointer inline (the ledger section and the three
  captures), adds d3's reading, the founder's word, the bounds his pick sets (the harness change and its tests;
  no GPU and no new live series; committed captures keep what they printed; the decision rule and every
  pre-registered section byte for byte; the rule counts emissions and never reads these tokens), and the
  contract coordinate where a second dated correction goes, add-only.
- **One labelled hypothesis** inside the block, as the relay gave it: the fix can be proven on recorded log
  lines without a live drive. The phase decides; if a live drive is the only honest proof it stops for the
  overseer's word, and a GPU run needs the founder's new word.
- **Form:** route-contract §Write row 17, one anchored Edit on `   ↓` + the version-close title. No annotation
  had to move with the insertion: the previous first markerless entry carried no `PREREQ:`, `WATCH:` or
  `BLOCKED-ON:`.

### 3 — the model-statement ruling

- The block on `:102` (679 chars) now says what the close itself does: state one known exception to the scope
  `.andromeda/security-plan.md:338` sets (model text is committed only inside a chunk's `evidence/` tree), name
  the report, and quote nothing. The frozen chunk is left byte for byte.
- **Ledger:** `matrix.py note --id v3-09`, +1081 chars, status `verified`, ref and acceptance untouched; read
  back with `matrix.py show`. The note also says the 2026-10-08 note's "the next route entry is the version
  close" is superseded on that point alone. Coverage after it: `verified 11/11 · deferred 0`.
- No master was amended. The security plan's own statement of the exception is the version close's work, which
  is what the block carries.

## Read-back

- `git diff --numstat` on the working route: 3 added, 1 deleted. 100 lines to 102; lines 1-99 byte-identical
  to `HEAD`; the file stays LF (`i/lf w/lf`), no CR.
- `route.py pins`: 4 blocks, one on `:100` and three on `:102`. `route.py markerless`: 2 of 44 entries.
  `route.py cursor`: next is `working-route.md:100`, half-promote 0.
- `route.py epoch`: Epoch 5b holds 8 entries (6 frozen complete, 2 markerless). Under the growth valve's
  threshold, so nothing was surfaced.
- `gate.py hygiene` on this run dir: `clean`, 0 host paths.

## A decision made here, the operator's to overrule

**No requirement was added.** The relay says "no new requirement beyond this fix" and the invocation says
"change nothing else"; read together with route-resolve's requirement step these could mean either an
entry-only chunk or a new `requirements.md` line with its ledger entry. Decided entry-only, on the route's own
record: 44 entries stand against 11 capabilities, the earlier correctives and the capture run claimed none, and
no earlier adaptation commit in this repository carries a `Requirement:` line. `requirements.md` is unedited
and the ledger still holds 11 entries, so the new chunk claims no capability and the done-test stays true. If
the founder wants the fix counted as a capability, that is one `matrix.py add` at a later wrap.

## What did not change

- No entry was reordered or retired, and no second entry was minted.
- `intent.md`, `requirements.md`, every master, every frozen chunk and every committed capture.
- The `master-route.md` records: this path makes no master write.

## Other step-6 duties

- **Gated re-check:** 0 `gated` records.
- **`BLOCKED-ON` on the tail:** none stands (`route.py pins` reads `CARRY:` rows only).
- **Curation of this session's own conversation:** no correction and no new learning; nothing written to any
  tier.
- No consolidation, supersession, registry migration or master amendment ran.
