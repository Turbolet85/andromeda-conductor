# knip — first report on the web plane (2026-09-07)

`npm --prefix crates/conductor-tauri/ui run knip`, knip installed as a devDependency at this chunk.
This closes the boundary-#4 code audit's A5 `dead-code-web` column, which read
`tool-missing: knip absent (not in package.json, not in node_modules)`.

**Nothing was deleted.** a11y-plan §11 (Anti-Patterns → CI / Strategy) forbids acting on a static
dead-code report against the a11y harness, and design-system §Component Patterns 7 protects a
primitive whose second mount is designed-but-not-built. Every finding is dispositioned below.

## Verdict: 20 findings, 20 false positives — all one class

knip resolves reachability by **import graph from its configured entry points**. This package's
largest consumer is **WebdriverIO**, which discovers specs through `wdio.conf.ts`'s `specs` /
`suites` config and loads devDependencies through its own plugin resolution — neither is an import
edge, so knip cannot see any of it. Every finding traces to that one blind spot.

### Unused files (3) — KEPT
| File | Why knip is wrong |
|---|---|
| `test/a11y/accessibility.e2e.ts` | The routine `--e2e` arm's spec. Discovered by `wdio.conf.ts` `specs`, run via `agent-run run --e2e`. |
| `test/a11y/operator-hold.e2e.ts` | The operator-only driven arm (`npm run a11y:driven`), a11y-plan §9 E2E row. |
| `test/a11y/screen-reader.e2e.ts` | The three `sr*` suites (`a11y:sr` / `sr-empty` / `sr-error`), a11y-plan §9. |

### Unused devDependencies (5) — KEPT
| Package | Why knip is wrong |
|---|---|
| `@axe-core/webdriverio` | a11y-plan §3 `a11y-tooling-install` mandates it BY NAME. Injected into the driver session, not imported by app code. |
| `axe-core@4.12.0` | Same §3 phase, pinned by version. The runtime axe step is mandatory — §11 bans lint-only a11y. |
| `colorjs.io@0.6.1` | a11y-plan §3 `contrast-verification-harness-setup`, pinned. Reads `:root` token pairs for WCAG-AA contrast. |
| `lighthouse@13.0.3` | a11y-plan §3, pinned. Its §3 role is a **secondary coarse gate**, so it legitimately has no importing spec — exactly the shape research flagged as knip-visible. |
| `@wdio/local-runner` | Loaded by WebdriverIO's own config resolution (`runner: 'local'`), never imported. |

### Unused exports (3) and exported types (9) — KEPT
Nine of the twelve are in `test/a11y/screen-reader/parse-nvda-log.ts` + `rows.ts` — the SR leg's
speech-log parser, whose consumers are the spec files knip already (wrongly) called unused, so these
inherit the same false positive. The remaining three are app-side surface:
`SUITE_SELECTION` (`ScenarioPicker.tsx`), `CoverageMode` (`CoverageMatrix.tsx`), and the parser's
`readSpeechLog` / `readStamps`.

## Owed, not done here: an entry-point config

As shipped, knip reports 20 findings and 0 are real — a dead-code series with a 100% false-positive
rate is not usable, and a future reader could act on it. Making it useful needs a `knip.json`
declaring the wdio entry points and the config-loaded devDependencies (knip ships a `wdio` plugin for
exactly this).

**Not done in this chunk, deliberately.** The plan's step 10 scopes the work to "a devDependency plus
a `knip` script … Run it once and disposition every finding in writing without deleting anything",
and the touchpoint list carries `New files: None`. Choosing which entry points to declare is a design
decision with real content, so it is surfaced here rather than improvised — see the chunk report.
