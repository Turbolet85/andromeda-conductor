# Attempt ledger — the 2026-09-30 real-model series

One row per drive fired, in firing order, under `contracts/pulse-real-model-leg-posture.md` §The 2026-09-30 series
(fixed before `d1`). Every drive is recorded; a canary-blocked drive is a measurement, never graded. Authored by
the agent from each drive's capture copy, Pulse's own log and its census; no Pulse file is edited.

## Gate B (plan entries 18-19, fired by hand 2026-09-30, read at Pulse's committed HEAD, no fetch)

- Part 1, a scrubber commit on Pulse's pushed branch after take-up HEAD `a08ae29`: **1** —
  `fcc31b2` (`fcc31b21666df70f3bbbaf124c4a6cd8597fa586`), "chore(2026-09-29-scrubber-path-false-positive): operator
  pre-CI commit", committed 2026-09-30T06:58:34+02:00, on `chore/migrate-pulse-to-v3`.
- Part 2, commits on Pulse's checked-out branch not on its upstream: **0** (HEAD = `@{u}` = `fcc31b2`, read
  against the local tracking ref).
- The diff, read with `git show fcc31b2 -- crates/security/src/scrubber.rs`: the `credit_card` arm's regex
  `\b(?:\d[ \-]?){13,19}\b` ("Doesn't check Luhn") becomes the candidate regex `\b[0-9]+(?:[ \-][0-9]+)*\b` with a
  predicate that matches only when some window of whole separator groups carrying 13-19 digits passes Luhn
  (`has_luhn_valid_window`, `luhn_valid`). Confirmed as the Luhn fix. The same commit's
  `crates/interpretation/src/markdown.rs` change is a test only.

## The binaries (built from `fcc31b2`, clean worktree, before `d1`)

The regex SOURCE is string data in each binary, so presence is a content proof. Probe: a byte count of the old
and the new `credit_card` pattern.

| Binary | Built | sha256 | old pattern | new pattern |
|---|---|---|---|---|
| `pulse-app.exe`, before (from `e98d838`, the control) | 2026-09-29 17:48 +02:00 | `dc3fca88efbaf6a5e0f0e63f79d8e7ef2c5159f5165fb8e7fcb6ad80a6f13950` | 1 | 0 |
| `pulse-app.exe`, `cargo build --release -p pulse-app` | 2026-09-30 07:15:58 +02:00 | `6aed4a9ae974c01ca7db17542df044e0c03e679004d8397fb5fd16f8dc301b47` | 0 | 1 |
| `andromeda-pulse-mcp.exe`, `cargo build --release -p mcp-server --bin andromeda-pulse-mcp --features mcp-server` | 2026-09-30 07:17:07 +02:00 | `2179caab9f7247a52cba867d52e0c7de14472dbc642433ca3d27bf56c34cc634` | 0 | 1 |

The sidecar was rebuilt too: it renders the report the drive grades through `assemble_report`, which scrubs the
workspace and every model-written field with the linked scrubber.

## The dir (plan entry 20)

`%TEMP%/pulse-legs` before the leaf was created held 13 entries (`20260901T160708Z`, ten `a11y-*`,
`huegradedhard`, `rm-20260923-093840`); `rm-clean-series` was absent.

## Pre-registration (plan step 11)

The contract section §The 2026-09-30 series — LF-normalized, from its heading line up to the next `## ` heading —
recorded before `d1` fired:

pre-registration sha256: 0091fe6f876d05dfcaa4d454320a31426927d94cbcf13fe6d8095070a0753c19

## The drives

| Label | Slot | Run | Route | Grade (P-033) | P-031 / P-034 / P-044 | `canary:` tokens | Key rendering | Census |
|---|---|---|---|---|---|---|---|---|
| **d1** — `rm-capture-d1.txt` | overseer grant for d1-d3 back to back (2026-09-30); `pulse-app` launched by the agent 05:19:04.645Z (PID 31216) from `fcc31b2`, posture read from its log: `inference_mode real`, model `loaded`, workspace basename `rm-clean-series`, no bootstrap override | preconditions probe exit 0 at 05:19:42Z; fired 05:19:47Z, ended 05:33:46Z; run `2026-09-30T05-19-51-573` | `PreflightBlocked` (`emission: none`; envelope `Blocked` / null) | not graded — canary blocked | Blocked: no attributable incident | `dismissed` ×3 (storms at 05:20:36Z, 05:22:06Z, 05:23:36Z, each tier autonomous, parse `ok`, no incident outcome); Pulse's log over the window: `incident.created` 0, `inference.error` 0, `inference.skipped` 0 — MODEL-SIDE, not a pipeline fault, so not re-fired | no report read; the leaf occurs 0 times in the capture | after: only the agent's `pulse-app` tree; no `conductor` / sidecar survivor |
| **d2** — `rm-capture-d2.txt` | the same grant; the same `pulse-app` launch (PID 31216) | quiet window: no incident formed by d1, so the 90 s floor (last canary 05:23:36Z) held; preconditions probe exit 0 at 05:34:25Z; fired 05:34:25Z, ended 05:50:45Z; run `2026-09-30T05-34-27-824` | preflight READY, the scenario emitted (emission instant 05:38:17.991Z); `attribution: none within 600s` over 61 polls and incidents 1-5, none carrying the scenario's fingerprint | not graded — **scenario-side dismissal**: Pulse's log shows the scenario's storm Autonomous at 05:38:23.090Z, its tier-1 digest prompted at 05:38:23.105Z and parsed `ok` at 05:38:27.164Z, then NO incident outcome before the next prompt (05:39:05Z) — the model dismissed the scenario's own digest. The 05:39:19Z `deduped=true` outcome belongs to a tier-2 `error_rate_spike` digest (05:39:14Z). `inference.error` 0, `inference.skipped` 0 | Blocked: no attributable incident | `surfaced` (05:35:12Z, incident created 05:35:17Z), `dismissed` (05:36:42Z), `surfaced` (05:38:12Z, created 05:38:17Z) — each tier autonomous, parse `ok`; 1 other cue-bearing digest kind (`error_rate_spike`) | no report read; the leaf occurs 0 times in the capture | after: only the agent's `pulse-app` tree |
| **d3** — `rm-capture-d3.txt` | the same grant; the same `pulse-app` launch (PID 31216) | quiet window: d2's last incident created 05:49:10.217Z, active set 0 from 05:51:32Z, fired 158 s after it; preconditions probe exit 0 at 05:51:54Z; fired 05:51:55Z, ended 06:08:14Z; run `2026-09-30T05-51-57-039` | preflight READY, the scenario emitted (emission instant 05:55:42.179Z); `attribution: none within 600s` over incidents 1-9, none carrying the scenario's fingerprint | not graded — **scenario-side dismissal**: the scenario's storm Autonomous at 05:55:47.291Z, its tier-1 digest prompted at 05:55:47.307Z and parsed `ok` at 05:55:52.026Z, then NO incident outcome before the next prompt (05:56:05Z). `inference.error` 0, `inference.skipped` 0 | Blocked: no attributable incident | `surfaced` (05:52:42Z, created), `surfaced` (05:54:12Z, deduped into the open canary incident); the third storm's tick (05:55:42.191Z) fell at the emission instant, outside the pairing window; 4 other cue-bearing digests (`error_rate_spike`) | no report read; the leaf occurs 0 times in the capture | after: the agent stopped `pulse-app` (PID 31216 + its 10 descendants; forced after `CloseMainWindow` did not exit within 15 s); `:4317`/`:4318` released; census back to the pre-series baseline (six `msedgewebview2.exe` from 2026-09-26, not this series') |

## Series verdict (the pre-registered rule, applied mechanically)

Three drives fired, none re-fired, no fourth. **Graded drives: 0.** d1 is canary-blocked (a measurement, three
model-side dismissals); d2 and d3 reached ready and emitted, and in each the model dismissed the scenario's own
retry-storm digest (parse `ok`, no incident outcome), so neither attributed an incident. Under §The drive series
(a) — carried unchanged into §The 2026-09-30 series — a series that ends with zero graded drives leaves **`v3-09`
NOT MET**. Recorded, never replaced by a further drive.

What the series did measure: the scrubber fix removed the cause the 2026-09-29 series recorded — no capture reads
back `[redacted: credit_card]`, and no report text was read at all. Across the three drives, read from Pulse's
own log, the real model decided nine canary storm digests — 4 surfaced, 5 dismissed (3 in d1; d2's second; d3's
third, whose tick fell at the emission instant and so carries no `canary:` line: prompted 05:55:42.199Z, parse `ok`
05:55:46.522Z, no outcome before the next prompt) — and the scenario's own storm digest twice, dismissed both times.

## Status smoke (plan entry 25), fired by hand after d3

`bash scripts/agent-run.sh status 2026-09-30T05-51-57-039` (d3's run_id): exit 0; atom
`contains "scenario": "real-model-interpretation"` held (1 hit); the envelope read back that `run_id`, `seed`
4317033, `state` `ManualCheck`, `verdict` null.
