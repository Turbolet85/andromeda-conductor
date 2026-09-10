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
- 2026-09-10: **A line-granular grep cannot DATE a clause inside a multi-KB single-line entry — it collapses
  every dated extension the line carries into ONE hit.** This project's rule-file entries accumulate
  `**Extended {date}**` clauses on the SAME physical line, so a sweep for a topic or a date returns one hit
  bearing the line's LEAD date however many later clauses sit inside it. Both readings then go wrong in
  opposite directions: "one hit, dated X" does not refute "a clause dated Y exists here", and citing the
  lead date for a clause written later misattributes it. Measured: three separate sweeps (the topic, a
  process-family name, and a cmdlet token) each returned exactly one line hit, while the clause the work
  actually rested on sat thousands of characters into that line under a later date. Resolve by OFFSET —
  locate the clause's position within the line, then read a bounded window around it — and cite such a
  clause as `{file}:{line}` — the `{lead-date}` entry **as extended** `{clause-date}`, never as an entry of
  its own date. This extends the *Long single-line files* rule above from Edit/read to SEARCH, and is kept
  here because that section is setup-project's rendered template text.
- 2026-09-08: The bash↔Windows-native boundary changes TWO things, and the second fails SILENTLY.
  (a) **Paths** — an MSYS path is invisible to a Windows-native tool at every depth, not only under
  `/tmp`: a native python `open('/c/Users/…')` raises FileNotFoundError where the same file opens fine
  as `C:/Users/…`. Write every cross-boundary path in the native form with forward slashes, which both
  shells accept. (b) **Environment** — a shell SPAWNED BY a Windows-native process does not inherit the
  MSYS shell's PATH, so a host tool that resolves in the Bash tool's shell can be ABSENT in the spawned
  one. That yields no error: the probe reports the TOOL missing and every dependent check false-reds
  (measured — a hooks smoke test run through native python reported `jq` absent and all five arms
  failing; the same hooks pass 5/5 invoked from the Bash tool's own shell). So run any probe that must
  see the host's tools in the shell the runtime itself uses — write the script to a file and invoke it
  by path from the Bash tool — never through a native-process wrapper, and read a tool-missing result
  as a claim about the SPAWNING path before believing it about the host.
- 2026-09-08: **The Bash tool's working directory PERSISTS across calls, so the "Paths & argument
  conversion" clause above saying `cd` "does not persist" is false as a reader would take it** —
  measured directly (a `cd` into a subdirectory in one call, `pwd` in the next, still there; the tool's
  own documentation states the same). The advice it attaches to — prefer absolute paths — is right, but
  the hazard runs the OPPOSITE way, and that inversion is what makes the stale clause costly: a
  FORGOTTEN `cd` would be harmless, while a REMEMBERED one silently rebases every later relative path
  against a base the next call never chose. Measured here: a `cd .andromeda` at the head of one command
  left the following call's `.andromeda/master-route.md` resolving one level too deep, which fails as a
  missing file — the benign-looking failure, since the file plainly exists. So anchor cross-call paths
  absolutely, or re-anchor explicitly at the head of each call, and never let one call's `cd` set up the
  next call's base. Kept HERE rather than fixed above because the false clause is setup-project's
  rendered TEMPLATE text: `## Session Additions` survives a re-render and the body does not.
- 2026-09-08: **`grep -E` accepts PCRE syntax SILENTLY — the pattern matches nothing and exits 0, so a
  negative-lookahead guard passes unconditionally.** The "Probes & pattern tools" clause above covers the
  LOUD failure (`grep -P` dies on the host locale); this is the quiet one, and it is the more dangerous of
  the two. A gate written as `grep -nE 'TokenA|TokenB|http://(?!127\.0\.0\.1)'` returns no output and exit
  0 against a file that plainly contains `http://` — ERE has no lookahead, so the construct can never
  match and the check reports green whatever the file holds. Measured this session: exactly that line was
  authored as a security gate asserting "no plaintext `http://` in the added step", and it would have
  passed no matter what shipped. Two rules follow. Write the POSITIVE probe — grep for the token and
  assert the expected hit COUNT and IDENTITY (here: exactly one hit, the known loopback `127.0.0.1:9515`
  POST) — rather than trying to express "everything except X" in a pattern language that cannot say it.
  And treat any `(?...)` construct under `-E` as a guaranteed silent pass: it needs no debugging, only
  rewriting. Kept HERE rather than folded into the clause above because that clause is setup-project's
  rendered TEMPLATE text.
