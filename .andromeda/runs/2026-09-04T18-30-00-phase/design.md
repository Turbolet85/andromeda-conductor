# design extract

## Relevance
Partial — the fix itself is Rust-side with no rendered output, but its user-visible effect (a foreign OS window seizing the frameless console's foreground) and CARRY 1's `No run yet` / envelope-banner doc comments land on design-owned surface contracts.

## Constraints
- The frameless window IS the surface: a single-station console with no browser-style navigation and no foreign OS chrome — a self-raised terminal pane on `Start` breaks that contract, so design-system §Surface: desktop-webview → Navigation Pattern + Platform-Specific Notes require the console to hold the OS foreground across a run start. Whether the shipped spawn already keeps the foreground is research's question (the scope reports `spawn.rs:80` sets no creation flags as of 2026-09-04).
- Conductor must not raise unbidden OS-level surfaces of its own — design-system §Platform-Specific Notes (desktop-webview) states native OS toasts are Pulse behavior Conductor *observes*, never performs; the same discipline governs any OS window the app raises on its own behalf.
- The remedy must be a silent suppression, not a compensating indicator: design-system §Brand Identity ("calm under load, no alarm", expression `0.3`) and §Motion hard limits forbid adding any animation, banner or alert to explain the spawn.
- "No result yet" must never read as "failed": design-system §Component Patterns 6 requires the shipped `No run yet` prose in `--text-tertiary` for the empty run-report, and §Anti-Patterns → Rejected Defaults bans conflating an empty state with a `Fail`/error rendering. CARRY 1's narrowed "never an error" doc comments must not license an error rendering for the absent-run / absent-envelope case.
- Token discipline holds for any text this chunk adds on either surface: design-system §Surface: cli Tokens keeps the lamp set closed at six with non-lamp captions reusing the Residual-mute pair (ANSI 246 ↔ `var(--status-residual)`), and §Self-Validation (Token Test) forbids raw hex / raw ms / new palette rows.
- If CARRY 3's mount-focus behaviour changes, the picker input must keep a visible `--color-focus` ring and keyboard-first operation — design-system §Border Progression (Focus) and §Component Patterns 5; §Per-Surface Bans (desktop-webview) forbids hover-only interaction without a keyboard path. Focus *order* belongs to layouts/a11y, not here.
- CLI-side artifact cleanliness: design-system §Surface: cli Component Pattern 1 requires run output be TTY-gated "so agent-captured artifacts stay clean", and §Per-Surface Bans (cli) reserves stdout for data — a spawned child's console must not inject stray output into the headless source-of-truth path.

## Patterns to follow
- §Component Patterns 1 (frameless titlebar + Paused-count heartbeat): `Start` is exactly the moment the count begins ticking in the titlebar; the foreground-steal window (~200 ms after `Start`, per the scope's mechanism claim) collides with the signature's primary placement.
- §Component Patterns 6 (Run-report view): the shipped empty-state string and `--text-tertiary` tier are the reference for CARRY 1's `run_report` comment; the in-progress case renders no report-area prose (titlebar phase line + count carry live state).
- §Component Patterns 5 (Scenario/suite picker): the picker's recessive filter-miss styling and persistently-mounted announced region are the established shape for anything CARRY 3 touches — reuse the list's existing styling, add no token.
- §Surface: cli Component Patterns 1 and 5: spinner stops in place (never hidden, never animated to 100%); `error:` / `hint:` reuse the shipped ANSI 203 / 246 pair rather than introducing a color — the standing precedent for adding a CLI edge without a palette row.

## Anti-patterns to avoid
- Do not compensate for the pane with a visible affordance — no toast, no flashing/pulsing alert, no spinner or banner announcing the sidecar spawn (design-system §Anti-Patterns → Rejected Defaults: flashing/animated alert toast; §Per-Surface Bans desktop-webview).
- Do not let the corrected `run_report` / `run_envelope` comments (or any behaviour they document) turn an absent run or absent envelope into a red/error rendering — an absent banner stays simply absent (§Anti-Patterns → Rejected Defaults: conflating "no result yet" with "failed").
- Do not introduce a new color, ANSI entry, hardcoded hex or literal ms anywhere this chunk touches (§Self-Validation Token Test; §Surface: cli Tokens — the set is named, never extended per row).

## Contract bindings
- **Foreground retention ↔ a11y + the `sr` harness:** design-system's "the frameless window IS the surface" mandate is what the SR suite measures as focus retention; the `reactivateWindow()` retire decision (scope `screen-reader.e2e.ts:121`) is a test-harness/a11y verdict, and design only supplies the requirement that no foreign window take the console's foreground.
- **`--color-focus` ring ↔ a11y SC 2.4.7 / SC 1.4.3:** the ID-cyan focus token (`#7DCFFF` dark / `#0969DA` light, §Border Progression) is the design half of CARRY 3's focus-visibility contract; contrast conformance is a11y's call.
- **`--status-residual` ↔ obs-plan envelope row:** the `[ENVIRONMENT-SUSPECT]` load-envelope caption is bound by name to the ANSI 246 / `var(--status-residual)` pair (§Surface: cli Tokens), which is the tier CARRY 1's `run_envelope` banner sits in.

## Acceptance criteria contributions
- (design) Starting a run from the GUI raises no foreign OS window over the console — the frameless single-station window remains the only surface and retains the OS foreground (per design-system §Surface: desktop-webview → Navigation Pattern + Platform-Specific Notes).
- (design) The fix adds no compensating visual affordance: no toast, banner, animation or new status indicator introduced to explain the sidecar spawn (per design-system §Anti-Patterns → Rejected Defaults + §Motion hard limits).
- (design) With the doc comments corrected, the absent-run case still renders the shipped `No run yet` prose in `--text-tertiary` and an absent envelope banner is simply absent — neither downgraded to an error/red state (per design-system §Component Patterns 6 + §Anti-Patterns → Rejected Defaults).
- (design) Any text or state this chunk adds on either surface uses only existing tokens by name — no new palette row, no new ANSI entry, no raw hex/ms (per design-system §Surface: cli Tokens + §Self-Validation Token Test).

## Relevant amendment history
- **2026-09-02-screen-reader-manual-spec** — corrected §Component Patterns 3/6/7 to the *shipped* empty/in-progress strings (`No run yet` with the `— pick a scenario/suite to begin` tail retired; no in-progress report prose ships). Same SR pass that produced this chunk's S1-01 spawn finding; it is the authority for CARRY 1's `run_report` comment wording.
- **2026-09-04-sr-findings-remediation** (immediately preceding chunk) — §Pattern 3 empty corrected to `No coverage data.` and §Pattern 5 gained the picker's filter-miss state, noting a region mounting together with its text announces nothing. Same SR-remediation lineage as CARRY 3/CARRY 4; establishes that empty-state strings are read off `App.tsx`, not assumed.
- **2026-09-01-desktop-a11y-sweep** — moved `--text-tertiary` dark to `#838EBA` (and `--text-muted` in both themes) for 4.5:1. Relevant because the `No run yet` empty state renders in `--text-tertiary`; any assertion about that prose must use the current value.
- **2026-08-09-sut-load-envelope** and **2026-09-03-live-pulse-preconditions-probed** — added `[ENVIRONMENT-SUSPECT]` then `[PRECONDITION]` to the Residual-mute non-lamp caption SET, deliberately naming the set rather than a count, with no new palette row and the lamp set kept closed at six. Governs CARRY 1's envelope-banner tier and any caption this chunk might add.
- **2026-06-24-sanitized-stderr-agent-mode-logging** — rejected a proposed new "hint grey" row; the `error:`/`hint:` edge reuses shipped ANSI 203/246. The standing precedent that a new CLI-side edge reuses tokens rather than extending the palette.
