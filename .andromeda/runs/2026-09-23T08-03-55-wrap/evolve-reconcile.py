import json
import re

ROOT = "D:/dev/projects/conductor"
TS = "2026-09-23T09:44:05Z"
EPOCH = open(f"{ROOT}/conductor-0.3.0/working-route.md", encoding="utf-8").read().split("\n")[40][4:]
assert EPOCH.startswith("Epoch 4"), EPOCH
base = {"v": 1, "ts": TS, "version": "conductor-0.3.0", "epoch": EPOCH,
        "chunk": "2026-09-22-interpretation-proven-live", "skill": "andromeda-wrap-session", "step": "reconcile"}

records = [
    dict(base, kind="step", id=f"{TS}-a", outcome="halted-resolved",
         counts={"retries": 0, "dialogue_rounds": 2, "halted": 1},
         consumed=[
             {"artifact": "report", "quality": "ok", "note": "every applied text re-derived from its Changes; two escalations needed evidence beyond it, located at validate (the 2026-09-10 fingerprint envelopes; the pre-existing multi-named P-ID census)"},
             {"artifact": "spec:architecture", "quality": "ok"}, {"artifact": "spec:security-plan", "quality": "ok"},
             {"artifact": "spec:test-plan", "quality": "ok"}, {"artifact": "spec:obs-plan", "quality": "ok"},
             {"artifact": "spec:layout-templates", "quality": "ok"}, {"artifact": "spec:design-system", "quality": "ok"},
             {"artifact": "spec:a11y-plan", "quality": "ok"}, {"artifact": "drift-base", "quality": "ok"},
             {"artifact": "wrap-playbook", "quality": "thin", "note": "no governing rule for S1/S2, S4 (rule 112's garde qualifier fails a closed unit enum) or S5; three rules minted with operator refinements"}],
         produced=[
             {"artifact": "spec:architecture", "signals": ["cascade-wide", "escalation-shaped"], "note": "23 body edits + 1 step-2 fold; payload-fidelity universal retired as measured"},
             {"artifact": "spec:security-plan", "signals": ["escalation-shaped"], "note": "15 edits + 3 step-2 folds; one ratified corpus exception"},
             {"artifact": "spec:test-plan", "signals": ["escalation-shaped"], "note": "21 edits incl. the check-5 raise (sec 6 harvest leg) and two pre-existing stale claims fixed on operator direction"},
             {"artifact": "spec:obs-plan", "signals": [], "note": "5 edits"},
             {"artifact": "spec:layout-templates", "signals": [], "note": "3 edits"},
             {"artifact": "sidecar:architecture", "signals": []}, {"artifact": "sidecar:security-plan", "signals": []},
             {"artifact": "sidecar:test-plan", "signals": []}, {"artifact": "sidecar:obs-plan", "signals": []},
             {"artifact": "sidecar:layout-templates", "signals": []},
             {"artifact": "wrap-playbook", "signals": ["rules-minted-3"], "note": "49 -> 52 rules, operator-approved with sharpened S4/S5 clauses"},
             {"artifact": "cascade-sweep", "signals": ["step2-caught-detector-misses"], "note": "the step-2 sweep found five restatement sites no detector proposed (arch :113 corpus ban, arch :239 tree, sec :114/:221/:325) and the check-5 raise covered the sixth"}],
         problem=[
             {"nature": "resources", "solution": "workaround", "note": "the prior session persisted the security/test/architecture fan-out returns as CONDENSED transcriptions at its context ceiling; this session validated from those twins, not the raw returns"},
             {"nature": "process", "solution": "workaround", "note": "validate's re-derivation-tell rule (reject a proposal whose basis cites non-report locations) applied as: strip the non-report pointers and re-derive the text from report facts, since every such proposal's FACT was report-carried or check-5-raisable"},
             {"nature": "product-logic", "solution": "deferred", "note": "discovered issues routed forward, not fixed: the capture's two unguarded joins + live_suite.rs's path-bearing panic (CARRY), run <P-ID> ambiguity (CARRY), test-plan :335 'permanently degraded_mode', residuals.md :11 'payload fidelity stays unattainable', storm_harvest.rs header, the posture doc's launch-section 'shell-declaration', conductor-verify preflight.rs:202-205 stale comment"},
             {"nature": "process", "solution": "overridden", "note": "the operator rewrote two proposed playbook rules: S4's clause sharpened to exclude a #[serde(other)] catch-all, S5 guarded so rule Boundary widening governs any flag whose EFFECT crosses"}]),
    dict(base, kind="friction", id=f"{TS}-b", type="contract.narrow-basis-claim",
         what="A17-A19 reached validate framed as 'code reading only, not live-measured' (doc-agent return, fanout-results, handoff); one grep of committed evidence found ten 2026-09-10 live envelopes whose fingerprints carry the cue's 32-hex value (leg1's equals its own 'canary fingerprint computed' self-obs line), so the escalation's recommendation moved from a hedged amendment to 'amend as measured'",
         impact={"extra_reads": 3},
         artifacts=["conductor-0.2.0/chunks/2026-09-10-release-build-and-bundle/evidence/leg1-2026-09-10T19-32-57-092.jsonl"],
         evidence=".andromeda/runs/2026-09-23T08-03-55-wrap/reconcile-sweep.md"),
    dict(base, kind="friction", id=f"{TS}-c", type="ambiguity.playbook-no-match",
         what="security S1/S2 (READ-SET posture arm), S4 (closed unit-enum scenario key fails rule 112's 'garde-validated at load') and S5 (CLI flag of an existing input class) had no governing rule; applied under the plan's P5-approved direction and three rules proposed at the card, approved with operator refinements",
         impact={"dialogue_rounds": 1},
         artifacts=[".andromeda/playbook.md"]),
    dict(base, kind="friction", id=f"{TS}-d", type="contract.skill-reference-drift",
         what="the four specialist summaries' header text says 'wrap-session does not modify' while amendment-flow's cascade table re-derives them for any plan change and prior wraps have edited them (git log); tests-summary and security-summary were re-derived this pass per the cascade table",
         impact={},
         artifacts=[".claude/docs/tests-summary.md:3", ".claude/docs/security-summary.md:3", "andromeda-wrap-session/references/amendment-flow.md"]),
]

R = re.compile(r"^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\dZ$")
bad = [r for r in records if not R.fullmatch(str(r.get("ts"))) or str(r.get("id", "")).rsplit("-", 1)[0] != r.get("ts")]
if bad:
    raise SystemExit(f"malformed ts/id - nothing written: {bad}")
lines = [json.dumps(r, ensure_ascii=False) for r in records]
for l in lines:
    json.loads(l)
with open(f"{ROOT}/.andromeda/friction-log.ndjson", "a", encoding="utf-8", newline="") as f:
    for l in lines:
        f.write(l + "\n")
print(f"appended {len(lines)} records; epoch = {EPOCH!r}")
