"""Evolve checkpoint append — wrap-session / curation (0-pending path, chunk: null).

Authored as python dicts and dumped by json.dumps, the sanctioned form: no
hand-escaped JSON, no shell quoting, and the file is run by path from the Bash
tool's own shell.
"""
import json

TS = "2026-09-08T17:07:54Z"
EPOCH = "Epoch 6b — Polish & ship"  # byte-exact from working-route.md:107

BASE = {
    "v": 1,
    "ts": TS,
    "version": "conductor-0.2.0",
    "epoch": EPOCH,
    "chunk": None,
    "skill": "andromeda-wrap-session",
    "step": "curation",
}

records = [
    {
        **BASE,
        "kind": "step",
        "id": f"{TS}-a",
        "outcome": "ok",
        "counts": {"dialogue_rounds": 0, "deferred": 0},
        "consumed": [
            {
                "artifact": "conversation",
                "quality": "ok",
                "note": (
                    "6 candidates surfaced from a setup-project re-run session; 1 applied, "
                    "4 deduped, 1 rejected task-specific"
                ),
            }
        ],
        "produced": [
            {
                "artifact": "conversation",
                "signals": [
                    "correction-applied",
                    "four-curated-rules-held",
                    "no-other-home-signal-fired",
                ],
                "note": (
                    "Four separate curated rules were consulted and correctly applied this "
                    "session, each surfacing as a Filter-1 duplicate rather than a recurrence: "
                    "the bash-to-native boundary rule (hook smoke run from the Bash tool's own "
                    "shell, 6/6 arms green), the token-proxy rule (a 9-hit carve-out sweep read "
                    "rather than pattern-refined, all 9 false positives), the write-time clippy "
                    "ban (hooks-matrix default NOT restored on a setup re-run, which that rule "
                    "explicitly anticipated), and the setup-re-render rule (Tier 2/3 validated, "
                    "not re-rendered). Positives recorded as signals per evolve-system, not as "
                    "untyped friction."
                ),
            }
        ],
        "problem": {
            "nature": "process",
            "solution": "workaround",
            "note": (
                "curation-guide's Corrections disposition prescribes editing the false entry "
                "IN PLACE; the false clause sits in host-win32.md's setup-GENERATED body, which "
                "the same guide's Tier 2 write logic forbids touching. Routed the correction to "
                "## Session Additions instead, extending Filter 1's generated-body reasoning "
                "(stated for the additive-facet case only) to the correction case by analogy."
            ),
        },
    },
    {
        **BASE,
        "kind": "friction",
        "id": f"{TS}-b",
        "type": "contract.skill-reference-drift",
        "what": (
            "curation-guide leaves the correction-against-a-generated-body case unowned: its "
            "Corrections section says to edit the false entry in place, its Tier 2 write logic "
            "says to preserve everything above the Session Additions heading, and Filter 1's "
            "generated-body carve-out ('never takes the additive-facet in-place amend') is "
            "written for the ADDITIVE case only. A measurement-backed correction whose target is "
            "generated body therefore satisfies two colliding rules and no explicit one."
        ),
        "impact": {"extra_reads": 2},
        "artifacts": [
            "references/curation-guide.md",
            "references/curation-tier-decision.md",
            ".claude/rules/host-win32.md",
        ],
        "evidence": ".andromeda/runs/2026-09-08T17-07-54-wrap/adaptation-record.md",
    },
    {
        **BASE,
        "kind": "friction",
        "id": f"{TS}-c",
        "type": "ambiguity.filter-borderline",
        "what": (
            "The one applied candidate scored EXACTLY 0.6 on its base signals (measurement 0.4 + "
            "specific-technical-detail 0.2) — the contract's named scoring mass point, which "
            "rejects — and survived only because the conditional no-other-home +0.2 fired. That "
            "signal's preconditions all had to be checked by hand (this wrap runs no P2, so no "
            "master/playbook/drift-base home; the fact is not a route PREREQ/CARRY; no matrix "
            "ledger note), which is the intended design but is the whole disposition resting on "
            "one conditional."
        ),
        "impact": {"extra_reads": 1},
        "artifacts": ["references/curation-tier-decision.md", ".claude/rules/host-win32.md"],
    },
    {
        **BASE,
        "kind": "friction",
        "id": f"{TS}-d",
        "type": "contract.skill-reference-drift",
        "what": (
            "A shipped rules TEMPLATE states a false host mechanic: setup-project's "
            "rules-templates/host-win32 body says a `cd` in a compound command 'does not persist', "
            "while the Bash tool's working directory demonstrably persists across calls (measured "
            "by a two-call cd/pwd probe; the tool's own documentation states the same). The stale "
            "clause inverts the hazard — the real cost is a REMEMBERED cwd silently rebasing the "
            "next call's relative paths — and it reproduced here as a missing-file failure on a "
            "path that exists. Corrected in the project's Session Additions, which survives a "
            "re-render; the template itself is pipeline territory and remains unfixed."
        ),
        "impact": {"retries": 1},
        "artifacts": [
            ".claude/rules/host-win32.md",
            "references/rules-templates/host-win32.md",
        ],
    },
]

with open(".andromeda/friction-log.ndjson", "a", encoding="utf-8") as fh:
    for rec in records:
        # round-trip so a malformed record fails loudly instead of landing invalid
        fh.write(json.dumps(json.loads(json.dumps(rec, ensure_ascii=False)), ensure_ascii=False) + "\n")

print(f"appended {len(records)} records at {TS}")
