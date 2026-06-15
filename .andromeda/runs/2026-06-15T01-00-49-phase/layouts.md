# layouts extract

## No domain coverage
Config validation is a backend / load-time validation concern (serde + garde rules, path canonicalization) — not a layout/surfaces concern. Layout templates cover visual structure, wireframes, component placement, focus order, and responsive breakpoints for the desktop-webview and cli surfaces. Config validation sits in the engine/crate boundary and does not introduce new visual elements, modify existing surfaces, or affect user-visible layout structure. The layout-templates plan has zero coverage of validation semantics, error bridging, or path guards.