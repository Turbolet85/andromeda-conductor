//! The workflow env-context gate — no `${{ env.X }}` expression and no bare `if: env.X` reads a key that no
//! workflow, job or step declares and no step writes to `GITHUB_ENV`.
//!
//! The `env` expression context holds only declared keys and resolves anything else to an empty string without
//! error, so a runner-process variable read through it fails silently (paid twice: `EDGEWEBDRIVER`, run
//! 34148079506; an `if: ${{ env.ImageOS == 'win22' }}` step that never ran, run 35190456907). A key written to
//! `GITHUB_ENV` is admitted; the `rust` job's `GITHUB_ENV context probe` steps assert on every run that the
//! context really carries one.
//!
//! The line model covers the YAML subset these workflows use. A `#` line outside a block scalar is a YAML
//! comment and is skipped. A key ending in a block indicator opens a block scalar whose content is SCANNED,
//! comment-looking lines included, because the runner substitutes `${{ }}` inside `run:` text before the shell
//! sees it. Admission is file-wide: a key declared on one step admits a read on another (a stated limit).

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use regex::Regex;

static EXPRESSION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\$\{\{(.*?)\}\}").expect("compiles"));
static ENV_TOKEN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?:^|[^A-Za-z0-9_.])env\.([A-Za-z_][A-Za-z0-9_]*)").expect("compiles")
});
static BLOCK_OPEN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:-\s+)?[A-Za-z0-9_.-]+:\s*[|>][+-]?\s*(?:#.*)?$").expect("compiles")
});
static ENV_OPEN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?:-\s+)?env:\s*(?:#.*)?$").expect("compiles"));
static ENV_KEY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([A-Za-z_][A-Za-z0-9_]*)\s*:").expect("compiles"));
static IF_KEY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?:-\s+)?if:\s*(.*)$").expect("compiles"));
static GITHUB_ENV_WRITE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"["']([A-Za-z_][A-Za-z0-9_]*)="#).expect("compiles"));

#[derive(Default)]
struct Analysis {
    expressions: usize,
    reads: Vec<(usize, String)>,
    declared: BTreeSet<String>,
}

impl Analysis {
    fn scan_content(&mut self, line: &str, number: usize) {
        for expression in EXPRESSION.captures_iter(line) {
            self.expressions += 1;
            self.read_tokens(&expression[1], number);
        }
        if line.contains("GITHUB_ENV") {
            for write in GITHUB_ENV_WRITE.captures_iter(line) {
                self.declared.insert(write[1].to_owned());
            }
        }
    }

    fn read_tokens(&mut self, text: &str, number: usize) {
        for token in ENV_TOKEN.captures_iter(text) {
            self.reads.push((number, token[1].to_owned()));
        }
    }

    fn undeclared(&self, file: &str) -> Vec<String> {
        self.reads
            .iter()
            .filter(|(_, key)| !self.declared.contains(key))
            .map(|(line, key)| {
                format!(
                    "{file}:{line}: env.{key} is read but no workflow, job or step declares it and no step writes it to GITHUB_ENV"
                )
            })
            .collect()
    }
}

fn indent(line: &str) -> usize {
    line.len() - line.trim_start().len()
}

fn analyse(text: &str) -> Analysis {
    let mut analysis = Analysis::default();
    let mut block: Option<usize> = None;
    let mut env_map: Option<usize> = None;

    for (index, raw) in text.split('\n').enumerate() {
        let line = raw.trim_end_matches('\r');
        let number = index + 1;
        let blank = line.trim().is_empty();

        if let Some(key_indent) = block {
            if blank || indent(line) > key_indent {
                analysis.scan_content(line, number);
                continue;
            }
            block = None;
        }
        if blank {
            continue;
        }
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') {
            continue;
        }
        let column = indent(line);

        if let Some(map_indent) = env_map {
            if column > map_indent {
                if let Some(key) = ENV_KEY.captures(trimmed) {
                    analysis.declared.insert(key[1].to_owned());
                }
                analysis.scan_content(line, number);
                continue;
            }
            env_map = None;
        }
        if ENV_OPEN.is_match(trimmed) {
            env_map = Some(column);
            continue;
        }
        if let Some(condition) = IF_KEY.captures(trimmed)
            && !condition[1].contains("${{")
        {
            analysis.read_tokens(&condition[1], number);
        }
        analysis.scan_content(line, number);
        if BLOCK_OPEN.is_match(trimmed) {
            block = Some(column);
        }
    }
    analysis
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn the_committed_workflows_read_only_declared_env_keys() {
    let dir = repo_root().join(".github/workflows");
    let mut workflows: Vec<PathBuf> = std::fs::read_dir(&dir)
        .expect("the workflow directory exists — the gate has a subject")
        .map(|entry| entry.expect("a readable directory entry").path())
        .filter(|path| {
            path.extension()
                .is_some_and(|ext| ext == "yml" || ext == "yaml")
        })
        .collect();
    workflows.sort();
    assert!(
        !workflows.is_empty(),
        "no workflow file — the gate would pass vacuously"
    );

    let (mut expressions, mut declared, mut reads) = (0, 0, 0);
    let mut problems = Vec::new();
    for path in &workflows {
        let name = format!(
            ".github/workflows/{}",
            path.file_name().expect("a file name").to_string_lossy()
        );
        let text = std::fs::read_to_string(path).expect("a workflow file is UTF-8 text");
        let analysis = analyse(&text);
        expressions += analysis.expressions;
        declared += analysis.declared.len();
        reads += analysis.reads.len();
        problems.extend(analysis.undeclared(&name));
    }

    assert!(
        expressions > 0,
        "no `${{{{ }}}}` expression parsed — the line model is blind"
    );
    assert!(
        declared > 0,
        "no env declaration found — the line model is blind"
    );
    assert!(
        reads > 0,
        "no env-context read found — the admission path is never exercised"
    );
    assert!(
        problems.is_empty(),
        "the workflow env-context gate found:\n{}",
        problems.join("\n")
    );
}

const STEP_HEAD: &str = "jobs:\n  build:\n    runs-on: windows-latest\n    steps:\n";

fn undeclared_in(steps: &str) -> Vec<String> {
    analyse(&format!("{STEP_HEAD}{steps}")).undeclared("ci.yml")
}

#[test]
fn an_undeclared_expression_read_fails_naming_the_key_and_line() {
    let problems = undeclared_in(
        "      - name: read\n        run: |\n          echo \"${{ env.UNDECLARED }}\"\n",
    );
    assert_eq!(
        problems.len(),
        1,
        "exactly one undeclared read, got: {problems:?}"
    );
    assert!(
        problems[0].starts_with("ci.yml:7:") && problems[0].contains("env.UNDECLARED"),
        "the failure must name the line and the key, got: {}",
        problems[0]
    );
}

#[test]
fn a_bare_if_read_fails_naming_the_key() {
    let problems = undeclared_in(
        "      - name: gated\n        if: env.UNDECLARED == 'y'\n        run: echo hi\n",
    );
    assert_eq!(
        problems.len(),
        1,
        "exactly one undeclared read, got: {problems:?}"
    );
    assert!(
        problems[0].starts_with("ci.yml:6:") && problems[0].contains("env.UNDECLARED"),
        "the bare if: form must fail naming the key, got: {}",
        problems[0]
    );
}

#[test]
fn a_yaml_comment_is_not_a_read_but_a_shell_comment_inside_run_is() {
    let yaml_comment = undeclared_in(
        "      # ${{ env.UNDECLARED }} resolves empty — kept as documentation\n      - name: ok\n        run: echo hi\n",
    );
    assert!(
        yaml_comment.is_empty(),
        "a YAML comment is never evaluated, got: {yaml_comment:?}"
    );

    let shell_comment = undeclared_in(
        "      - name: commented\n        run: |\n          # ${{ env.UNDECLARED }}\n          echo hi\n",
    );
    assert_eq!(
        shell_comment.len(),
        1,
        "the runner substitutes expressions inside run: text, shell comments included, got: {shell_comment:?}"
    );
}

#[test]
fn declared_written_and_other_context_reads_pass() {
    let problems = undeclared_in(concat!(
        "      - name: declared on the step\n",
        "        env:\n",
        "          DECLARED: '1'\n",
        "        run: echo \"${{ env.DECLARED }}\"\n",
        "      - name: pwsh writer\n",
        "        shell: pwsh\n",
        "        run: |\n",
        "          Add-Content -Path $env:GITHUB_ENV -Value \"FROM_PWSH=$dir\"\n",
        "      - name: bash writer\n",
        "        run: echo \"FROM_BASH=x\" >> \"$GITHUB_ENV\"\n",
        "      - name: readers\n",
        "        if: env.FROM_PWSH != ''\n",
        "        shell: pwsh\n",
        "        run: |\n",
        "          echo \"${{ env.FROM_PWSH }} ${{ env.FROM_BASH }}\"\n",
        "          echo \"${{ github.sha }} ${{ runner.temp }}\"\n",
        "          Write-Output $env:RUNNER_ONLY\n",
    ));
    assert!(
        problems.is_empty(),
        "declared, GITHUB_ENV-written, non-env-context and shell reads must all pass, got: {problems:?}"
    );
}
