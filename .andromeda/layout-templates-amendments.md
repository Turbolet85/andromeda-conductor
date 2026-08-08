# Layout Templates — Amendments

_Append-only changelog of amendments to `layout-templates.md` (the body holds only current truth; history lives here + in git). Written by /andromeda-wrap-session P2._

## 2026-06-23-5-command-agent-run-harness — conductor preflight verb + agent-run stage flags registered
**Section:** §Surface: cli — Primary screens (commands)
**Change:** added the `conductor preflight [--json]` verb (the readiness gate / `agent-run boot` entrypoint; exits 0 iff ready:true, else non-zero) to the cli verb list, and named the `agent-run.sh` 5-command set + the `run` `--unit`/`--integration`/`--e2e` stage flags.
**Why:** the chunk landed the `conductor preflight` verb (cli verb surface 3→4) + the `run` stage flags (report §Changes); §cli Primary screens listed only run/suite/report + a generic `agent-run.sh` pointer. The preflight verb is a genuinely-new this-chunk surface (the `report`-verb precedent registered at ch1); D-layout-surface routine. No cascade — layout-templates has no specialist-summary doc and the cli-verb addition does not touch the frontend.md rule (webview-scoped). First amendment to layout-templates.

## 2026-06-23-line-oriented-output-rendering — conductor coverage verb registered; report output + indicatif version corrected
**Section:** §Surface: cli — Primary screens (commands) · §Surface: cli tooling context
**Change:** added the `conductor coverage [--write]` verb (render the static 60-P-ID matrix as a `comfy-table`; `--write` regenerates `coverage-matrix.md` at repo root) to the cli verb list; refined the `conductor report` line to "colored `comfy-table` results view (reads the JSONL journal; the `<run_id>.md` artifact stays Markdown)" per the P4-D2 output switch; corrected the tooling-context `indicatif 0.18`→`0.17` to the resolved lock.
**Why:** the chunk landed the `conductor coverage` verb (cli verb surface 4→5, the P4-D1 user decision) + switched `report` stdout from raw Markdown to the colored results table (P4-D2); §cli Primary screens listed run/suite/report/preflight + agent-run.sh but not coverage, and the report line + indicatif version were stale. The coverage verb is a genuinely-new this-chunk surface (the preflight-verb precedent at ch2); D-layout-surface routine. No cascade — layout-templates has no specialist-summary doc and the cli-verb addition does not touch the frontend.md rule (webview-scoped).

## 2026-06-24-frameless-window-shell — titlebar height space-lg → space-xl
**Section:** §Wireframe (Run console idle) · §Component — Header (frameless titlebar)
**Change:** reconciled the titlebar height `space-lg` (20px) → `space-xl` (32px) in both the idle-console wireframe annotation and the Component-Header spec; `space-lg` cannot contain the `Heading`-tier (18px) phase line + the window-control glyphs.
**Why:** D-layout-surface (warning). The frameless titlebar + window controls ARE already documented in §Component-Header (so the surface invariant holds) — only the height drifted: the implementation shipped `space-xl` because 20px is too short for the 18px heading + 20px controls (report Deviations). Routine per the spec-illustration→sound-impl reconcile rule (the doc tracks the shipped value). No cascade — layout-templates has no specialist-summary doc; the height is webview-scoped but does not change the frontend.md rule (which binds tokens by name, not component heights).

## 2026-08-08-sut-capability-manifest — De-hardcoded cli coverage range labels
**Section:** §Surface: desktop-webview wireframes (coverage header strip) · §Surface: cli (`conductor suite` header, Primary screens)
**Change:** `COVERAGE P-001..P-060` / `CONDUCTOR suite P-001..P-060` range labels and the "all 60 rows" virtual-scroll note now name the manifest's accepted set.
**Why:** Layout labels must not name a span wider or narrower than the rendered row set, which is now manifest-sourced.
