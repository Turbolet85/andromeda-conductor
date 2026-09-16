# The limited-token remedy is structurally unavailable on the hosted runner

**Established:** 2026-09-16 · **By:** direct variation with a control on both sides
**Runs:** probe `35105588216` (runner) · local dry run (dev-host control)

## What was asked

The CARRY named a limited-token launch as the next arm against the established elevation cause. Three
CI runs later the arm had never actually been exercised, and this record is the reading that says why.

## The measurement

| Reading | Hosted runner | Dev host (control) |
|---|---|---|
| Account RID | **500** | **1003** |
| Built-in Administrator | **True** | False |
| `EnableLUA` | 1 | 1 |
| `FilterAdministratorToken` | **ABSENT** | ABSENT |
| `ConsentPromptBehaviorAdmin` | **0** | 5 |
| Scheduled-task run level, read BACK from the scheduler | `Limited` | `Limited` |
| Resulting token integrity | **High** (`S-1-16-12288`) | **Medium** (`S-1-16-8192`) |
| `IsElevatedAdmin` in the task | **True** | False |

Same launcher, same registration request, same stored run level — opposite token outcome, differing on
the account's identity and filtering configuration alone. That is the variation-with-a-control-on-both-
sides bar this project applies, and it is the same bar the original elevation finding met.

## The mechanism

Measured: the runner's job account is the **built-in Administrator (RID 500)** and
`FilterAdministratorToken` is absent, so its default of `0` applies.

Explanation (standard Windows behaviour, not a reading of this run): Admin Approval Mode does not apply
to the built-in Administrator unless `FilterAdministratorToken` is `1`. With it off, that account has no
split token — there is exactly one token and it is elevated. `RunLevel Limited` asks the scheduler for
the account's **filtered** token; where no filtered token exists the request cannot be honoured, and the
task runs with the only token available. `ConsentPromptBehaviorAdmin: 0` ("elevate without prompting")
corroborates the posture.

This is why the scheduler could truthfully store `RunLevel=Limited` while the task ran at High integrity,
and why `A11Y_LIMITED_TOKEN_REGISTER: ok` was never evidence of a drop.

## What this does and does not establish

**Establishes.** The limited-token arm, via scheduled tasks, **cannot** de-elevate this leg on this
runner. Not "did not" — cannot, for RID 500 with `FilterAdministratorToken` off. This explains all three
CI runs (`35079315258`, `35095825604`, and the probe series) and retires the arm as written.

**Does not establish.** That elevation causes `DevToolsActivePort file doesn't exist` **on the runner**.
That remains untested there, because the runner cannot be de-elevated by this mechanism. The elevation
cause stands where it was established — on the dev host, by variation with a control on both sides — and
this reading neither strengthens nor weakens it.

The distinction matters for what may be claimed: a ratified exclusion may now rest on *this* measured
cause (the remedy is unavailable), and must not be written as though elevation had been confirmed on the
runner.

## Candidates that remain

1. **A different, non-administrator account** — creating a local user inside the job and registering the
   task against it would provide an account that HAS a filtered token. A significant new surface: account
   lifecycle, profile creation, and the leg's own file/profile access under a fresh principal.
2. **The `--remote-debugging-pipe` route** — sidesteps the DevToolsActivePort file rather than the
   elevation, so it is unaffected by this finding. Note the standing caveat that the shipped pipe
   diagnostic cannot yet discriminate an accepted switch from an ignored one.
3. **A ratified permanent exclusion** resting on the measured cause above, with a named owner.

Which of these to take is the operator's call; none is designed here.

## Leg signature, unchanged throughout

Every run, including this one, fails identically: `session not created: DevToolsActivePort file doesn't
exist` on the first attempt (logged at WARN, the retried-attempt severity), then a session-creation
timeout on the retry, `Spec Files: 0 passed, 1 failed, 1 total` in 00:02:01. The arm's behaviour never
varied, because the variable the arm was meant to change never changed.
