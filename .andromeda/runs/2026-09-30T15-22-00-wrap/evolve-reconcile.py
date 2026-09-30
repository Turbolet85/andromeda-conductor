import json, sys

route = open("conductor-0.3.0/working-route.md", encoding="utf-8").read().splitlines()
epoch = [l[4:] for l in route if l.startswith("### Epoch 5")][0]
TS = sys.argv[1]
M = "2026-09-30-the-sr-pass-regrades-on-the-os-input-path"
base = {"v": 1, "ts": TS, "version": "conductor-0.3.0", "epoch": epoch, "chunk": M, "skill": "andromeda-wrap-session", "step": "reconcile"}
L = iter("abcdefgh")
def r(**k):
    d = dict(base); d["id"] = TS + "-" + next(L); d.update(k); return d

rs = [
    r(kind="step", outcome="ok", counts={"retries": 0, "dialogue_rounds": 0, "halted": 0},
      consumed=[{"artifact": "report", "quality": "thin", "note": "its Counts bullet marked all five security-plan :367 'seven' hits as change; two are ordinals that stay true (the security doc-agent's reading), corrected in the report"},
                {"artifact": "spec:architecture", "quality": "ok"}, {"artifact": "spec:security-plan", "quality": "ok"},
                {"artifact": "spec:design-system", "quality": "ok"}, {"artifact": "spec:layout-templates", "quality": "ok"},
                {"artifact": "spec:test-plan", "quality": "ok"}, {"artifact": "spec:obs-plan", "quality": "ok"},
                {"artifact": "spec:a11y-plan", "quality": "ok"}, {"artifact": "drift-base", "quality": "ok"},
                {"artifact": "wrap-playbook", "quality": "ok"}],
      produced=[{"artifact": "spec:a11y-plan", "signals": ["cascade-wide"]}, {"artifact": "spec:security-plan", "signals": ["escalation-shaped"]},
                {"artifact": "spec:test-plan", "signals": []}, {"artifact": "spec:architecture", "signals": ["escalation-shaped"]},
                {"artifact": "sidecar:a11y-plan", "signals": []}, {"artifact": "sidecar:test-plan", "signals": []},
                {"artifact": "sidecar:security-plan", "signals": []}, {"artifact": "sidecar:architecture", "signals": []}],
      problem=[{"nature": "process", "solution": "workaround",
                "note": "the 5 boundary-widening escalations (A1, S1-S4) were resolved on the founder's live ratification relayed in the operator's wrap relay, without a second HALT; the relay named rule (b) seven to eight and the arch registration"}]),
    r(kind="friction", type="contract.cascade-miss",
      what="a11y-plan:268 kept 'the agent arm — WebDriver-injected keys … grade not-announced' after the first apply edited the same line (Y3 had asked for it); the cascade sweep's injected-arm pattern caught it and it was reworded to the injected path",
      impact={"iterations": 1}, artifacts=[".andromeda/a11y-plan.md:268"], evidence=".andromeda/runs/2026-09-30T15-22-00-wrap/cascade-dispositions.md"),
    r(kind="friction", type="contract.cascade-miss",
      what="CLAUDE.md:34 (GENERATED warnings) and .claude/docs/gotchas.md:54 listed the SR leg's children as NVDA alone, omitting the window-activation script since 2026-09-02; found by a read of the children restatements, not by any pattern, and re-derived",
      impact={"extra_reads": 1}, artifacts=["CLAUDE.md:34", ".claude/docs/gotchas.md:54"], evidence=".andromeda/runs/2026-09-30T15-22-00-wrap/cascade-dispositions.md"),
    r(kind="friction", type="input.report-insufficient",
      what="the report's Counts bullet classed all five security-plan :367 seven-hits as change; the security doc-agent showed two are ordinals that stay true; the report was corrected at P2",
      impact={"reformulations": 1}, artifacts=["conductor-0.3.0/chunks/" + M + "/report.md"]),
    r(kind="friction", type=None, untyped=True,
      what="the orchestrator's prompt builder gave D-platform-claim (doc: all seven) to architecture alone and added a keyed-contracts line to three docs whose render wrote no file; both caught by counting the built prompts before the fan-out and fixed",
      impact={"iterations": 2}, artifacts=[".andromeda/runs/2026-09-30T15-22-00-wrap/build-prompts.py"]),
    r(kind="friction", type="recall.corpus-recurrence",
      what="the P1 site sweep read grep -c over the masters' multi-KB single lines and an escaped pipe under -E, returning line counts and false zeros, though CLAUDE.md's Tier-1 2026-09-17 entry states the grep -c line-count trap; re-counted with -oiE and wc -l",
      impact={"reformulations": 1}, artifacts=["CLAUDE.md USER:session-learnings 2026-09-17"]),
]
print("\n".join(json.dumps(x, ensure_ascii=False) for x in rs))
