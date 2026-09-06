# Coverage matrix

**Capabilities** 82 · 66 in scope (43 auto (8 unbacked) · 16 drive+observe · 7 static-only) · 16 not-conductors

| P-ID | Title | Category | Mode |
|---|---|---|---|
| `P-001` | Receiver Lifecycle State | Connection & Health Awareness | drive+observe |
| `P-002` | Last-Span-Ago Tracking | Connection & Health Awareness | drive+observe |
| `P-003` | Receiver Failure Surface | Connection & Health Awareness | drive+observe |
| `P-004` | Orthogonal Health Domains | Connection & Health Awareness | drive+observe |
| `P-005` | Span Status Error Detection | Hard Signal Detection | auto |
| `P-006` | Exception Event Capture | Hard Signal Detection | auto |
| `P-007` | High-Severity Log Capture | Hard Signal Detection | auto |
| `P-008` | Root-Span Error Scope Distinction | Hard Signal Detection | auto |
| `P-009` | Per-Service Error Rate Baseline | Statistical Anomaly Detection | auto |
| `P-010` | Error Rate Spike Detection | Statistical Anomaly Detection | auto |
| `P-011` | Per-Operation Latency Baseline | Statistical Anomaly Detection | auto |
| `P-012` | Latency Regression Detection | Statistical Anomaly Detection | auto |
| `P-013` | Service Activity Floor Learning | Statistical Anomaly Detection | auto |
| `P-014` | Service Went Silent Detection | Statistical Anomaly Detection | auto |
| `P-015` | Restart Event Detection | Pattern Recognition | auto |
| `P-016` | Restart-Window Suppression (Surgical) | Pattern Recognition | auto |
| `P-017` | Exception Fingerprinting | Pattern Recognition | auto |
| `P-018` | Retry Storm Detection | Pattern Recognition | auto |
| `P-019` | Three-Tier Severity Model | Severity Calibration | auto |
| `P-020` | Model-Driven Severity Decision | Severity Calibration | auto |
| `P-021` | Algorithmic Attention Cues | Severity Calibration | auto |
| `P-022` | Auto-Resolution and Lifecycle | Severity Calibration | auto |
| `P-023` | Acknowledge Cool-Down | Severity Calibration | auto |
| `P-024` | Widget Ambient Surface | Three-Surface Communication | drive+observe |
| `P-025` | Halo Hue Encoding | Three-Surface Communication | drive+observe |
| `P-026` | Halo Breathing Encoding | Three-Surface Communication | drive+observe |
| `P-027` | Service Constellation Auto-Discovery | Three-Surface Communication | drive+observe |
| `P-028` | Findings Counter | Three-Surface Communication | drive+observe |
| `P-029` | Findings Dropdown | Three-Surface Communication | drive+observe |
| `P-030` | No Interrupting Notifications by Default | Three-Surface Communication | drive+observe |
| `P-031` | Report Structure | Diagnostic Quality | auto |
| `P-032` | Project Context Grounding | Diagnostic Quality | auto |
| `P-033` | Ranked Hypothesis Generation | Diagnostic Quality | auto |
| `P-034` | Suggested Investigation Steps | Diagnostic Quality | auto |
| `P-035` | Anonymized Telemetry Excerpts | Diagnostic Quality | auto |
| `P-036` | Cross-Incident Pattern Reference | Diagnostic Quality | auto |
| `P-037` | In-App Report Surface | Output Channels | drive+observe |
| `P-038` | Copy to Clipboard | Output Channels | static-only |
| `P-039` | MCP Delivery When Configured | Output Channels | auto |
| `P-040` | MCP Independence | Output Channels | static-only |
| `P-041` | Persistent Incident Corpus | Memory & Learning | auto |
| `P-042` | Cross-Session Continuity | Memory & Learning | auto |
| `P-043` | Project-Scoped Memory | Memory & Learning | auto |
| `P-044` | Retrieval-Augmented Interpretation | Memory & Learning | auto |
| `P-045` | Counter Derivation from Corpus | Memory & Learning | auto |
| `P-046` | Export for Community Training | Memory & Learning | static-only |
| `P-047` | PII Scrubbing at Ingestion | Privacy & Trust | auto |
| `P-048` | No Raw OTLP Attribute Values Stored | Privacy & Trust | auto |
| `P-049` | Encryption at Rest | Privacy & Trust | static-only |
| `P-050` | Cross-Project Sharing Opt-In | Privacy & Trust | static-only |
| `P-051` | Transparent Storage | Privacy & Trust | static-only |
| `P-052` | Cadence Configuration | Pipeline Operations | auto |
| `P-053` | Fallback Model Tier | Pipeline Operations | auto |
| `P-054` | Hardware Profile Awareness | Pipeline Operations | static-only |
| `P-055` | Configuration Hot Reload | Pipeline Operations | auto |
| `P-056` | Prospective Threshold Application | Pipeline Operations | auto |
| `P-057` | Dual-Condition Suppression Bypass | Pipeline Operations | auto |
| `P-058` | Pipeline Self-Observability | Pipeline Operations | drive+observe |
| `P-059` | Active-Incident Interpretation Continuity | Pipeline Operations | auto |
| `P-060` | Tiered Triggering Priority | Severity Calibration | auto |
| `P-061` | Window geometry + movable shell | Window & shell hygiene | _not-conductors_ |
| `P-062` | Window size constraints | Window & shell hygiene | _not-conductors_ |
| `P-063` | Predictable close + honest tray | Window & shell hygiene | _not-conductors_ |
| `P-064` | Suppress browser context menu | Window & shell hygiene | _not-conductors_ |
| `P-065` | Canvas not a browser image | Window & shell hygiene | _not-conductors_ |
| `P-066` | Widget-to-dashboard navigation | Window & shell hygiene | _not-conductors_ |
| `P-067` | Live-only service truth | State honesty & legibility | drive+observe |
| `P-068` | Anomaly surfacing | State honesty & legibility | _not-conductors_ |
| `P-069` | Legible labeled constellation | State honesty & legibility | _not-conductors_ |
| `P-070` | Plain-language connection status | State honesty & legibility | _not-conductors_ |
| `P-071` | Self-explaining empty states | State honesty & legibility | _not-conductors_ |
| `P-072` | Investigate actions functional | AI-debug climax | drive+observe |
| `P-073` | Deterministic env-gated L4 mode | AI-debug climax | auto |
| `P-074` | Tier1 incident-path reliability under load | AI-debug climax | auto |
| `P-075` | Conductor e2e + delegated-timing verification | External verification | drive+observe |
| `P-076` | Integration UX e2e test | Test gap & housekeeping | _not-conductors_ |
| `P-077` | Demo telemetry injector formalized | Test gap & housekeeping | _not-conductors_ |
| `P-078` | Agent-headful self-verify harness | Test gap & housekeeping | _not-conductors_ |
| `P-079` | Constellation severity live-wiring | Operator-surfaced | auto |
| `P-080` | Incidents dropdown bounded popover | Operator-surfaced | _not-conductors_ |
| `P-081` | Traces table live refresh | Operator-surfaced | _not-conductors_ |
| `P-082` | Traces table internal scroll | Operator-surfaced | _not-conductors_ |
