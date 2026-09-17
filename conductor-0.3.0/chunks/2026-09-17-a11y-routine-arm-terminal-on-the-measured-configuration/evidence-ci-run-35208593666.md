# Evidence — CI run 35208593666, the a11y routine arm's terminal

The operator-reserved gate entries (the push, and the probe that reads the resulting run) fired once, at
implement, with their values recorded here. The wrap never re-runs an `operator` entry, so this file is the
record.

## The push

`PUSHED_SHA=fc4a9c2da9149a9cce5aece64dca6416bf2ae32e` — branch `build/conductor-0.3.0`, `38d21d1..fc4a9c2`.
The operator's act; `/implement` does not commit or push.

## The pinned probe (gate entry 14, its firing form)

```
SHA=fc4a9c2da9149a9cce5aece64dca6416bf2ae32e \
gh run list --branch build/conductor-0.3.0 --workflow ci.yml --limit 1 \
  --json databaseId,headSha,conclusion --jq '.[] | select(.headSha==env.SHA)'
```

returned, exit 0:

```json
{"conclusion":"success","databaseId":35208593666,"headSha":"fc4a9c2da9149a9cce5aece64dca6416bf2ae32e"}
```

Both `expect` atoms held: `exit 0` and `contains databaseId`. The selection is pinned to THIS push inside the
entry — the `--jq` filter admits only the record whose `headSha` equals the exported value — so no
predecessor run could have satisfied it.

## Per-job conclusions

| job | conclusion |
|---|---|
| Rust gate (build · test · lint · supply-chain · coverage) | success |
| **A11y gate (routine arm · axe · contrast · violation JSON)** | **success** |
| Frontend gate (npm audit · build) | success |

## The pin gate — the one step with no prior measurement behind it

It passed by construction rather than by luck: the runtime on the image chose the driver, and the two then
agreed.

```
[diag] WebView2 Evergreen Runtime in effect: 131.0.2903.86
[precondition] MSEDGEDRIVER_SIGNATURE: valid, Microsoft Corporation
[diag] msedgedriver: 131.0.2903.86
[precondition] MSEDGEDRIVER_COHERENCE: both major 131
```

Readings taken with the step-source echo filtered out. GitHub Actions echoes each step's own script, so the
log also carries a `VALID BUT UNEXPECTED SIGNER` line and a second `valid, Microsoft Corporation` line, both
ending in a stray quote and an ANSI reset — those are the echoed SOURCE of the two branches, not readings.
The real emissions are the clean lines above (`verification-harness.md:66a`).

## The arm

```
✓ every idle-console control is keyboard-reachable by Tab alone (SC 2.1.1)
✓ idle focus order follows the run-console-idle layout: window controls, then the picker (SC 2.4.3)
[webview2 131.0.2903.86 windows #0-0] 12 passing (7s)
[webview2 131.0.2903.86 windows #0-0] 2 skipped
Spec Files:  1 passed, 1 total (100% completed) in 00:00:16
[a11y] verdict asserted - 0 failed | 2 skipped (expected 2) | driven session present
```

The a11y violation-JSON conformance gate ran in the same job: `8 tests run: 8 passed, 0 skipped`.

The two skips are the expected live-hold set (the operator-pause dialog and the operator-checklist rows),
matched against the expected-skip SET rather than tolerated — a wholly skipped run could not have read as a
pass, which is the clause `v3-02`'s acceptance turns on.

## `session not created` — attribution, measured not assumed

The log carries the string on **4 lines / 8 occurrences** (each driver JSON body repeats it in its `error`
and `message` fields; a line count under-reads an occurrence count on bodies like these). Every one is
prefixed `[diag] (driver-alone) body`, from the Medium-integrity diagnostic steps. **Zero occur in the
asserting step.**

They are not noise to ignore: on the SHIPPING configuration they re-confirm the inversion this chunk's
ruling rests on — the High-integrity path creates a session and the Medium-integrity path does not. Integrity's
SIGN stays configuration-bound (Medium helped at runtime 153 on the dev host), so the 2026-09-16 legs remain
bounded by their configuration rather than retired.

## Local corroboration, on a different configuration

The same two specs were driven on the dev host at runtime 153 under driver 152 — an INCOHERENT pair, unlike
the runner's — and passed there too: 12 passing / 0 failing / 2 skipped, against a measured 10/2/2 baseline
on the untouched tree. Passing on two configurations that differ in runtime major, driver major and
coherence is the substance of the environment-independence a11y-plan §11 → CI requires.

A deliberate one-element perturbation was run as a known-positive control and turned the assertion red,
naming what went unreached:

```
Expected: "reached 6/6 [Minimize window | Close window | Filter scenarios by name or P-ID… | Start | DIV | DIV]"
Received: "reached 5/6 [Close window | Filter scenarios by name or P-ID… | Start | DIV | DIV]"
```

`DIV | DIV` appearing as two distinct members is the direct evidence that the identity-keyed basis survives
the accessible-name collision that would collapse them under a name-keyed set. The control was reverted and
the revert verified.
