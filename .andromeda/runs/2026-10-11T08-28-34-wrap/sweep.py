"""Per-master hit counts for the plan's Expected amendments (wrap) entries.

Each entry's pattern is matched line-wise over its master body and over every registries file the master owns
(the key-file tree is master text). Prints `{entry} · {file} · {lines hit}` and a per-master total.
"""
import pathlib
import re
import sys

root = pathlib.Path(sys.argv[1])
A = root / ".andromeda"
REG = A / "registries"

OWNER = {
    "architecture": ["architecture-contracts.toml", "contracts/architecture"],
    "security-plan": [],
    "design-system": [],
    "layout-templates": [],
    "test-plan": ["test-plan-contracts.toml", "contracts/test-plan"],
    "obs-plan": ["obs-plan-contracts.toml", "contracts/obs-plan"],
    "a11y-plan": ["a11y-plan-contracts.toml", "contracts/a11y-plan"],
}

ENTRIES = {
    "E1 arch": ("architecture", r"conductor-tauri|\bTauri\b|\btauri\b|4444|4445|msedgedriver|webview|WebView2|frontend|wdio"),
    "E2 security": ("security-plan", r"\btauri\b|\bTauri\b|msedgedriver|wdio|Authenticode|rule \(b\)|npm audit|MPL-2\.0|LLVM-exception|tauri::command|speech[- ]log|CONDUCTOR_(MSEDGEDRIVER|NVDA)"),
    "E3 tests": ("test-plan", r"--e2e|conductor-tauri|tauri-cli|wdio|tauri-driver|`a11y` job|`frontend` job|conductor-tauri/ui|desktop-webview"),
    "E4 obs": ("obs-plan", r"conductor-tauri|conductor-ui|desktop-webview|ipc-internal|tauri\.command|--e2e|a11y record"),
    "E5 a11y": ("a11y-plan", r"wdio|CONDUCTOR_A11Y_STRICT|`a11y` job|webview|WebView2|claim-ownership|tauri-driver"),
    "E6 design": ("design-system", r"desktop-webview|status-residual|webview"),
    "E7 layouts": ("layout-templates", r"desktop-webview|--e2e|all three surfaces|webview"),
}


def files_for(master):
    out = [A / f"{master}.md"]
    for rel in OWNER[master]:
        p = REG / rel
        if p.is_file():
            out.append(p)
        elif p.is_dir():
            out.extend(sorted(x for x in p.rglob("*") if x.is_file()))
    return out


for name, (master, pat) in ENTRIES.items():
    rx = re.compile(pat)
    total = 0
    for f in files_for(master):
        n = sum(1 for line in f.read_text(encoding="utf-8").splitlines() if rx.search(line))
        total += n
        if n:
            print(f"{name} · {f.relative_to(root)} · {n}")
    print(f"{name} · TOTAL {master} · {total} · pattern {pat}")
