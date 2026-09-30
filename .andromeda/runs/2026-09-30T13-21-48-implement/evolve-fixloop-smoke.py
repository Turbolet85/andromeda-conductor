import json
ts = "2026-09-30T13:49:13Z"
base = dict(v=1, ts=ts, version="conductor-0.3.0", epoch="Epoch 5 — Polish & ship",
            chunk="2026-09-30-the-sr-cause-isolated-on-this-host", skill="andromeda-implement")
fl = dict(base, step="fix-loop")
sm = dict(base, step="smoke")
rs = [
    dict(fl, kind="step", id=ts + "-a", outcome="ok",
         counts={"iterations": 2, "retries": 2, "soft_exit": 0, "deferred": 0},
         consumed=[{"artifact": "plan", "quality": "ok", "note": "every non-leg entry ran as written and read green first time; entry 11's form executes the driver's --version in the same command as the Authenticode check, not gated on it"},
                   {"artifact": "research", "quality": "ok"},
                   {"artifact": "tests", "quality": "ok"},
                   {"artifact": "source", "quality": "ok"}],
         produced=[{"artifact": "source", "signals": ["evidence-authored"], "note": "evidence/no-boundary-control.md (8 arm rows, 3 routed reds) + evidence/nvda-pass.153.json; scripts spliced from disk verbatim"},
                   {"artifact": "matrix", "signals": ["no-capability-owned"], "note": "0 claimed by this chunk; no ref write"},
                   {"artifact": "implement-outcome", "signals": ["green"], "note": "PREREQ closed (unit 1136/1136, clippy green, no defer key); S 13 Tabs logged, W 0 Input and session created, 153 attached to 153.0.4234.48; teardown census back to the SearchHost baseline"}],
         problem=[{"nature": "process", "solution": "workaround", "note": "entry 11 (driver admission) runs the driver's --version beside the Authenticode check in one command; ran the signature + SHA-256 half alone first so the operator-supplied driver never executed before it read Valid, then fired the entry in full"},
                  {"nature": "product-logic", "solution": "removed-cause", "note": "arm S needed three starts: portable NVDA resolves relative -c/-f against its own install folder (fixed to absolute paths, stray config tree removed) and PowerShell 5 lacks the [ushort] accelerator (fixed to [uint16], key path dry-tested with nothing sent); no key was sent before the third start"},
                  {"nature": "environment", "solution": "removed-cause", "note": "Edge relaunched itself in the background (--no-startup-window) when the S leg closed its window and outlived the script; stopped by its root after reading its command line"}]),
    dict(fl, kind="friction", id=ts + "-b", type="retry.fix-iterations",
         what="Arm S's session script failed twice before any stimulus (relative NVDA paths resolved against NVDA's install folder; PowerShell 5 has no [ushort] accelerator), both invisible to the PS5 parse check run at authoring; the third start produced the reading",
         impact={"iterations": 2, "retries": 2}, artifacts=["runs/sr-control/key-walk.ps1", "runs/sr-control/edge-webdriver-walk.ps1"]),
    dict(fl, kind="friction", id=ts + "-c", type=None, untyped=True,
         what="Plan entry 11's note says the driver is Authenticode-verified before any execution, but its run string executes the driver's --version in the same command whatever the signature status prints; the letter split the check to honour the note",
         impact={"extra_reads": 0}, artifacts=["conductor-0.3.0/chunks/2026-09-30-the-sr-cause-isolated-on-this-host/plan.md"]),
    dict(sm, kind="step", id=ts + "-d", outcome="ok", counts={"retries": 0, "deferred": 0},
         consumed=[{"artifact": "agent-harness", "quality": "ok", "note": "strict --e2e rebuilt the release bundle, verdict line 0 failed / 2 skipped (expected 2), driven session banner webview2 154.0.4258.37"},
                   {"artifact": "plan", "quality": "ok"}],
         produced=[{"artifact": "conversation", "signals": ["recorded-not-rerun"], "note": "the role=self-verify entry ran as a P2 gate; P3 did not re-drive it"},
                   {"artifact": "implement-outcome", "signals": ["green"]}],
         problem=None),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
