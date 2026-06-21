# layouts extract

## Relevance
Out-of-scope for this chunk

## No domain coverage
This chunk (connection-lifecycle scenarios P-001..P-004) is a **scenario catalog & model-extension** task — it lives in `conductor-core` TOML/serde layers and covers only declarative scenario config, `ExpectedOutcome` wiring, and optional `holds` — not UI layout, surface rendering, or component placement. The layouts domain (per layout-templates §Primary Surfaces) owns the desktop-webview and cli surfaces on which scenarios *eventually* run; this chunk builds the scenario data model that those surfaces consume. No layout structure, wireframe, component hierarchy, focus order, or responsive behavior is affected by scenario authorship.