# design extract

## Relevance
Partial — the chunk redacts logs and errors (obs surfaces), which do not render UI, but the error-handling contract binds to the cli surface's error-output shape and the color-never-alone rule.

## Constraints
1. Per design-system §Anti-Patterns / §Self-Validation: "NEVER mix stdout (data) and stderr (messages) without intention" — error output uses the cli surface pattern (sanitized `error:` + `hint:` shape, no stack traces / host paths / struct names to stderr).
2. Per design-system §Surface: cli / Component Patterns §5 (Error output): "To stderr, sanitized (no absolute host paths / internal struct names / stack traces per security plan): `error: <short>` + contextual detail + `hint: <fix>`."
3. Per design-system §Color Palette / §Semantic Colors: error messages at the cli edge use `--status-fail` / ANSI 203 prefix only (never color alone — pair with `[FAIL]` / `✗` text prefix per §Surface: cli / Component Patterns §4).
4. Per design-system §Anti-Patterns / Per-Surface Bans (cli): "NEVER print stack traces in normal mode — only under `--debug`/`-v`; NEVER rely on color alone."
5. Per design-system §Brand Identity / §Signature: error handling preserves "calm under load, no alarm" — no blink/flash/native-OS-toasts (those are Pulse behavior Conductor observes).

## Patterns to follow
1. **Error surface shape (§Surface: cli / Component Patterns §5):** sanitized `error: <short>` + detail + `hint: <fix>` on stderr; never stack traces / absolute paths in normal mode.
2. **Status prefix pairing (§Surface: cli / Component Patterns §4):** every error message pairs colored `[FAIL]` prefix with ASCII text, never color alone (a11y + `NO_COLOR` compliance).
3. **Field allowlist as redaction source (§Platform-Specific Notes):** the obs log JSON schema's explicit allowlist (obs-plan §3) is the single source of truth for which fields reach any sink; non-allowlisted fields dropped before write.

## Anti-patterns to avoid
1. **Color-only status signals:** never emit `[FAIL]` or error messages with color alone — always pair with ASCII text (`error:`, `[FAIL]`, `✗`) per a11y and `NO_COLOR` (design-system §Anti-Patterns / Per-Surface Bans (cli)).
2. **Stack traces or host paths to the operator:** redaction mandatory at the anyhow edge; never leak absolute paths / struct names / stack traces to cli stderr or Tauri returns in normal mode (design-system §Anti-Patterns).
3. **Silent swallowing of redacted data:** if a field is redacted, show a `[redacted]` placeholder so a log-debugging operator knows a field was dropped (design-system §Brand Identity: transparency, no mystery).

## Contract bindings
- **design ↔ obs**: the obs log JSON schema's allowlist (obs-plan §3) sources the field-allowlist layer; logs and cli/tauri errors respect the same redaction boundary.
- **design ↔ security**: error outputs at the cli/tauri edge must not leak stack traces / absolute paths / struct names (security-plan §Error Handling).
- **design ↔ a11y**: every error/status on the cli pairs a colored prefix with ASCII text, never color alone (binding to WCAG SC 1.4.1 "Use of Color").

## Acceptance criteria contributions
1. (design) Every error/redacted output on the cli pairs a colored status prefix with ASCII text (e.g. `[FAIL] error: …`), never color alone (a11y §Use of Color + design-system §Surface: cli).
2. (design) Redacted fields in logs and errors show a `[redacted]` / `<redacted>` placeholder (not silent drops), maintaining transparency.
3. (design) CLI error messages follow the sanitized shape (`error: <short>` + detail + `hint: <fix>`) with no stack traces / absolute paths / struct names in normal mode (design-system §Surface: cli / Component Patterns §5).

## Relevant amendment history
(none)
