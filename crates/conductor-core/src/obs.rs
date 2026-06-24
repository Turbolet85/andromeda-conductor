//! Self-observation logging — the `tracing` JSON stack.
//!
//! Conductor's self-observation is structured `tracing` JSON to stdout/file ONLY; it uses **no
//! OpenTelemetry SDK** (the sole OTLP Conductor speaks is the PRODUCT fault stream to Pulse
//! `:4317`, a separate path — obs-plan §11). [`init_observability`] installs the global subscriber
//! plus a panic hook so every emitted line is one flat JSON object carrying the service identity
//! and the `run_id` correlation key, and no panic goes unlogged (obs-plan §3 / §10).

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::redact::{is_allowlisted, redact_value};
use serde_json::{Map, Value};
use tracing::field::{Field, Visit};
use tracing::{Event, Subscriber};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::{Context, Layer, SubscriberExt};
use tracing_subscriber::{EnvFilter, Registry};

/// Service identity stamped as flat fields on every self-observation log line (obs-plan §3).
#[derive(Debug, Clone)]
pub struct ServiceIdentity {
    pub service_name: String,
    pub service_version: String,
    pub deployment_environment: String,
    pub run_id: String,
}

impl ServiceIdentity {
    /// Resolve identity from the process environment, falling back to `default_service_name`
    /// (override `$CONDUCTOR_SERVICE_NAME`), `$CONDUCTOR_ENV` (default `local`), and a freshly
    /// minted `run_id` when none is supplied.
    pub fn resolve(default_service_name: &str, run_id: Option<String>) -> Self {
        Self::resolve_with(default_service_name, run_id, |key| std::env::var(key).ok())
    }

    fn resolve_with(
        default_service_name: &str,
        run_id: Option<String>,
        get_env: impl Fn(&str) -> Option<String>,
    ) -> Self {
        Self {
            service_name: get_env("CONDUCTOR_SERVICE_NAME")
                .unwrap_or_else(|| default_service_name.to_string()),
            service_version: env!("CARGO_PKG_VERSION").to_string(),
            deployment_environment: get_env("CONDUCTOR_ENV").unwrap_or_else(|| "local".to_string()),
            run_id: run_id.unwrap_or_else(mint_run_id),
        }
    }
}

/// Where the self-observation JSON stream is written: the default operator-facing `stderr`, or a
/// `logs/agent-latest.jsonl` file in `--agent-mode` (obs-plan §3 dual sink). The agent file keeps the
/// machine stream off stderr, so a piped agent reads a clean stderr (the `error:`/`hint:` summary only).
#[derive(Debug, Clone)]
pub enum ObsSink {
    Stderr,
    AgentFile(PathBuf),
}

/// Install the global `tracing` JSON subscriber over `sink` + the panic hook, then emit a startup
/// line. Call once at binary startup, before any other logic. Returns the resolved identity (its
/// `run_id` is the run's correlation key). The first install in a process wins; a later call leaves
/// the global subscriber untouched. Infallible: an `AgentFile` that cannot be opened falls back to
/// stderr (startup logging is best-effort, never a hard failure).
pub fn init_observability(
    default_service_name: &str,
    run_id: Option<String>,
    sink: ObsSink,
) -> ServiceIdentity {
    let identity = ServiceIdentity::resolve(default_service_name, run_id);
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
    let subscriber = build_subscriber(identity.clone(), resolve_writer(sink), filter);
    if tracing::subscriber::set_global_default(subscriber).is_ok() {
        install_panic_hook();
        tracing::info!("observability initialized");
    }
    identity
}

/// Resolve an [`ObsSink`] to its writer, falling back to stderr when the agent log file cannot be
/// opened — startup logging never blocks on a sink failure.
fn resolve_writer(sink: ObsSink) -> ObsWriter {
    match sink {
        ObsSink::Stderr => ObsWriter::Stderr,
        ObsSink::AgentFile(path) => match open_agent_file(&path) {
            Ok(file) => ObsWriter::File(Arc::new(Mutex::new(file))),
            Err(_) => ObsWriter::Stderr,
        },
    }
}

/// Create the parent directory and open (truncating) the agent log file — it holds the latest
/// invocation's self-obs stream (the `-latest` name).
fn open_agent_file(path: &Path) -> std::io::Result<File> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    OpenOptions::new().create(true).write(true).truncate(true).open(path)
}

/// The self-observation sink writer — stderr, or a shared agent-log file handle.
enum ObsWriter {
    Stderr,
    File(Arc<Mutex<File>>),
}

impl<'a> MakeWriter<'a> for ObsWriter {
    type Writer = ObsWriterGuard;
    fn make_writer(&'a self) -> Self::Writer {
        match self {
            ObsWriter::Stderr => ObsWriterGuard::Stderr(std::io::stderr()),
            ObsWriter::File(file) => ObsWriterGuard::File(Arc::clone(file)),
        }
    }
}

/// The per-line write target produced by [`ObsWriter`].
enum ObsWriterGuard {
    Stderr(std::io::Stderr),
    File(Arc<Mutex<File>>),
}

impl Write for ObsWriterGuard {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            ObsWriterGuard::Stderr(w) => w.write(buf),
            ObsWriterGuard::File(file) => file.lock().expect("agent log mutex not poisoned").write(buf),
        }
    }
    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            ObsWriterGuard::Stderr(w) => w.flush(),
            ObsWriterGuard::File(file) => file.lock().expect("agent log mutex not poisoned").flush(),
        }
    }
}

fn build_subscriber<W>(
    identity: ServiceIdentity,
    make_writer: W,
    filter: EnvFilter,
) -> impl Subscriber + Send + Sync + 'static
where
    W: for<'w> MakeWriter<'w> + Send + Sync + 'static,
{
    Registry::default()
        .with(filter)
        .with(JsonObsLayer { identity, make_writer })
}

fn install_panic_hook() {
    std::panic::set_hook(Box::new(log_panic));
}

fn log_panic(info: &std::panic::PanicHookInfo<'_>) {
    let location = info
        .location()
        .map(|l| format!("{}:{}", l.file(), l.line()))
        .unwrap_or_else(|| "unknown".to_string());
    tracing::error!(panic = %panic_payload(info), location = %location, "panic");
}

fn panic_payload(info: &std::panic::PanicHookInfo<'_>) -> String {
    let payload = info.payload();
    if let Some(s) = payload.downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "<non-string panic payload>".to_string()
    }
}

/// Mint a filesystem-safe `run_id` — `YYYY-MM-DDTHH-MM-SS-mmm` from the system clock
/// (`std::time`, never tokio's virtual clock). Hyphen-delimited so it is legal as a filename
/// stem on the Windows dev host (colons are illegal there).
pub fn mint_run_id() -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    let (y, mo, d, h, mi, s) = civil_from_unix(now.as_secs());
    format!(
        "{y:04}-{mo:02}-{d:02}T{h:02}-{mi:02}-{s:02}-{:03}",
        now.subsec_millis()
    )
}

/// The current UTC instant as a colon-delimited RFC-3339 stamp with a trailing `Z`
/// (`2026-06-16T21:10:06Z`, seconds precision) from `std::time::SystemTime` — never tokio's virtual
/// clock. The `journal_emitted_at` source for the run-report envelope (arch §Timestamp formats;
/// obs-plan §3).
pub fn now_rfc3339() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let (y, mo, d, h, mi, s) = civil_from_unix(secs);
    format!("{y:04}-{mo:02}-{d:02}T{h:02}:{mi:02}:{s:02}Z")
}

fn unix_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// Howard Hinnant's `civil_from_days` (UTC): `(year, month, day, hour, min, sec)` from a Unix
/// timestamp. Dependency-free so the `run_id` stem needs no date crate.
fn civil_from_unix(secs: u64) -> (i64, u32, u32, u32, u32, u32) {
    let days = (secs / 86_400) as i64;
    let tod = (secs % 86_400) as u32;
    let (hour, min, sec) = (tod / 3600, (tod % 3600) / 60, tod % 60);

    let z = days + 719_468;
    let era = (if z >= 0 { z } else { z - 146_096 }) / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let month = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
    let year = if month <= 2 { year + 1 } else { year };
    (year, month, day, hour, min, sec)
}

struct JsonObsLayer<W> {
    identity: ServiceIdentity,
    make_writer: W,
}

impl<S, W> Layer<S> for JsonObsLayer<W>
where
    S: Subscriber,
    W: for<'w> MakeWriter<'w> + 'static,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let meta = event.metadata();
        let mut map = Map::new();
        map.insert("timestamp_ms".to_string(), Value::from(unix_millis()));
        map.insert("level".to_string(), Value::from(meta.level().as_str()));
        map.insert(
            "target".to_string(),
            Value::from(redact_value(meta.target()).as_ref()),
        );
        map.insert(
            "service.name".to_string(),
            Value::from(self.identity.service_name.clone()),
        );
        map.insert(
            "service.version".to_string(),
            Value::from(self.identity.service_version.clone()),
        );
        map.insert(
            "deployment.environment".to_string(),
            Value::from(self.identity.deployment_environment.clone()),
        );
        map.insert(
            "run_id".to_string(),
            Value::from(self.identity.run_id.clone()),
        );
        event.record(&mut JsonVisitor(&mut map));

        let mut line = serde_json::to_vec(&Value::Object(map)).unwrap_or_else(|_| b"{}".to_vec());
        line.push(b'\n');
        let mut writer = self.make_writer.make_writer();
        let _ = writer.write_all(&line);
    }
}

struct JsonVisitor<'a>(&'a mut Map<String, Value>);

impl Visit for JsonVisitor<'_> {
    fn record_bool(&mut self, field: &Field, value: bool) {
        if is_allowlisted(field.name()) {
            self.0.insert(field.name().to_string(), Value::from(value));
        }
    }
    fn record_i64(&mut self, field: &Field, value: i64) {
        if is_allowlisted(field.name()) {
            self.0.insert(field.name().to_string(), Value::from(value));
        }
    }
    fn record_u64(&mut self, field: &Field, value: u64) {
        if is_allowlisted(field.name()) {
            self.0.insert(field.name().to_string(), Value::from(value));
        }
    }
    fn record_f64(&mut self, field: &Field, value: f64) {
        if is_allowlisted(field.name()) {
            self.0.insert(field.name().to_string(), Value::from(value));
        }
    }
    fn record_str(&mut self, field: &Field, value: &str) {
        if is_allowlisted(field.name()) {
            self.0.insert(
                field.name().to_string(),
                Value::from(redact_value(value).as_ref()),
            );
        }
    }
    fn record_error(&mut self, field: &Field, value: &(dyn std::error::Error + 'static)) {
        if is_allowlisted(field.name()) {
            self.0.insert(
                field.name().to_string(),
                Value::from(redact_value(&value.to_string()).as_ref()),
            );
        }
    }
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        if is_allowlisted(field.name()) {
            self.0.insert(
                field.name().to_string(),
                Value::from(redact_value(&format!("{value:?}")).as_ref()),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[derive(Clone, Default)]
    struct SharedBuf(Arc<Mutex<Vec<u8>>>);

    impl SharedBuf {
        fn lines(&self) -> Vec<Value> {
            let bytes = self.0.lock().unwrap().clone();
            String::from_utf8(bytes)
                .unwrap()
                .lines()
                .map(|l| serde_json::from_str::<Value>(l).expect("each line is valid JSON"))
                .collect()
        }
    }

    impl Write for SharedBuf {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl<'a> MakeWriter<'a> for SharedBuf {
        type Writer = SharedBuf;
        fn make_writer(&'a self) -> Self::Writer {
            self.clone()
        }
    }

    fn fixed_identity(run_id: &str) -> ServiceIdentity {
        ServiceIdentity {
            service_name: "conductor".to_string(),
            service_version: "0.0.0-test".to_string(),
            deployment_environment: "test".to_string(),
            run_id: run_id.to_string(),
        }
    }

    fn capture(identity: ServiceIdentity, f: impl FnOnce()) -> SharedBuf {
        let buf = SharedBuf::default();
        let subscriber = build_subscriber(identity, buf.clone(), EnvFilter::new("info"));
        tracing::subscriber::with_default(subscriber, f);
        buf
    }

    #[test]
    fn every_line_is_flat_json_with_identity_and_run_id() {
        let buf = capture(fixed_identity("RUN-TEST-1"), || {
            tracing::info!(phase = "a", "first");
            tracing::info!(count = 3u64, "second");
        });
        let lines = buf.lines();
        assert_eq!(lines.len(), 2);
        for line in &lines {
            let obj = line.as_object().unwrap();
            assert_eq!(obj["service.name"], Value::from("conductor"));
            assert_eq!(obj["service.version"], Value::from("0.0.0-test"));
            assert_eq!(obj["deployment.environment"], Value::from("test"));
            assert_eq!(obj["run_id"], Value::from("RUN-TEST-1"));
            assert_eq!(obj["level"], Value::from("INFO"));
            assert!(obj.contains_key("timestamp_ms"));
        }
        assert_eq!(lines[0].as_object().unwrap()["message"], Value::from("first"));
        assert_eq!(lines[1].as_object().unwrap()["count"], Value::from(3u64));
    }

    #[test]
    fn service_name_honors_env_override() {
        let id = ServiceIdentity::resolve_with("conductor", Some("R".to_string()), |k| {
            if k == "CONDUCTOR_SERVICE_NAME" {
                Some("conductor-override".to_string())
            } else {
                None
            }
        });
        assert_eq!(id.service_name, "conductor-override");
        assert_eq!(id.deployment_environment, "local");
        assert_eq!(id.run_id, "R");
    }

    #[test]
    fn mint_run_id_is_filesystem_safe_shape() {
        let id = mint_run_id();
        assert!(!id.contains(':'), "run_id must be colon-free: {id}");
        assert_eq!(id.matches('-').count(), 5, "Y-M-DTH-M-S-mmm: {id}");
        assert!(id.contains('T'), "{id}");
        assert_eq!(id.len(), 23, "{id}");
    }

    #[test]
    fn now_rfc3339_is_colon_delimited_z_shape() {
        let ts = now_rfc3339();
        // YYYY-MM-DDTHH:MM:SSZ — 20 chars, RFC-3339 Z, time colon-delimited (distinct from the
        // all-hyphen run_id stem).
        assert_eq!(ts.len(), 20, "{ts}");
        assert!(ts.ends_with('Z'), "{ts}");
        assert_eq!(ts.as_bytes()[10], b'T', "{ts}");
        let (date, time) = ts[..ts.len() - 1].split_once('T').unwrap();
        assert_eq!(date.matches('-').count(), 2, "{date}");
        assert_eq!(time.matches(':').count(), 2, "{time}");
    }

    #[test]
    fn civil_from_unix_known_values() {
        assert_eq!(civil_from_unix(0), (1970, 1, 1, 0, 0, 0));
        assert_eq!(civil_from_unix(1_000_000_000), (2001, 9, 9, 1, 46, 40));
    }

    #[test]
    fn panic_hook_emits_one_error_line() {
        let buf = SharedBuf::default();
        let subscriber = build_subscriber(fixed_identity("RUN-PANIC"), buf.clone(), EnvFilter::new("info"));
        let prev = std::panic::take_hook();
        std::panic::set_hook(Box::new(log_panic));
        tracing::subscriber::with_default(subscriber, || {
            let _ = std::panic::catch_unwind(|| panic!("boom-xyz"));
        });
        std::panic::set_hook(prev);

        let lines = buf.lines();
        assert_eq!(lines.len(), 1);
        let obj = lines[0].as_object().unwrap();
        assert_eq!(obj["level"], Value::from("ERROR"));
        assert_eq!(obj["run_id"], Value::from("RUN-PANIC"));
        assert!(obj["panic"].as_str().unwrap().contains("boom-xyz"));
        assert!(obj.contains_key("location"));
    }

    #[test]
    fn non_allowlisted_field_is_dropped() {
        let buf = capture(fixed_identity("RUN-DROP"), || {
            tracing::info!(secret = "leak", phase = "ok", "m");
        });
        let obj = buf.lines()[0].as_object().unwrap().clone();
        assert!(!obj.contains_key("secret"), "non-allowlisted field must be dropped");
        assert_eq!(obj["phase"], Value::from("ok"));
        assert_eq!(obj["message"], Value::from("m"));
    }

    #[test]
    fn host_path_in_allowlisted_field_is_redacted() {
        let buf = capture(fixed_identity("RUN-REDACT"), || {
            tracing::info!(phase = "open C:\\Users\\turbo\\corpus.db", "m");
        });
        let obj = buf.lines()[0].as_object().unwrap().clone();
        assert_eq!(obj["phase"], Value::from("open <redacted>"));
    }

    #[test]
    fn panic_hook_redacts_host_path_in_payload() {
        let buf = SharedBuf::default();
        let subscriber =
            build_subscriber(fixed_identity("RUN-PANIC-PATH"), buf.clone(), EnvFilter::new("info"));
        let prev = std::panic::take_hook();
        std::panic::set_hook(Box::new(log_panic));
        tracing::subscriber::with_default(subscriber, || {
            let _ = std::panic::catch_unwind(|| panic!("open C:\\Users\\turbo\\corpus.db failed"));
        });
        std::panic::set_hook(prev);

        let obj = buf.lines()[0].as_object().unwrap().clone();
        let panic = obj["panic"].as_str().unwrap();
        assert!(!panic.contains("C:\\Users"), "host path leaked in panic: {panic}");
        assert!(panic.contains("<redacted>"), "{panic}");
    }

    #[test]
    fn agent_file_sink_writes_redacted_json_to_the_file() {
        let dir = std::env::temp_dir().join(format!("conductor-obs-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let path = dir.join("logs").join("agent-latest.jsonl");
        let writer = resolve_writer(ObsSink::AgentFile(path.clone()));
        assert!(matches!(writer, ObsWriter::File(_)), "a creatable path yields a file sink");
        let subscriber = build_subscriber(fixed_identity("RUN-FILE"), writer, EnvFilter::new("info"));
        tracing::subscriber::with_default(subscriber, || {
            tracing::info!(phase = "open C:\\Users\\turbo\\corpus.db", "to file");
        });
        let body = std::fs::read_to_string(&path).expect("agent log file written");
        let line: Value = serde_json::from_str(body.lines().next().expect("a line")).unwrap();
        assert_eq!(line["run_id"], Value::from("RUN-FILE"));
        assert_eq!(line["phase"], Value::from("open <redacted>"), "file sink inherits processor redaction");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn agent_file_sink_falls_back_to_stderr_when_unopenable() {
        // A path whose parent is a regular file cannot be created → the infallible stderr fallback.
        let base = std::env::temp_dir().join(format!("conductor-obs-fallback-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(&base).unwrap();
        let blocking_file = base.join("not-a-dir");
        std::fs::write(&blocking_file, b"x").unwrap();
        let unopenable = blocking_file.join("logs").join("agent-latest.jsonl");
        assert!(matches!(resolve_writer(ObsSink::AgentFile(unopenable)), ObsWriter::Stderr));
        let _ = std::fs::remove_dir_all(&base);
    }
}
