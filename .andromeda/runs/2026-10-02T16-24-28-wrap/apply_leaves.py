"""Cascade step 3 — re-derive the security leaves' passages from the amended security-plan :121 / :336."""
ROOT = "D:/dev/projects/conductor/"
OLD_PAREN = "(every fingerprint-shaped token — an all-digit run is not one and passes)"
NEW_PAREN = "(every `fingerprint_hex=` value whatever its class, and every unkeyed fingerprint-shaped token — an unkeyed all-digit run, a stamp or a seed, passes)"

EDITS = [
    (".claude/rules/security.md", OLD_PAREN, NEW_PAREN, 2),
    (".claude/rules/security.md",
     "the graded 2026-09-23 copy is elided — the frozen 2026-09-22 file keeps its one `fingerprint_hex` prefix as a stated residual, and the graded 2026-10-01 d3 capture one all-digit synthetic prefix (the canary's storm) — overseer-ruled, founder ratification PENDING.",
     "the graded 2026-09-23 copy is elided. Both formerly stated residuals — the frozen 2026-09-22 file's `fingerprint_hex` prefix and the graded 2026-10-01 d3 capture's all-digit prefix — were FIXED on 2026-10-02 under the founder's ruling, never ratified: the frozen file was elided in place (a recorded exception to frozen evidence never being edited) and equals the graded copy, d3 was re-elided with its pin moved, every committed capture holds zero un-elided `fingerprint_hex` values, and no committed quote of either value remains.",
     1),
    (".claude/docs/security-summary.md", OLD_PAREN, NEW_PAREN, 1),
    (".claude/docs/security-summary.md",
     ", the frozen 2026-09-22 file keeping one `fingerprint_hex` prefix as a stated residual, and the graded 2026-10-01 d3 capture one all-digit synthetic prefix (overseer-ruled, founder ratification pending) |",
     "; both formerly stated residuals (the frozen 2026-09-22 file's prefix, the graded 2026-10-01 d3 all-digit prefix) FIXED 2026-10-02 under the founder's ruling, the frozen file elided in place |",
     1),
]

for path in sorted({p for p, *_ in EDITS}):
    raw = open(ROOT + path, "rb").read()
    text = raw.decode("utf-8")
    lines = text.count("\n")
    for p, old, new, expect in EDITS:
        if p != path:
            continue
        n = text.count(old)
        assert n == expect, "%s: found %d, expected %d" % (path, n, expect)
        text = text.replace(old, new)
    assert text.count("\n") == lines
    open(ROOT + path, "wb").write(text.encode("utf-8"))
    print(path, "written")
