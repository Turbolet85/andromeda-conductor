# layouts extract

## Relevance
Partial. This chunk changes no surface, region, focus order, modal or breakpoint. The layouts domain only touches it where architecture §Occupied Resources rows repeat facts that layout-templates owns for the cli and desktop-webview surfaces.

## Constraints
- layout-templates is the only place that lists the cli verbs. Per layout-templates §Component — Primary navigation (verb structure), the verb SET is "whichever `Commands` declares, enumerated in §Primary screens below". The Decisions Log cli bullet repeats that ownership. Compaction must not move the verb list into architecture §Occupied Resources or §Established Decisions, and must not paste a literal verb list there.
- The harness stage-flag set (`--unit`/`--integration`/`--e2e`/`--live`) and the `--live real-model` selector are stated in layout-templates §Surface: cli → Primary screens (commands), in the `scripts/agent-run.{sh,ps1}` bullet. Any registry row that mentions them must still agree with that bullet after it is shortened. Whether the arch rows restate them today is research's question.
- layout-templates §Surface: cli → Primary screens names these handles: the `:4317` port, `ANDROMEDA_PULSE_*`, `CONDUCTOR_MSEDGEDRIVER`, `CONDUCTOR_NVDA` and `CONDUCTOR_A11Y_STRICT`. Under "no registered fact lost", each one that §Occupied Resources registers today must still be registered after compaction. Research must establish which of them are registered today.
- The two §Tooling context lines state library and framework versions: §Surface: cli (clap 4.5, indicatif 0.18, inquire 0.9, comfy-table 7, owo-colors 4.x) and §Surface: desktop-webview (Tauri ≥ 2.10.3, React 19, Tailwind v4.1). Any architecture roster row kept in the compacted body must match them. Compaction must not bring back a stale version literal.
- The layouts plan covers exactly two surfaces: desktop-webview and cli (per layout-templates §Decisions Log, "Surfaces covered"). If §Occupied Resources registers surface handles, both must stay registered. Whether it does, and in what form, is research's question.

## Patterns to follow
- Name the set instead of listing it. layout-templates §Component — Primary navigation (verb structure) and the §Decisions Log cli bullet say what defines the set (`Commands`) and do not enumerate it. A compacted registry row can point to the owning section the same way.
- The spec body holds only current truth. History goes to the sidecar as dated entries with **Section / Change / Why**, plus a **Search performed** or **Sweep** ledger (the shape of every entry in `layout-templates-amendments.md`). This is the form history takes when it leaves the architecture body.
- After an edit, grep for every retired phrase and confirm zero hits, and list the "Leaves" (downstream derived docs) checked. See the 2026-09-04 and 2026-09-22 entries in the sidecar. This is the sweep that premise 4 (stale readers) needs.

## Anti-patterns to avoid
- Stating one fact in two masters with different wording. The 2026-09-07-dependency-polish amendment records `inquire` "split three ways" across three masters. Shortening a registry row must not create a new mismatch with layout-templates §Tooling context.
- Putting a literal enumeration into the compacted body. Per the 2026-09-03 amendment, SET-NAMING rules out a literal that goes stale again.

## Contract bindings
- architecture §Occupied Resources ↔ layout-templates §Surface: cli → Primary screens (commands). layout-templates owns the verb set, stage flags and probe behaviour. arch registers the handles. Compaction has to keep both sides consistent.
- architecture registry ↔ `docs/commands.md`. Per the 2026-09-22 amendment's Sweep "Leaves", the cli command reference was re-derived from architecture plus the layout-templates cli bullets. If compaction changes what architecture says about cli handles, that derived doc needs re-checking.

## Acceptance criteria contributions
- (layouts) Every cli verb, stage flag, env-var handle and port named in layout-templates §Surface: cli → Primary screens that §Occupied Resources registers before compaction is still registered after it. The check is a before/after handle diff (per layout-templates §Surface: cli → Primary screens (commands)).
- (layouts) No literal cli verb enumeration is added to architecture §Established Decisions or §Occupied Resources. The verb set stays owned by layout-templates (per layout-templates §Component — Primary navigation (verb structure)).
- (layouts) Every library or framework version still stated in the compacted registry sections matches the matching §Tooling context line: §Surface: cli / §Surface: desktop-webview (per layout-templates §Surface: cli — Tooling context).
- (layouts) `layout-templates.md` has an empty diff at wrap. The chunk makes no surface, region or focus change (per layout-templates §Decisions Log — "Surfaces covered").

## Relevant amendment history
- **2026-09-03-live-pulse-preconditions-probed.** The `preconditions` verb was registered in layout-templates "rather than in arch per playbook `:61`". Both sites that listed the three verbs as a literal were changed to name the `Commands` set. Why: layouts owns the CLI verb surface, and literals had kept going stale.
- **2026-09-06-run-report-envelope-conformance-gate.** The `conductor cleanup` verb was added. The chunk plan had aimed this amendment at architecture. The wrap re-aimed it at layout-templates because of "architecture:33's explicit refusal of the role". Line 33 is above both in-scope sections, so compaction does not move it. This entry is precedent that registry text must not take over the verb list.
- **2026-09-07-dependency-polish.** The cli versions for indicatif and inquire were reconciled. Architecture's roster row already said inquire 0.9, but the masters had been "split three ways". Why it matters here: shortening roster rows is exactly where version text can drift apart.
- **2026-09-22-interpretation-proven-live.** `preconditions --for` and the `--live real-model` selector were added. Its Sweep says `docs/commands.md` "was re-derived from architecture + these bullets". Why it matters here: architecture text feeds a derived doc, so compacting it has a downstream reader.
