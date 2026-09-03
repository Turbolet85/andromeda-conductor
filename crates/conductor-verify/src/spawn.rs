//! Hardened sidecar spawn for the Pulse MCP read-back sidecar.
//!
//! The sidecar program is a fixed constant (never operator-chosen) and the Pulse data-dir is
//! propagated strictly through the process `.env(...)` builder after rejecting injection
//! metacharacters — never interpolated into argv or a shell (the rmcp STDIO injection class,
//! security-plan §Anti-Patterns §Code Patterns).

use std::ffi::OsStr;
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

/// Whether the fixed sidecar program resolves on the inherited `PATH` — WITHOUT spawning it.
///
/// A scheduling-time precondition probe: an unresolvable sidecar makes `connect` fail and every
/// read-back arm report the unreachable path in ~0s, which at row level is indistinguishable from a
/// genuine SUT-side gate failure (security-plan §Anti-Patterns §Input).
///
/// It must not spawn. [`build_command`] above sets no creation flags, so starting the sidecar to
/// test its presence would raise a console pane titled with its absolute path — a host-path
/// disclosure channel outside the sanitize/allowlist edges. A directory walk answers the same
/// question with no process.
///
/// Returns a boolean-grade fact only: the resolved path is never returned, logged or rendered.
pub fn sidecar_resolves_on_path() -> bool {
    resolves_on(
        std::env::var_os("PATH").as_deref(),
        std::env::var_os("PATHEXT").as_deref(),
        PULSE_MCP_PROGRAM,
    )
}

/// The pure half of [`sidecar_resolves_on_path`] — every input arrives as a value, so both arms are
/// reachable in a test with no env mutation (which edition 2024 makes `unsafe`).
fn resolves_on(path: Option<&OsStr>, path_ext: Option<&OsStr>, program: &str) -> bool {
    let Some(path) = path else {
        return false;
    };
    let extensions = executable_extensions(path_ext);
    std::env::split_paths(path)
        .filter(|d| !d.as_os_str().is_empty())
        .any(|dir| {
            extensions.iter().any(|ext| {
                let mut name = String::with_capacity(program.len() + ext.len());
                name.push_str(program);
                name.push_str(ext);
                dir.join(name).is_file()
            })
        })
}

/// The suffixes an executable may carry. The bare name always participates (it is the only form on
/// Unix, and a `PATHEXT`-less Windows host still resolves an explicit `.exe` through the default).
fn executable_extensions(path_ext: Option<&OsStr>) -> Vec<String> {
    let mut extensions = vec![String::new()];
    if cfg!(windows) {
        match path_ext
            .and_then(|e| e.to_str())
            .filter(|e| !e.trim().is_empty())
        {
            Some(raw) => extensions.extend(
                raw.split(';')
                    .map(str::trim)
                    .filter(|e| !e.is_empty())
                    .map(str::to_ascii_lowercase),
            ),
            None => extensions.push(".exe".to_string()),
        }
    }
    extensions
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

    /// The suffix a resolvable fake must carry on this platform.
    fn host_suffix() -> &'static str {
        if cfg!(windows) {
            ".exe"
        } else {
            ""
        }
    }

    fn joined(dirs: &[&std::path::Path]) -> std::ffi::OsString {
        std::env::join_paths(dirs.iter().map(|d| d.as_os_str())).expect("joinable")
    }

    #[test]
    fn an_absent_path_resolves_nothing() {
        assert!(!resolves_on(None, None, PULSE_MCP_PROGRAM));
    }

    #[test]
    fn an_empty_path_resolves_nothing() {
        assert!(!resolves_on(Some(OsStr::new("")), None, PULSE_MCP_PROGRAM));
    }

    #[test]
    fn a_directory_without_the_program_resolves_nothing() {
        let dir = assert_fs::TempDir::new().unwrap();
        assert!(!resolves_on(
            Some(&joined(&[dir.path()])),
            None,
            PULSE_MCP_PROGRAM
        ));
    }

    #[test]
    fn the_program_is_found_when_present_on_the_path() {
        use assert_fs::prelude::*;
        let dir = assert_fs::TempDir::new().unwrap();
        dir.child(format!("{PULSE_MCP_PROGRAM}{}", host_suffix()))
            .touch()
            .unwrap();

        assert!(resolves_on(
            Some(&joined(&[dir.path()])),
            None,
            PULSE_MCP_PROGRAM
        ));
    }

    #[test]
    fn a_later_path_entry_still_resolves() {
        use assert_fs::prelude::*;
        let empty = assert_fs::TempDir::new().unwrap();
        let holding = assert_fs::TempDir::new().unwrap();
        holding
            .child(format!("{PULSE_MCP_PROGRAM}{}", host_suffix()))
            .touch()
            .unwrap();

        let path = joined(&[empty.path(), holding.path()]);
        assert!(resolves_on(Some(&path), None, PULSE_MCP_PROGRAM));
    }

    #[test]
    fn a_directory_named_like_the_program_is_not_an_executable() {
        use assert_fs::prelude::*;
        let dir = assert_fs::TempDir::new().unwrap();
        dir.child(PULSE_MCP_PROGRAM).create_dir_all().unwrap();

        // `is_file()` is what separates them — a directory on PATH resolves nothing.
        assert!(!resolves_on(
            Some(&joined(&[dir.path()])),
            None,
            PULSE_MCP_PROGRAM
        ));
    }

    #[test]
    fn a_different_program_name_does_not_satisfy_the_probe() {
        use assert_fs::prelude::*;
        let dir = assert_fs::TempDir::new().unwrap();
        dir.child(format!("some-other-tool{}", host_suffix()))
            .touch()
            .unwrap();

        assert!(!resolves_on(
            Some(&joined(&[dir.path()])),
            None,
            PULSE_MCP_PROGRAM
        ));
    }

    #[test]
    fn the_bare_name_always_participates() {
        assert!(executable_extensions(None).contains(&String::new()));
    }

    #[cfg(windows)]
    #[test]
    fn pathext_entries_are_honoured_and_normalised() {
        let exts = executable_extensions(Some(OsStr::new(".COM;.EXE; ;.CMD")));
        assert!(
            exts.contains(&".exe".to_string()),
            "upper-case PATHEXT is normalised: {exts:?}"
        );
        assert!(exts.contains(&".cmd".to_string()));
        assert!(
            !exts.contains(&" ".to_string()),
            "blank entries are dropped: {exts:?}"
        );
    }

    #[cfg(windows)]
    #[test]
    fn a_blank_pathext_falls_back_to_exe() {
        assert!(executable_extensions(Some(OsStr::new("   "))).contains(&".exe".to_string()));
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
        assert!(std_cmd
            .get_program()
            .to_string_lossy()
            .contains(PULSE_MCP_PROGRAM));
        let envs: Vec<_> = std_cmd.get_envs().collect();
        assert_eq!(envs.len(), 1, "only the data-dir env is set");
        assert_eq!(envs[0].0.to_string_lossy(), DATA_DIR_ENV);
        assert_eq!(envs[0].1.unwrap().to_string_lossy(), "/srv/pulse");
    }
}
