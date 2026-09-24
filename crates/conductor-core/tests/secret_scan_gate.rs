//! The secret-scan gate — no secret-shaped string and no secret-shaped file name survives in the workspace
//! (security-plan §Secret Management; `verification-matrix.json#v3-11`).
//!
//! The subject is the workspace as git sees it: tracked files plus untracked files that are not ignored, so a
//! new file is scanned before it is committed. An ignored file (a local `.env`) is guarded by the ignore rules,
//! not by this gate. Every bad input the control and negative arms use is assembled at test time, so no file in
//! the repository — this one included — carries a literal match.
//!
//! A hit is reported as `path:line` with its rule and match length, never the matched text (security-plan
//! §Security Anti-Patterns → Logging).

use std::path::{Path, PathBuf};
use std::process::Command;

use regex::Regex;

const THIS_GATE: &str = "crates/conductor-core/tests/secret_scan_gate.rs";

const CONTENT_RULES: &[(&str, &str)] = &[
    ("private-key-block", r"-----BEGIN [A-Z ]*PRIVATE KEY-----"),
    ("aws-access-key-id", r"\b(?:AKIA|ASIA)[0-9A-Z]{16}\b"),
    ("github-token", r"\bgh[pousr]_[A-Za-z0-9]{36,}"),
    ("github-fine-grained-pat", r"\bgithub_pat_[A-Za-z0-9_]{22,}"),
    ("slack-token", r"\bxox[abposr]-[A-Za-z0-9-]{10,}"),
    (
        "slack-webhook",
        r"hooks\.slack\.com/services/T[A-Za-z0-9]+/B[A-Za-z0-9]+/[A-Za-z0-9]+",
    ),
    ("stripe-live-key", r"\b[rs]k_live_[A-Za-z0-9]{16,}"),
    ("google-api-key", r"\bAIza[0-9A-Za-z_-]{35}"),
    (
        "jwt",
        r"\beyJ[A-Za-z0-9_-]{10,}\.eyJ[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]{10,}",
    ),
    ("anthropic-key", r"\bsk-ant-[A-Za-z0-9_-]{20,}"),
    ("openai-key", r"\bsk-(?:proj-)?[A-Za-z0-9]{32,}"),
    ("npm-token", r"\bnpm_[A-Za-z0-9]{36}\b"),
    (
        "url-credentials",
        r#"\b[a-z][a-z0-9+.-]*://[^/\s:@"'<>]{1,64}:[^/\s:@"'<>]{1,64}@"#,
    ),
    (
        "generic-assignment",
        r#"(?i)\b(?:password|passwd|pwd|secret|api[_-]?key|access[_-]?token|auth[_-]?token|client[_-]?secret)\b["']?\s*[:=]\s*["'][^"'\s]{8,}["']"#,
    ),
];

// The same classes `.gitignore`'s secrets block names.
const FILE_NAME_RULE: &str = r"(?i)(?:^|/)(?:\.env[^/]*|id_(?:rsa|dsa|ecdsa|ed25519)[^/]*|[^/]*\.(?:pem|p12|pfx|key|cer|crt|jks|keystore))$";
const FILE_NAME_CLASS: &str = "secret-file-name";

/// A deliberately accepted hit. Every entry states why; an entry that stops matching fails the gate as rot.
struct Allowed {
    path: &'static str,
    rule: &'static str,
    #[allow(dead_code)]
    reason: &'static str,
}

const ALLOWLIST: &[Allowed] = &[];

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hit {
    path: String,
    line: usize,
    rule: &'static str,
    len: usize,
}

impl Hit {
    fn render(&self) -> String {
        if self.rule == FILE_NAME_CLASS {
            format!("{}: secret-shaped file name ({})", self.path, self.rule)
        } else {
            format!(
                "{}:{}: secret-shaped string ({}, {} chars)",
                self.path, self.line, self.rule, self.len
            )
        }
    }
}

struct Scanner {
    content: Vec<(&'static str, Regex)>,
    file_name: Regex,
}

impl Scanner {
    fn new() -> Self {
        Self {
            content: CONTENT_RULES
                .iter()
                .map(|(name, pattern)| (*name, Regex::new(pattern).expect("a rule compiles")))
                .collect(),
            file_name: Regex::new(FILE_NAME_RULE).expect("the file-name rule compiles"),
        }
    }

    fn scan_text(&self, path: &str, text: &str) -> Vec<Hit> {
        let mut hits = Vec::new();
        for (index, line) in text.split('\n').enumerate() {
            let line = line.trim_end_matches('\r');
            for (rule, regex) in &self.content {
                for found in regex.find_iter(line) {
                    hits.push(Hit {
                        path: path.to_owned(),
                        line: index + 1,
                        rule,
                        len: found.as_str().chars().count(),
                    });
                }
            }
        }
        hits
    }

    fn scan_name(&self, path: &str) -> Option<Hit> {
        self.file_name.is_match(path).then(|| Hit {
            path: path.to_owned(),
            line: 0,
            rule: FILE_NAME_CLASS,
            len: 0,
        })
    }

    fn scan(&self, files: &[(String, String)]) -> Vec<Hit> {
        files
            .iter()
            .flat_map(|(path, text)| {
                let mut hits: Vec<Hit> = self.scan_name(path).into_iter().collect();
                hits.extend(self.scan_text(path, text));
                hits
            })
            .collect()
    }
}

/// Grades the hits against the allowlist as an exact set in both directions.
fn grade(hits: &[Hit], allowlist: &[Allowed]) -> Result<(), String> {
    let allowed = |hit: &Hit| {
        allowlist
            .iter()
            .any(|a| a.path == hit.path && a.rule == hit.rule)
    };
    let mut problems: Vec<String> = hits
        .iter()
        .filter(|hit| !allowed(hit))
        .map(Hit::render)
        .collect();
    problems.extend(
        allowlist
            .iter()
            .filter(|a| !hits.iter().any(|h| h.path == a.path && h.rule == a.rule))
            .map(|a| {
                format!(
                    "allowlist rot: {} ({}) matches no hit — remove the entry",
                    a.path, a.rule
                )
            }),
    );
    if problems.is_empty() {
        Ok(())
    } else {
        Err(problems.join("\n"))
    }
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn workspace_files() -> Vec<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root())
        .args([
            "ls-files",
            "-z",
            "--cached",
            "--others",
            "--exclude-standard",
        ])
        .output()
        .expect("git runs — the gate enumerates its subject with `git ls-files`");
    assert!(
        output.status.success(),
        "`git ls-files` exited {} — the gate has no subject",
        output.status
    );
    String::from_utf8(output.stdout)
        .expect("git prints UTF-8 paths")
        .split('\0')
        .filter(|p| !p.is_empty())
        .map(str::to_owned)
        .collect()
}

/// Reads every listed file that exists and is text. A tracked file deleted in the worktree has nothing to scan;
/// a file with a NUL byte in its first 8 KiB is binary and is checked by name only.
fn read_workspace(paths: &[String]) -> (Vec<(String, String)>, usize) {
    let root = repo_root();
    let mut files = Vec::new();
    let mut text_files = 0;
    for path in paths {
        let bytes = match std::fs::read(root.join(path)) {
            Ok(bytes) => bytes,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(e) => panic!("{path} is listed but unreadable: {}", e.kind()),
        };
        let binary = bytes.iter().take(8192).any(|b| *b == 0);
        let text = if binary {
            String::new()
        } else {
            text_files += 1;
            String::from_utf8_lossy(&bytes).into_owned()
        };
        files.push((path.clone(), text));
    }
    (files, text_files)
}

/// One sample per content rule, assembled so that no source line holds a contiguous match.
fn samples() -> Vec<(&'static str, String)> {
    let forty = "A".repeat(40);
    vec![
        (
            "private-key-block",
            ["-----BEGIN ", "RSA PRIVATE", " KEY-----"].concat(),
        ),
        ("aws-access-key-id", ["AK", "IA", &"Q".repeat(16)].concat()),
        ("github-token", ["gh", "p_", &forty].concat()),
        (
            "github-fine-grained-pat",
            ["github", "_pat_", &forty].concat(),
        ),
        ("slack-token", ["xo", "xb-", "1234567890ab"].concat()),
        (
            "slack-webhook",
            ["hooks.slack.com/", "services/", "T0AB12/B0CD34/xyz789"].concat(),
        ),
        ("stripe-live-key", ["sk", "_live_", &forty].concat()),
        ("google-api-key", ["AI", "za", &"B".repeat(35)].concat()),
        (
            "jwt",
            [
                "ey",
                "J",
                &"x".repeat(12),
                ".ey",
                "J",
                &"y".repeat(12),
                ".",
                &"z".repeat(12),
            ]
            .concat(),
        ),
        ("anthropic-key", ["sk-", "ant-", &forty].concat()),
        ("openai-key", ["sk-", &forty].concat()),
        ("npm-token", ["np", "m_", &"C".repeat(36)].concat()),
        (
            "url-credentials",
            [
                "https:",
                "//",
                "user",
                ":",
                "hunter22x",
                "@",
                "host.invalid",
            ]
            .concat(),
        ),
        (
            "generic-assignment",
            ["api", "_key", " = ", "\"", &"a1".repeat(10), "\""].concat(),
        ),
    ]
}

#[test]
fn the_workspace_holds_no_secret_shaped_string() {
    let paths = workspace_files();
    assert!(
        !paths.is_empty(),
        "`git ls-files` listed nothing — the gate would pass vacuously"
    );
    assert!(
        paths.iter().any(|p| p == THIS_GATE),
        "the listing does not include {THIS_GATE} — the subject is not the workspace this gate lives in"
    );
    let (files, text_files) = read_workspace(&paths);
    assert!(
        text_files > 0,
        "no text file was scanned — the gate would pass vacuously"
    );

    let hits = Scanner::new().scan(&files);
    if let Err(problems) = grade(&hits, ALLOWLIST) {
        panic!("the secret-scan gate found:\n{problems}");
    }
}

#[test]
fn every_rule_fires_on_its_runtime_built_sample() {
    let scanner = Scanner::new();
    let samples = samples();

    let mut rules: Vec<&str> = CONTENT_RULES.iter().map(|(name, _)| *name).collect();
    let mut sampled: Vec<&str> = samples.iter().map(|(name, _)| *name).collect();
    rules.sort_unstable();
    sampled.sort_unstable();
    assert_eq!(
        rules, sampled,
        "every content rule has exactly one sample, and no sample names a missing rule"
    );

    for (rule, sample) in &samples {
        let hits = scanner.scan_text("sample.txt", sample);
        assert!(
            hits.iter().any(|h| h.rule == *rule),
            "rule {rule} does not fire on its own sample"
        );
    }
}

#[test]
fn a_planted_sample_fails_the_gate_naming_its_location_without_echoing_it() {
    let scanner = Scanner::new();
    for (rule, sample) in samples() {
        let files = vec![(
            "src/planted.rs".to_owned(),
            format!("fn untouched() {{}}\nlet planted = {sample};\n"),
        )];
        let rendered = grade(&scanner.scan(&files), ALLOWLIST)
            .expect_err("a planted secret-shaped string must fail the gate");
        assert!(
            rendered.contains("src/planted.rs:2:") && rendered.contains(rule),
            "the failure must name src/planted.rs:2 and rule {rule}, got: {rendered}"
        );
        assert!(
            !rendered.contains(&sample),
            "the failure must never echo the matched text (rule {rule})"
        );
    }
}

#[test]
fn a_secret_shaped_file_name_fails_the_gate() {
    let scanner = Scanner::new();
    for path in [
        ".env",
        ".env.production",
        "crates/x/.envrc",
        "deploy/server.pem",
        "certs/client.p12",
        "certs/client.pfx",
        "tls/private.key",
        "tls/chain.cer",
        "tls/chain.crt",
        "android/release.jks",
        "android/release.keystore",
        "id_rsa",
        "home/id_ed25519.pub",
    ] {
        let files = vec![(path.to_owned(), String::new())];
        let rendered = grade(&scanner.scan(&files), ALLOWLIST)
            .expect_err("a secret-shaped file name must fail the gate");
        assert!(
            rendered.contains(path) && rendered.contains(FILE_NAME_CLASS),
            "the failure must name {path}, got: {rendered}"
        );
    }
    for path in [
        "src/environment.rs",
        "docs/keys.md",
        "src/pem.rs",
        "scripts/keystore-notes.txt",
    ] {
        assert!(
            scanner.scan_name(path).is_none(),
            "{path} is not a secret file name and must not fire"
        );
    }
}

#[test]
fn the_allowlist_is_graded_as_an_exact_set_in_both_directions() {
    let scanner = Scanner::new();
    let (rule, sample) = samples().remove(0);
    let files = vec![("fixtures/doc.md".to_owned(), sample)];
    let hits = scanner.scan(&files);

    let covering = [Allowed {
        path: "fixtures/doc.md",
        rule,
        reason: "an allowlisted hit is suppressed",
    }];
    grade(&hits, &covering).expect("an entry suppresses exactly its own hit");

    let stale = [Allowed {
        path: "fixtures/gone.md",
        rule,
        reason: "describes nothing",
    }];
    let rendered = grade(&[], &stale).expect_err("an entry matching no hit must fail as rot");
    assert!(
        rendered.contains("allowlist rot") && rendered.contains("fixtures/gone.md"),
        "the rot failure must name the stale entry, got: {rendered}"
    );
}
