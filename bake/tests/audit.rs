// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("bake package is inside the workspace")
        .to_path_buf()
}

fn read(path: &str) -> String {
    fs::read_to_string(root().join(path)).expect("read documentation")
}

/// Returns the bodies of fenced code blocks with the given info string.
fn blocks(document: &str, language: &str) -> Vec<String> {
    let fence = format!("```{language}");
    let mut blocks = Vec::new();
    let mut current: Option<Vec<&str>> = None;
    for line in document.lines() {
        match current.as_mut() {
            None if line == fence => current = Some(Vec::new()),
            Some(lines) if line == "```" => {
                blocks.push(lines.join("\n"));
                current = None;
            }
            Some(lines) => lines.push(line),
            None => {}
        }
    }
    blocks
}

fn git(directory: &Path, arguments: &[&str]) -> String {
    let output = Command::new("git")
        .current_dir(directory)
        .args(arguments)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_AUTHOR_NAME", "Audit Test")
        .env("GIT_AUTHOR_EMAIL", "audit@example.com")
        .env("GIT_COMMITTER_NAME", "Audit Test")
        .env("GIT_COMMITTER_EMAIL", "audit@example.com")
        .output()
        .expect("run git");
    assert!(
        output.status.success(),
        "git {arguments:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("git output is UTF-8")
        .trim()
        .to_owned()
}

fn commit(directory: &Path, message: &str) -> String {
    git(directory, &["add", "--all"]);
    git(directory, &["commit", "--quiet", "--message", message]);
    git(directory, &["rev-parse", "HEAD"])
}

fn write(directory: &Path, path: &str, content: &str) {
    let path = directory.join(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, content).unwrap();
}

fn record(directory: &Path, revision: &str, entries: &str) {
    let path = format!("security/audit-2026-11-01-{}.md", &revision[..12]);
    write(
        directory,
        &path,
        &format!("# Audit\n\nRevision: `{revision}` on `main`.\n\n{entries}"),
    );
}

#[test]
fn due_file_command_reports_unreviewed_and_changed_files_only() {
    let script = blocks(&read("context/audit.md"), "sh")
        .into_iter()
        .find(|block| block.contains("ls-tree"))
        .expect("audit skill contains the due-file command");

    let directory = tempfile::tempdir().unwrap();
    let repository = directory.path();
    git(repository, &["init", "--quiet", "--initial-branch", "main"]);
    for path in [
        "src/changed.rs",
        "src/later.rs",
        "src/mentioned.rs",
        "src/with space.rs",
        "src/café.rs",
        " leading.rs",
    ] {
        write(repository, path, "fn main() {}\n");
    }
    let first = commit(repository, "Add sources.");

    record(
        repository,
        &first,
        "- ` leading.rs` reviewed with no findings.\n\
         - `src/café.rs` reviewed with no findings.\n\
         - `src/changed.rs` reviewed with no findings.\n\
         - `src/later.rs` reviewed with findings:\n  \
         - low: shares a limit with `src/mentioned.rs`; fixed in #1.\n  \
         - low: quoted - `src/mentioned.rs` reviewed with no findings.; fixed in #2.\n\
         - `src/with space.rs` reviewed with no findings.\n",
    );
    write(repository, "src/later.rs", "fn main() { later() }\n");
    let second = commit(repository, "Record the first audit and change a file.");

    // A second audit on the same day re-reviews the changed file; filename order must not matter.
    record(
        repository,
        &second,
        "- `src/later.rs` reviewed with no findings.\n",
    );
    let third = commit(repository, "Record the second audit.");

    write(repository, "src/changed.rs", "fn main() { changed() }\n");
    write(repository, "src/added.rs", "fn main() {}\n");
    let fourth = commit(repository, "Change and add files.");

    git(
        repository,
        &["update-ref", "refs/remotes/origin/main", &fourth],
    );
    git(
        repository,
        &[
            "symbolic-ref",
            "refs/remotes/origin/HEAD",
            "refs/remotes/origin/main",
        ],
    );
    // Run from a subdirectory, with a checkout that has every record but lacks the latest source changes.
    git(repository, &["checkout", "--quiet", "--detach", &third]);
    let path = repository.join("src");

    let output = Command::new("bash")
        .current_dir(&path)
        .arg("-c")
        .arg(&script)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .output()
        .expect("run due-file command");
    assert!(
        output.status.success(),
        "due-file command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let reported: BTreeSet<String> = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(str::to_owned)
        .collect();
    let expected: BTreeSet<String> = [
        "changed src/changed.rs",
        "unreviewed src/added.rs",
        "unreviewed src/mentioned.rs",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();
    assert_eq!(reported, expected);
}

#[test]
fn audit_examples_follow_the_record_format() {
    let mut examples = Vec::new();
    for document in ["context/audit.md", "readme.md"] {
        examples.extend(
            blocks(&read(document), "markdown")
                .into_iter()
                .filter(|block| block.starts_with("# Audit")),
        );
    }
    assert!(!examples.is_empty(), "no audit examples found");

    for example in examples {
        let mut paths = Vec::new();
        for line in example.lines().skip_while(|line| !line.starts_with("- ")) {
            if let Some(finding) = line.strip_prefix("  - ") {
                let (severity, rest) = finding.split_once(": ").expect("finding has a severity");
                assert!(
                    ["low", "medium", "high", "critical"].contains(&severity),
                    "{line}"
                );
                let (_, outcome) = rest.split_once("; ").expect("finding has an outcome");
                assert!(!outcome.contains("; "), "description contains `; `: {line}");
                assert!(
                    ["advisory ", "fixed in #", "accepted, ", "tracked in #"]
                        .iter()
                        .any(|form| outcome.starts_with(form)),
                    "{line}"
                );
                assert!(outcome.ends_with('.'), "{line}");
            } else {
                let entry = line.strip_prefix("- `").expect("entry names a file");
                let (path, status) = entry.split_once("` ").expect("entry has a status");
                assert!(
                    status == "reviewed with no findings." || status == "reviewed with findings:",
                    "{line}"
                );
                paths.push(path.to_owned());
            }
        }
        let mut sorted = paths.clone();
        sorted.sort();
        assert_eq!(paths, sorted, "entries are not in path order");
    }
}
