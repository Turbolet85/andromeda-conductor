# obs extract

## Relevance
Partial — Design tokens + typography is foundational frontend infrastructure (Epoch 1); obs observability is a critical-path concern, but this chunk's direct instrumentation scope is minimal (static asset bundle, no runtime telemetry).

## Constraints
- Per obs-plan §1 Obs Scope Summary: Minimal-tier observability applies; no OTel SDK for self-observation; self-observation is structured tracing logs only (JSONL via `tracing-subscriber`).
- Per obs-plan §3 Service identity: Asset bundle inherits service identity via parent surfaces (`conductor-tauri` for Tauri backend). No service-specific instrumentation required at token-bundle level.
- Per obs-plan §1 Telemetry surfaces → desktop-webview: Frontend observability is console JSON logging only — no OTel JS SDK, no browser OTLP exporter (recursion guard). Token stylesheet is static (zero runtime telemetry).
- Per obs-plan §11 Logs anti-pattern: Never log in hot path; design tokens are compile-time/load-time assets, not runtime instrumentation targets.
- Per obs-plan §9 CI Integration: Artifact handling for `logs/agent-latest.jsonl` + supply-chain audit. Tailwind build is part of Tauri workspace; output conforms to frontend artifact path expectations (no separate telemetry per chunk).
- Per security-plan Minimal-tier offline hardening: No runtime CDN fetches; npm/Fontsource deps pinned + lockfiled (supply chain audit signal, not obs direct).

## Patterns to follow
- Design-system token literal ownership: Token names in `@theme` block MUST match `design-system.md` exactly (§Tokens) so Epoch 9 UI chunks bind `var(--…)` without renaming. This is a contract binding, not telemetry.
- Offline vendor-first: Self-hosted `@fontsource/jetbrains-mono` + `@fontsource/ibm-plex-sans` WOFF2 — no runtime CDN, aligning with Minimal-tier determinism (static asset load, zero async fetch telemetry).
- Tailwind v4.1 Oxide engine (static zero-runtime): CSS builds to static stylesheet; no runtime instrumentation scope. A future frontend chunk adding dynamic theme switching would add runtime logging; this chunk is build-time only.

## Anti-patterns to avoid
- No OTel JS SDK for frontend tokens: CSS is compile-time asset; frontend observability (if any) is console JSON only (obs-plan §1, recursion guard). Avoid introducing a browser-side OTel SDK as a token-bundle extension.
- No runtime font fetch instrumentation: WOFF2 are vendored static assets (no CDN fallback, no async fetch span). Do not add fetch instrumentation to the design-token chunk.
- No unstructured CSS build warnings: if Tailwind/Vite build produces warnings, route to stderr with machine-parseable hints (aligns with obs §6 "never unstructured stderr text"); don't emit raw build logs.

## Contract bindings
- Design-system token source: `design-system.md` §Color Palette / §Typography / §Tokens §Surface: desktop-webview — the chunk realizes the token literal shape; downstream Epoch 9 UI chunks reference these names via CSS `var(--…)`.
- Tauri workspace build: Frontend asset layer integrates into `crates/conductor-tauri/`; inherits service identity (`service.name` = `"conductor-tauri"`) for any future runtime observability (deferred to Epoch 9).
- Security supply-chain: npm/Fontsource deps subject to lockfile pinning (security-plan Minimal-tier offline hardening); obs CI gate reads supply-chain audit signal from GitHub Actions artifact.

## Acceptance criteria contributions
- (obs) Static asset build produces zero unlogged errors: Tailwind v4.1 + Vite build must exit code 0 on success; any errors logged to stderr with machine-parseable output (aligns with obs §6 CI log conformance).
- (obs) Design-token names match `design-system.md` exactly: CSS `@theme` block token names must be identical to the source spec; contract pass/fail (verifiable via token-name audit at build time).
- (obs) No runtime CDN fetch instrumentation required: self-hosted WOFF2 are static vendored assets; no Font Loading API instrumentation in this chunk.
- (obs) CI artifact supply-chain audit passes: npm/Fontsource dependencies + lockfile satisfy the audit gate (no security escalations; build gated).

## Relevant amendment history
- **2026-06-15-structured-logging-stack:** Clarified two-record-shape model (self-obs base line vs Run-report envelope). Not directly relevant to the token bundle (static asset, no log lines), but anchors the Tauri backend instrumentation model that will consume these tokens in Epoch 9 (`service.name` = `"conductor-tauri"`).
- **2026-06-15-log-error-boundary-redaction:** Reconciled redaction model: absolute host-file path masking (not `::` type-token redaction); allowlisted `target` preserved. Not directly relevant (static CSS, no logs/panics), but clarifies the boundary the Tauri build wrapper must apply in Epoch 9 (no absolute paths leaked on build failure).
