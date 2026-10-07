# 0-pending wrap — operator-requested route adaptation (2026-10-07): a capture run ahead of the sixth series

**Path:** Setup step 6, the no-op path with P5's operator-requested adaptation. 161 master records, all `complete`;
0 pending, 0 gated (`route.py cursor`). The tree at entry was dirty only with the handoff's session-end stamp.
Branch `build/conductor-0.3.0`, 0 ahead of `origin/build/conductor-0.3.0` at Setup (HEAD `3afe4f6`).

**The request:** the pc overseer's relay `conductor-wrap-capture-2026-10-07.md` (founder-delegated; read first, as
the wrap's arguments direct). One new route entry ahead of "A sixth pre-registered real-model series for `v3-09`":
a three-drive capture run that records the prompt each drive's model received. Not a `v3-09` verdict.

## Whose word

| Item | Word | Authority |
|---|---|---|
| The builder must be able to read the digest itself | «единственный вариант переделать так чтоб билдер сам смог прочитать дайджест» | the founder, 2026-10-07 13:13 local, by dialog, relayed verbatim by the pc overseer |
| The entry, its place at the head, its shape | relay §3 and §4 | delegate pc overseer, 2026-10-07 — PROVISIONAL (trajectory is the founder's fork) |
| A capture, never a `v3-09` verdict | relay §3 | delegate pc overseer, 2026-10-07 — PROVISIONAL |
| Captured prompts stay on this host; committed evidence is derived facts only | relay §3 | delegate pc overseer, 2026-10-07 — PROVISIONAL |
| The vehicle stays open between (A) and (B); no launch before the founder's own word | the answer to this wrap's escalation, below | delegate pc overseer, 2026-10-07, given here |

The trajectory gate (a new chunk ahead) was satisfied by the relay as a recorded pre-direction naming both the
entry and its disposition. It is a delegate's word on a founder-owned fork, so it stands PROVISIONAL and the
founder's later word supersedes it.

## The relay's facts, re-derived at this wrap

| Relay claim | Reading here | Basis |
|---|---|---|
| Pulse's reproduction read NOT REPRODUCED, 199 of 200 | holds: 200 rows, 10 shapes × 20, all arm `shipped`; `identifies` = `both` 199, `signal_only` 1. `names_trigger` = `rank1` on 200 of 200, a different field from the one the 199 counts | Pulse `target/l4-decision-probe/repro-20261007T104447Z/runs.json`, counted in python |
| Nothing is committed in Pulse; no remedy exists | holds: Pulse master carries the chunk `pending`; its chunk folder and two run dirs are untracked; `pulse-app/examples/l4_decision_probe.rs` is modified, uncommitted | `git status --short` in the Pulse repo; its `master-route.md:103` |
| Pulse HEAD `48714f0` differs from `f70be92` in docs and route only | holds for committed state: `git diff --stat f70be92 HEAD -- crates pulse-app xtask` is empty. The WORKING TREE differs in the one example file above, so a build of the tree is not a build of `f70be92` | git, in the Pulse repo |
| The prompt reaches the model binary on argv, `-p`, `:491` | holds: `"-p".to_string()` at `pulse-app/src/llamacli_inference.rs:491`, the prompt the next element; the file is unchanged since `f70be92` | grep; `git diff --quiet f70be92 -- {file}` exit 0 |
| The binary path is canonicalized and asserted a regular file (`:285`, `:599-626`) | holds: `binary.is_file()` at `:285`; `validate_path_input` at `:606`, its `metadata.is_file()` at `:620` | grep |
| `ANDROMEDA_PULSE_L4_ALLOW_ROOT` is unset on this host | holds for THIS wrap's shell (unset). Not a reading of the shell that will launch `pulse-app` | the Bash tool's environment |
| The Pulse entry sits at `working-route.md:180` | holds; it is stamped `[2026-10-07-l4-probe-reproduces-the-canary-history-miss]` | read at this wrap |

## The escalation, and its answer

The relay reads the capture as "a widening of what a leg records … not a widening of what the repository holds"
and says to stop and ask if this project's Boundary-widening rule reads it otherwise. It does, on two clauses the
relay did not weigh:

- `.andromeda/security-plan.md:338` — corpus-rendered text is reached on the MCP tool surface only and persisted
  nowhere but scrubbed committed evidence ("anywhere else the ban binds unchanged"). The prompt carries the digest,
  which is corpus-rendered text, taken off the MCP surface and kept in the clear.
- `.andromeda/architecture.md:204` (§Occupied Resources) — `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` is registered as a
  handle Conductor "neither SETS nor READS". A Conductor-authored program named by that handle changes that.

The founder's verbatim word covers the end (the builder reads the digest). The vehicle is the overseer's, and a
boundary widening is ratified only by the founder's own word, given here or relayed verbatim
(`.claude/rules/security.md`, the 2026-09-29 entry; the gate contract's *Whose word*).

Put to the operator channel as one question with three dispositions (mint with the vehicle unratified · mint as
the relay reads it · hold the entry). **Answer, 2026-10-07, the pc overseer, given here:** mint with the vehicle
unratified, with this note, quoted whole:

> Your reading is right and mine was too narrow: security-plan:338 and architecture:204 both bind, and a delegate
> cannot ratify a widening. I am asking the founder by dialog now (he answers by phone). He gets two vehicles:
> (A) the pass-through is the OPERATOR's, set in the operator env file the launching shell sources, captures kept
> in the operator folder, so Conductor still neither sets nor reads the handle and nothing persists into a
> Conductor artifact, the leg only discloses the posture; (B) a Conductor leg feature with both clauses amended on
> his word. Write the CARRY so the vehicle stays open between them until his own word arrives; no launch before it.

Disposition: applied as answered. The entry's first `CARRY:` holds the vehicle open between (A) and (B) and bars a
launch until the founder's own word arrives. No master was amended at this wrap: nothing is widened yet, and this
path applies no drift-derived amendment.

## The edit

| | |
|---|---|
| File | `conductor-0.3.0/working-route.md` |
| Form | `splice.py append --after-line 95 --lines 2` (the entry, then its `   ↓` separator); payload `route-entry.txt` in this run dir |
| Result | 98 → 100 lines, 11909 → 16211 B; `git diff --stat`: 2 insertions, 0 deletions; 0 `[`-prefixed (frozen) lines in the diff |
| New entry | `:96`, 4295 B, sha256 `4820889c25f385de…` — "A capture run records the prompt the model received in each drive" |
| Freight on it | `CONTEXT:` (1992 chars) · `CARRY:` the open vehicle (1686) · `CARRY:` architecture §Occupied Resources bytes (368) — `route.py pins`, no `INDETERMINATE:` row |
| The sixth series | moved `:96` → `:98`, byte-identical (sha256 `b673348fc6ec2c21…` before and after); its `BLOCKED-ON:`, `CONTEXT:` and `CARRY:` unmoved |
| Cursor after | next `working-route.md:96`, markerless 3 of 43 entries |

Annotations that move with an insertion ahead of the first markerless entry are its next-entry `PREREQ`s and
`WATCH`es; the sixth series carried neither. Its architecture-bytes `CARRY:` binds whichever chunk next amends that
section, so the new head carries a copy with the origin kept and the sixth series keeps its own (relay §4).

## Re-verifications this path owes

- **Gated records:** none (`route.py cursor`: gated 0).
- **The sixth series' `BLOCKED-ON:`** (it sits on the tail this wrap touched): STANDING. Its clearing event is the
  overseer relaying the sha that ships Pulse's chunk; that chunk is `pending` and uncommitted in Pulse and its
  reproduction read not reproduced today, so no such sha exists. Its supporting clauses still read true (the entry
  at Pulse `working-route.md:180`, promoted in Pulse's working tree). Unedited.
- **Architecture §Occupied Resources:** 38115 B (`scripts/arch-registry-check.py measure --file
  .andromeda/architecture.md`; the tool prints 60.0% of cap). The bare `measure` form the sixth series' `CARRY:`
  quotes exits 2 — it needs `--file` or `--rev`; the new entry's `CARRY:` quotes the form that ran.
- **Epoch growth:** `Epoch 5b — Version close` is at 7 entries (4 frozen complete, 3 markerless); below the valve.

## Not done here, by design

- No requirement added, no `requirements.md` line, no matrix write: the capture serves `v3-09`, which stays
  `planned` and un-claimed.
- No master, sidecar, contract or source file changed.
- No launch, no shim written, no Pulse file read beyond the reads in the table above; no key read.

## Curation

One candidate, from this wrap's escalation: a proposed leg is checked against every boundary clause it touches
(the access channel, a handle's registered posture), not only against what the repository would hold. Filter 4:
verified by measurement +0.4, specific technical detail +0.2 = 0.6, exactly at the threshold, and the fact is
already carried at an annotation position on the new entry, so neither conditional signal applies. Rejected by the
lean default. No tier write; no `curation.md`.
