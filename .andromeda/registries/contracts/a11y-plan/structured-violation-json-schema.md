### Structured violation JSON schema

Binding contract — see upstream-context Section 6 Obs Plan Excerpt → Log Format JSON Schema. A11y emissions align to the obs schema (a11y narrows; obs fixed it), reproduced verbatim:

```jsonl
{
  "journal_emitted_at": "ISO-8601 from std::time::SystemTime",
  "read_back_observed_at": "ISO-8601 from std::time::SystemTime (null until read-back; null for blocked rows)",
  "run_id": "YYYY-MM-DDTHH-MM-SS-<suffix> (filesystem-safe hyphen-delimited)",
  "seed": "u64",
  "scenario": "string",
  "p_ids": ["P-001", "P-002", ...],
  "verdict": "Pass | Fail | CalibrationRegion",
  "state": "Pass | Fail | ManualCheck | KnownResidual | Blocked",
  "latency_ms": "integer or null (null for blocked rows)",
  "slo_tier": "<5s | <20s | <90s",
  "fingerprints": ["fingerprint1", "fingerprint2", ...] or empty array
}
```

WCAG-violation field mapping onto this envelope (without diverging from the obs contract): each axe-core `violations[]` entry folds into a per-run a11y artifact whose envelope `state` = `Fail` when `violations.length > 0` else `Pass`; the axe `id` (e.g., `color-contrast`, `button-name`), `impact`, WCAG `tags`, and node `target` selector ride inside the `fingerprints[]` array (axe rule-id + WCAG-SC tag + selector tuple) so no new SCENARIO-DATA field is introduced. **The shipped artifact's shape, measured 2026-09-07:** the eleven envelope keys PLUS the obs resource tags `service.name` (`conductor-ui`) and `deployment.environment` — 13 top-level keys locally, 15 under CI where `ci.run.id` + `git.commit.sha` join them. Those extras are RESOURCE TAGS, required by obs §9 and admitted by obs §3's extension point, which `conductor-run`'s `journal_conformance` honours by asserting key PRESENCE rather than key exclusivity; the earlier unqualified "no new top-level fields" reading is retired, because plan-step and gate both require the tags. The scenario-shaped keys are MAPPED, not measured (`seed: 0` · `scenario` = the invoked suite · `p_ids: []` · `slo_tier` `<90s` · `read_back_observed_at` = suite end; `verdict`/`state` = `Fail` when any violation tuple was recorded OR any spec failed), and `fingerprints[]` carries one tuple PER NODE rather than per violation — `target` is a per-node field, and a single `color-contrast` violation has spanned 18 of them. The colorjs.io token-name assertion and WebdriverIO-keyboard PASS/FAIL emit the same way. Per tests' Status JSON constraint, artifacts MUST NOT leak absolute host paths or internal seam-crate struct names — node `target` selectors are stored as DOM-relative selectors / `data-testid`, never filesystem paths. Resource attributes `service.name` = `"conductor-tauri"` / `"conductor-ui"` and `deployment.environment` tag each artifact for obs cross-correlation.
