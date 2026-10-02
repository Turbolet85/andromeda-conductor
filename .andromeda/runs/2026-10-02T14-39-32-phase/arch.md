# arch extract

## Relevance
partial — no new seam, surface or resource. Arch binds where the work lands (test tier vs a seam crate), the dependency posture of the digest pins, the LF and byte stability of re-written evidence, and the two arch passages that state or rest on the residuals.

## Constraints
- The residual wording lives in arch as well as in security-plan. Arch §Standard Contracts (Readiness gate, the "Corpus access" passage) names both residuals: the frozen 2026-09-22 file's one `fingerprint_hex` prefix, and the d3 all-digit prefix that `elide_fingerprints` "keeps by definition", which it calls overseer-ruled with founder ratification pending. A fix makes that clause false, so it is a wrap-time amendment target in this master.
- Arch §Established Decisions [Read-Back Dependency Posture] (the "Freshness is the canary's carrier BY CHOICE" paragraph) cites "Ten committed live envelopes of 2026-09-10" that carry a 32-hex grounded cue fingerprint. It treats them as evidence that payload fidelity is PARTIAL, meaning one read-back value varies with what Conductor emitted.
  - Under the wide P4 reading (elide every committed fingerprint value), that sentence and the evidence it rests on would be removed together.
  - The fork must weigh that the arch claim cites those values as its basis. P3 has to measure the envelopes and confirm the passage's location (this paragraph).
- Arch §Established Decisions [Read-Back Dependency Posture] (the "Conductor's fingerprint now IS Pulse's derivation" paragraph) fixes the shapes. Conductor's identity is 32 lowercase hex chars. Pulse's 8-char prefix (`fingerprint_to_hex_prefix`) is a logging convenience, not a wire identity.
  - The elider's shape definition has to cover the forms that actually occur: an 8-char prefix and a 32-hex value.
  - Which shapes `elide_fingerprints` matches today, and why an 8-char all-digit run escapes it, is research's question.
- Arch §Cross-cutting Patterns (Determinism discipline) makes fingerprints seed-drawn content ("timing, counts, order and content (fingerprints, …)"). So a value outside the capture class is Conductor-reproducible synthetic data, not corpus-rendered model text. This is an input to the narrow-vs-wide P4 fork, not a ruling on it.
- Arch §Established Decisions [Probabilistic-Assertion Policy] puts the pre-committed-rule arm at test tier in `conductor-run/tests/real_model_harvest.rs` (`row()`), and states that `conductor-verify`'s `classify()` and every `Verdict`/`ReportState` variant and envelope key stay unchanged.
  - Re-eliding d3 must leave its graded `(verdict, state)` exactly as it is.
  - Whether the grading ever reads the elided field is research's question.
- Arch §Stack and Technologies (the Hashing / digest row) and §Infrastructure Patterns → Build system scope `sha2 0.10` as a TEST-only `conductor-run` dev-dependency for the real-model harvest's sha256 digest pins over committed capture evidence, already locked with no new package. A moved pin reuses that edge. blake3 stays the only SHIPPED hashing dependency.
- Arch §Established Decisions [Module Boundaries] makes the crate-per-seam split compiler-enforced. If the elider widening touches a seam crate's `src/` rather than a test module, no new cross-seam edge may appear. Where `elide_fingerprints` lives (the `capture_paths` / harvest test modules or a crate `src/`) is research's question.

## Patterns to follow
- Digest-pinned graders over committed files: each capture is held by a sha256 pin and graded from its file, with no capture text in test source (arch §Standard Contracts Readiness gate, "Corpus access"; §Stack and Technologies Hashing row). A re-written graded capture moves its pin in the same change.
- Repo-wide LF at checkout (`* text=auto eol=lf`, arch §Infrastructure Patterns → Build system) is what keeps a byte-level digest stable on the Windows host and the Windows CI runners. A re-elided capture must land LF so its pin holds after checkout.
- Real-model capture evidence lands first at `runs/live-suite/rm-capture.txt` (git-ignored, harness-owned, cleared non-recursively; arch §Occupied Resources → On-disk artifacts). The committed copy sits in a chunk's `evidence/` tree. A fix to committed evidence is a tree edit and never needs a re-drive.
- Name the evidence for every claim in arch prose by its committed path ("as measured at `…/evidence/…`"; seen throughout §Established Decisions). Any arch sentence re-stated at wrap names the re-measured artifact.

## Anti-patterns to avoid
- Do not mint a new `CONDUCTOR_*` handle, `contracts/` member, `agent-run` verb or flag, or `runs/` subtree for this work. The registries in arch §Occupied Resources (Environment variables, On-disk artifacts) and §Cross-cutting Patterns (Config management) are closed to it.
- Do not change any `Verdict`/`ReportState` variant, envelope key or `classify()` to fit a re-elided capture. Grading stays at harvest tier (arch §Established Decisions [Probabilistic-Assertion Policy]; §Standard Contracts Run report envelope).
- Do not leave the arch residual clause standing once the fix lands. A clause stating a residual the tree no longer carries is drift in this master (arch §Standard Contracts Readiness gate, "Corpus access").

## Contract bindings
- arch ↔ security: arch §Standard Contracts Readiness gate ("Corpus access") restates security-plan §Security Anti-Patterns → Data Protection's scoped exception and its two residuals. Both masters retire the residual wording together at wrap, and neither may be edited into disagreement.
- arch ↔ tests: the digest-pin graders and the harvest arm that holds the residuals exactly are the test-tier carriers named in arch §Stack and Technologies (Hashing row) and §Established Decisions [Probabilistic-Assertion Policy]. Re-expressing the arm as "zero un-elided values" is a test-plan concern that binds to arch's digest-pin pattern.
- arch ↔ arch (internal): under the wide reading, §Established Decisions [Read-Back Dependency Posture]'s "ten committed live envelopes" evidence clause depends on the same committed values the fix would elide.

## Acceptance criteria contributions
- (arch) No new package enters `Cargo.lock`. The package count is unchanged and any digest pin uses the existing test-only `sha2` dev-dependency of `conductor-run` (per arch §Stack and Technologies, the Hashing / digest row).
- (arch) No new `CONDUCTOR_*` handle, `contracts/` member, `agent-run` verb or flag, or `runs/` subtree is added (per arch §Occupied Resources).
- (arch) Every re-written committed capture is LF in the index (`git ls-files --eol` shows `i/lf`), and its sha256 pin matches the committed bytes on a fresh checkout (per arch §Infrastructure Patterns → Build system).
- (arch) The re-elided 2026-10-01 d3 capture still grades to the same `(verdict, state)` through `real_model_harvest.rs`'s pre-committed-rule arm, with no `Verdict`/`ReportState` variant or envelope key changed (per arch §Established Decisions [Probabilistic-Assertion Policy]).
