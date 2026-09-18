
## 2026-09-18 — the baseline for a host-tool or advisory reading is the NEWEST recorded one, found bare

When a chunk report states a DELTA for a host-tool or advisory-database reading — `cargo audit`'s advisory
count, a driver version, a bundler version — the baseline is the **most recent** reading recorded anywhere in
the chunk corpus, located by a bare sweep. It is not the figure a previous report happened to compare against,
and not the first older reading that comes to hand.

Measured twice in one session, in the same direction. This chunk's implement report compared today's 1247
against **1243** (the 2026-09-10 reading), reaching past two newer ones, and framed the delta as "wrong by
four". A wrap directive corrected the baseline to **1246** (`chunks/2026-09-16-scenario-assertion-audit-gate/report.md:229`)
— which itself reached past **1247**, recorded a day earlier at
`chunks/2026-09-17-a11y-routine-arm-terminal-on-the-measured-configuration/report.md:292` with the identical
triple (1247 advisories · 562 packages · 7 allowed). Against the correct baseline the count had moved by
**zero**. Both intermediate figures were artifacts of where the search started, not measurements — and the
directive's own stated principle ("the four is an artifact of reaching past the newer reading") applied one
step further than the directive itself reached.

The sweep that settles it: `grep -rnoE '1[0-9]{3} advisories' conductor-*/chunks/*/report.md`, run **bare**.
Every reading comes back in path order; the newest chunk's is the baseline. The same shape applies to any
figure the corpus records per chunk rather than per commit — a clipped or filtered view of that sweep is what
produces a plausible wrong baseline, and the report template's "a stated number carries its basis" rule is
satisfied by naming this command, not by naming the older report you compared against.
