
## 2026-10-03-p-075-re-round-on-incident-events — the cli readiness sample's tool count names the pinned set
**Section:** §Surface: cli → Output structure (the `conductor run` and `conductor suite` readiness-line samples)
**Change:** Both samples were `tools 4/4`; they now read `tools <present>/<required>`, with `<required>` the pinned manifest's `required_tools` set (`contracts/mcp-contract.toml`), never a baked count — and never a substituted `5/5`.
**Why:** The pinned set grew from four to five tools this chunk, and a literal sample re-stales on every addition; the set is owned by the manifest.
**Ref:** .andromeda/runs/2026-10-03T23-44-32-wrap/
