# arch extract

## Relevance
partial — no `crates/*/src` seam work is expected; arch binds this chunk through the trust boundary, the one-automation-stack scope law, the harness-lifetime port and env-handle registry, and the 5-command harness surface that control (1)'s driving method and the (un-runnable) 153 arm would cross.

## Constraints
- architecture §Cross-cutting Patterns (Scope law) requires that driving Conductor's OWN webview happens over ONE WebdriverIO + tauri-driver stack of exactly three suite families (routine `--e2e`, `a11y:driven`, `sr*`), and never a second automation stack. If control (1) points the wdio/msedgedriver stack at Edge 154 or at another WebView2 app instead of `conductor-tauri`, it is outside the registered families. P4 decides whether that counts as a fourth family, which would need registration, or as a separate stack, which is banned. Whether `wdio.conf.ts` can target a non-Conductor host without a new suite or spawn form is research's question.
- architecture §Cross-cutting Patterns (Trust boundary) requires the dev-host SR leg to stay listener-free apart from the harness-lifetime `4444`/`4445` pair. The `sr*` suites add only the host NVDA from `CONDUCTOR_NVDA` and a fixed-argv PowerShell window-activation script, and neither may open a listener. No control arm may add an inbound listener or a non-loopback egress. The CI-only msedgedriver fetch stays the ONE non-loopback surface.
- architecture §Occupied Resources (Ports) scopes `127.0.0.1:4444`/`4445` as dev-only and harness-lifetime: each suite family binds and releases the pair with its own session, and the pair is never present in `conductor-cli`/`conductor-tauri`. Any control arm that reuses the driver stack inherits this bind-and-release duty.
- architecture §Occupied Resources (env handles) says `CONDUCTOR_NVDA` is read ONLY by `wdio.conf.ts` (`startNvda`/`stopNvda`). It is spawned array-form and detached with the registered fixed argv, validated at the harness edge, and its value is never logged, echoed or committed. The loader handles `WEBVIEW2_USER_DATA_FOLDER`/`WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS` are registered only for the lifetime of the CI `WebView2 session isolation (diagnostic)` step. A dev-host use of any `WEBVIEW2_*` loader variable counts as an unregistered resource, including the executable-folder variable the 153 arm names, which is not registered at all.
- architecture §Infrastructure Patterns requires every added script to carry the "wired into neither harness shell, no 6th command" qualifier. The `agent-run.{sh,ps1}` 5-command surface must stay unchanged whatever form control (1) takes.
- architecture §Design Philosophy (Headless-drivable core, thin shells) and §Inherited Defaults (Module boundaries) place any product lever (Tauri `additionalBrowserArgs` / the loader's browser-arguments variable) in the `conductor-tauri` shell. Scope keeps that lever named and unbuilt, so this chunk plans no crate-edge or `Cargo.toml` change.
- architecture §Established Decisions sets the precedent that a cause's MECHANISM is "recorded, not established" when no probe measured it, citing the 2026-09-11 hosted-runner endpoint reading. Control (1)'s evidence record should use the same wording for whatever reading it licenses.

## Patterns to follow
- The operator-local `sr*` bracket: start NVDA around the driver and quit it after, with the speech log as the driver on the Windows dev host (per architecture §Cross-cutting Patterns, Scope law; §Occupied Resources `CONDUCTOR_NVDA`).
- The harness-edge handle guard: existence + `isFile` + shell-metacharacter rejection, then an array-form spawn. An unset handle SKIPs at exit 0 unless `CONDUCTOR_A11Y_STRICT` is set, which makes it exit non-zero (per architecture §Occupied Resources, `CONDUCTOR_MSEDGEDRIVER`/`CONDUCTOR_NVDA`).
- Evidence lives in the chunk's own `evidence/` tree and is cited by path from any master clause it updates, following the style of the `…/evidence/reading.md` citations (per architecture §Established Decisions).
- A read-only diagnostic script is qualified in the directory tree by where it is invoked and by adding no 6th command (per architecture §Infrastructure Patterns, the `webview2-cause-probe.ps1` row).

## Anti-patterns to avoid
- Serving the control's "plain focus page" over a local HTTP server, or through any other new bind. That is an inbound listener the trust boundary does not admit (per architecture §Cross-cutting Patterns, Trust boundary).
- Standing up a second automation stack, or aiming the arm at Pulse's UI, to reach a non-Conductor page (per architecture §Cross-cutting Patterns, Scope law).
- Setting a `WEBVIEW2_*` loader variable, or any new `CONDUCTOR_*` handle, on the dev host without a §Occupied Resources registration. This covers pointing the loader at the 153 runtime folder, which additionally needs the founder's quoted word (per architecture §Occupied Resources, env handles).

## Contract bindings
- arch ↔ security: a new harness spawn form for control (1) is the EIGHTH crossing of security rule (b) and escalates. Arch's trust-boundary and port-registry clauses (§Cross-cutting Patterns, §Occupied Resources Ports) describe the same spawn census that rule counts.
- arch ↔ a11y: the `sr*` suite families registered in §Occupied Resources (Ports) are the ones a11y-plan §3's *Screen reader test pattern* drives. A new control page or arm changes both registries together.
- arch ↔ tests: the 5-command `agent-run` surface (§Infrastructure Patterns) is the tests harness contract. The PREREQ's `agent-run.sh run --unit` and clippy entries run through it unchanged.
- arch ↔ obs/security hygiene: the `CONDUCTOR_NVDA` value and host paths never reach a committed artifact (§Occupied Resources). This binds the control's evidence record and speech-log ingest.

## Acceptance criteria contributions
- (arch) The only listeners during any control arm are the harness-lifetime `127.0.0.1:4444`/`4445` pair, released at the leg's end, and no process the leg started survives it. Checked by a loopback-port probe plus a census after each leg (per architecture §Occupied Resources, Ports; §Cross-cutting Patterns, Trust boundary).
- (arch) `git diff --numstat 4460307 -- crates/*/src crates/*/Cargo.toml Cargo.toml` is empty: no product lever and no seam change (per architecture §Design Philosophy, Headless-drivable core; §Inherited Defaults, Module boundaries).
- (arch) `scripts/agent-run.{sh,ps1}` expose the same 5 commands at wrap as at `4460307`. Any script the chunk adds is recorded as wired into neither harness shell (per architecture §Infrastructure Patterns).
- (arch) No env handle is read or set on the dev host beyond those §Occupied Resources registers, or else the chunk records a registration amendment for it. The `CONDUCTOR_NVDA` value appears in no committed file (per architecture §Occupied Resources, env handles).
