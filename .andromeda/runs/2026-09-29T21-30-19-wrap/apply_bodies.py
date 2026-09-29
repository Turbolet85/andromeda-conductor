import pathlib, sys

ROOT = pathlib.Path("D:/dev/projects/conductor/.andromeda")


def replace_once(path, old, new):
    p = ROOT / path
    src = p.read_bytes().decode("utf-8")
    n = src.count(old)
    if n != 1:
        sys.exit(f"{path}: old span matched {n} times — nothing written")
    out = src.replace(old, new)
    p.write_bytes(out.encode("utf-8"))
    back = p.read_bytes().decode("utf-8")
    assert new in back and old not in back, f"{path}: read-back failed"
    print(f"{path}: applied ({len(old)} → {len(new)} chars)")


# --- architecture.md:181 (A1) ---
arch_old = (
    "the P-025 hue-shift measurement contract: what Pulse would have to emit for its delegated ≤2 000 ms hue-shift "
    "budget to become measurable at all (the observable and its literal field names · the resolution · the window as "
    "two named Pulse-internal instants · the comparison constituting a hard grade), carrying"
)
arch_new = (
    "the P-025 hue-shift measurement contract: the observable Pulse emits since its P-025 fix `e98d838` "
    "(`metric.constellation.hue_update_ms`, `duration_ms` = paint − `tier_effective_at`, per changed service, "
    "witnessed only) with its literal field names · the resolution · the window as two named Pulse-internal instants · "
    "the comparison constituting a hard grade, plus **§The grading rule, stated before the drive** (its sha256 "
    "recorded in the graded leg's evidence before the leg fired) — under which the ≤2 000 ms budget grades HARD at "
    "the harvest tier, as measured at `conductor-0.3.0/chunks/2026-09-29-hue-shift-budget-graded-hard/evidence/hue-verdict.md` "
    "(PASS, worst 684.98 ms); the RETIRED `83d4060` instrument stays in it as past-tense history under its original "
    "section headings, beside dated corrections. It carries"
)
replace_once("architecture.md", arch_old, arch_new)

arch_old2 = (
    "Its `provenance` states it is a **Conductor measurement, not a transcribed SUT record** — the inverse of the "
    "load envelope and the run contract — so a reader can tell how far to trust it: the readings are Conductor's own "
    "(the 2026-08-21 and 2026-09-07 live legs) while every Pulse coordinate it cites is a reading of the SUT as "
    "measured at HEAD `83d4060` on 2026-09-13 and expires when that moves."
)
arch_new2 = (
    "Its `provenance` is stated **per clause** (MIXED): every Pulse coordinate is a transcribed SUT record read by "
    "`git show` at HEAD `226554a` on 2026-09-29, while the readings are Conductor measurements (the 2026-08-21 legs "
    "against `f0c38f5`, the 2026-09-07 leg against `83d4060`, the 2026-09-29 graded leg against a checkout carrying "
    "`e98d838`) — so a reader can tell how far to trust each; the coordinates expire when that HEAD moves."
)
replace_once("architecture.md", arch_old2, arch_new2)

# --- obs-plan.md:350 (O1 + O2, one merged edit of the P-025 passage) ---
obs_start = "**P-025's ≤2s bound is UNMEASURABLE through `metric.constellation.hue_update_ms`"
obs_end = "Every Pulse coordinate here is as measured at HEAD `83d4060` on 2026-09-13 and expires when that moves."
p = ROOT / "obs-plan.md"
src = p.read_bytes().decode("utf-8")
i, j = src.find(obs_start), src.find(obs_end)
if src.count(obs_start) != 1 or src.count(obs_end) != 1 or not (0 <= i < j):
    sys.exit("obs-plan.md: P-025 passage anchors not unique/ordered — nothing written")
obs_old = src[i:j + len(obs_end)]
obs_new = (
    "**P-025's ≤2s bound grades HARD at the harvest tier through `metric.constellation.hue_update_ms`, as measured at "
    "`conductor-0.3.0/chunks/2026-09-29-hue-shift-budget-graded-hard/evidence/hue-verdict.md`: PASS, worst 684.98ms — "
    "one graded leg against a Pulse checkout (`4502d5d`) carrying the P-025 fix `e98d838`, whose in-window samples were "
    "a rise (684.98ms, anchored to its incident's `interpretation.incident.created` line at 29.98ms) and a fall to "
    "`none` (430.79ms).** Since `e98d838` the leaf's `duration_ms` is the paint instant minus "
    "`ServiceListItem.tier_effective_at_unix_nano`, the instant the service's maximum incident tier last changed: the "
    "raising incident's `opened_at_unix_nano` on a rise, the last max-holder's `resolved_at_unix_nano` on a fall "
    "(acknowledgement does NOT lower the tier), one sample per changed service and only for a change the canvas "
    "witnessed. The grade is `grade_in_window` "
    "(`delegated_timing_harvest.rs::p025_the_graded_leg_meets_its_two_second_budget_at_its_real_value`) under the rule "
    "`contracts/pulse-p025-measurement-contract.md` §The grading rule states before the drive: every sample in "
    "[phase-2 start, `scenario.run` close], worst observation, inclusive, absence UNGRADED, a breach a hard Fail "
    "relayed to Pulse — so all four delegated-timing bounds now grade hard. SCOPE: deterministic L4, the compact widget "
    "visible, the rise in the poll-bounded case (the dot listed before the flip). The RETIRED instrument at Pulse HEAD "
    "`83d4060` could not grade the bound, as measured at "
    "`conductor-0.2.0/chunks/2026-09-06-halo-hue-budget-re-driven/evidence/hue-verdict.md`: its leaf reported "
    "`t_sample − t_last_refreshing_tick` (TICK QUANTIZATION, U(0, 15s) at a randomly-timed flip, independent of the "
    "dispatch rate — 14525.9ms matching its offset to the preceding tick to 1.1ms), under slowest-wins aggregation and "
    "a staleness start instant; that reading stays pinned as the retired instrument's record "
    "(`p025_the_re_driven_leg_measures_tick_quantization_not_update_latency`). Neither SUT-side candidate once named "
    "as lifting that bound (stamping `last_seen` from span arrival; copying the activity floor's "
    "`last_observed_unix_nanos`) would have done so — each addressed the staleness term alone, as measured at "
    "`conductor-0.3.0/chunks/2026-09-13-p-025-measurement-contract-for-pulse/research.md` — and the shipped fix is "
    "neither: it exposes the tier-effective instant. The bound is never bound to a journal-relative term "
    "(`latency_ms` / `budget_ms` / `effective_deadline_ms`). Pulse coordinates here are read at HEAD `226554a` on "
    "2026-09-29 unless dated `83d4060`, and expire when that HEAD moves."
)
out = src[:i] + obs_new + src[j + len(obs_end):]
p.write_bytes(out.encode("utf-8"))
back = p.read_bytes().decode("utf-8")
assert obs_new in back and obs_old not in back
print(f"obs-plan.md: applied ({len(obs_old)} → {len(obs_new)} chars)")
