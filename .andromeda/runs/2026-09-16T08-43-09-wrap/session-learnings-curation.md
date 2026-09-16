
## An enumeration's SCOPE is established by what already sits OUTSIDE it (2026-09-16)

A drift proposal wanted `architecture.md`'s "**A second integrity gate sits beside it on a different
axis**" sentence widened to name a third gate, on the reading that it is architecture's enumerating
home for `conductor-core`'s static integrity gates. Read by offset — the sentence lives at `:53`
+1655 inside a 2 862-char line — it names `check_sut_drift` (is every accepted capability
*classified*?) and `check_scenario_backing` (does every `Auto` classification have a scenario?), both
on the accepted-capability-set axis.

What settled it was not the sentence but its neighbourhood: `check_load_envelope` is an existing,
shipped static gate of *identical shape* — a catalog-wide check returning a named `CoreError` — and it
appears nowhere in that pair, only at its own artifact row (`:177`). A sibling of the same shape
already living outside an enumeration is proof the enumeration was never exhaustive of that shape. So
the sentence was not stale, and the new gate's correct home was the artifact row a different proposal
in the same batch already covered.

The move generalizes to any doc-reconcile: before widening an enumeration because your new thing
resembles its members, look for a thing that already resembles them and is NOT in it. If one exists,
the enumeration is scoped by something narrower than resemblance, and widening it would make the doc
say less precisely what it used to say. The corollary is procedural — this is only visible from the
neighbourhood, so a proposal about an enumeration is checked against the doc's other mentions of the
same class, never against the proposed sentence alone.

## Ownerless work discovered mid-chunk takes the armed-orphan form, not a plan note (2026-09-16)

This chunk carried a finding with no owner: `scripts/mutation-gate.py` computes its verdict from
`missed.txt` and `caught.txt` and never reads `timeout.txt`, so a caught→timeout regression passes it
silently — and no route entry names the mutation gate, so nothing in the version owned the repair.

The instinct was to write it into the plan as "a route candidate". The operator's correction: that is
a CARRY, and a carry is how the previous version accumulated debt — it survives on attention rather
than on a mechanism. The pipeline already has the channel, `route-resolve.md:24`: an IN-VERSION
follow-up with no plausible owner entry is a trajectory halt that arrives **ARMED**, enumerating four
dispositions with a lean — pin to a named entry · mint an entry · `residuals.md` · drop — so the
decision reaches the operator at a named moment with the work already done, as one question rather
than a re-derivation.

Two details worth keeping. The lean has to be argued from the candidates: "pin to the nearest
plausible entry" was rejected here because the nearest was a full-gate regression sweep, and burying a
fifteen-item per-item disposition inside a sweep is the mixing hazard the finding is about. And the
operator's own sharpening on effort — reading `timeout.txt` is a small patch, but rostering the
fifteen timeouts is judgment, not a patch — is what made "mint an entry" the honest lean rather than
"absorb it here".
