import re, subprocess, sys, os

ROOT = "."
RUN = ".andromeda/runs/2026-09-30T15-22-00-wrap"
TOOLS = os.environ.get("ANDROMEDA_TOOLS", "andromeda-tools/scripts")
M = "2026-09-30-the-sr-pass-regrades-on-the-os-input-path"
REPORT = "conductor-0.3.0/chunks/" + M + "/report.md"
os.chdir(ROOT)

db = open(".andromeda/drift-base.md", encoding="utf-8").read()
body = db[db.index("## Detectors"):]
entries = re.split(r"(?m)^(?=- id: )", body)[1:]
by_doc = {}
for e in entries:
    e = re.sub(r"(?m)^# .*\n", "", e).rstrip() + "\n"
    for doc in [d.strip() for d in re.search(r"(?m)^  doc: (.+)$", e).group(1).split("|")]:
        by_doc.setdefault(doc, []).append(e)

docs = {"architecture": "arch", "security-plan": "security-plan", "design-system": "design-system",
        "layout-templates": "layout-templates", "test-plan": "test-plan", "obs-plan": "obs-plan", "a11y-plan": "a11y-plan"}
alias = {"a": "a11y-plan"}
for k in list(by_doc):
    if k in alias:
        by_doc.setdefault(alias[k], []).extend(by_doc.pop(k))

template = open(RUN + "/doc-agent-template.txt", encoding="utf-8").read()
for doc, key in docs.items():
    dets = by_doc.get(key, [])
    out = f"{RUN}/{doc}-contracts.md"
    r = subprocess.run([sys.executable, "-X", "utf8", f"{TOOLS}/registry.py", "contracts", "--master", f".andromeda/{doc}.md",
                        "--out", out, "--run-dir", RUN, "--marker", M], capture_output=True, text=True)
    if r.returncode == 0 and os.path.exists(out):
        cl = f"- your keyed contracts (they left the body; one row per key): {out} — read every key file a detector or a Change touches"
    else:
        cl = None
    p = template.replace("{doc_path}", f".andromeda/{doc}.md").replace("{report_path}", REPORT)
    p = p.replace("{detectors_yaml}", "".join(dets).rstrip()).replace("{doc}", doc)
    p = p.replace("{contracts_line}\n", (cl + "\n") if cl else "")
    left = re.findall(r"\{(doc|doc_path|report_path|detectors_yaml|contracts_line)\}", p)
    open(f"{RUN}/prompt-{doc}.txt", "w", encoding="utf-8", newline="\n").write(p)
    print(doc, "detectors", len(dets), "contracts", r.returncode, (r.stdout.strip().splitlines() or [""])[-1][:90], "unsubstituted", left)
