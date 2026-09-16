# PREREQ discharge — the Scenario-assertion audit gate's first CI run

**Read at:** 2026-09-16 · **Run:** 35079315258 · **Head:** `468f4d34` · **Duration:** 11m38s

## The obligation

The working-route entry for this chunk carried:

> PREREQ: read the first CI run of the Scenario-assertion audit gate (registered 2026-09-16 at
> `2026-09-16-scenario-assertion-audit-gate`, never yet executed …); a red there is THAT gate's own debt,
> not this chunk's, and it is read before this entry's own CI work begins.

The obligation was TIME-triggered rather than delta-triggered: the gate's first run necessarily follows the
operator's push, so it could not have been read at the gate's own wrap.

## The reading

| Job | Status |
|---|---|
| Rust gate (build · test · lint · supply-chain · coverage) | **success** |
| Frontend gate (npm audit · build) | **success** |
| A11y gate (routine arm · axe · contrast · violation JSON) | failure — this chunk's own subject |

The gate in question is **step 13 of the Rust job**, `Scenario-assertion audit gate`, conclusion **success**.
It executed for the first time in this run. Its workflow definition carries the ledger presence guard
(a `test -f` on the committed audit ledger emitting a GitHub `::error::` annotation and exiting 1 when the
subject is absent) followed by the gate's own nextest target, and it carries no `continue-on-error` key, so
the platform default applies and a red there would fail the build.

## Disposition

**Discharged, and green.** No debt from that gate reaches this chunk. The PREREQ is spent and does not
migrate further.

The A11y job's failure is this chunk's own subject and is read separately; it is not audit-gate debt.

## Basis

Read from the run itself via the GitHub CLI (`gh run view 35079315258`), job and step conclusions taken from
the run's own JSON rather than from the job log's prose. Handle names, job names and step names only — no
resolved paths.
