# Adaptation Record — 0-pending wrap, 2026-08-31

**Path:** no-op / operator-requested route adaptation. 0 pending in master-route; tree dirty only with
expected-transient bookkeeping (`session-handoff.md`, `friction-log.ndjson`), which the session-state
contract classes as git-clean. No P1 report, no P2 fan-out, no P4/P7 gates.

**Trigger:** operator relay (2026-08-31) — the P-075 assert round, plus a follow-up answer settling
`incident_events` reachability and directing option (b).

---

## Verification performed before acting

The relay invited an independent re-verify. All four load-bearing claims were re-derived first-hand;
three held exactly, one was sharpened.

| Claim (relay) | Verified | Result |
|---|---|---|
| Pulse HEAD `83d4060` | `git log -1` in `D:\dev\projects\andromeda-pulse` | **holds** |
| ZERO `incident_events` refs in `crates/mcp-server` | repo-wide grep, per-file counts | **holds** — hits only in `crates/corpus` (contract.rs 6, disposition.rs 5, schema.rs 3), `crates/triage/src/incident/persistence.rs` 1, one pulse-app test |
| eight MCP tools on the wire | `crates/mcp-server/src/tools.rs:44-51` | **holds** — the four live-buffer + four corpus tools, names byte-matching Conductor's pinned manifest |
| "the table's only reader is corpus-side (contract.rs:1410)" | read the site | **sharpened** — `contract.rs:1410` is a `#[cfg(test)]` helper (`event_kinds`), not a production read; the production writer is `crates/triage/src/incident/persistence.rs::save_incident_event` and the production reader is `crates/corpus/src/disposition.rs`. This *strengthens* the finding: there is no production read path outside the corpus crate, and none at all in `crates/mcp-server`. |

**Local re-derivation that changed the round's shape.** The relay's stated purpose — assert P-027 ≤5s,
P-037 ≤2s, P-045 ≤1s — is scope Conductor's own ledger already records as delivered: `v2-20`
("Four delegated timing budgets return real Pass/Fail") is `status: verified`, `chunk:
2026-08-21-delegated-timing-budgets-proven`, `ref` naming three shipped tests in
`crates/conductor-run/tests/delegated_timing_harvest.rs`. P-025's ≤2s clause is already excluded there by
the 2026-08-21 operator ruling. Surfaced at the session dashboard; the operator's answer ratified option
(b) — record, do not rebuild.

---

## Route edits (P5) — markerless tail only

**33 frozen lines verified byte-identical** against `HEAD:conductor-0.2.0/working-route.md` (diff over
`^\[`-prefixed lines: no output).

1. **NEW entry, first in Epoch 5** — _P-075 assert round — the read-back fidelity Pulse's last 0.3.0
   capability can honestly claim, and the payload-level premise measured false_. Placed ahead of
   _Webview self-verify_, per the operator's pre-direction naming both entry and disposition (which
   satisfies route-resolve's trajectory gate; the direction is cited in the entry's own CONTEXT).
   Its CONTEXT carries the four rulings: (1) do not re-assert the three budgets — quote v2-20's
   fixtures as P-075's external evidence reference; (2) payload-level content fidelity stays
   unattainable, so the entry records a second premise disproof, with the `incident_events`
   coordinates above; (3) the positive content is runtime-STATE fidelity —
   `mark_incident_resolved`'s applied/declined round-trip and `query_incident_list` active-set
   membership; (4) `incident_events`-through-MCP is a **Pulse 0.4.0 residual candidate**, noted for
   the operator, deliberately not Conductor work.
2. **40th `cargo audit` PREREQ migrated** onto the new head entry (route-resolve: an insertion ahead
   of the previous first markerless entry re-pins its next-entry PREREQs, origins preserved). Origin
   `2026-08-08-sut-capability-manifest` and the `2026-08-10` ratification marker intact; attached at
   the conventional three spaces, per the 2026-08-22 Tier-3 separator learning.

**The entry claims NO verification-matrix capability.** None of the 11 unclaimed ids covers read-back
fidelity, and the round's content is evidence-quoting plus premise recording — so its wrap's coverage
gate is a no-op by construction. Recorded here so that is not read later as an omission.

---

## Standing-pin probe — 39th discharged, PROBE-AUTO-SATISFY

Exit codes captured into variables **before any pipe** (the pinned caveat).

- `cargo audit` → true exit **1**, first diagnostic `error: error loading advisory database: parse
  error: duplicate advisory ID: RUSTSEC-2026-0244`
- `cargo deny check advisories bans licenses sources` → true exit **0** (`advisories ok, bans ok,
  licenses ok, sources ok`)
- dependency delta: `git diff HEAD --stat` over `Cargo.toml`, `Cargo.lock`, `package.json`,
  `package-lock.json` → **empty** (all byte-untouched)

Signature reproduced byte-identically → **`probe unchanged, 39th consecutive`**, the SIXTH pure
auto-satisfy fire. No basis re-authoring. Pin re-issued as the 40th on the new head entry.

---

## Queued items folded in (relay items 8–9)

All three were audit-ledger record-integrity items. **Two dissolved on re-reading their origin
artifacts rather than needing an owner assigned** — the `owner: null` values were ledger-schema gaps,
not unowned work.

| Item | Disposition | Basis |
|---|---|---|
| `mutation-timeouts` | owner `2026-08-20-read-back-seam-survivors-closed`, status **accepted-deliberate** | that chunk's report measured the plan's timeout-conversion speculation FALSE and recorded both timeouts (`jsonrpc.rs:49:30`, `:70:9`) as a ratified observe-and-report negative; its master desc says so |
| `command-field-omission` | owner `2026-08-22T13-31-54-code-audit`, status **discharged** | the record's own `commands` object carries all 8 metric keys — the remedy is present in the artifact, not merely proposed |
| **D3** verify-72.97 caveat | `mutation.baseline_caveat` written | the +16.02 pt conductor-verify gain rests on a baseline that does not reconcile (72.97 with 28 survivors implies a non-integer caught count of 75.6); conductor-run's 58.82 reconciles exactly at 40/68 and is unaffected |

`record.json` re-parsed after the edits; `mutation.scores` unchanged.

**Disclosure — out-of-surface write.** The no-op path's sanctioned write surface is `state.yaml`,
curation tiers, the markerless route tail, this record, the handoff, and amendments for facts the wrap
itself measured. A committed code-audit `record.json` is none of those. These three edits were made
because the directive required folding the items in "per their own channels" and each is a measured-fact
correction rather than a judgment; flagged here rather than widened silently. Also recorded as evolve
friction (untyped).

---

## Corrections to the incoming framing

- **"P-075 is Pulse's LAST unclaimed 0.3.0 cap"** is true of *Pulse's* ledger and says nothing about
  Conductor's. On this side P-075 sits inside `v2-20`, verified 2026-08-21. Two ledgers, not one — the
  Pulse-side claim closes by citing Conductor's fixtures, which v2-20's acceptance text already
  anticipated.
- **The relay's assert round would have re-certified verified work from a weaker basis.** Recorded as
  the reason the entry is research-and-record shaped.
- **Relay item 8 (the binary-name/report-name class count is TWO, not one)** names a *sidecar's* record.
  Grepped this repo: no Conductor artifact carries that count in either form, so there is nothing here
  to fix. Noted so the correction is not left looking unapplied.

## Curation

- **Tier 1 ×1, extended in place** — the 2026-08-09 verification chain gains an **eighth axis,
  COMPLETION**: check your own ledger for whether directed work is already done before planning it,
  with the sibling rule that a null `owner` field is a record gap until the origin is checked.
  Confidence 0.7 (0.4 verified-by-measurement + 0.3 repeated-pattern across three distinct events).
- Filtered: **1 rejected** as friction telemetry (the no-op path's missing ledger-write channel —
  curation excludes the evolve stream); **1 rejected** as task-specific (the `incident_events`
  coordinates belong to the route entry's CONTEXT, not to a universal rule).
- `CLAUDE.md` **131/200** — unchanged (an in-place extension grows a line, not the line count).

## Pre-existing, recorded not fixed

- `working-route.md` line 41 carries a dangling `   ↓` at the end of Epoch 2, and line 39 is missing its
  separator. Both verified present at `HEAD` via `cat -A`, both inside the frozen region, both inert for
  cursor derivation. Same standing as the 2026-08-22 record.
- `.andromeda/runs/_wrap_tmp` — 7 tracked files from `01c6dac`, surfaced by the 2026-08-22 audit,
  still present. Out of this path's scope.
