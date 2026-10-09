# Merge decisions — conductor-0.4.0 route draft, third pass

_Phase 3 of the third pass (after the operator's edit of F7b at the second Phase 4). One line per suggestion:
`{validator} {kind} · {applied|adjusted|rejected|deferred} · {reason / adjustment / tradeoff}`. 5 suggestions from
two validators; the design, obs and a11y validators returned none. 4 applied · 1 adjusted · 0 rejected ·
0 deferred. No suggestion installed, extended or sequenced work for a retired surface. The earlier records stand in
`first/merge-decisions.md` and `second/merge-decisions.md`; the draft as it stood before this merge is
`.draft-before-merge.md`._

## security

security Rewrite `Channel refusals` ("no token or door credential in plaintext to another host") · applied · Epoch 6 takes two surfaces beyond the local machine, the stream and the door read path, and a run holds a credential for each; the refusal named only the token.
security Rewrite `Channel refusals` ("rejected credential reported blocked") · applied · Beside "unverifiable engine refused", "unauthenticated engine" read as the same engine-identity failure; v4-11's arm is the other direction — the engine refusing what Conductor presents.

## tests

tests Insert `Two runs back to back` (Epoch 3, after `Harness verbs on the one form`) · applied · The revised v4-18 makes every proof two runs on one engine that differ in exactly one declared thing; what the first run leaves on the engine would be a second, undeclared difference, and today the separation lives only in the live suite's leg order, which Epoch 5 retires. It rests on v4-12 (what the engine already held is told apart) and v4-18. Its cited mechanism — a new incident deduped against an open one — was measured against Pulse 0.3.0; the entry asks only that what separates two runs be stated, so it holds whether or not the 0.4.0 engine behaves the same. Surfaced at Phase 4.
tests Rewrite `Short regression runs: quiet and lifecycle` (+ "engine start-state dependence stated") · applied · A silent-service run inside a freshly started engine's learning window reads "nothing reported", the same as its event-less control, so under the revised v4-18 the pair would not discriminate and an engine's age would grade as a miss. v4-12 already has a run state the engine state it starts from. The window itself is a Pulse 0.3.0 measurement; the same note as above applies.
tests Rewrite `Channel refusals` (+ "refusals checked engine-less in CI") · adjusted · Applied as an entry of its own, `Channel refusals checked without an engine`, directly after — with both security rewrites the line could not hold the clause in 25 words. It is the channel's twin of `Door reads checked without an engine`: each refusal is Conductor's own behaviour, so under v4-12 it belongs to CI.

## design

design — · no suggestions · The two rewrites of the second pass stand; this pass's edits add no token, label or surface.

## obs

obs — · no suggestions · All three second-pass suggestions are in the draft; this pass's edits add no own-log, gate or scrubbing surface.

## a11y

a11y — · no suggestions · Unchanged from both earlier passes.
