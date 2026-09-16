# Surfaced — the launch mechanism composes a command line, and the criterion forbids it

**Measured:** 2026-09-16, dev host · **Raised at:** /implement P2 · **Authored change:** none

## The gap

| | |
|---|---|
| **Contract** | The chunk's security acceptance criterion: "No shell string, `Invoke-Expression` or composed command line appears in any launch path added to the `a11y` job; every program is fixed and **every supplied value is a separate array-form argv element**" (per security-plan §Security Anti-Patterns → Code Patterns rule (b)) |
| **Impl reality** | `scripts/a11y-limited-token-launch.ps1` joins its `$ArgumentList` into one string before handing it to the scheduler |
| **Why it cannot be fixed in the impl** | The scheduler API takes a command-line STRING and has no array form |

## The measurement that settles it

`New-ScheduledTaskAction`'s `-Argument` parameter is typed `System.String`, and passing an array is
rejected outright rather than coerced:

- `(Get-Command New-ScheduledTaskAction).Parameters['Argument'].ParameterType.FullName` → `System.String`
- passing `@('-NoProfile','-File','x.ps1')` → `ParameterBindingArgumentTransformationException`

So **no scheduled-task mechanism can satisfy the criterion's "separate array-form argv element" wording.**
The choice is not between a good and a bad implementation of the same mechanism; it is between the wording
and the mechanism class the CARRY named and the operator directed. A `runas` alternative is excluded
separately — it prompts, and the headless invariant forbids an interactive prompt on the agent-driven path.

## What the risk actually is, measured

The concrete hazard of a composed command line is an argument containing whitespace or a metacharacter
silently changing the command's shape. **That failure mode is already closed, and loudly.** The script
guards every element against whitespace and shell metacharacters before composing, mirroring
`wdio.conf.ts`'s `UNSAFE_PATH`. Both controls run on this host:

| Control | Input | Result |
|---|---|---|
| A | an argument containing a space | `[precondition] A11Y_LIMITED_TOKEN_ARGV: REJECTED` · exit **90** |
| B | an argument containing `;` | `[precondition] A11Y_LIMITED_TOKEN_ARGV: REJECTED` · exit **90** |

So the residual is a documentation gap, not a live injection path: a space-bearing argument fails with a
named precondition and a distinct exit code rather than mis-composing silently. The five values the
workflow supplies are fixed literals authored in `ci.yml` (`-NoProfile`, `-File`, `scripts/agent-run.ps1`,
`run`, `--e2e`); none is interpolated and none carries a space or a quote.

## Disposition — OPERATOR RULING, 2026-09-16

**The master is NOT violated; the plan's restatement is over-strict.** Rule (b) reads, verbatim at
`security-plan.md:361`:

> NEVER spawn a child via a shell / `eval`-equivalent with operator-supplied input.

Its subject is a shell or eval path carrying operator input. A scheduled task is neither: Task Scheduler
hands the string to the target process's own `CommandLineToArgvW`, and the argv passed is `-File`, never
`-Command` — `-Command` would BE the eval path. And there is no operator-supplied input: five literals
authored in `ci.yml`, with the two controls above proving metacharacters and whitespace are rejected
before composition. "Every supplied value is a separate array-form argv element" is an implementation
SHAPE the master never demands.

The ruling is **both** of the following, in order.

### (1) Reword the criterion — a faithful restatement, not a weakening

/implement is read-only on `plan.md` and did not apply this; the text is recorded here ready to land
through the wrap, so the criterion and the code stop contradicting each other:

> **(security)** No shell and no `eval`-equivalent appears in any launch path added to the `a11y` job —
> the scheduled-task action invokes a fixed resolved program with `-File`, never `-Command`. No
> operator-supplied value reaches argv: every element is a literal authored in `ci.yml`, and the launcher
> rejects whitespace and shell metacharacters before any composition (measured: exit 90 with a named
> precondition line on both a space-bearing and a `;`-bearing argument). If the change constitutes a
> seventh governed harness-spawn form it is NAMED as such in the report and operator-ratified at wrap
> (per security-plan §Security Anti-Patterns → Code Patterns rule (b)).

### (2) Register the seventh governed form at the wrap — unchanged from plan step 2

Not permission for a violation, but because the scheduled-task launch is a new spawn CLASS on the
project's spawn surface, which is what rule (b)'s registry exists to track. Registration text:

> **Seventh governed harness-spawn form (CI-only).** `.github/workflows/ci.yml`'s `a11y` job launches the
> routine arm under a Limited-token scheduled task via `scripts/a11y-limited-token-launch.ps1`. The
> program is a fixed resolved `pwsh`; the argv is five literals authored in the workflow. **The form is
> `-File` only — never `-Command`**, which is what keeps it outside rule (b)'s eval class; the launcher
> guards every element against whitespace and shell metacharacters before composition, and the composition
> itself is forced by the API (`New-ScheduledTaskAction -Argument` is `System.String` and rejects an
> array). Admitting control: the guard plus the fixed workflow-authored argv. Its cause is elevation on
> hosted runners, established by direct variation with a control on both sides.

The `-File`-only clause is stated deliberately: the guard rejects metacharacters and whitespace but would
not reject an argv that is itself `-Command`-shaped. That cannot arise today — the argv is fixed in
`ci.yml` — so it is recorded as a property of the registered form rather than added as another check.

## Unplanned corroboration — the mechanism itself works

Running the launcher with the workflow's real argument list on this dev host registered the task, ran it,
and returned its exit code:

```
[precondition] A11Y_LIMITED_TOKEN_REGISTER: ok (RunLevel Limited, LogonType Interactive)
[diag] A11Y_LIMITED_TOKEN_LEG_EXIT: 1
```

The leg's own exit of 1 is expected — the control forwarded no handles, so the driver handle was
unresolved and the leg took its designed non-zero arm. What the run establishes is that **registration
with a Limited run level and an Interactive logon succeeds on a Windows host, the task executes, and the
exit code propagates back**. A `92 register` or `93 start` failure in CI would therefore be a
runner-specific difference, not a broken script.

Teardown was exact and measured afterwards: no `conductor-a11y-limited-*` task left registered (the
`finally` unregister held), no leg-family process survivors, and no User-scope environment residue from
the run.
