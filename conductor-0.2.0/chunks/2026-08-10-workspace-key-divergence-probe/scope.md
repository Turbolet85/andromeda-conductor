# Scope — 2026-08-10-workspace-key-divergence-probe

**Working-route entry (verbatim intent anchor):**
> Workspace-key divergence probe — two-launch verdict plus a host-path-free named precondition replacing the
> opaque corpus Blocked

**Epoch:** 2 — Live-path enablement · **Version:** conductor-0.2.0 · **Capability:** `v2-17`

---

## What this chunk builds

Two deliverables, one investigative and one code:

1. **The two-launch verdict** — run the live check the version's intent calls its *first action*: launch
   `pulse-app` from the repo root and from a marker-less temp dir, and record which one lets
   `query_incident_list` return rows. **Amended at P5** (see the corrected boundary below): a third arm,
   `cwd == the data dir`, is added because research showed the second arm cannot do what the intent expected
   and only the third can come back positive. The verdict is the deliverable — it decides whether a Pulse-side
   chunk gates the live legs.

2. **A named precondition for the divergence** — Conductor must DETECT the app/sidecar workspace-key mismatch
   and surface it as a *distinct, named* `Blocked` precondition, replacing today's opaque
   `Blocked: incident not found in corpus`. The precondition string carries no absolute host path.

## Why (the observed gap)

Pulse filters incidents `WHERE workspace = ?1`. `pulse-app` derives that key from
`resolve_workspace_for_incidents` — the **detected project root** when detection succeeds, falling back to
`data_dir` only when it fails. The **sidecar Conductor spawns** always uses `data_dir`. So a `pulse-app`
launched anywhere the detector recognises writes incidents under a key the sidecar never queries:
`query_incident_list` returns zero rows and preflight reports the generic corpus `Blocked` — *even with
deterministic L4 on*. The failure is indistinguishable from "the LLM produced no incident", which is why it
has to become a named precondition.

## Boundaries

- **The real fix is Pulse-side and is NOT this chunk's work.** Aligning the sidecar with
  `resolve_workspace_for_incidents` is a Pulse edit; this chunk only makes Conductor *detect and name* the
  condition. Conductor does not modify, patch, or work around Pulse's corpus code.
- **The interim unblock the intent describes does not exist — CORRECTED at P5 (validation-1,
  intent-incomplete).** The intent holds that launching `pulse-app` from a marker-less dir makes both sides
  fall back to `data_dir`. Research falsified this: `workspace_detector::detect` records the `.andromeda`
  marker and VCS as **fields on the returned context, not success conditions**, and returns `Err` only on a
  traversal component or a non-existent path — so detection succeeds for virtually any real cwd, and a
  marker-less temp dir keys to *that temp dir*, not to `data_dir`. The `data_dir` fallback is effectively
  unreachable from a normal launch. Consequently: no cwd-based interim unblock is known to exist, the probe
  gains a third arm (`cwd == the data dir`, the only position where the two keys could agree), and whatever
  any arm shows is a diagnostic, never the standing run configuration.
- **Not the run contract.** `ANDROMEDA_PULSE_L4_DETERMINISTIC` + the shared-data-dir assertion are intent F11
  and belong to the *next* Epoch-2 entry (*Pulse run contract*, P-073) — this chunk stops at the workspace key.
- **Not a live green preflight.** Reaching `ready:true` against a real Pulse is `v2-10` (*First live green
  preflight*), later in Epoch 2. This chunk may leave preflight still blocked — but blocked with a name.
- Conductor never opens `corpus.db` directly (read-back via MCP only) and never persists SUT corpus content.

## Surfaces and contracts touched

- **The preflight readiness gate** (`conductor-verify`) — the `blocked_precondition` field of the readiness
  result (architecture §Standard Contracts) gains a distinct value for this condition. The gate's existing
  three failure modes (protocol mismatch · missing tool · empty canary) are unchanged.
- **The sidecar spawn** (`conductor-verify/src/spawn.rs`) — the `ANDROMEDA_PULSE_DATA_DIR` `.env(...)` path is
  where Conductor knows the sidecar's key; the hardened fixed-path + injection-reject spawn is unchanged.
- **Artifact hygiene** — the precondition string is operator-facing and lands in the run report / logs, so it
  is bound by the no-absolute-host-paths invariant (`.claude/rules/security.md` §Error handling).
- **The verdict/error wall** — a detected divergence is a typed `Blocked` VALUE, never a `Result::Err` and
  never a panic.

## PREREQ folded in (from 2026-08-09-sut-load-envelope)

**Re-check `cargo audit` at this chunk's gates — the SIXTH consecutive check.** Five prior checks were red
with a byte-identical `error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244`,
re-proven on 0.22.2 (the latest published), true exit 1. This is an advisory-**DATABASE** fault: the duplicate
id is committed data in RustSec's advisory-db, so there is nothing to raise a floor to. The remedy stays the
**bounded wait alone**, with `cargo deny check` **observed** green as the overlapping signal (all four classes
ok at the last check). Do NOT raise the floor, do NOT add a `deny.toml` ignore, do NOT edit CI. Close the
deferral the moment it parses. (`playbook.md` external-decay · `.claude/rules/security.md` 2026-08-09 ·
security-plan §Dependency Security.)

## Backing beyond the working entry

Every bullet below is stated in the version's authored sources, not invented here — cited so P4 can
re-verify each against its source first.

- `[inferred]` The divergence mechanism, the two file-level derivations, and the "first action of this
  version: run the two-launch check and record the verdict" directive — **intent §Theme 3 F10**. F10 marks its
  own observation as *code read, 2026-08-08 — **verify live before designing around it***, so P3/P4 must treat
  the Pulse-side line numbers as claims to re-check, not as facts.
- `[inferred]` The acceptance shape — a simulated mismatch producing a distinct named precondition, the
  no-absolute-host-path constraint, and the once-run recorded two-launch verdict — **requirements `v2-17`** /
  `verification-matrix.json#v2-17` (`method: integration`).
- `[inferred]` The boundary against F11 (run contract) and `v2-10` (live `ready:true`) — read off the Epoch-2
  ordering in `working-route.md`, where both are separate later entries.
- `[inferred]` The invariants named under *Surfaces and contracts* — `architecture.md` §Standard Contracts
  (readiness result shape), the verdict/error wall, and the artifact-hygiene rule. These constrain HOW the
  precondition may be surfaced; none of them is a new decision this chunk makes.
