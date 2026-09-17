# Process census — the `--e2e` routine-arm leg

Taken before and after the leg, as `.claude/rules/verification-harness.md:58` requires ("the pre-leg baseline
is what makes the post-leg reading mean anything").

## Before the leg

| image | count | note |
|---|---|---|
| `msedgewebview2` | 6 | all at StartTime `2026-09-16 18:33:31` — the set CARRY 3 carries |
| `msedgedriver` | 0 | |
| `tauri-driver` | 0 | |
| `conductor-tauri` | 0 | |
| `node` | 0 | |

## After the leg

| image | count | note |
|---|---|---|
| `msedgewebview2` | 12 | 6 at `2026-09-16 18:33:31` + 6 at `2026-09-17 18:04:15` |
| `msedgedriver` | 0 | |
| `tauri-driver` | 0 | |
| `conductor-tauri` | 0 | |
| `node` | 0 | |

The driver stack tore down completely — wdio's `onComplete` → `tauriDriver.kill()` held, matching the
pre-leg baseline exactly for every image the leg starts.

## The `msedgewebview2` delta, resolved by PARENTAGE

A count and a StartTime cannot attribute a `msedgewebview2` process: every WebView2-hosting desktop
application on Windows spawns processes under that one image name, which is exactly the ambiguity CARRY 3
warns about ("a live leg's own children are indistinguishable from these by name alone"). Parentage is the
discriminator, and no prior census took it.

Measured via `Win32_Process` (`ProcessId`, `ParentProcessId`, `CreationDate`), each parent resolved to its
own image name:

| root pid | root parent image | children | created |
|---|---|---|---|
| 33580 | `SearchHost.exe` | 5 | `2026-09-16 18:33:31` |
| 31484 | `WhatsApp.Root.exe` | 5 | `2026-09-17 18:04:15` |

Every one of the twelve is a child of an ordinary desktop application — Windows Search and WhatsApp — and
**none descends from `tauri-driver`, `msedgedriver` or `conductor-tauri`**.

## What this establishes

1. **This leg left ZERO `msedgewebview2` survivors of its own.** The six that appeared mid-run arrived at
   `18:04:15` while `cargo` was still compiling, before any driver session existed, and are WhatsApp's.
2. **The six CARRY 3 carries are not orphans at all.** They are live children of a running Windows Search
   host, not residue from any Conductor leg.
3. The predecessor wrap's conclusion — "the defect is NOT the routine arm's teardown path" — is
   **confirmed, and now on a mechanism rather than on timing**. On this evidence there is no orphan defect
   in the routine arm to fix, because none of the processes attributed to it were ever its.

CARRY 3 is the route's to dispose of, not this chunk's; this is the measurement it asked for, with the
discriminator it named as missing.
