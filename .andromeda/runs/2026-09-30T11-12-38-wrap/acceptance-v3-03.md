Every a11y-plan §5 keyboard-reachability, focus-order and focus-visible claim is owned by exactly one suite and actually asserted there.

The claim population is the committed enumeration `crates/conductor-tauri/ui/test/a11y/claim-ownership.ts`: 11 claims, including SC 2.4.7's visible `--color-focus` ring on the active element. They split as 9 owned, 2 n/a-by-construction with a stated basis, and 0 unasserted.

Ownership is proven by `npm run a11y:ownership`, which exits 0 printing `11 claims · 9 owned (5 operator-local, carve-out) · 2 n/a-by-construction · 0 recorded gaps` with last line `ownership: every claim resolved`. The checker fails any owned row whose named spec is missing or has no `expect(` in its body.

Each owning suite's specs pass:
- The routine arm (CI) owns the idle-console reachability and focus order, SC 2.4.7, and idle-with-report coverage-matrix row navigation. It passes as `CONDUCTOR_A11Y_STRICT=1 bash scripts/agent-run.sh run --e2e`, 0 failed, with the expected-skip SET held at two.
- The operator-local driven arm owns the HOLD trap, focus order and restoration, run-console-live row navigation, and the start/stop/proceed/abort shortcuts, each a real keypress. It passes as one live run: `npm run a11y:driven` prints `Spec Files:`, a TAB, then ` 1 passed, 1 total` (the runner's own bytes; the single-space quote this acceptance first carried was a whitespace collapse).

For each of the 5 claims whose owner continuous integration does not run, the enumeration states what CI gates in its place, and the checker FAILS a carve-out row that states nothing. a11y-plan §11 Strategy restates those in-place gates as the requirement's own record.