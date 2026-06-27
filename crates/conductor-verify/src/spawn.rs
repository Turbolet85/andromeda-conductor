//! Hardened sidecar spawn for the Pulse MCP read-back sidecar.
//!
//! The sidecar program is a fixed constant (never operator-chosen) and the Pulse data-dir is
//! propagated strictly through the process `.env(...)` builder after rejecting injection
//! metacharacters — never interpolated into argv or a shell (the rmcp STDIO injection class,
//! security-plan §Anti-Patterns §Code Patterns).

use std::path::{Path, PathBuf};

use tokio::process::Command;

use crate::error::VerifyError;

/// The fixed sidecar program — resolved from `PATH`, never an operator-supplied command.
pub(crate) const PULSE_MCP_PROGRAM: &str = "andromeda-pulse-mcp";

/// The env var carrying the live Pulse's data directory to the spawned sidecar.
pub(crate) const DATA_DIR_ENV: &str = "ANDROMEDA_PULSE_DATA_DIR";

/// Shell/injection metacharacters refused in the data-dir before it reaches `.env(...)`. Path
/// separators, drive colons, spaces, dots and hyphens are intentionally NOT here — they are legal in
/// Windows/Unix paths and `.env(...)` never reaches a shell.
const FORBIDDEN: &[char] = &[
    '$', '`', ';', '|', '&', '<', '>', '(', ')', '{', '}', '*', '?', '!', '"', '\'', '\n', '\r',
    '\0',
];

/// Resolve the effective Pulse data-dir — the caller's value, or the platform default when `None` —
/// and reject it if it carries an injection metacharacter (defense-in-depth for `.env(...)`).
pub(crate) fn resolve_data_dir(explicit: Option<PathBuf>) -> Result<PathBuf, VerifyError> {
    let dir = match explicit {
        Some(d) => d,
        None => platform_default()?,
    };
    validate_no_injection(&dir)?;
    Ok(dir)
}

/// The platform default Pulse data-dir, used when the live Pulse leaves `ANDROMEDA_PULSE_DATA_DIR`
/// unset (architecture §Occupied Resources): `%APPDATA%\andromeda-pulse` on Windows,
/// `$XDG_CONFIG_HOME/andromeda-pulse` or `~/.andromeda-pulse` elsewhere.
fn platform_default() -> Result<PathBuf, VerifyError> {
    #[cfg(windows)]
    {
        std::env::var_os("APPDATA")
            .map(|base| PathBuf::from(base).join("andromeda-pulse"))
            .ok_or_else(|| VerifyError::DataDirRejected {
                reason: "%APPDATA% is unset".to_string(),
            })
    }
    #[cfg(not(windows))]
    {
        if let Some(base) = std::env::var_os("XDG_CONFIG_HOME") {
            Ok(PathBuf::from(base).join("andromeda-pulse"))
        } else {
            std::env::var_os("HOME")
                .map(|home| PathBuf::from(home).join(".andromeda-pulse"))
                .ok_or_else(|| VerifyError::DataDirRejected {
                    reason: "$HOME is unset".to_string(),
                })
        }
    }
}

fn validate_no_injection(dir: &Path) -> Result<(), VerifyError> {
    let s = dir.to_str().ok_or_else(|| VerifyError::DataDirRejected {
        reason: "path is not valid UTF-8".to_string(),
    })?;
    if let Some(c) = s.chars().find(|c| FORBIDDEN.contains(c)) {
        return Err(VerifyError::DataDirRejected {
            reason: format!("metacharacter {c:?}"),
        });
    }
    Ok(())
}

/// Build the hardened sidecar command: the fixed program + the validated data-dir passed only via
/// `.env(...)`. The caller hands the result to `TokioChildProcess`.
pub(crate) fn build_command(data_dir: &Path) -> Command {
    let mut command = Command::new(PULSE_MCP_PROGRAM);
    command.env(DATA_DIR_ENV, data_dir);
    command
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("/home/dev/.andromeda-pulse")]
    #[case("C:\\Users\\dev\\AppData\\Roaming\\andromeda-pulse")]
    #[case("/var/lib/pulse data/with spaces")]
    fn accepts_well_formed_data_dirs(#[case] path: &str) {
        assert!(resolve_data_dir(Some(PathBuf::from(path))).is_ok());
    }

    #[rstest]
    #[case("/tmp/$(rm -rf ~)")]
    #[case("/tmp/a;reboot")]
    #[case("/tmp/a|cat")]
    #[case("/tmp/a&b")]
    #[case("/tmp/`whoami`")]
    #[case("/tmp/a\nb")]
    fn rejects_injection_metacharacters(#[case] path: &str) {
        let err = resolve_data_dir(Some(PathBuf::from(path))).unwrap_err();
        assert!(matches!(err, VerifyError::DataDirRejected { .. }));
    }

    #[test]
    fn explicit_data_dir_is_returned_verbatim() {
        let dir = PathBuf::from("/srv/pulse");
        assert_eq!(resolve_data_dir(Some(dir.clone())).unwrap(), dir);
    }

    #[test]
    fn default_data_dir_resolves_to_the_platform_location() {
        let dir = resolve_data_dir(None).unwrap();
        let tail = dir.file_name().unwrap().to_string_lossy();
        assert!(
            tail == "andromeda-pulse" || tail == ".andromeda-pulse",
            "unexpected default data-dir: {dir:?}"
        );
    }

    #[test]
    fn build_command_targets_the_fixed_program_and_sets_only_the_data_dir_env() {
        let cmd = build_command(Path::new("/srv/pulse"));
        let std_cmd = cmd.as_std();
        assert!(
            std_cmd
                .get_program()
                .to_string_lossy()
                .contains(PULSE_MCP_PROGRAM)
        );
        let envs: Vec<_> = std_cmd.get_envs().collect();
        assert_eq!(envs.len(), 1, "only the data-dir env is set");
        assert_eq!(envs[0].0.to_string_lossy(), DATA_DIR_ENV);
        assert_eq!(envs[0].1.unwrap().to_string_lossy(), "/srv/pulse");
    }
}
