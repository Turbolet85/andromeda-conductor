# Revert-red control — the two-drive identity test

The plan's step 5 control: the two-drive test is read RED with the re-key call removed, then GREEN with it restored. This
is the "fails when the fix is reverted" witness. The tree was mutated once by /implement and then restored. Nothing
from the mutated state remains: the restored file is a byte copy of the pre-mutation file, and the restored call
counts 1.

## The mutation

In `crates/conductor-run/src/dispatch.rs`, `Dispatcher::export_traces`, the one line
`rekey_trace_identity(&mut request, salt);` was replaced with `let _ = (salt, rekey_trace_identity);`. That leaves
every other line compiling and identical, so the salted tier exports seed-pure identity.

## RED (re-key removed) — 2026-10-01T21:57:15Z

`cargo nextest run -p conductor-run --test dispatch_wire --profile ci` → exit 100.

- `FAIL conductor-run::dispatch_wire two_same_seed_drives_inside_one_window_share_no_span_identity`
- `panicked at crates\conductor-run\tests\dispatch_wire.rs:539:5`
- `assertion left == right failed: two same-seed executions share no (trace_id, span_id) pair — Pulse's span-store key`
- `left: 13` · `right: 26`. The union of the two drives' pairs held 13 members where |A| + |B| = 26, so every one of
  drive B's 13 identities replayed drive A's. That is the d2 shape.
- `Summary 16 tests run: 15 passed, 1 failed, 0 skipped`. The three frozen goldens and every other test stayed green,
  because the unsalted tier never passed through the call.

## GREEN (re-key restored) — 2026-10-01T22:03:37Z

The same command → exit 0.

- `PASS conductor-run::dispatch_wire two_same_seed_drives_inside_one_window_share_no_span_identity`
- `Summary 16 tests run: 16 passed, 0 skipped`

## What the drive covers

13 spans per drive, made of two phases:

- an Exception phase with 5 occurrences (`Identical` and `Path` variants), giving 5 single-span traces with exception
  events;
- an Error phase with 4 occurrences at `depth = 2` and `error_percent = 50`. That gives 2 three-span linked chains and
  2 plain spans.
