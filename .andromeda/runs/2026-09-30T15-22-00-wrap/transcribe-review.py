import json

P = "conductor-0.3.0/chunks/2026-09-30-the-sr-pass-regrades-on-the-os-input-path/evidence/nvda-pass.json"
rec = json.load(open(P, encoding="utf-8"))

basis = [
    "The 31 announced-as-expected rows (E0-02 E0-03 E0-04 E0-06 R0-01..R0-04 S0-01..S0-07 S0-16 S1-01 S1-03 S1-04 S2-01..S2-05 S2-07 S2-08 S3-01..S3-05): ACCEPTED as heard.",
    "The 8 announced-differently rows (E0-05 E0-07 E0-08 E0-09 S0-09 S0-15 S3-06 S3-07): FINDING each. The missing token is CONTENT (a P-ID, a status, a fixture name), not NVDA's connective wording, so the value was not conveyed. The review never accepts a row whose required content is absent.",
    "S0-13 (no h1 among three h2) and S0-14 (no contentinfo landmark): FINDING each; product structure, not the harness.",
    "The 8 not-run-here rows (E0-01 S0-08 S0-10 S0-11 S0-12 S1-02 S2-06 T-01): FINDING, reason 'no OS-path browse key was sent in the row window'. The OS path now exists, so this is a leg coverage gap, not an unreachable class.",
    "E0-10 and S1-05 (subject-absent): ACCEPTED with their reasons.",
]

reasons = {}
for f in rec["findings"]:
    rid, rest = f.split(" ", 1)
    reasons[rid] = rest

accepted, withreason, finding = [], [], {}
for row in rec["rows"]:
    o = row["outcome"]
    if o == "announced-as-expected":
        accepted.append(row["id"]); row["review_grade"] = "accepted-as-heard"
    elif o == "subject-absent":
        withreason.append(row["id"]); row["review_grade"] = "accepted-with-reason"
    else:
        # T-01 carries no findings-list entry (a notRun row); its reason is the record's own note.
        finding[row["id"]] = reasons.get(row["id"]) or f"{o} ({row.get('note', '')})"
        row["review_grade"] = "finding"

assert len(accepted) == 31 and len(withreason) == 2 and len(finding) == 18, (len(accepted), len(withreason), len(finding))

rec["operator_review"] = {
    "reviewed_at": "2026-09-30",
    "reviewer": "operator (pc overseer, founder-delegated)",
    "verdict": "reviewed: 31 accepted as heard · 10 findings (8 announced-differently + 2 not-announced) · 8 not-run-here findings (no OS browse key in the row window) · 2 accepted with reason (subject-absent)",
    "basis": basis,
    "grades": {"accepted-as-heard": accepted, "finding": finding, "accepted-with-reason": withreason},
    "notes": (
        "Transcribed by the agent from the operator's relay conductor-wrap-osinput-2026-09-30 §1; the grades are the "
        "operator's judgment, the transcription adds nothing. Each finding's reason is the record's own findings-list "
        "line. One reading the transcription does not carry over: T-01 is a live-class row whose not-run-here reason is "
        "the record's note (the live subject is stopped by design), not a missing OS browse key, so it has no findings-"
        "list line and its reason here is that note; the operator's basis line 4 is kept verbatim."
    ),
}
text = json.dumps(rec, indent=2, ensure_ascii=False) + "\n"
json.loads(text)
open(P, "w", encoding="utf-8", newline="\n").write(text)
print("accepted", len(accepted), "with-reason", len(withreason), "finding", len(finding))
