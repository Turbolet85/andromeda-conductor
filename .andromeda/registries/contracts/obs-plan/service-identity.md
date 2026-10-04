### Service identity

- **service.name:** Hardcoded `"conductor"` (CLI) or `"conductor-tauri"` (Tauri GUI) or `"conductor-ui"` (browser frontend); overrideable via `$CONDUCTOR_SERVICE_NAME` env var at runtime
- **service.version:** Compile-time `env!("CARGO_PKG_VERSION")` from root `Cargo.toml`
- **deployment.environment:** Runtime `std::env::var("CONDUCTOR_ENV").unwrap_or_else(|_| "local".into())`
- **Resource attributes:** Emitted as flat fields on every JSONL log line via `tracing` structured fields (`service.name`, `service.version`, `deployment.environment`)
