//! The `CONDUCTOR_*` path-handle edge + scenario resolution.
//!
//! Reads the env handles, resolves each under the current dir with `conductor_core::resolve_under`
//! (rejecting traversal / absolute escapes before any IO — security-plan §Input Validation), and
//! resolves a `run` target (a scenario name or a Pulse P-ID) to a validated [`Scenario`].

use std::path::{Path, PathBuf};

use anyhow::Context as _;
use conductor_core::{CapabilityManifest, LoadEnvelope, Scenario, resolve_under, scenario_files};

/// The resolved artifact + config locations for a CLI invocation.
pub struct Paths {
    pub scenarios_dir: PathBuf,
    pub runs_dir: PathBuf,
    pub manifest_path: PathBuf,
    pub capability_manifest_path: PathBuf,
    pub load_envelope_path: PathBuf,
}

impl Paths {
    /// Resolve the handles against the current directory, applying the `CONDUCTOR_*` overrides. The
    /// capability manifest is a fixed in-repo path (no override), guarded the same way.
    pub fn resolve() -> anyhow::Result<Self> {
        let base = std::env::current_dir().context("resolve current directory")?;
        Ok(Self {
            scenarios_dir: resolve_handle(&base, "CONDUCTOR_SCENARIOS_DIR", "scenarios")?,
            runs_dir: resolve_handle(&base, "CONDUCTOR_RUNS_DIR", "runs")?,
            manifest_path: resolve_handle(
                &base,
                "CONDUCTOR_CONTRACT_MANIFEST",
                "contracts/mcp-contract.toml",
            )?,
            capability_manifest_path: resolve_under(&base, &CapabilityManifest::default_path())?,
            load_envelope_path: resolve_under(&base, &LoadEnvelope::default_path())?,
        })
    }

    /// The SUT capability set every scenario this invocation loads is checked against.
    fn capabilities(&self) -> anyhow::Result<CapabilityManifest> {
        Ok(CapabilityManifest::load(&self.capability_manifest_path)?)
    }

    /// The pinned load envelope this invocation's run is judged against. A fixed in-repo path (no
    /// `CONDUCTOR_*` override), guarded the same way as the capability manifest.
    pub fn load_envelope(&self) -> anyhow::Result<LoadEnvelope> {
        Ok(LoadEnvelope::load(&self.load_envelope_path)?)
    }

    /// Resolve a `run` target — a scenario name (`<name>.toml`) or a Pulse P-ID — to a seeded scenario.
    pub fn load_scenario(&self, target: &str, seed: Option<u64>) -> anyhow::Result<Scenario> {
        let capabilities = self.capabilities()?;
        if let Ok(by_name) =
            resolve_under(&self.scenarios_dir, Path::new(&format!("{target}.toml")))
            && by_name.is_file()
        {
            return Ok(apply_seed(load_one(&by_name, &capabilities)?, seed));
        }
        let mut matches = find_by_pid(&self.scenarios_dir, target, &capabilities)?;
        match matches.len() {
            0 => anyhow::bail!("no scenario matches \"{target}\" (by name or P-ID)"),
            1 => {
                let (_, by_pid) = matches.remove(0);
                Ok(apply_seed(by_pid, seed))
            }
            n => {
                let stems: Vec<String> = matches.into_iter().map(|(stem, _)| stem).collect();
                anyhow::bail!(
                    "{target} is named by {n} scenarios ({}); run one by name",
                    stems.join(", ")
                )
            }
        }
    }

    /// Load every scenario in the catalog (sorted, optionally name-filtered), each seeded.
    pub fn load_all_scenarios(
        &self,
        filter: Option<&str>,
        seed: Option<u64>,
    ) -> anyhow::Result<Vec<Scenario>> {
        let files = scenario_files(&self.scenarios_dir)?;
        let capabilities = self.capabilities()?;

        let mut scenarios = Vec::new();
        for path in files {
            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or_default();
            if filter.is_some_and(|f| !stem.contains(f)) {
                continue;
            }
            scenarios.push(apply_seed(load_one(&path, &capabilities)?, seed));
        }
        Ok(scenarios)
    }
}

/// The agent-mode self-obs log path — `logs/agent-latest.jsonl` as a sibling of the runs dir, moving
/// with `CONDUCTOR_RUNS_DIR` (obs-plan §3). Resolved through the same `resolve_under` traversal guard
/// the artifact handles use; not a `Paths` method because obs init runs before `Paths::resolve`.
pub fn agent_log_path() -> anyhow::Result<PathBuf> {
    let base = std::env::current_dir().context("resolve current directory")?;
    let runs_dir = resolve_handle(&base, "CONDUCTOR_RUNS_DIR", "runs")?;
    let logs_dir = runs_dir
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or(base)
        .join("logs");
    Ok(logs_dir.join("agent-latest.jsonl"))
}

fn resolve_handle(base: &Path, var: &str, default: &str) -> anyhow::Result<PathBuf> {
    let candidate = std::env::var(var).unwrap_or_else(|_| default.to_string());
    Ok(resolve_under(base, Path::new(&candidate))?)
}

fn load_one(path: &Path, capabilities: &CapabilityManifest) -> anyhow::Result<Scenario> {
    let text = std::fs::read_to_string(path).context("read scenario file")?;
    Ok(Scenario::from_toml_str_with(&text, capabilities)?)
}

/// Every loadable scenario naming `pid`, as `(file stem, scenario)` in sorted catalog order;
/// unloadable files are skipped.
fn find_by_pid(
    dir: &Path,
    pid: &str,
    capabilities: &CapabilityManifest,
) -> anyhow::Result<Vec<(String, Scenario)>> {
    let mut matches = Vec::new();
    for path in scenario_files(dir)? {
        if let Ok(scenario) = load_one(&path, capabilities)
            && scenario.p_ids.iter().any(|p| p.0 == pid)
        {
            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or_default()
                .to_owned();
            matches.push((stem, scenario));
        }
    }
    Ok(matches)
}

/// Seed precedence: an explicit `--seed` wins, else `CONDUCTOR_SEED`, else the scenario's own seed.
fn apply_seed(mut scenario: Scenario, seed: Option<u64>) -> Scenario {
    if let Some(seed) = seed.or_else(env_seed) {
        scenario.seed = seed;
    }
    scenario
}

fn env_seed() -> Option<u64> {
    std::env::var("CONDUCTOR_SEED")
        .ok()
        .and_then(|v| v.parse().ok())
}
