# Fan-out results — 2026-09-03-live-pulse-preconditions-probed

7 Explore doc-agents, one per spec source, one parallel batch. **36 proposals.**

| Doc | Proposals | Verdict | Twin |
|---|---|---|---|
| architecture | 8 (3 primary + 5 sweep) | 7 apply · 1 dismiss (`:61`) | `.raw-fanout-architecture.md` |
| security-plan | 6 (2 primary + 4 sweep) | **6 escalate** | `.raw-fanout-security-plan.md` |
| design-system | 5 (2 primary + 3 sweep) | 5 apply | `.raw-fanout-design-system.md` |
| layout-templates | 7 (3 primary + 4 sweep) | 7 apply | `.raw-fanout-layout-templates.md` |
| test-plan | 9 (4 primary + 5 sweep) | 9 apply | `.raw-fanout-test-plan.md` |
| a11y-plan | 1 | 1 apply | `.raw-fanout-a11y-plan.md` |
| obs-plan | **0 — clear** | no drift | *(no twin — empty-and-clean; recorded here)* |

## obs-plan — the clean return, recorded

All four detectors returned reasoned negatives rather than a bare null:

- **D-obs-instrumentation** — clear. Every new operation symbol carries §4/§6 instrumentation: `conductor preconditions` and `observe_preconditions` each emit one `info!`/`warn!` on the allowlisted `message` field inside the caller's span with NO new span name (bounded §11 set unwidened); `sidecar_resolves_on_path` is a pure predicate logged by its caller; the `boot` arm inherits the probe's line. `run_id` minted by `ServiceIdentity::resolve`; the probe mints no journal row, so no virtual-clock stamp is possible. §6's boundary-call enumeration does not yet name the probe, but that is doc incompleteness, not one of the three drift conditions the check names.
- **D-obs-stack** — clear. Dependencies "none added, none bumped", `Cargo.lock` byte-unchanged, 564 → 564, `which` explicitly rejected. No OTel SDK, no OTLP exporter enters the tree; `:4317` is contacted as a CLIENT only via `probe_egress`, emitting no OTLP.
- **D-obs-redaction** — clear. Every new surface reports PII redacted ✓; the negative test asserts no host path, no `PATH` contents, no struct name in BOTH rendered and JSON forms; `sidecar_resolves_on_path` returns `bool` and never the resolved path; `observe_preconditions` carries only handle NAMES.
- **D-platform-claim** — clear. obs-plan names `scripts/agent-run.sh` only at `:42` (the CI arrangement — a choice, not a capability) and `:237` (a usage example); neither states a capability verdict for the measurement to retire. `anstream` is absent from obs-plan (grep: zero hits); `:79`/`:202`'s "no TTY detection" scopes the log SINK's JSON format, not the render layer's colour gate.

## Validation summary (orchestrator)

- **Check 1 — playbook.** `:28` governs the 5 `anstream` reconciles (routine). `:61` governs and **dismisses** arch's CLI-verb registration (qualifiers verified: this chunk added no port/socket/endpoint/IPC/event/env-var/crate; the verb's home is layout-templates §cli Primary screens, where it landed). `:58`'s dismiss precondition **fails** for the security set — the chunk *does* add external-input surfaces — so those are not dismissible; `:124` (boundary widening, never-routine) plausibly governs → escalated.
- **Check 2 — cross-contradiction.** None. The `anstream` proposals all move one way; the derived-count proposals all widen a singular to a named set.
- **Check 3 — intent-consistency.** Aligned. The report's Outcome table re-asserts all 16 acceptance criteria against the diff; none contradicted.
- **Check 4 — absence needs evidence.** Every absence claim cites its search: a11y named `:119`/`:149` as checked-and-not-the-claim; design named `:321`/`:282`/`:406`; obs named `:42`/`:237`/`:79`/`:202`; security named `:113`.
- **Check 5 — expected-amendments reconciliation.** All 4 plan entries covered: (1) arch verb+boot arm → arch #3 + layout #1/#2 (verb dismissed from arch per `:61`, applied at its correct home); (2) arch `mcp-enabled` + `sidecar-built` → arch #5/#8; (3) test-plan §3 boot composition → test-plan #4–#7; (4) `anstream` → design #3–#5 + layout #6/#7.
- **Check 6 — disproved-claims disposition.** Both report entries DISPOSED: `anstream` matched by 5 proposals; `sidecar-built` rationale matched by arch #8.

## Orchestrator-caught proposal defect

design-system's `§Toolkit` change line carries collateral facts outside its detector's scope — it asserts `indicatif 0.18` and `inquire 0.7`. Measured against the workspace manifest and lockfile: **`indicatif 0.17.11`**, **`inquire 0.9.4`**. The report's Dependencies bullet says none added or bumped, so only the `anstream` half of that proposal is applied and no version is touched.
