# design extract

## Relevance
partial — timeline engine chunk (Epoch 2 scheduler) involves no rendering or user-facing surfaces; design constraints apply only to future emission/journal chunks and CLI/webview delivery surfaces (Epochs 3+).

## Constraints
- Per design-system §Brand Identity: Expression level `0.3` governs animation intensity across the entire project; seeded scheduler must preserve determinism substrate that later motion tokens bind to (no ambient entropy leaking into timing).
- Per design-system §Motion (calibrated to expression level `0.3`): the virtual-clock-only discipline (tokio::time, never std::time for sequencing) ensures phase transitions and hold-points hit exact timing declared in the phase list — the frozen heartbeat's 150ms color transition + pause-state motion binds to precise phase-boundary signals this scheduler surfaces.
- Per design-system §Anti-Patterns Universal Bans: "NEVER converge on common safe choices across generations — anchor to THIS domain (mission-control console, frozen heartbeat)"; seeded scheduler owns the timeline determinism that makes the frozen-count signature repeatable (same seed → same phase shape → same hold-point replay).
- Per design-system §Self-Validation Protocol / Sameness Test: the scheduler's determinism discipline (seedable PRNG, current_thread runtime, zero work-stealing) is sourced from THIS product's mission-control domain, not generic async concurrency patterns; it exists to make the paused-count hold-point *deterministic and observable*, not to be a general-purpose executor.

## Patterns to follow
- Per design-system §Brand Identity / Signature element: the "frozen heartbeat at the operator-pause" depends on phase boundaries surfaced by this scheduler to the emission/journal seams; the scheduler's phase-transition events are the substrate the UI will later bind CSS motion to. Preserve phase-boundary observability for the `current_thread` channel the UI consumes.
- Per design-system §Surface: desktop-webview / Component Patterns / Frameless titlebar: the count ticks on a Tauri `Channel` from the phase-scheduler's emit-pace; ensure the scheduler's phase-tick cadence is surfaced to the channel in a way later emission chunks can hook (phase index / elapsed count).

## Anti-patterns to avoid
- Do NOT leak ambient entropy or system-clock decisions into the phase timeline — seedable PRNG owns all non-determinism (design-system §Motion 0.3 discipline: the timeline must be exactly reproducible so the frozen-count hold-point replay works).
- Do NOT couple the scheduler to the emission / journal / OTLP surfaces — per scope, this chunk is timing-only; actual span/metric/log emission is Epoch 3 (design-system §Anti-Patterns Universal Bans: "NEVER use the same layout for different information types"; timeline sequencing and emission are separate concerns).
- Do NOT use motion/animation libraries or spring physics in the scheduler itself (expression 0.3 forbids `framer-motion`, spring, parallax, staggered reveals; the scheduler is pure timing, no visual choreography here).

## Contract bindings
Timeline scheduler ↔ UI motion tokens: the phase-boundary events this chunk surfaces bind to the 150ms ease-out color transitions (count tint: green → amber → slate-violet) when the operator-pause hold-state flips (design-system §Motion, Paused-count hold-point signature). The CLI mirror (component pattern 1) depends on the same phase-tick observability (indicatif spinner stop / amber HOLD phase-line print).

## Acceptance criteria contributions
- (design / timeline determinism) Same scenario + seed reproduces identical phase sequence / relative timing — the substrate for deterministic operator-pause hold-point replay.
- (design / motion binding) Phase-boundary transition events are surfaced to the caller (e.g., Tauri `Channel`) so downstream UI chunks can bind motion/state-change signals to the 150ms color transition; phase indices are observable so the frozen-count signature can track "where it stopped."
- (design / expression-level discipline) All timing is tokio-virtual-clock, zero ambient entropy, zero system-clock scheduling; seeded RNG is the sole source of non-determinism, making the phase timeline fully reproducible.

## Relevant amendment history
2026-06-15-design-token-typography-bundle (§Surface: desktop-webview / Tokens): token declaration moved from `@theme` to `:root` to preserve all 34 tokens (was dropping spacing/radius/motion tokens under Tailwind v4 tree-shaking). **Relevance to this chunk:** design tokens bind to motion timing (e.g., `--motion-micro: 150ms` for the count-tint transition); the scheduler does not render tokens directly, but phase-timing must respect the 150ms color-transition duration the downstream UI will use (scheduler's hold-point surfacing must occur at a granularity that allows the 150ms motion to complete synchronously without jitter from the timeline).
