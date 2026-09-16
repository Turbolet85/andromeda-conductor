# Integrity level is the discriminator — established by variation with a control on both sides

**Established:** 2026-09-16 · **Three legs, one host, WebView2 runtime 153 throughout**
Legs A and B run by the operator from one elevated shell; leg C from a normal shell.

## The measurement

| Leg | Administrator role | Integrity | Session | Elapsed |
|---|---|---|---|---|
| A — direct, elevated | admin | **High** | **NO** — `DevToolsActivePort file doesn't exist` | 2:00 |
| B — under `runas /trustlevel` | **False** | **High** | **NO** — `DevToolsActivePort file doesn't exist` | 2:00 |
| C — normal shell | False | **MEDIUM** | **YES** — `[webview2 153.0.4234.32 windows]` ×35 | 0:06 |

**A→B varies the ROLE with integrity held High.** No change — the role is exonerated.
**B→C varies INTEGRITY with the role held False.** The session appears.

One variable moved at a time, a control on both sides of each. This is the project's own bar, met on
this question for the first time.

## What it establishes

**The blocker is the mandatory integrity level, not the administrator role.** A High-integrity process
does not get a WebView2 debugging endpoint on this host; a Medium-integrity one does, with the role
identical across both.

Every prior reading is now one explanation rather than an accumulation of dead ends:

- `RunLevel Limited` on the scheduled task **could not** lower the label — RID 500 has no split token,
  so there was no filtered token to drop to, and the task ran High.
- `runas /trustlevel:0x20000` **could not** lower it either — Basic User strips the Administrators
  group and leaves the mandatory label untouched, which is exactly what probe 35111618735 measured
  (role False, integrity High) and what leg B reproduces locally.
- The runner is High in every run. Both routes were coherent with the cause and neither could reach it.

## What it does NOT establish — keep these apart

**Integrity buys the SESSION, not a green arm.** Leg C created a session and the spec still FAILED:
0 passed, 1 failed, 2 skipped in 6 s, because `:384` and `:397` are independently broken by WebView2
153 on this host. Those are two different questions and conflating them is how this terminal earned its
reputation:

1. **Does a session get created?** — answered: yes at Medium, no at High. Integrity decides it.
2. **Does the arm go green?** — unanswered here. It additionally requires `:384`/`:397` to pass, which
   the 2026-09-10 record says they did at runtime **152** — the major CI currently runs.

A green CI arm therefore needs BOTH: a medium-integrity launch (not yet built) AND those two assertions
holding at the runner's runtime. Neither implies the other.

## A worry retired

Runtime 153 did **not** break session creation — leg C's session came up *on* 153. So 153's damage is
confined to `:384`/`:397`, and the dev-host control is alive again for the endpoint question. The
earlier framing of 153 as the live suspect for the endpoint is withdrawn.

## What this makes of the remaining option

An explicit medium-integrity launch — `CreateRestrictedToken` plus `SetTokenInformation` setting the
mandatory label to the Medium SID, then `CreateProcessAsUser` — is the mechanism this measurement says
would work. It now rests on a measured premise rather than on elimination, which is the condition under
which it was worth writing at all.

The window-station axis remains unvaried. It is no longer the last candidate standing — integrity is
measured, not inferred — but nothing here excludes it as a *second* contributor on the runner, where
the session context differs from this host's.
