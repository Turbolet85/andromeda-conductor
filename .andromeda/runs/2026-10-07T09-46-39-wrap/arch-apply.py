"""Apply this wrap's architecture amendments: each replacement must match exactly once. `--dry` prints only."""
import sys

PATH = ".andromeda/architecture.md"
NEW_BULLET = (
    "- `ANDROMEDA_PULSE_MODEL_PATH` · `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` — Pulse's real-model launch handles, "
    "set by Pulse's launching shell. Conductor neither SETS nor READS either; registered on the entry "
    "above's basis: the committed `contracts/pulse-real-model-leg-posture.md` names both. Neither is a run-contract "
    "term or a `conductor preconditions` subject.\n"
)
TARGET_TAIL = (
    " Which posture actually booted is read from Pulse's own log at the TARGET "
    "`triage.baseline.bootstrap_window.override` (`reason=\"env_override\"`, `resolved_seconds`), never from the "
    "emitting function name.\n- `WEBVIEW2_USER_DATA_FOLDER`"
)
EDITS = [
    # the posture-contract entry: the fifth series date
    ("2026-10-01, 2026-10-06 — each an add-only section", "2026-10-01, 2026-10-06, 2026-10-07 — each an add-only section"),
    # the posture-contract entry: the latest per-series pin
    ("(re-pinned per series, `5f77859` at 2026-10-06)", "(re-pinned per series, `f70be92` at 2026-10-07)"),
    # the bootstrap-window handle is no longer the only one of its class
    ("**The one registered handle Conductor neither SETS nor READS**", "**A registered handle Conductor neither SETS nor READS**"),
    # the log-target sentence leaves (test-plan section 9 carries it); the two model handles are registered
    (TARGET_TAIL, "\n" + NEW_BULLET + "- `WEBVIEW2_USER_DATA_FOLDER`"),
    # three trailing clauses that restate each bullet's own opening
    ("does not apply, and no `CONDUCTOR_*` namespace claim is made.", "does not apply."),
    ("carries a configuration contract, and no `CONDUCTOR_*` namespace claim is made.", "carries a configuration contract."),
    ("logged or committed, and no `CONDUCTOR_*` namespace claim is made.", "logged or committed."),
]

with open(PATH, encoding="utf-8", newline="") as f:
    text = f.read()
before = len(text.encode("utf-8"))
print("new bullet bytes", len(NEW_BULLET.encode("utf-8")))
for old, new in EDITS:
    n = text.count(old)
    print(n, "x", repr(old[:70]), len(new.encode("utf-8")) - len(old.encode("utf-8")))
    if n != 1:
        sys.exit(f"REFUSED: {n} matches")
    text = text.replace(old, new)
after = len(text.encode("utf-8"))
print("file bytes", before, "->", after, "delta", after - before)
if "--dry" not in sys.argv:
    with open(PATH, "w", encoding="utf-8", newline="") as f:
        f.write(text)
    print("written")
