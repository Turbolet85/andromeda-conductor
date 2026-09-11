# Security validation — route draft

## Insert
- Between `A11y CI gate at an honest terminal` and `Keyboard and focus-order coverage ownership`: **"CI-fetched WebView2 runtime disposition — the always-latest float pinned to a version or retired, per whichever terminal the gate reaches"** (epoch: `Epoch 3`)
  Reason: Per security-plan §Dependency Security (third dependency class), the Evergreen bootstrapper sits in no lockfile and is blind to every audit gate, and its float carries one stated exit condition — the `a11y` job actually gating — which this epoch decides and the draft leaves undisposed.

## Rewrite
- `Hosted-runner endpoint cause probed`: "diagnose-only, the reading recorded whatever it says" → "diagnose-only, the reading recorded scrubbed of host paths and runner identity"
  Reason: Per security-plan §Security Anti-Patterns → Logging, committed evidence records must leak no absolute host paths, and a policy-registry / loaded-module / service-account reading is exactly that class (the §Input Validation speech-log ingest row is the standing scrub precedent, added after a real host path arrived through a captured third-party reading).
- `Interpretation proven live`: "known root cause injected" → "known root cause injected through the emission path"
  Reason: Per security-plan §Security Anti-Patterns → Data Protection, staging a corpus row directly to induce a SUT state is the banned route, so the injection route belongs in the chunk's own words rather than being left open.
- `Structurally-dead assertion class retired`: "three live CountAtLeast keys and one Hard Contains to declare-only" → "three live CountAtLeast keys and one Hard Contains to declare-only, the validated config boundary held"
  Reason: Per security-plan §Input Validation (scenario-config row), this retirement edits the governed config surface, so any declare-only shape must remain inside that boundary rather than arriving as an unvalidated key.
