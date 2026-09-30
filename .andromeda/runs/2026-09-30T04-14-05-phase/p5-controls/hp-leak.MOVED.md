# hp-leak.txt — moved out of the committed tree

`hp-leak.txt` was this phase run's known-positive control for plan entry 26 (the evidence host-path probe): a
five-line file the probe must count 5 on and fail. It is synthetic, and no line names a real host path. It was
moved 2026-09-30 at the /implement operator pass because the hygiene gate (`gate.py hygiene`, plan entry 28)
refuses any would-be-committed file carrying host-path forms, and the overseer ruled (founder-delegated) that the
control is never committed but stays traceable.

- sha256 (unchanged by the move): `27f414696976ae3019cb26691fb88408f130bf2538b53adb20d6d5408145d8a4`
- Now at: `.andromeda/cache/p5-controls/2026-09-30T04-14-05-phase/hp-leak.txt` (gitignored by `/.andromeda/cache/`;
  local to this host, not versioned).
- Its five lines, described in words:
  1. a drive-letter path inside backticks, after the word "see";
  2. a drive-letter path under a users directory, opened by a parenthesis;
  3. a drive-letter path as the value of a `path=` assignment;
  4. a drive-letter path behind the extended-length (backslash, backslash, question mark, backslash) prefix;
  5. an MSYS-style single-letter root path after a `cd` command.

The three other controls beside this note (`hp-clean.txt`, `key-leak.txt`, `key-clean.txt`) pass the hygiene gate
and stay committed.
