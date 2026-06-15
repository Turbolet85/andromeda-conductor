# Session Learnings

_This file is curated by `/wrap-session`. Learnings captured here are too detailed or specific for CLAUDE.md but worth preserving as reference material for future sessions._

_Entries are added in reverse chronological order (newest first). Each entry has an ISO date, short title, and body._

_This file is entirely wrap-session's territory. `/andromeda-setup-project` creates it if missing but NEVER regenerates it. Manual edits are preserved across all Andromeda skill runs._

---

## 2026-06-15 — garde 0.22.1 API gotchas (config validation)

Conductor pins **garde 0.22.1**, not the arch's original 0.23.0: `garde_derive 0.23.0` is absent from the registry (latest 0.22.1), so garde 0.23.0 + the `derive` feature is unbuildable here. When wiring garde into a seam crate:
- `derive` is **not** a default feature — the edge must be `garde = { workspace = true, features = ["derive"] }`, or `#[derive(Validate)]` / the `#[garde(...)]` helper attribute won't resolve ("cannot find derive macro `Validate`").
- `Validate::validate(&self)` takes **no** context argument (returns `Result<(), garde::Report>`); call `.validate()`, not `.validate(&())`, for the default `()` context. The error type is `garde::Report`, bridged into `CoreError` via `#[from]`.
- `#[garde(custom(fn))]` is **field-level only** — there is no container/struct-level `custom` in 0.22.1 (it errors "unrecognized attribute"). A whole-list invariant rides on the one field it concerns (e.g. no-duplicate-P-IDs on `p_ids`); invariants spanning *distinct* fields (p50≤p95≤p99, severity-mix sums — the Epoch-2 emission spec) need garde's `Context` pattern or a manual `Validate` impl.

Applies to every future garde validation surface (Epoch-2 `Scenario-config model` especially). See arch §Established Decisions [Validation Library] for the pinned-version decision.

---

---

## Entry format

```
## {ISO-date} — {short title}
{1-3 paragraphs describing what was learned, why it matters, and where it applies. Reference specific files or documented decisions when relevant.}
```

## Tier classification

This file is **Tier 3 — on-demand**. Claude reads it when explicitly needed (debugging, planning, reviewing patterns), not at session start.
- **Tier 1** (always loaded) — universal safety rules in `CLAUDE.md` `USER:session-learnings` (critical, short).
- **Tier 2** (path-triggered) — directives in `.claude/rules/*.md` `## Session Additions` (loaded when matching files touched).
- **Tier 3** (on-demand) — this file (detailed reference, lazy-read).
