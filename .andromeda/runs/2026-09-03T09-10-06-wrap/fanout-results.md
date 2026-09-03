# Fan-out results — 2026-09-03-conductor-tauri-survivors-dispositioned

7 doc-agents, one parallel batch. Aggregate check A: **all 7 present.**
Coverage sanity (check B): 0 `No domain coverage`-equivalent — every agent evaluated its detectors.

| doc | verdict | detectors evaluated |
|---|---|---|
| arch | `proposals: []` | D-arch-resources · D-arch-decisions · D-platform-claim |
| security | `proposals: []` | D-security-input · D-security-subprocess · D-security-deps · D-platform-claim |
| design-system | **1 proposal** | D-design-tokens (hit) · D-design-derived-count · D-platform-claim |
| layout-templates | `proposals: []` | D-layout-surface · D-layout-derived-count · D-platform-claim |
| test-plan | **5 proposals** | D-tests-derived-count (5 hits) · D-tests-coverage · D-tests-framework · D-tests-obs-harness · D-platform-claim |
| obs-plan | `proposals: []` | D-obs-instrumentation · D-obs-stack · D-obs-redaction · D-platform-claim |
| a11y-plan | `proposals: []` | D-a11y-surface · D-a11y-obs-schema · D-platform-claim |

Raw twins saved for the two docs carrying proposals (both also needed entity-decoding):
`.raw-fanout-design-system.md` · `.raw-fanout-test-plan.md`.

## Notable clean verdicts (reasoning worth keeping)

- **security** distinguished security-plan `:123`'s `tauri::test` reference (the mock-runtime
  **command-dispatch** tier — still live, and exercised by this chunk's new `invoke_expecting_error`
  helper) from v2-25's **parity-arm** claim. Different claims sharing a token; correctly not a hit.
- **obs** noted §4 `:380`/`:383` already sanctions `tokio::time` for fault-span gating, so the
  dev-dependency `time` feature raises nothing against the §11 virtual-clock ban (which scopes to
  journal wall-clock stamps).
- **layouts** separated the `43` at layout-templates `:187` (the manifest-derived *auto capability*
  count) from `Found 43 mutants` — a numeric collision, not a moved count.
- **design** swept its own doc and confirmed `:406` is the ONLY other occurrence of the retired
  `@theme` claim within design-system, so no `dependent-of` proposal is owed there.

## Proposals

### design-system — 1
1. **D-design-tokens** · §Design Decisions Log, 2026-06-14 `Surfaces:` bullet (`:406`) — retire
   "Tailwind v4.1 `@theme`" for the `:root` truth §Tokens `:201` already carries.
   *Basis verified:* `:201` states "declare these on plain `:root`, NOT `@theme`"; the shipped
   `crates/conductor-tauri/ui/src/styles/tokens.css:5` header reads "Declared on :root (not Tailwind's
   `@theme`)" with `:root {` blocks at `:9` and `:60` and no `@theme` rule anywhere.
   *Authority:* this wrap's operator directive item 3 (orchestrator-raised; the M1 fanout surfaced it as
   not-that-chunk's drift). **Routine** — playbook `:28-30` spec-wording→sound-impl reconciliation.

### test-plan — 5
1. **§10 `:500`** — "the `declares` env-reading edge is the shipped case" names a single instance where a
   SET now exists. **Routine** (de-literalization; the detector's own note prescribes naming the set).
2. **§12 `:607`** — record the `conductor-tauri` accepted-deliberate TRIPLE beside `declares` ×6.
   **Routine, and this is the plan's ONE `Expected amendments (wrap)` entry** — check 5 satisfied by a
   detector proposal rather than an orchestrator raise. Purely additive: the cascade check found no
   exclusivity claim anywhere that the addition would falsify.
3. **§4 `:226`** — the mutants read-out gate says `missed.txt` **empty**. → **ESCALATED** (see below).
4. **§7 `:399`** — the committed-fixture meaning-pinner SET names two `conductor-run` files; a third
   member now exists (`the_committed_scenarios_fixture_stays_loadable`). **Routine** (de-literalization).
5. **§7 `:398`** — the sanctioned-provenance SET names `crates/conductor-run/tests/fixtures/` literally;
   a committed fixture now lives outside that path. **Routine**, `dependent-of` #4.

## Orchestrator-raised (no detector's scope)

- **Cascade — layout-templates `:11` and `:305`** carry the same retired `@theme` mechanism as
  design-system `:406` (`:11` "CSS Tailwind v4.1 (`@theme` Oxide static stylesheet)"; `:305`
  "React 19 / Tailwind v4.1 `@theme` / shadcn"). Neither is in any layouts detector's scope — they are
  cross-master **cascade** consequences of amending design-system, which the flow mandates folding into
  THIS pass. **Routine.**
- **Not hits, checked and dismissed:** `a11y-plan:447` ("the two font-family tokens are machine-assertable
  as `@theme` tokens") is a correct statement about Tailwind NAMESPACE tokens, which survive `@theme`
  tree-shaking — consistent with `:201`'s own explanation. `architecture.md:28` and `:256` already state
  "design tokens on `:root`", naming `@theme` only as the reason not to use it.

## Validation outcome

- **Check 1 (playbook):** 7 routine · 1 escalate.
- **Check 2 (cross-contradiction):** none — no two proposals edit the same section in opposing directions.
- **Check 3 (intent-consistency):** the report matches the chunk's working-route entry and plan
  acceptance criteria; no divergence.
- **Check 4 (absence needs evidence):** every "only occurrence" claim re-derived by the orchestrator with
  its own grep across all seven masters, not taken from an agent's word.
- **Check 5 (expected-amendments):** the plan's one entry is covered by test-plan proposal 2. ✓
- **Check 6 (disproved-claims disposition):** the report's one entry (`v2-25`'s parity-arm premise) is
  routed to the channel that owns it — a PREMISE-CORRECTION `notes` narrative on the matrix at P7.3, per
  operator directive item 2. **DISPOSED**, not silent.
