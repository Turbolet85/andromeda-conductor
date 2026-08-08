# Design validation — route draft

## Insert
- Between `Current-SUT coverage classification` and `In-lane SUT scenarios`: **"Out-of-scope classification treatment — not-Conductor's rows distinct in the coverage matrix, footer roll-up and CLI table, never Blocked or Fail"** (epoch: `Epoch 1 — Foundation: re-aim at the SUT`)
  Reason: The route's fourth classification bucket has no treatment in design-system §Iconography (which defines exactly six visually distinct status treatments) or §Surface: cli Tokens (six bracket prefixes), so an out-of-scope capability silently collapses into `Blocked` against §Rejected Defaults.

## Reorder
- Move `Live per-P-ID verdict lamps` before `Operator-pause and checklist live firing`
  Reason: Per layout-templates §Signature placement (desktop-webview #3), the coverage-matrix header strip echoes the frozen step-index while held — that reinforcement can only be exercised if the matrix is run-aware before the first live hold fires.

## Rewrite
- `Operator-pause and checklist live firing`: "go/no-go hold and ManualCheck items exercised against a running Pulse" → "frozen-count hold-point, go/no-go hold and ManualCheck items exercised against a running Pulse"
  Reason: Per design-system §Brand Identity (Signature element) and layout-templates §Signature placement, the count freezing at the exact hold value — not blanking, not continuing — is the behavior a live hold must prove, and the current text omits the signature entirely.
- `Live per-P-ID verdict lamps`: "coverage view rows carrying each capability's actual run verdict state" → "coverage rows carrying each capability's actual verdict state, not-yet-run rows distinct, header echoing the held step index"
  Reason: Per design-system §Rejected Defaults ("Conflating 'no result yet' with 'failed'"), a capability with no run record must keep its distinct not-run treatment rather than defaulting into the verdict triad, and layout-templates §Signature placement #3 puts the held step-index echo on this same header strip.
- `Dependency polish`: "indicatif bump and opentelemetry-proto default-features trim" → "indicatif bump preserving the stop-in-place hold spinner, plus opentelemetry-proto default-features trim"
  Reason: Per design-system §Surface: cli Component Patterns 1 and §Per-Surface Bans (cli), the spinner must STOP in place at the operator-pause — never hide, never animate-to-100% — which is the CLI mirror of the signature a version bump can silently regress.
