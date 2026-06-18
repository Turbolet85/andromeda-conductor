# Session Handoff

**Last Updated:** 2026-06-18T00:45:24Z
**Branch:** build/conductor-0.1.0
**Status:** clean
**Last Commit:** 2026-06-18-exception-events-fingerprint-control — feat: exception events + fingerprint control (conductor-emit)

## Position
- Done: **2026-06-18-exception-events-fingerprint-control** — `conductor-emit/src/exception.rs`: OTel `exception` span events (`exception.type`/`message`/`stacktrace`) + a path+line-insensitive `fingerprint()` (FNV-1a) over five variants — identical/path/line ⇒ SAME fp; type/frame ⇒ DIFFERENT (amended P-017 clause (c)). **Epoch 3 (Emission primitives) 3/8.**
- Next: **Epoch 3 chunk 4 — "Severity logs"** (log records with controlled `SeverityNumber` across the 17-boundary, P-007) → `/andromeda-phase` to promote + plan.

## Work done
Added `conductor-emit/src/exception.rs` (`ExceptionSpec`/`Frame`/`FingerprintVariant` + `exception_trace_request` + content-derived FNV-1a `fingerprint` + 7 unit tests) and `tests/exception_events.rs` (loopback gRPC-stub integration). `message.rs` gained `span_with_events`/`string_kv` (+ `service_resource` refactor); `unix_nanos` and `span_tree::gen_id` → `pub(crate)`. Gates green: nextest 99/99 · clippy `-D` · audit + deny · `Cargo.lock` un-drifted (no new deps).

## Drift resolved
1 amendment (arch, routine): clarified fingerprint ownership — the per-exception fingerprint PRIMITIVE lives in `conductor-emit`; `conductor-faults`' "fingerprint generation" narrowed to the fingerprint-STORM fault (Epoch-7). architecture.md §dir-tree body + CLAUDE.md §Modules cascade + sidecar. Playbook rule #5 (spec→sound-impl) + the crate-seam was user-ratified in /andromeda-phase. The other 6 docs returned `proposals: []`. **drift = 0.**

## Notes
- **Key decisions:** crate-seam ratified (fingerprint in emit, not faults); the fingerprint is **content-derived** (FNV-1a, version-stable — never `DefaultHasher`), NOT seed-derived (the seed drives only span identity); amended P-017 clause (c) — identical/path/line variants share one fp, type/frame differ.
- **Curation:** Tier 2 → testing.md (content-hash / version-stable determinism rule) · Tier 3 → session-learnings.md (primitive-in-producing-crate heuristic). Filtered: 1 low-confidence (scope.md-error process note).
- **Follow-up (tracked, not a route chunk):** `opentelemetry-proto` `default-features = false` to drop the dormant transitive `opentelemetry_sdk` — **still open** (no deps touched this chunk). Do opportunistically in a later emission chunk.
- **Last failed command:** none.
