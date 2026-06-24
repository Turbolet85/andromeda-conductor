# Fan-out results — 2026-06-24-sanitized-stderr-agent-mode-logging wrap

7 drift-detector doc-agents (raw returns in the session transcript). 5 proposals, all `warning`, 0 escalations → all routine, applied. Cascade no-op (verified).

## Proposals + verdicts
| Doc | Detector | Proposal | Verdict | Applied |
|---|---|---|---|---|
| arch | D-arch-resources | register `CONDUCTOR_AGENT_MODE` env var | routine (genuine new env-var; arch §Occupied Resources tracks env-vars) | §Occupied Resources Environment variables + sidecar |
| arch | D-arch-resources | register `logs/agent-latest.jsonl` artifact | routine (genuine new on-disk artifact) | §Occupied Resources On-disk artifacts + sidecar |
| security-plan | D-security-input (→warning) | add `CONDUCTOR_AGENT_MODE` to non-path no-validation note | routine (doc-align; invariant holds — bool, value unused, log path via `resolve_under`) | §Input Validation + sidecar |
| obs-plan | D-obs-stack | reword §3 "sets `CONDUCTOR_AGENT_MODE=1`" → read-only trigger (D4) | routine (spec-wording→sound-impl reconcile; observable mode identical) | §3 (2 sink blocks) + sidecar |
| design-system | D-design-tokens | document error:/hint: token colors | routine, **CORRECTED** — agent proposed a new "Hint grey" palette row; 246 IS the existing Residual-mute → applied a use-site note (reuses Fail 203 / Residual 246), not a duplicate color | §cli "Error output" + sidecar |
| layout-templates | D-layout-surface | — | `proposals: []` (error:/hint: already in §cli; --agent-mode is plumbing) | — |
| test-plan | D-tests-* | — | `proposals: []` (tests present; CONDUCTOR_AGENT_MODE wording is obs-plan's, not §3) | — |
| a11y-plan | D-a11y-* | — | `proposals: []` (cli not-assertable; violation schema unchanged) | — |

## Validation notes
- **No cross-contradictions** (the two arch proposals are complementary additions to the same registry).
- **Intent-consistent** — every proposal faithfully reflects a report Changes-bullet (the new env var, artifact, and the D4 wording) and the chunk's plan/scope intent.
- **Playbook hits:** the CLI-flag/symbol over-reach rule (kept the `--agent-mode`/`--debug` flags OUT of arch); the consuming-shipped-hardened-infra rule (D-security-input / D-obs-redaction not escalated — the file sink reuses the unchanged redaction layer); the spec-wording→sound-impl reconcile rule (obs §3 + the design use-site note). No NEW playbook rule needed (all matched existing rules).
- **Cascade:** no-op — grep-confirmed CLAUDE.md / stack.md / the 5 summaries / the 6 rules carry no "sets CONDUCTOR_AGENT_MODE=1" wording and no per-env-var/per-artifact/per-cli-token inventory; obs-summary + observability.md already say "`logs/agent-latest.jsonl` (`--agent-mode`)".
