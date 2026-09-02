# Windows/MSYS Host Recipes

The Bash tool on this host is Git Bash (MSYS) over Windows; the primary shell is PowerShell.
POSIX-shaped recipes break here in the recurring ways below. Every rule is measured on live work,
not hypothetical. (Rendered only on Windows-host projects; inert elsewhere.)

## Paths & argument conversion
- MSYS auto-converts leading-`/` arguments into Windows paths (the `/X`-mangling class) — pass
  `//X` for a literal slash-arg, or scope `MSYS2_ARG_CONV_EXCL` for the call.
- Bash's `/tmp` is NOT the Windows temp dir and is invisible to native tools (a Windows-native
  python cannot open `/tmp/x`) — use explicit full paths across every bash↔native boundary.
- `cd` inside a compound command can trip permission prompts and does not persist — prefer
  absolute paths over `cd`.

## Probes & pattern tools
- `grep -P` dies on the host locale ("supports only unibyte and UTF-8 locales") — use `grep -E`
  or a python char-class instead.
- A zero-is-healthy count probe (`grep -c` / `grep -q`) exits non-zero on no matches and aborts a
  `&&` chain — suffix `|| true`, or chain with `;`.

## Transports (JSON & documents)
- Never build JSON — or any multi-KB document — through shell quoting: `printf` collapses escapes
  content-dependently, and a PowerShell redirect writes UTF-16/BOM.
- Documents: the Write tool, whole-content. Ledger appends: a VALIDATED python append
  (`json.dumps(json.loads(...))` — a mangled payload fails loudly instead of landing).
- Inline `python -c` is fine for a SHORT, quote-free, single-expression probe; anything longer,
  quote-bearing, or document-carrying goes through a scratchpad file run by path (the measured failure
  modes are size and unquoted expansion, not inlining or quoted heredocs).

## Compound commands & permissions
- `rm -rf` + `mkdir` + launch compounds get denied by permission layers and abort mid-chain —
  granular steps, fresh unique dirs, no `rm` in a launch path.
- `;` over `&&` for optional probes: an optional probe's failure must not abort the chain.

## Processes & ports
- MSYS and Windows pids are DIFFERENT spaces — bash's `$!` is the app's Windows pid only when the
  binary was spawned by path (no shell wrapper in between).
- When an msys probe is ambiguous, verify liveness/kill via PowerShell (`Get-Process -Id` /
  `Stop-Process -Id`); judge teardown by loopback port probes plus pid-liveness, never pid
  inference alone.

## Exit codes
- Read `$?` from the BARE command — `| tail` / `| head` report the pipe's LAST command's exit
  (measured green-masking red gates). Redirect output to a file and inspect the file.
- When a tool prints its own verdict, the printed text outranks an unisolated `$?`.

## Encoding & heredocs
- Force UTF-8 on python invocations (`-X utf8`; the project settings.json env block sets
  PYTHONUTF8 for hook-mediated runs).
- Heredoc terminators must be column-0 and whitespace-exact — a padded terminator silently
  swallows the rest of the script.
- The Bash tool corrupts a command past a fixed offset near 7.5 KB (measured 2026-09-02: a 100-line
  and a 60-line quoted heredoc both died at script line 57 with `unexpected EOF while looking for
  matching`; 6.0 KB passed; all 22 logged heredoc failures were larger calls) — keep every command
  under 6500 bytes: a document goes through the Write tool, a script to a scratchpad file run by
  path, a ledger payload splits across calls. The quoted heredoc itself is sound below the cut.

## Long single-line files
- A multi-KB single-line entry defeats anchored Edit and capped reads — use a python
  read-modify-write by path, and perform an in-full read as a structural extraction, never replace it:
  `grep -n` the headers AND per-entry introducers for an index → offset-bounded reads covering every
  indexed span; a `head`-limited view never answers a membership question.

## Session Additions
_This section is owned by `/wrap-session`. setup-project preserves content added here on re-run. See `section-markers.md` for the convention._
