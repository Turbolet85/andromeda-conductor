# layouts extract

## Relevance
Partial — operator-pause orchestration is a **behavioral/runtime concern** (hold/resume logic), not a **surface layout concern** (wireframe/component placement/focus order).

## Constraints
1. **Desktop surface signatures fixed** — per layout-templates.md §"Signature placement (Paused-count hold-point)" and §"Wireframe — Run console (HOLD)", the operator-pause moment is already *designed* at layout: titlebar count freezes to `count-hold`, phase line flips to "HOLD — operator pause", and the AlertDialog opens. This chunk implements the core hold model, not the dialog UI.
2. **CLI surface signatures fixed** — per layout-templates.md §"Signature placement (Paused-count hold-point)" and §"Output structure — `conductor run <scenario>`", the CLI's `indicatif` heartbeat stops in place and a hold-amber HOLD phase line precedes the `inquire` prompt. This chunk ships the headless auto-resolving behavior; the CLI shell (Epoch 8) surfaces the prompt.
3. **Hold-point is a typed value** — per layout-templates.md §"Component — Primary content block 2" and §"Component — Operator-checklist", holds must render as semantic states in both surfaces (text prompts + visible focus traps). The hold-point model (what the core ships) must be redaction-safe (no host paths / internal names).
4. **No modal/dialog pattern changes** — the go/no-go dialog layout (shadcn AlertDialog, focus-trapped, 200ms fade, frozen count echoed in header per §"Component — Hero / signature section") is **Epoch 9** (Tauri surface work), not this chunk.
5. **Focus/resume determinism** — per layout-templates.md §"Determinism preservation" and §"IA notes — Multi-surface coordination", the hold must preserve seeded stream shape (same seed ⇒ same shape) and not fold wall-clock pause into the deterministic schedule. Layout surfaces (titlebar, matrix header, footer) must reflect "HOLD" state accurately while paused; the core supplies the outcome value for both shells to render.

## Patterns to follow
1. **Signature placement pattern** — per layout-templates.md §"Signature placement strategy", apply the same hold-point *name* on ≥2 distinct components: the desktop titlebar + dialog header (Epoch 9) and the CLI spinner + hold phase line (Epoch 8) both echo the frozen count value, so the held moment is legible at the decision point across surfaces.
2. **Redaction-safe value pattern** — per layout-templates.md §"IA notes — Security guardrails honored at the layout level", hold-point prompts and outcomes must omit absolute paths, internal struct names, and stack traces; the hold model serializes cleanly to artifact and IPC.
3. **No-go is a typed decision, not an error** — per layout-templates.md §"Component — Hero / signature section", a no-go outcome (operator abort) is a legal, recorded *value* (`no-go` variant), never a `Result::Err` or panic. Both surfaces render "Abort" / "no-go" as a distinct state in the report and footer roll-up.
4. **Headless never-blocks invariant** — per layout-templates.md §"Component — Primary navigation (verb structure)" and §"IA notes — Headless invariant", the core resolver must auto-resolve holds in non-TTY runs so agent-driven `agent-run.sh` never blocks; the hold outcome is recorded deterministically, and interactive shells (CLI Epoch 8 / Tauri Epoch 9) override the resolver for human interaction.

## Anti-patterns to avoid
1. **Do NOT add new surface regions or dialog types** — the operator-pause dialog layout is fixed per layout-templates.md §"Wireframe — Run console (HOLD)" (shadcn AlertDialog, focus-trapped, color-raised-3). This chunk supplies the hold *model* and *resolver trait*; Tauri/CLI shells (Epoch 8–9) consume it and render the surface.
2. **Do NOT perturb seeded emission stream** — per layout-templates.md §"Determinism preservation", the hold must not change stream shape or introduce non-seeded jitter. The pause is wall-clock-only; tests use `tokio::time` `start_paused` stubs; any timing stays journal-relative from `std::time`.
3. **Do NOT hide or collapse non-verdict states** — per layout-templates.md §"Component — Primary content block 2", `ManualCheck`, `KnownResidual`, and `Blocked` must remain distinct in renders. A hold outcome (e.g., operator decline of a `ManualCheck`) must surface as a typed value so both shells can render it accurately in the footer roll-up and report.

## Contract bindings
- **Hold model → a11y focus trap** — per layout-templates.md §"Cross-domain bindings to flag" and §"Component — Hero / signature section", the Tauri AlertDialog (Epoch 9) must focus-trap over the paused console and bind Escape to abort. This chunk supplies the hold-point type; a11y derives the ARIA attributes (alertdialog role, aria-modal, aria-describedby for the held value).
- **Hold outcome → desktop/CLI surface rendering** — per layout-templates.md §"Signature placement strategy" and §"Footer / terminator", both shells must render the frozen count value and the abort/proceed decision. This chunk's go/no-go type is the contract; Epoch 8 (CLI) and Epoch 9 (Tauri) bind it to their surfaces.
- **Headless resolver → no-blocking invariant** — per layout-templates.md §"IA notes — Headless invariant", the core's default resolver must auto-resolve so `agent-run.sh` never blocks on a prompt. The resolver abstraction is the seam; CLI/Tauri shells provide interactive overrides (out of scope here).

## Acceptance criteria contributions
1. **(layouts) Hold outcome surfaces in footer roll-up** — the core's go/no-go decision type is recorded so both shells can render it in the footer status line (e.g., "1 Aborted" count with a distinct token color), per layout-templates.md §"Component — Footer (status strip)."
2. **(layouts) Headless hold never blocks** — the core's default resolver auto-resolves to a deterministic outcome (e.g., `go` / proceed) in non-TTY runs; the pause is transparent to `agent-run.sh`, and the outcome is recorded to the artifact, per layout-templates.md §"IA notes — Headless invariant."
3. **(layouts) Hold-point model is redaction-safe** — the typed hold-point (prompt, P-ID, scenario, step) omits absolute paths and internal struct names; it serializes cleanly to the run report artifact and over Tauri IPC, per layout-templates.md §"IA notes — Security guardrails honored at the layout level."
4. **(layouts) Seeded stream shape preserved** — same scenario + seed + auto-resolving resolver ⇒ identical emission-stream shape (hold introduces wall-clock gap only); tests confirm determinism under `tokio::time` `start_paused`, per layout-templates.md §"Determinism preservation."

## Relevant amendment history
(none) — amendment history file does not exist; this is a fresh project.