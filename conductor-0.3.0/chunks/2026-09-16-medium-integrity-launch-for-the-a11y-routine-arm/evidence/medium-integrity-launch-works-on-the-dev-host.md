# The medium-integrity launch works — dev host, measured at implement

**Chunk:** `2026-09-16-medium-integrity-launch-for-the-a11y-routine-arm`
**Date:** 2026-09-16 · **Host:** dev host (Windows 11, WebView2 runtime 153)

Host paths are DESCRIBED rather than quoted throughout: the project's host-path gate anchors on a
letter-colon-slash run, and a quoted path would trip it on content that is not a leak (`host-win32.md`,
2026-09-12).

## What was measured

The launcher `scripts/a11y-limited-token-launch.ps1` now builds its own token —
`CreateRestrictedToken` (LUA_TOKEN) → `SetTokenInformation(TokenIntegrityLevel = S-1-16-8192)` →
`CreateProcessAsUser` — instead of shelling out to `runas /trustlevel`. Driven with a fixed OS binary
(the system `whoami.exe`, addressed by absolute path) and the argument that prints the caller's groups,
so the CHILD reports its own mandatory label rather than the launcher reporting what it set:

```
[diag] A11Y_LIMITED_TOKEN_PARENT_INTEGRITY: S-1-16-8192
[diag] A11Y_LIMITED_TOKEN_CHILD_INTEGRITY: S-1-16-8192
[diag] A11Y_LIMITED_TOKEN_FORWARDED: 0 handle(s)
[precondition] A11Y_LIMITED_TOKEN_START: ok (CreateProcessAsUser at medium integrity)
Mandatory Label\Medium Mandatory Level      Label      S-1-16-8192
[diag] A11Y_LIMITED_TOKEN_LEG_EXIT: 0
```

Exit 0. Four facts land together: the token APIs are reachable from PowerShell 7 through `Add-Type` with
no new dependency; `CreateProcessAsUser` succeeded without `ERROR_PRIVILEGE_NOT_HELD`, because the token
is a restricted version of the caller's own primary token; the child's stdout reaches the launcher's
stdout (handle inheritance works, which is how the leg's printed verdict will reach the job log); and the
child's exit code propagates through the process handle, so the sentinel is genuinely a fallback now.

## What this does NOT show — stated so the report cannot overclaim

**The High→Medium LOWERING direction is unproven here.** The dev-host shell is already Medium
(`PARENT_INTEGRITY: S-1-16-8192`), so setting Medium on the child was a no-op in effect. An integrity
label may always be lowered and never raised, which is a documented rule, not a measurement taken here.
The runner runs at High, so the lowering direction is exactly what the CI probe measures.

**Nothing here is about the runner.** One host, one already-Medium starting point.

## The argv guard survives the mechanism swap

`CreateProcessAsUser` takes `lpCommandLine` as a string, so the form stays the COMPOSING class and the
whitespace/metacharacter guard is what keeps it outside spawn-rule (b)'s eval class. Both controls still
reject before composition:

| control | exit | printed |
|---|---|---|
| whitespace-bearing argument | 90 | `A11Y_LIMITED_TOKEN_ARGV: REJECTED` |
| metacharacter-bearing argument | 90 | `A11Y_LIMITED_TOKEN_ARGV: REJECTED` |

Both were GREEN at the phase P5 baseline on the untouched tree too — they are regression guards on an
existing property, not red-before-green. The launch entry was the red-before-green one: it timed out at
180s against the old `runas` mechanism (which detaches and waits on a sentinel the fixed OS binary never
writes) and is green at exit 0 against the new one.

## One defect found and fixed during implementation

`SetTokenInformation` first failed with `ERROR_NOACCESS` (998). Cause: assigning through a nested struct
property in PowerShell mutates a COPY, so `$tml.Label.Sid = …` left the SID null and the API read an
invalid pointer. The struct is now laid out by hand with `WriteIntPtr` / `WriteInt32`, which is the form
the phase's own feasibility probe had already used successfully. The comment at the site records the
mechanism so the next reader does not reintroduce it.

## Gate results at implement

| gate | result |
|---|---|
| launcher + fixed OS binary (child integrity) | green · exit 0 |
| argv guard, whitespace control | green · exit 90 |
| argv guard, metacharacter control | green · exit 90 |
| `cargo audit` | green · exit 0 |
| `cargo deny check advisories bans licenses sources` | green · exit 0 |
| driver-handle liveness probe | green · exit 0 |
| `cargo nextest run --workspace --profile ci` | green · **986 tests run, 986 passed, 0 skipped** |

The two Rust workspace gates were listed DEFERRED in the plan for zero compiled-source delta. That
deferral was **voided at implement** by the plan's own stated rule: `conductor-0.3.0/verification-matrix.json`
is uncommitted (the phase P5 claim wrote it) and `crates/conductor-report/tests/matrix_ledger_gate.rs:140`
reads that file at runtime, so a Rust test does read an uncommitted file and the gates had to run. They
did, and they are green.
