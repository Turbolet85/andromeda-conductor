# Cascade dispositions — 2026-10-04-host-portable-tauri-ipc-tests

The pass carries ONE amendment: architecture §Infrastructure Patterns → Build system (`architecture.md:214`), the
2026-09-01 custom-protocol origin measurement qualified as the Windows form, the origin stated per-OS.

## The search
`cascade.py sweep --patterns-file cascade-patterns.toml` (cascade v1.1, baseline `f5076ad9`), over the seven masters,
`.andromeda/registries/**`, the three curation homes, the two judgment bases and the leaf bodies:
- `origin-literal` — regex `tauri\.localhost` (the retired claim's own value; control fired at `architecture.md:214`)
- `custom-protocol` — fixed `custom-protocol` (the mechanism the claim is stated under; control `architecture.md:150`)
- `opened-on` — regex ``opened on `http`` (the measurement's verb; control `architecture.md:214`)
Patterns not runnable: `tauri://localhost`, `Plugin not found`, "mock webview's real origin" — none appears in any
pre-pass master, so the tool would refuse them for a control that cannot fire. Their sites were searched by the
report's python sweep of the seven masters (0 hits outside `:214`) and by `origin-literal` here (which reaches the
curation homes and leaves, where `testing.md:65` carries the fixed-literal prescription).
Sections read: `architecture.md:214` in full (P1 + the amended window @c4763-5163).

## Rows (13) — every row dispositioned
| row | disposition |
|---|---|
| `architecture.md:214` origin-literal (edited, ×2 @c4804,4963) | AMENDED — the two matches are the kept Windows measurement and the new per-OS clause; no retired wording stands |
| `architecture.md:214` custom-protocol (edited, ×4) | no change — the `custom-protocol` FEATURE decides bundle embedding (true); the two added matches sit in the amended clause |
| `architecture.md:214` opened-on (edited, ×2 @c4696,4786) | no change — `opened on http://localhost:5173` (the dev build, true) and the kept Windows measurement, now host-scoped |
| `architecture.md:150` custom-protocol | no change — the bundled-vs-devUrl port statement; states no origin value |
| `security-plan.md:370` custom-protocol | no change — the remote-origin-iframe ban citing CVE-2026-42184's origin confusion; not the origin value |
| `test-plan.md:155` custom-protocol ×2 (@c1138, @c2754) | no change — the `--live` leg order and the `--e2e` release-build recipe; feature, not origin |
| `test-plan.md:464` custom-protocol | no change — the `--e2e` recipe; feature, not origin |
| `.claude/rules/testing.md:65` origin-literal (curation) | ROUTED to P3 — the 2026-06-27 Session Addition prescribes the fixed literal and says `tauri://localhost` fails dispatch; false on Linux. An in-place correction of that entry (preserve-verbatim home, never a cascade edit) |
| `.claude/rules/frontend.md:51` custom-protocol (curation) | no change — the feature-not-profile gotcha (true) |
| `.claude/rules/verification-harness.md:57` custom-protocol ×2 (curation) | no change — the feature-not-profile correction (true) |
| `.claude/rules/verification-harness.md:62` custom-protocol (curation) | no change — the `--e2e` build recipe (true) |
| `.claude/docs/commands.md:30` custom-protocol (leaf) | no change on recompute — the `--e2e` recipe states the feature, no origin |
| per-pattern count lines | read; no `base` (playbook / drift-base) hit for any pattern |

## Leaves (step 3)
architecture changed in §Infrastructure Patterns → Build system. Recomputed: CLAUDE.md `GENERATED:setup:*` blocks
(overview / modules / warnings / pointer table / architecture) carry no Build-system origin fact — no change;
`.claude/docs/stack.md` and the other `Extracted from` architecture leaves carry no origin value (`origin-literal` →
0 leaf rows) — no change. `.claude/docs/commands.md:30` (the one leaf row) — no change. No plan changed, so no
specialist summary or rule file re-derives. Binds (test-plan §3 ↔ obs-plan §3; a11y ↔ obs schema): untouched.
