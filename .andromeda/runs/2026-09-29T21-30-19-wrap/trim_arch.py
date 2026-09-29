import pathlib, sys

p = pathlib.Path("D:/dev/projects/conductor/.andromeda/architecture.md")
src = p.read_bytes().decode("utf-8")
pairs = [
    (
        "the P-025 hue-shift measurement contract: the observable Pulse emits since its P-025 fix `e98d838` "
        "(`metric.constellation.hue_update_ms`, `duration_ms` = paint − `tier_effective_at`, per changed service, "
        "witnessed only) with its literal field names · the resolution · the window as two named Pulse-internal instants · "
        "the comparison constituting a hard grade, plus **§The grading rule, stated before the drive** (its sha256 "
        "recorded in the graded leg's evidence before the leg fired) — under which the ≤2 000 ms budget grades HARD at "
        "the harvest tier, as measured at `conductor-0.3.0/chunks/2026-09-29-hue-shift-budget-graded-hard/evidence/hue-verdict.md` "
        "(PASS, worst 684.98 ms); the RETIRED `83d4060` instrument stays in it as past-tense history under its original "
        "section headings, beside dated corrections. It carries",
        "the P-025 hue-shift measurement contract: the observable Pulse emits since `e98d838` (`hue_update_ms` = paint − "
        "`tier_effective_at`) with its field names · resolution · window · hard grade, and **§The grading rule, stated "
        "before the drive**, under which the ≤2 000 ms budget grades HARD (PASS, worst 684.98 ms, 2026-09-29); the retired "
        "`83d4060` instrument stays as history. It carries",
    ),
    (
        "Its `provenance` is stated **per clause** (MIXED): every Pulse coordinate is a transcribed SUT record read by "
        "`git show` at HEAD `226554a` on 2026-09-29, while the readings are Conductor measurements (the 2026-08-21 legs "
        "against `f0c38f5`, the 2026-09-07 leg against `83d4060`, the 2026-09-29 graded leg against a checkout carrying "
        "`e98d838`) — so a reader can tell how far to trust each; the coordinates expire when that HEAD moves.",
        "Its `provenance` is **per clause** (MIXED): Pulse coordinates are transcribed SUT records read at HEAD "
        "`226554a` (2026-09-29), the readings Conductor measurements — so a reader can tell how far to trust each; the "
        "coordinates expire when that HEAD moves.",
    ),
]
for old, new in pairs:
    if src.count(old) != 1:
        sys.exit("anchor not unique — nothing written")
    src = src.replace(old, new)
p.write_bytes(src.encode("utf-8"))
print("trimmed")
