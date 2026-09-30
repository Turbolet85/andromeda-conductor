# security extract

## Relevance
partial — no product/shipped-binary surface moves. The chunk touches the dev-only driver-stack harness (a key-input path committed into the `sr*` leg), the screen-reader speech-log ingest into committed evidence, and operator-supplied host-tool binaries. All three are governed surfaces in the plan.

## Constraints
- **Harness-spawn rule (b) registry.** Per security-plan §Security Anti-Patterns → Code Patterns (rule (b)), the dev-only driver stack is one of the three loci. Every harness spawn there must be one of the SEVEN governed forms. None may be a shell string or an `eval`-equivalent. An operator-supplied value may appear only as a separate array-form argv element after metacharacter rejection. The plan records each new form as an operator-ratified "deliberate boundary widening". Arms W and 153 are recorded as session-level CONTROLS: they sit outside the loci, add no form and leave the count at seven. That status covered an UNCOMMITTED script. Research must decide whether committing an OS-level key-injection step into the leg (1) fits the existing window-activation form (a fixed `powershell.exe` with fixed argv `-NoProfile -ExecutionPolicy Bypass -File <repo script> -Title <constant>`) or (2) is an eighth form. The plan never names `SendInput` / `keybd_event`, so it cannot settle that question. The scope routes an eighth form to a P4 escalation that halts for the founder.
- **`-File`, never `-Command`.** Per the same §Code Patterns seventh-form clause, the `-File`-only shape is what keeps a PowerShell launch out of the `eval` class. Any committed key-injection script invoked from the leg needs the same shape: fixed program, fixed argv of literals, and no operator-supplied value in the vector.
- **Speech-log ingest.** The speech-log ingest into committed evidence is an untrusted third-party boundary, per security-plan §Input Validation (Screen-reader speech-log ingest row). A regraded record needs:
  - a spec-id cross-check against `nvda-pass-spec.md`;
  - closed `outcome` / `arm` / `review_grade` sets;
  - `heard` bounded at 400 chars;
  - a mandatory host-path scrub to `<host-path>` that adds a `security_finding` to the row, via `writeNvdaPass` in `parse-nvda-log.ts`.

  The raw log stays under gitignored `runs/`. Whether the regrade's new record (a new file beside the prior chunk's `nvda-pass.json`) goes through that same writer is research's question.
- **No host paths in committed evidence.** Per security-plan §Security Anti-Patterns → Logging, committed evidence must not leak absolute host paths or seam-crate struct names. That covers the per-chunk `evidence/` tree, `nvda-pass.json` included. The scope requires every verdict to name "runtime × driver × NVDA × OS build × input path". Those must be rendered as VERSIONS and path CLASSES, never as a handle's value. Examples: the portable NVDA copy on `CONDUCTOR_NVDA`, any `WEBVIEW2_BROWSER_EXECUTABLE_FOLDER` value, a driver path.
- **Host-dev-tool handles.** Per security-plan §Input Validation (`CONDUCTOR_MSEDGEDRIVER` and `CONDUCTOR_NVDA` rows), each needs existence + `isFile` + shell-metacharacter rejection at the wdio edge before its array-form spawn. Unset or not-a-file means the leg skips at exit 0 with a host-path-free precondition. The plan binds the duty per READER. So a variable the leg newly COMMITS as a read or set needs its own validation disposition. That includes `WEBVIEW2_BROWSER_EXECUTABLE_FOLDER`, which the plan does not currently list. Whether the harness will read or set it is research's question.
- **Driver binaries.** Per security-plan §Security Anti-Patterns → Code Patterns (arm 153 clause) and §Dependency Security (third dependency class), an operator-supplied driver is executed only after its pre-execution checks have been read:
  - `Get-AuthenticodeSignature` returns `Valid` with an `O=Microsoft Corporation` signer;
  - its SHA-256 and version match.

  A binary is never executed before its signature verdict is read. No CI or network fetch is added; the class stays at one member.
- **Listeners.** Per security-plan §Threat Model Summary (Attack surface: the port-bind vector) and §Security Anti-Patterns → Universal, the dev-only listeners are the harness-lifetime loopback `127.0.0.1:4444` (tauri-driver) and `:4445` (native WebDriver). Both exist only for the leg's duration. The chunk must introduce no other listener, and no shipped binary may gain one.

## Patterns to follow
- The window-activation form in security-plan §Security Anti-Patterns → Code Patterns: `spawnSync('powershell.exe', [...fixed flags, '-File', <repo script>, '-Title', <constant>])`, a fixed OS program with no operator input anywhere in the vector. This is the natural shape for any committed OS-level key-input step, if research maps it there.
- The guarded host-tool spawn in the same § and in §Input Validation (`CONDUCTOR_NVDA` row): run the `UNSAFE_PATH` guard, then a detached array-form `spawn(exe, [fixed argv])`, with the handle NAME (never its value) in any skip text.
- The arm-153 admission sequence in §Security Anti-Patterns → Code Patterns: Authenticode + SHA-256 + version check before any execution, for any operator-supplied msedgedriver the regrade or step 1 needs.
- Speech-log ingest in §Input Validation: the committed record is written only through `writeNvdaPass`'s scrub and closed sets. Never hand-author `heard` text into evidence.

## Anti-patterns to avoid
- Spawning a child through a shell or an `eval`-equivalent (PowerShell `-Command`, a composed command string carrying any operator-derived value) from the leg (per security-plan §Security Anti-Patterns → Code Patterns).
- Committing a new harness spawn form into the driver-stack locus as if it were routine. The plan registers each form only as a ratified boundary widening (per §Security Anti-Patterns → Code Patterns).
- Writing a configuration string, speech transcript or session-script output containing an absolute host path into `evidence/` (per §Security Anti-Patterns → Logging).

## Contract bindings
- **security ↔ a11y:** the SR pass record's closed sets and spec-id cross-check (`nvda-pass-spec.md`, a11y-plan §3) are the same mechanism as the §Input Validation speech-log ingest row's scrub. The configuration-bound verdict wording belongs to a11y. The redaction of that wording belongs here.
- **security ↔ tests:** the hygiene grep (no host path / seam-crate struct name) and the closed-set gate over the committed record, named in §Input Validation (speech-log row: "gate-1 hygiene grep … + gate 2").
- **security ↔ arch:** registering a committed binder or reader (the `:4445` binder, `WEBVIEW2_BROWSER_EXECUTABLE_FOLDER`) in architecture.md §Occupied Resources, which §Threat Model Summary cites for ports and egress. The scope records the operator's E1 ruling that registration happens only when this chunk COMMITS one.

## Acceptance criteria contributions
- Hygiene gate: every committed file under the chunk's `evidence/` has 0 absolute host paths and 0 seam-crate struct names. Any `<host-path>` placeholder in a speech-record row is paired with a `security_finding` on that row. (per security-plan §Input Validation — Screen-reader speech-log ingest row; §Security Anti-Patterns → Logging)
- Every harness spawn the chunk commits into the driver-stack leg meets all of the following, or halts at P4 for the founder's live word before commit:
  - a fixed program;
  - array-form fixed argv;
  - `-File` never `-Command`;
  - no operator-supplied value except a guarded host-tool path;
  - shown to map to one of the seven registered forms.

  (per security-plan §Security Anti-Patterns → Code Patterns, rule (b))
- For every operator-supplied driver binary executed in step 1 or the regrade, the record shows the Authenticode verdict (`Valid` + Microsoft signer) and the SHA-256/version match read BEFORE the first execution. (per security-plan §Security Anti-Patterns → Code Patterns, arm-153 clause; §Dependency Security, third dependency class)
- After each leg's teardown, a loopback probe finds no listener on `:4444`/`:4445` and no new bind anywhere. If the chunk has any dependency delta, `npm audit --omit=dev` is clean with `package-lock.json` committed, and `cargo audit` + `cargo deny check` are green. (per security-plan §Threat Model Summary — port-bind vector; §Dependency Security — Frontend (npm) supply chain; §Security Anti-Patterns → Universal)
