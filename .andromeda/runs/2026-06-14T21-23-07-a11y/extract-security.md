## 2. Security Plan Excerpt

### Security Tier
- **Tier:** Minimal
- **Justification:** "Every signal points to a minimal-tier local utility — a single-developer, local-only, no-cloud, no-multi-tenancy tool with no user accounts (auth 'none'), where the only persisted data is self-generated synthetic test telemetry and run-metadata in an embedded SQLite index (`runs.db`) plus on-disk JSONL/Markdown artifacts (no PII/payment/health/credentials owned by Conductor), and there is zero network exposure."

### Anti-Patterns Rejected (a11y-relevant)
(No a11y-relevant anti-patterns in security plan — Phase 3 will apply default a11y discipline to all auth / verification UI.)

### A11y Compliance Triggers
(No a11y compliance triggers in security plan — Phase 1 will derive a11y tier from creator brief + project intent + surface count.)
