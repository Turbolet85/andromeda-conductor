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
  path, a ledger payload splits across calls. The quoted heredoc itself is sound below the cut **[corrected 2026-09-23: except for BACKSLASH PAIRS — the Bash tool's transport collapses a doubled backslash to one before bash ever sees the quoted heredoc (measured on three payloads: a Python `'\\'` arrived as `'\'`, a `\\n` planted real newlines in Rust string literals, and `grep '\\'` died on a trailing backslash); a backslash-bearing payload goes through a scratchpad file written by the Write tool]**.

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
- 2026-09-10: **A `| head` on a search over SEVERAL paths silently answers about only the paths that sort
  FIRST — so a per-path zero taken from a clipped combined search is unfounded, and the command's own text
  is exactly what makes it look sourced.** The *Exit codes* clause above covers what a pipe does to `$?`;
  this is what it does to COVERAGE, and it fails in the quieter direction — nothing is masked, the output
  is honest, and only the sentence written afterwards is wrong. `grep -rn '<token>' dirA/ dirB/ | head -5`
  walks argv in order, so all five slots can be consumed by `dirA/` while `dirB/` is never reached; a later
  claim that "`grep -rn '<token>' dirB/` returns nothing" then cites a command that was never run, and it
  reads as sourced because the quoted form is a plausible command whose output nobody has seen. Measured
  this session: exactly that sentence reached a committed report and the operator caught it — the true
  count for the second path was FOUR hits, all inside one inline `#[cfg(test)]` module. Two rules. **Never
  state a per-path result from a search that covered several paths** — re-run it against that path alone,
  bare, and read the count from the bare command. And when a clip is genuinely what you want, describe the
  peek's OWN scope in the sentence ("the first five hits across both trees"), never the narrower path's
  name. Same family as *Exit codes*: the shell answered a wider question than the claim reported.

- 2026-09-11: **A host-path guard anchored on a drive letter also matches the PowerShell REGISTRY provider form, so a probe that reports registry keys trips the project's own hygiene check on a string that is not a host path.** The anchor `[A-Za-z]:[\/]` is satisfied by `HKLM:\` and `HKCU:\` — one letter, colon, backslash — exactly as it is by a real drive path. Measured while writing a CI diagnostic that enumerates Edge policy keys: had the committed reading quoted the keys in the provider form the probe uses, the host-path gate would have reported a leak that does not exist. **Fix the OUTPUT, never the pattern**: probe with the provider form (`Test-Path` / `Get-ItemProperty` need it) and REPORT in the colon-free reg.exe form (`HKLM\SOFTWARE\...`), which is both the idiomatic rendering and unambiguously not a path. Loosening the anchor to admit the hive prefixes would blind the gate to the class it exists for. Same family as the false-positive clause in *Probes & pattern tools*: ask what OTHER content a pattern admits before trusting it — here the answer was found by writing the probe, not by running the gate. **Extended 2026-09-12 (`2026-09-11-hosted-runner-endpoint-cause-closed`) — n=2 in two days, so this is now a standing CHECK rather than a pair of anecdotes. The anchor `[A-Za-z]:[\/]` is satisfied by ANY `letter+colon+slash` run, and two non-path classes have now tripped it: the registry provider form (2026-09-11) and a URL SCHEME (2026-09-12 — `http://` is `p` + `:` + `/`). The check, to run whenever this anchor guards a committed artifact: before asserting the gate green, enumerate the artifact's `letter:slash` runs and confirm each is a real host path — a quoted URL, a registry provider path, a `C:`-style example inside prose and a scheme-bearing log line all match and none is a leak. Both fixes are the same: change the OUTPUT (elide the scheme, render registry keys in the colon-free `reg.exe` form) and state the elision where a reader would otherwise think the quote verbatim. Third instance, same day: the PROSE written to explain the false positive tripped it too, because the explanation spelled the offending tokens — describe such tokens, never quote them, inside an artifact the anchor guards.** **Extended 2026-09-14 — the fourth class is the GATE'S OWN COMMAND TEXT, and it is the one the check above does not reach, because the tripping tokens are not `letter:slash` runs at all.** A host-path gate's `run` string necessarily spells its whole alternation — the drive-letter anchor plus the sibling literals — so any sweep that scans an artifact CONTAINING that command matches the command rather than the content: sweeping this chunk's artifacts returned hits whose matched text was `/home/`, `/Users/` and `%APPDATA%` inside the quoted gate string, with **zero** drive-letter runs anywhere. Nothing was wrong and nothing needed fixing, which is the hazard — the same reading with one real leak present is indistinguishable at a glance. Two consequences. The enumerate-the-runs check above is necessary and NOT sufficient: also ask whether the artifact under sweep CONTAINS the pattern, and read the matched text (`grep -o`) rather than the matching lines, since the matched token is what separates a sibling literal from a drive path. And when scoping a hygiene gate, point it at the CONTENT artifact alone — this chunk's shipped gate scans only the contract document and is unaffected; it was the ad-hoc sweep across the whole chunk folder that pulled the plan's own gate text into scope.
- 2026-09-15: **`$TMPDIR` is UNSET in the Bash tool's shell — the OS temp dir is `$TEMP` — so a redirect to `$TMPDIR/...` silently writes to the MSYS ROOT, not to a temp dir.** An unset variable expands to nothing, so `$TMPDIR/foo.log` becomes `/foo.log`: the write succeeds, the shell reports nothing, and the file lands somewhere nobody looks. Measured this session: `TMPDIR` empty, `TEMP` and `TMP` both the real Windows temp path. This is a DIFFERENT mechanism from the *Paths & argument conversion* clause above, which warns that bash's `/tmp` is invisible to native tools — that one is about a real directory the other side cannot see; this one is about a path that was never formed. Write `$TEMP` (or an explicit absolute path) whenever a command needs the OS temp dir, and treat any `$VAR/`-rooted redirect as requiring proof the var is actually set — `${VAR:?}` fails loudly where a bare `$VAR` writes to `/`.
- 2026-09-15: **A saved sub-agent extract is persisted with the Write tool, never a text-mode python write — the python route silently rewrites every line ending to CRLF.** Python's text mode translates `\n` to the HOST's terminator, so an extract written that way lands 100 % CRLF while the same content through the Write tool stays LF. Measured this session: all twelve extracts in one phase run dir (`a11y`/`arch`/`design`/`layouts`/`obs`/`security`/`tests` plus five `.raw-*` twins) are CRLF on every line. The pipeline already prescribes the Write tool for this (`andromeda-phase/references/fan-out.md:59` — "persist via the Write tool, never a shell heredoc … Write is the default, not the fallback"), and the *Transports* clause above says the same for documents generally; what neither states is the CRLF consequence or that a python write is the route that produces it. Where a python write is genuinely required (a validated ledger append), open with `newline=''` so the terminator is the one you wrote, not the host's.
- 2026-09-16: **A phrase WRAPPED across two comment lines is invisible to a line-granular grep — the inverse of the 2026-09-10 clause above, where one long line hid many clauses.** Measured: `grep -l 'MCP round-trip' scenarios/*.toml` returned 3 where the truth was 4, because one file ends a line with `Conductor's own MCP` and resumes the next with `round-trip, …`. Sweep such prose by stripping the leading comment marker PER LINE, joining, then collapsing whitespace — flattening whitespace alone is not enough, since it leaves the `#` mid-phrase and still misses the hit.
- 2026-09-16: **A Windows capability check returning False can mean "denied right now", not "never had it".** Under a UAC split token an administrator's non-elevated process carries `BUILTIN\Administrators` as `Group used for deny only`, and `IsInRole(...)` returns **False** for it — measured: the account WAS a listed local admin while both role calls said False, and that reading was reported as "no admin rights". Read the token, not the predicate: `whoami /groups` by ABSOLUTE path (a bare `whoami` is the MSYS build here and rejects `/groups`) gives the deny-only flag and the `S-1-16-*` integrity SID; map the SID yourself, its label text is localized, and `.NET`'s `WindowsIdentity.Groups` omits the label entirely. Role and integrity are SEPARATE token attributes — `runas /trustlevel` strips the group and leaves the label — so an elevated-vs-not comparison moves both and can never say which one a behaviour depends on. Detail: `docs/session-learnings.md` 2026-09-16.
- 2026-09-16: **A surviving grandchild holding an inherited stdout handle pins the parent after the child is dead — and `Start-Process -Wait` does not escape it** (it drains the redirection too). `$output = & cmd *>&1` holds the pipeline until every holder exits; measured as ~15 min of a CI step after the leg died, reproduced twice locally. Use `Start-Process … -PassThru` WITHOUT `-Wait`, then `$proc.WaitForExit()` — waits on the child alone (88 s on the same leg). A bare `*>` is not the alternative: PowerShell redirects write UTF-16/BOM that greps read as empty. Reap orphans yourself by START TIME, in REPEATED passes with a settle — the app tree outlives the child, so one enumeration at exit caught 7 of 27 where repeated passes took 17.
- 2026-09-17: **A GitHub Actions `if: ${{ env.X }}` cannot read a runner-PROCESS variable, and it fails SILENTLY by skipping the step.** The `${{ env.* }}` context holds only what a workflow, job or step DECLARED **[corrected 2026-09-24: …or what an earlier step of the same job wrote to `GITHUB_ENV` — measured on `windows-latest` at run 36006370951; an image-set runner variable like `ImageOS` still never resolves]**, so `if: ${{ env.ImageOS == 'win22' }}` is always false — measured at run 35190456907, where a driver-pin step never ran, the job still looked like it executed, and the result read as a genuine measurement of the configuration the step was supposed to create. `ci.yml` already documented this trap for `EDGEWEBDRIVER` (run 34148079506), where it surfaced LOUDLY as an empty path; the `if:` form is the quiet variant and is worse. Put the image/environment test in the step's SHELL (`if ($env:ImageOS -ne 'win22') { …; exit 0 }`), which prints what it saw. Same family as the *Exit codes* rule: the shell answered a different question than the one asked.
- 2026-09-17: **A Windows process is attributed by PARENTAGE, never by image name or StartTime** — every WebView2-hosting desktop app spawns children under the one `msedgewebview2` image, so a name census cannot tell a harness orphan from Windows Search's or WhatsApp's, and a shared start time only means two things launched together. Read `Win32_Process` `ProcessId`/`ParentProcessId`/`CreationDate` and resolve each parent to its own image name; a census that reports counts without parents cannot support any claim about whose the processes are.
- 2026-09-23: **A multi-file count probe (`grep -c A B | grep -vc ':0$'`) reads GREEN when a file is missing** — under `pipefail` the exit is the rightmost failing stage, so stage 1's exit 2 is masked by stage 2's exit 1; lead with `test -f` on every file the probe must see.
