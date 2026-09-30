# Leak control — moved to the gitignored cache

The host-path sweep entry's known-positive control is a synthetic leak file. It lives in `.andromeda/cache/p5-controls/2026-09-30T07-52-13-phase/leaks.txt`, never in this committed run dir (security.md 2026-09-30).

- sha256: `2041eade41e0cab7074561a75d513e6ade0f07f028bf94407dffb4ffec331998`
- Beside it, `shell-true.py` (the spawn-grep entry's control: one shell-string subprocess call) moved to the same
  cache dir, so a committed run dir carries no Python source.
- Forms, in words: a drive-letter path in a `file =` value; a Unix home-directory path in a `citation_home =` value; one line carrying the Windows roaming-appdata variable, a user-home cargo registry directory and a user-home rustup toolchains directory; and one clean repo-relative tally line as the negative.
