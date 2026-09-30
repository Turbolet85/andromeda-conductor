# Operator legs — 2026-09-30-the-screen-reader-content-findings-fixed

The `leg = 'operator'` entries of `plan.md` §Test Commands, driven by hand by the implementing session on the
operator's quiet-desktop slots (every slot asked first, stating its length; the live slot on the operator's
`:4317` grant). The graded record is `nvda-pass.json` beside this file; its `subjects.*.clock` carries the
per-key pair table the clock calibration used.

**Configuration of the record:** WebView2 154.0.4258.37 × msedgedriver 154.0.4258.37 (coherent, gate 6) ×
NVDA 2026.2 (the operator-supplied portable copy, passed per invocation) × Windows 26200.9457 × the OS input path.
Bundle built by the strict `--e2e` arm on the chunk base `d7da5d0` plus this chunk's working tree.
pulse-app: the Pulse `5fbf762` release binary, sha256
`e3a65f3cc0cade02c804372a1f6829312702109ff147d591dfe9caa8ffe109dc`, launched by this session on the
operator's word for each live leg on a fresh data dir (`pulse-legs/srcontent`, `srcontenttwo`,
`srcontentthree`), posture confirmed from its own log (`inference_mode: deterministic`, the workspace key
naming the dir, the OTLP bind on loopback 4317/4318), and stopped by this session after each capture.

## Entries (final configuration, 2026-09-30)

| # | entry | exit | atoms read |
|---|---|---|---|
| 13 | `rm -f runs/sr-leg/nvda-pass.json` | 0 | the record absent afterwards |
| 14 | `npm run a11y:sr-empty` | 0 | `Spec Files:` TAB ` 1 passed, 1 total` (19:54:15Z) |
| 15 | `npm run a11y:sr-error` | 0 | `Spec Files:` TAB ` 1 passed, 1 total` (19:54:42Z) |
| 16 | liveness probe (sidecar on PATH, `:4317` answering; never `boot`) | 0 | `sidecar on PATH` · last line `True` |
| 17 | `npm run a11y:sr` (live, the firing form's handles + the Pulse release dir on PATH) | 0 | `Spec Files:` TAB ` 1 passed, 1 total` (20:01:25Z) |
| 18 | copy `runs/sr-leg/nvda-pass.json` here | 0 | the operator review then transcribed into it |

## Slot history (each slot's outcome, in order)

1. Strict `--e2e` — green: 19 passing, 2 skipped (the expected two), the three new routine-arm specs passing
   (row names read through the driver's own computed-label endpoint).
2. sr-empty + sr-error — neither gradable. sr-empty VOID: NVDA started while a terminal held the foreground and
   never bound the Conductor window (its log: "Foreground took too long to change … Should be … (Tauri
   Window)"). sr-error: harness defect #1 — the injected reset cycle ran its 10-Tab cap without a BODY stop.
   Fixed in the spec (an in-page reset when the cycle ends off BODY).
3. Re-fire, same pair — both bound; E0-01 re-positioned after it walked the report table from BODY.
4. sr-empty, then live leg #1 on `srcontent` — live harness defect #1 at S2-07 (Shift+Tab from NVDA's browse
   caret on the roll-up stays on the checkbox) plus an NVDA log-clock step of ~2.5 s over keys 8–29. Fixed:
   Tab ×2; a held test-only caret target; the step-aware clock calibration in `parse-nvda-log.ts` (operator's
   word, `scope-record.md`), proven on the saved logs first — the aligned records re-parsed byte-identical, the
   stepped live log moved S0-01…S1-04 onto their own speech, and a doctored 500 ms pair voided the session.
   The footer separators made plain text (NVDA had read "conductorseed —idle").
5. `--e2e` rebuild, sr-empty + sr-error, live leg #2 on `srcontenttwo` — 48 of 51 as expected; S0-11/S0-12
   silent before NVDA binds the document at the first OS Tab, S1-02 heard the bare "0" (browse mode never reads
   a role-less span's aria-label). Fixed: the rows re-positioned after S0-02; the count's name moved into
   visually-hidden text (plan step 9's named remedy).
6. `--e2e` rebuild, sr-empty + sr-error, live leg #3 on `srcontentthree` — the record: 49
   announced-as-expected, 2 subject-absent, every clock aligned (48/3/71 pairs, max excess 230/33/233 ms,
   nothing shifted or void). The operator declared this the last full re-run of the chunk.

## Process census (after the last leg)

| process | started by | final state |
|---|---|---|
| `nvda.exe` | each sr leg | terminated (the harness's `nvda -q`) |
| `conductor-tauri.exe`, `msedgedriver.exe`, `tauri-driver` | each leg's driver stack | terminated (`onComplete`) |
| `pulse-app.exe` (pids 25220, 58556, 55128) | this session, on the operator's word | terminated by this session (matched by pid and creation time; `CloseMainWindow`, then `Stop-Process -Force` after 15 s) |
| `andromeda-pulse-mcp.exe` | the app, per run | terminated with it |
| `msedgewebview2.exe` created during the session | the legs' apps | none survives |

Loopback after the last leg: no listener on 4317, 4318, 4444 or 4445.

## Operator pass (2026-09-30, on the operator's word; no window opened)

| # | entry | exit | atoms read |
|---|---|---|---|
| 30 | `gate.py hygiene` | 0 | `hygiene: clean — read 32 (runs 30 · evidence 2) · trails 12 not read · binary 0 not read by P1` |
| — | the operator pre-CI commit | 0 | `dfa4f87` — `chore(2026-09-30-the-screen-reader-content-findings-fixed): operator pre-CI commit, for the run this chunk's verdict reads` |
| 31 | the guarded push | 0 | `PUSHED_SHA=dfa4f87e7e98cddae3e477966b0f8aadc5b906af` (`d7da5d0..dfa4f87` on `build/conductor-0.3.0`) |
| 32 | `ci.py conclusion --sha HEAD --wait 1200` | 0 | `dfa4f87e7e98 verdict: green · checks 3/3 · wall 566 s · runs CI#36770454038 completed/success` |
