## 4. Layout Templates Excerpt

### Layout Types per Surface

- **desktop-webview:** run-console-idle, run-console-live, run-console-HOLD, run-report-terminal (four run-states of one frameless window, not four routes — no router, no breakpoints)
- **cli:** conductor-run, conductor-suite, conductor-report (plus headless `scripts/agent-run.sh` path; line-oriented, ratatui omitted)

### Error Boundary Placement

(No explicit error boundary placement in layouts — Phase 3 will recommend defaults per layout category, typically section-level for dashboards / page-root for forms with focus-on-error pattern.)

Note: cli surface documents sanitized error output to stderr (`error: <short>` + `hint: <fix>`, no stack traces unless `--debug`), but this is output-stream discipline, not a component error-boundary placement.

### Focus Management Anchors

- **run-console-HOLD (operator-pause go/no-go dialog):** Modal focus trap — shadcn AlertDialog (Radix) traps focus within the dialog region over a dimmed/inert console; visible `color-focus` ring on the trapped region; Proceed/Abort are the in-trap interactive targets.
- **run-console-HOLD (dialog):** Focus restoration implied on close (standard Radix AlertDialog behavior) — not explicitly stated in layout text.
- **desktop-webview (global navigation):** Keyboard-first — focus ring (`color-focus`) visible on every interactive control; active picker item carries `border-emphasis` edge so focus is never color-only; keybindings (start/stop/proceed/abort) are first-class via Radix primitives; selecting a control by keyboard moves visible focus and that position is communicated to assistive tech.
- **desktop-webview (window controls):** Icon-only minimize/close carry derived accessible names.
- **cli (operator-pause):** `inquire` confirm gated behind `isatty`; headless/non-TTY path skips the prompt entirely (never blocks) — no focus-trap concept on cli.
- (Initial-focus-on-mount, skip links, and route-change focus are NOT documented — single-station console has no routes; Phase 3 will derive WCAG SC 2.4.3 + SC 3.2.1 defaults for initial focus and route/state-change focus.)

### Heading Hierarchy Anchors

(No explicit h1/h2/h3 numeric hierarchy in layouts — typography is referenced by role name only.)

Role-level and landmark-equivalent signals present:

- **run-console-idle / -live / -HOLD / -report (desktop-webview):** phase line carries the **Heading** role (named operational segment); window top is a frameless titlebar (`data-tauri-drag-region`); footer is a single-line status strip (**Label** role); landmark roles are NOT explicitly assigned — Phase 3 to map main / banner / contentinfo equivalents.
- **run-console-HOLD (go/no-go dialog):** shadcn AlertDialog provides **alertdialog** semantics; dialog header (Display role) carries frozen-count + step index; body is **Body** role.
- **cli surfaces:** non-DOM (line-oriented stdout) — heading hierarchy / landmark roles N/A; section title rendered as bold colored header text, always paired with ASCII bracket prefixes (`[PASS]`/`[HOLD]`/`[FAIL]`/`[BLOCKED]`) for NO_COLOR/screen-reader survival.
- (Sequential h1/h2/h3 nesting and main/navigation/contentinfo landmark assignment not specified — Phase 3 will recommend WCAG SC 1.3.1 + SC 2.4.6 defaults: single h1 per surface, sequential h2/h3 nesting, main + contentinfo landmarks for the desktop-webview window.)
