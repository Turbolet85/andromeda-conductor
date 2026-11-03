# design extract

## Relevance
partial: the chunk renders nothing new (CI, the harness and fixtures only). Moving CI from `scripts/agent-run.ps1` to `scripts/agent-run.sh` on a Linux runner does touch the headless cli path, and the cli surface's output rules govern that path.

## Constraints
- design-system §Surface: cli (sec-fz6rrgmi) names `scripts/agent-run.sh` as the headless source of truth and release gate. The CI step that becomes `agent-run.sh run` must stay the one harness that path runs through. The same section's Platform line still names Windows and macOS terminals. v4-07 makes that line stale, but it changes only through the wrap's design-system amendment, never through a phase edit.
- design-system §Surface: cli · Tokens (platform-specific) (sec-pmewmd5c) requires styling to be gated separately on stdout and stderr: a terminal, `NO_COLOR` unset, and `TERM != dumb`. Output captured on the Linux runner must therefore carry no ANSI. Research has to answer whether the Linux runner's environment (TTY allocation, `TERM`) keeps that gate closed the way the Windows runner did.
- design-system §Surface: cli · Component Patterns (sec-u72niyoo) item 2 requires that the headless path never blocks on an `inquire` prompt when stdin is not a TTY, and that the decision is recorded to the artifact. This applies to the non-interactive `agent-run.sh run` step in the Linux job.
- design-system §Surface: cli · Component Patterns (sec-u72niyoo) item 5 requires stderr errors to be sanitized, with no absolute host paths. On Linux those paths are POSIX-shaped (for example the runner's home directory). Research has to answer whether the sanitizer already covers POSIX paths and not only Windows-shaped ones.
- design-system §Surface: cli · Platform-Specific Notes (sec-6jinwxjo) requires that `NO_COLOR`, `TERM=dumb` and piped-stdout stripping are always respected, and that interactive prompts always check `isatty` first. Re-stating Windows-shaped CI text for Linux (the scope's item 1) must not drop either of these.

## Patterns to follow
- design-system §Surface: cli · Tokens (platform-specific) (sec-pmewmd5c): one `std::io::IsTerminal` gate per stream, so CI logs and uploaded artifacts stay plain text.
- design-system §Surface: cli · Navigation Pattern (sec-wzqnc5q6): linear, pipe-friendly output, with artifact data on stdout and human messages on stderr. The headless agent path strips ANSI and never blocks.
- design-system §Surface: cli · Platform-Specific Notes (sec-6jinwxjo): every status carries its ASCII prefix (`[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`), so CI logs read correctly without color.

## Anti-patterns to avoid
- design-system §Surface: cli · Per-Surface Bans (sec-2xqr4gwm): never colorize without checking `NO_COLOR` / `TERM` / pipe status, and never block the headless agent-driven path on a prompt.
- design-system §Surface: cli · Per-Surface Bans (sec-2xqr4gwm): no emoji in machine-parseable piped output, and no stack traces outside `--debug`/`-v`. Both apply to the run output the Linux job captures.

## Contract bindings
- design §Surface: cli ↔ tests/obs harness: the Linux job's JUnit, obs and run-journal conformance gates consume headless output, which must be free of ANSI and prompts.
- design §Component Patterns item 5 ↔ security plan: sanitized error output (no absolute host paths) must hold for POSIX paths on the Linux runner.
- design §Surface: cli Platform line ↔ the wrap's design-system amendment (`.andromeda/design-system-amendments.md`): v4-07 retires the Windows terminal platform. This is recorded at the wrap, not edited in this phase.

## Acceptance criteria contributions
- The existing CI run of `agent-run.sh run` on the Linux runner completes without waiting on any interactive prompt, with stdin not a TTY (per design-system §Surface: cli · Component Patterns).
- The output the Linux job captures and uploads (JUnit, obs, run journal) contains no ANSI escape sequences, checked by the existing conformance gates and not by an extra run (per design-system §Surface: cli · Tokens (platform-specific)).
