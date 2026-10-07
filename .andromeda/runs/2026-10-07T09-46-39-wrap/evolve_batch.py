"""Evolve records for this wrap, one batch per checkpoint: `python evolve_batch.py {step}` prints the batch."""
import json
import sys

BASE = {
    "version": "conductor-0.3.0",
    "epoch": "Epoch 5b — Version close",
    "chunk": "2026-10-07-a-fifth-pre-registered-real-model-series-for-v3-09",
    "skill": "wrap-session",
}


def rec(ts, letter, step, kind, **kw):
    r = {"v": 1, "kind": kind, "ts": ts, "id": f"{ts}-{letter}"}
    r.update(BASE)
    r["step"] = step
    r.update(kw)
    return r


def report():
    ts = "2026-10-07T09:50:38Z"
    s = "report"
    return [
        rec(ts, "a", s, "step", outcome="ok", counts={"extra_reads": 3},
            consumed=[
                {"artifact": "conversation", "quality": "thin", "note": "compacted between the operator pass and this wrap; the summary stood in and the ledger and the plan's gate block were re-read whole"},
                {"artifact": "implement-outcome", "quality": "ok"},
                {"artifact": "operator-directive", "quality": "ok", "note": "the wrap relay's covariate table adds a second count per cell; all twelve figures recounted from the six captures and matched"},
                {"artifact": "git-state", "quality": "ok"},
                {"artifact": "plan", "quality": "ok"},
            ],
            produced=[{"artifact": "report", "signals": ["reconstructed", "scope-read-clean", "inputs-verify-zero-drift"], "note": "the gate list and the cross-project bullet were rebuilt from the ledger and the plan; every Expected amendment carries its site search"}],
            problem=[
                {"nature": "process", "solution": "workaround", "note": "inputs.py snap takes no wrap step, so the wrap relay read at this step is named in the report as read live with its sha256 instead of a snapshot"},
                {"nature": "process", "solution": "removed-cause", "note": "the ledger's walker's-mark heading was missing in the pushed pre-CI commit (dropped by the covariate insertion); restored at this step, outside the report's own work"},
                {"nature": "environment", "solution": "workaround", "note": "a scratchpad probe script written by a cat heredoc was blocked by the Bash guard; written with the Write tool and run by path"},
            ]),
        rec(ts, "b", s, "friction", type="recall.change-reconstruction",
            what="the conversation was compacted before the wrap; the gate verdicts by run text and the cross-project facts were rebuilt by re-reading the attempt ledger whole and the plan's Test Commands block",
            impact={"extra_reads": 2},
            artifacts=["evidence/attempt-ledger.md", "plan.md Test Commands"]),
        rec(ts, "c", s, "friction", type="contract.structural-blind-spot",
            what="the wrap's P1 reads an operator relay from outside the repo, and inputs.py snap's --step admits phase:P1, phase:P3 and implement only, so a wrap-time external input cannot be snapshotted",
            impact={"extra_reads": 1},
            artifacts=["inputs.py snap --step"],
            evidence="exit 2: argument --step: invalid choice: 'wrap'"),
        rec(ts, "d", s, "friction", type=None, untyped=True,
            what="an anchored Edit that inserted the covariate section into the ledger at implement removed the next section's heading line; hygiene, the pre-CI commit and CI all passed over it and the pushed evidence lacked the heading until this wrap",
            impact={"retries": 1},
            artifacts=["evidence/attempt-ledger.md"]),
    ]


def reconcile():
    ts = "2026-10-07T10:03:04Z"
    s = "reconcile"
    docs = ["architecture", "security-plan", "design-system", "layout-templates", "test-plan", "obs-plan", "a11y-plan"]
    return [
        rec(ts, "a", s, "step", outcome="ok", counts={"retries": 0, "dialogue_rounds": 0, "halted": 0},
            consumed=[{"artifact": "report", "quality": "ok", "note": "all seven detectors ran from it; no gap exposed"}]
            + [{"artifact": f"spec:{d}", "quality": "ok"} for d in docs]
            + [{"artifact": "drift-base", "quality": "ok"},
               {"artifact": "wrap-playbook", "quality": "thin", "note": "one rule governed the handle registration; no rule covers a real-model series' dated-record extension (9 of 11 proposals), settled by the plan's approved expected amendments"}],
            produced=[
                {"artifact": "spec:architecture", "signals": ["registry-at-threshold"], "note": "5 amendments; the env registration needed 341 B freed inside a section with 29 B of headroom; the section ends at 38115 of 38115 B"},
                {"artifact": "spec:security-plan", "signals": [], "note": "4 amendments, all dated-record extensions"},
                {"artifact": "spec:test-plan", "signals": []},
                {"artifact": "spec:obs-plan", "signals": []},
                {"artifact": "sidecar:architecture", "signals": ["payload-check-caught-two-blocks"], "note": "sidecar.py check refused a two-entry payload before the append; split into two files, both on-form"},
                {"artifact": "sidecar:security-plan", "signals": []},
                {"artifact": "sidecar:test-plan", "signals": []},
                {"artifact": "sidecar:obs-plan", "signals": []},
                {"artifact": "cascade", "signals": ["control-refused-a-pattern"], "note": "27 rows dispositioned; 1 leaf re-derived (tests-summary); the tool refused a pattern with no master control, which was then run by hand at 0 hits"},
            ],
            problem=[
                {"nature": "process", "solution": "workaround", "note": "the security-plan, test-plan and obs-plan bodies were applied while the architecture detector was still running and before the dispositions were written to fanout-results.md; no proposal collided, and the dispositions were written before any sidecar entry"},
                {"nature": "resources", "solution": "removed-cause", "note": "the architecture registry section had 29 B of headroom for a 360 B registration; two redundancies (a sentence test-plan carries, three self-restating clauses) were removed from bullets this chunk did not otherwise touch"},
            ]),
        rec(ts, "b", s, "friction", type="ambiguity.playbook-no-match",
            what="no playbook rule matches a real-model series' dated-record extension; 9 of 11 proposals were settled by the plan's approved expected amendments instead, for the fourth consecutive series wrap",
            impact={"extra_reads": 1},
            artifacts=[".andromeda/playbook.md"], evidence="fanout-results.md, Checks over the whole set"),
        rec(ts, "c", s, "friction", type=None, untyped=True,
            what="7 of 11 proposals carried a basis outside the report (capture line numbers, posture-contract lines, a sibling-repo git diff, the registry check script) against the prompt's own ban; each was rejected as proposed and raised by the orchestrator from the report's fact",
            impact={"reformulations": 7},
            artifacts=["spec:security-plan detector ×4", "spec:architecture detector ×3"], evidence="fanout-results.md"),
        rec(ts, "d", s, "friction", type="contract.false-positive-proposal",
            what="D-arch-registry-size proposed moving two evidence pointers out of the body to free bytes; narrowed, because a mechanism statement keeps its measurement pointer, and a sentence another master states in full left instead",
            impact={"reformulations": 1},
            artifacts=["D-arch-registry-size"], evidence="fanout-results.md, architecture 5"),
    ]


def curation():
    ts = "2026-10-07T10:04:11Z"
    s = "curation"
    return [
        rec(ts, "a", s, "step", outcome="ok", counts={"dialogue_rounds": 0},
            consumed=[{"artifact": "conversation", "quality": "thin", "note": "compacted; the report's Decisions and the overseer's wrap relay carried the candidates"},
                      {"artifact": "report", "quality": "ok"}],
            produced=[{"artifact": "conversation", "signals": []}],
            problem=None),
        rec(ts, "b", s, "friction", type="ambiguity.filter-borderline",
            what="two Filter 4 calls sat at the edge: a candidate landed exactly at 0.6 and rejected (its fact has a home in the contract section), and the applied one reaches 0.6 on measurement and detail alone, passing on the reading that the overseer's listed curation candidates count as an explicit curation request",
            impact={"extra_reads": 0}, artifacts=["Filter 4"], evidence="curation.md"),
        rec(ts, "c", s, "friction", type="recall.corpus-recurrence",
            what="an anchored Edit dropped a heading from the attempt ledger and reported success; the Tier-1 entry on verifying the artifact after a write already states the rule, and the write was not read back",
            impact={"retries": 1}, artifacts=["CLAUDE.md USER:session-learnings 2026-08-21"], evidence="curation.md, To the handoff"),
    ]


def route_resolve():
    ts = "2026-10-07T10:08:24Z"
    s = "route-resolve"
    return [
        rec(ts, "a", s, "step", outcome="halted-resolved", counts={"dialogue_rounds": 1, "halted": 1, "extra_reads": 1},
            consumed=[{"artifact": "report", "quality": "ok"}, {"artifact": "working-route", "quality": "ok"}],
            produced=[{"artifact": "working-route", "signals": ["tail-reshaped", "relayed-citation-verified"], "note": "one entry minted ahead of the version close with BLOCKED-ON, CONTEXT and one CARRY; the relayed Pulse route line and commit were read in Pulse's tree before the entry named them, and both held"}],
            problem=[{"nature": "process", "solution": "overridden", "note": "the authority rule reads a relayed summary on a trajectory fork as provisional; the direction named the entry not provisional as the founder's own pick, so the entry carries no provisional word and records the option's title as his pick with its content marked as the overseer's summary"}]),
        rec(ts, "b", s, "friction", type="ambiguity.trajectory-halt",
            what="a second not-met series left what follows undecided; one halt, one round, class new-chunk-ahead, answered with the founder's pick relayed by the overseer and a route line in the sibling repo to verify",
            impact={"dialogue_rounds": 1, "halted": 1, "extra_reads": 1},
            artifacts=["conductor-0.3.0/working-route.md:96"], evidence="route-resolve.md"),
    ]


STEPS = {"report": report, "reconcile": reconcile, "curation": curation, "route-resolve": route_resolve}

if __name__ == "__main__":
    rs = STEPS[sys.argv[1]]()
    print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
