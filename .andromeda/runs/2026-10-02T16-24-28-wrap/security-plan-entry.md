
## 2026-10-02-captured-fingerprint-values-elided — both capture fingerprint residuals fixed; the keyed elision rule
**Section:** §Input Validation → the real-model capture ingest row · §Security Anti-Patterns → Data Protection (the `corpus.db` exception: its elision clause and its residual statements)
**Change:**
- Ingest row: `elide_fingerprints` is now two rules. KEYED: the alphanumeric value after every `fingerprint_hex=` prints `<fingerprint>` whatever its class, all-digit included; an existing `<fingerprint>` or an empty keyed value is left as it is. This is the shape of the `workspace=` rule. UNKEYED: unchanged — a fingerprint-shaped token is elided, and an unkeyed all-digit run (a stamp, a seed) passes.
- The ingest row now records the harvest's population read: every committed `rm-capture*.txt` under the chunks' `evidence/`, workspace-root anchored with no handle, held at zero un-elided keyed values and a fixed point of the elision, the population pinned by count beside an inverse control.
- The row's "the 2026-10-01 d3 residual" pointer is retired.
- Data Protection: the exception clause was "every fingerprint-shaped token elided — an all-digit run … passes"; it is now every `fingerprint_hex=` value of any class plus every unkeyed fingerprint-shaped token.
- Data Protection: both stated residuals are retired as FIXED (was: the frozen 2026-09-22 file keeps its prefix; the d3 all-digit prefix overseer-ruled, founder ratification pending, per the 2026-10-01 entry "the capture's skip witness; an all-digit residual pending the founder"; now both are fixed).
- The frozen file was elided in place, a recorded exception to "frozen evidence is never edited", and now equals the graded copy.
- d3 was re-elided with its pin moved, and the 2026-10-01 harvest arm asserts zero for every drive.
- No committed quote of either value remains in the tree; git history was not rewritten.
**Why:** The founder ruled the residuals FIXED, never ratified (2026-10-02, relayed by the overseer), and the overseer answered the frozen-file fork as elide-in-place. The change is a narrowing: more is elided, and no input class or write is admitted. A keyed value is a fingerprint whatever its digits, while an unkeyed all-digit run stays a stamp or a seed, so widening the shape rule instead would break every capture's fixed point.
**Ref:** .andromeda/runs/2026-10-02T16-24-28-wrap/
