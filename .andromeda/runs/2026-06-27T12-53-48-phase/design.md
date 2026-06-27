# design extract

## No domain coverage
This chunk is CI/test infrastructure configuration (YAML workflows, TOML nextest config, artifact uploads) — it produces machine-parseable artifacts (JUnit XML, coverage lcov) and does not render any user-facing surface (desktop-webview UI or cli ANSI output), falling outside the design domain per the focus guide's out-of-scope exclusions (backend / API / test commands).
