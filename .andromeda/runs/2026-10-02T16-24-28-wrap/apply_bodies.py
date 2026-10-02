"""P2 apply — five body edits (S1, S2, S3, A1, T1), each an exact single-occurrence replacement."""
A = "D:/dev/projects/conductor/.andromeda/"

EDITS = [
    ("security-plan", "S1",
     "(a fingerprint-shaped token — eight or more lowercase hex digits holding at least one letter — prints as `<fingerprint>`, so an all-digit run passes by definition: the 2026-10-01 d3 residual, §Security Anti-Patterns → Data Protection; envelope `fingerprints` stay elided to counts)",
     "(two rules since 2026-10-02 — KEYED: the alphanumeric value after every `fingerprint_hex=` prints `<fingerprint>` whatever its class, all-digit included, an existing `<fingerprint>` or an empty keyed value left as it is, the shape of the `workspace=` rule; UNKEYED: a fingerprint-shaped token — eight or more lowercase hex digits holding at least one letter — prints as `<fingerprint>`, while an unkeyed all-digit run, a nanosecond stamp or a seed, passes; the harvest walks every committed `rm-capture*.txt` under `conductor-0.3.0/chunks/*/evidence/`, workspace-root anchored from `CARGO_MANIFEST_DIR` with no handle, and holds each at zero un-elided keyed values and a fixed point of the elision, the population pinned by count beside an inverse control; envelope `fingerprints` stay elided to counts)"),
    ("security-plan", "S2",
     "every fingerprint-shaped token elided — an all-digit run is not one by that tool's definition and passes (§Input Validation — the real-model capture ingest row)",
     "every `fingerprint_hex=` value elided whatever its class, and every unkeyed fingerprint-shaped token — an unkeyed all-digit run is a stamp or a seed and passes (§Input Validation — the real-model capture ingest row)"),
    ("security-plan", "S3",
     "Stated residual: the FROZEN 2026-09-22 file keeps its one un-elided storm `fingerprint_hex` prefix, because frozen evidence is never edited — it is not the graded copy. A second residual is RULED BY THE OVERSEER, FOUNDER RATIFICATION PENDING (2026-10-01; a delegate does not decide whether it is a widening): the GRADED 2026-10-01 `rm-capture-d3.txt` carries one all-digit 8-character `fingerprint_hex` prefix on two lines — the preflight canary's synthetic storm, no real data — which the harvest counts exactly (`the_2026_10_01_captures_carry_no_fingerprint_and_no_workspace_key`); no elision code changed.",
     "Both formerly stated residuals were FIXED on 2026-10-02 under the founder's ruling, never ratified: the frozen 2026-09-22 file was elided in place on its two keyed lines — a recorded exception to \"frozen evidence is never edited\" — and is byte-identical to the graded copy; the graded 2026-10-01 `rm-capture-d3.txt` was re-elided on its two keyed lines and its digest pin moved, and `the_2026_10_01_captures_carry_no_fingerprint_and_no_workspace_key` asserts zero all-digit keyed values for every drive; every committed real-model capture carries zero un-elided `fingerprint_hex` values (`every_committed_capture_carries_no_un_elided_fingerprint_value`), and no committed quote of either value remains in the tree (git history keeps the originals and was not rewritten)."),
    ("architecture", "A1",
     "; the frozen 2026-09-22 file keeps its one `fingerprint_hex` prefix as a stated residual; a second — the graded 2026-10-01 d3 capture's all-digit synthetic prefix, which `elide_fingerprints` keeps by definition — is overseer-ruled, founder ratification pending)",
     "; both stated residuals — the frozen 2026-09-22 file's prefix and the graded 2026-10-01 d3 capture's all-digit prefix — were fixed 2026-10-02 under the founder's ruling, the frozen file elided in place)"),
    ("test-plan", "T1",
     ", the frozen 2026-09-22 file keeping its prefix as a stated residual, and the 2026-10-01 d3 capture one all-digit `fingerprint_hex` prefix (the canary's synthetic storm; `elide_fingerprints` keeps all-digit runs by definition) that the harvest counts exactly — overseer-ruled, founder ratification pending.",
     ", the frozen 2026-09-22 file since elided in place (2026-10-02, the founder's ruling; a recorded exception to frozen evidence never being edited) and byte-identical to it. Since 2026-10-02 `elide_fingerprints` elides every `fingerprint_hex=` value whatever its class — the 2026-10-01 d3 capture's all-digit prefix re-elided and its pin moved, the 2026-10-01 arm asserting zero for every drive — and a population arm (`every_committed_capture_carries_no_un_elided_fingerprint_value`) holds every committed `rm-capture*.txt` at zero un-elided keyed values and a fixed point of the elision, the population pinned by count, beside its inverse control (`un_elided_keyed_values_counts_a_planted_value`)."),
]

for doc in sorted({d for d, *_ in EDITS}):
    path = A + doc + ".md"
    raw = open(path, "rb").read()
    text = raw.decode("utf-8")
    before_lines = text.count("\n")
    for d, label, old, new in EDITS:
        if d != doc:
            continue
        n = text.count(old)
        assert n == 1, "%s: old span found %d times" % (label, n)
        text = text.replace(old, new)
        print(label, "applied · bytes delta", len(new.encode()) - len(old.encode()))
    assert text.count("\n") == before_lines, doc + ": line count moved"
    open(path, "wb").write(text.encode("utf-8"))
    print(doc, "written · lines", before_lines)
