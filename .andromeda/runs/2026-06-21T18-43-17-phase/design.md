# design extract

## No domain coverage
The runs.db index chunk is a backend SQLite schema & rusqlite synchronous write seam — no UI surface, no typography, no token-dependent rendering. The design system covers the desktop-webview (React + Tailwind) and cli frontends, not the persistence layer. The envelope schema (`RunRecord`) this chunk persists was shaped by the verdict/report-state triad + NULL-for-blocked conventions, but those are already captured in conductor-core and the arch contract — no new design decisions apply to the index implementation.

_(Normalized by orchestrator: the agent emitted a `## Relevance` line plus `## No domain coverage`; collapsed to the canonical out-of-domain form. Meaning unchanged — backend, no rendered surface.)_
