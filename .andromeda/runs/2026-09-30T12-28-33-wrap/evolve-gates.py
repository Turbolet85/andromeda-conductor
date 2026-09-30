import json, sys
TS = sys.argv[1]
E = dict(v=1, ts=TS, version="conductor-0.3.0", epoch="Epoch 5 — Polish & ship",
         chunk="2026-09-30-the-screen-reader-pass-grades-again-on-this-host", skill="andromeda-wrap-session")
def rec(letter, **kw):
    r = dict(E); r["id"] = TS + "-" + letter; r.update(kw); return r
rs = [
    rec("a", kind="step", step="gates", outcome="ok",
        counts={"halted": 0, "deferred": 2},
        consumed=[{"artifact": "plan", "quality": "ok", "note": "the block re-ran identically to implement: green 8 · red 3 (10,11,12) · recorded 1 · not-run 6"},
                  {"artifact": "matrix", "quality": "ok"},
                  {"artifact": "master-route", "quality": "ok"}],
        produced=[{"artifact": "matrix", "signals": [], "note": "claimed 0 — gate no-op; version 10 verified · 1 deferred"},
                  {"artifact": "master-route", "signals": [], "note": "route.py flip exit 0, desc rewritten from the falsified promise to the measured Branch-B outcome; compact archived 1 line"},
                  {"artifact": "git-state", "signals": ["deferral-open", "pushed"], "note": "4460307 pushed; tree.db.commit stamped; graph refresh done rust 2982n/14304e ts 895n/1805e"}],
        problem=None),
    rec("b", kind="friction", step="gates", type="tooling.gate-deferral",
        what="Rust unit + clippy entries deferred again by their defer keys (zero Rust delta); the void-check over this wrap's changed files found only the doc-comment basename false positive; re-pinned as PREREQ on the successor with origin this chunk",
        impact={"deferred": 2}),
    rec("c", kind="friction", step="gates", type=None, untyped=True,
        what="the plan designed three regrade entries to read red by construction under Branch B without naming an owner; the P7.1 ASSERT admits a red only with basis + owner, so the wrap measured the basis (same filters red over the parent's untouched record) and pinned an owner CARRY on the successor before the flip",
        impact={"extra_reads": 1}),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
