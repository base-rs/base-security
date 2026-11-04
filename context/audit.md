---
name: base-security-audit
description: Review project files for security issues and record the outcome in security/audit-*.md. Use when asked to audit or security-review files or a project, to find files that are due for review, or to record a security risk the maintainers accept.
metadata:
  type: skill
---

# Security Audit

An audit reviews files as they exist at a commit on the project's default branch, and records which files were reviewed, at which revision, and with what outcome. The record avoids repeated reviews of unchanged code, shows which files are due for review, and identifies the changes to review since the last audit. Audits complement pull request review: they examine the project as it stands, including code that no current change touches.

Each audit is recorded in one tracked file, `security/audit-{date}-{sha}.md`. Vulnerabilities that warrant an advisory follow `base-security-advisory`.

## Work privately until outcomes are known

Until each finding's outcome is decided, keep reproductions, tests, branches, and notes local. Do not commit, push, open pull requests or issues, run CI, or comment about a finding: any of these can disclose a vulnerability before a fix is available. Report findings to the user in the conversation.

## Establish the revision and scope

Fetch the default branch and audit a commit on it, normally its tip. Pull request branches are rewritten by squash and rebase merges, so their commits are not stable references. Check out the revision, or an up-to-date checkout of the default branch, so the files and existing records match it.

Inventory the tracked files in scope. Security-relevant files include source, build scripts, manifests, and CI workflows, not only the code under `src/`. Exclude generated and vendored files unless the user asks for them.

A file is current when some existing audit record has an entry for it and the file is unchanged between that record's revision and the revision under audit. Every other file is due for review: it has no entry, or it changed after its latest review. This lists due files in bash, given the default branch as `origin/HEAD`; if that is missing, set it with `git remote set-head origin --auto`. The subshell keeps the caller's shell and working directory unchanged:

```sh
(
  cd "$(git rev-parse --show-toplevel)" || exit
  revision=$(git rev-parse --verify 'origin/HEAD^{commit}') || exit
  git -c core.quotePath=false ls-tree -r -z --name-only "$revision" | while IFS= read -r -d '' path; do
    case $path in security/*) continue ;; esac
    current=
    for audit in $(grep -lxF -e "- \`$path\` reviewed with no findings." -e "- \`$path\` reviewed with findings:" security/audit-*.md 2>/dev/null); do
      current=${current:-stale}; sha=${audit##*-}; sha=${sha%.md}
      if git diff --quiet "$sha" "$revision" -- "$path" 2>/dev/null; then current=yes; break; fi
    done
    case $current in "") echo "unreviewed $path" ;; stale) echo "changed $path" ;; esac
  done
)
```

Renamed files appear as unreviewed, and files changed by an earlier audit's own fixes or `// SECURITY:` comments appear as changed. Both are intended: the code at that path has not been reviewed in its current form.

Skip current files, unless the user asks for a full review or something they depend on has changed: a trust assumption, a dependency, or how the file is used. For a changed file, focus on `git diff {sha}..{revision} -- {path}`, and review enough of the rest of the file and its callers to judge the change. A small change can invalidate a property established elsewhere.

Security issues often span files: an input is accepted in one place, parsed in another, and limited in a third. Follow such data flows across files. Record an entry only for a file you reviewed in full; a file read only to understand one call stays due for review.

## Establish trust boundaries

Identify which inputs an attacker can control: network data, files, environment variables, command-line arguments, data from dependencies or other processes. Read applicable `design/*.md`, which may document trust boundaries and requirements, and the `// SECURITY:` comments in the files under review. Without a documented assumption, treat external input as untrusted.

## Review

Assess each file against the concerns that apply to it:

- **Untrusted input:** parsing, validation, encoding, and normalization of paths, URLs, and headers. Injection into shell commands, queries, markup, or format strings.
- **Resource limits:** sizes, counts, depths, and durations controlled by input. Unbounded allocation, recursion, or iteration, and costs that grow faster than the input.
- **Arithmetic:** overflow, truncating casts, and length or index calculations.
- **Panics and errors:** `unwrap`, `expect`, indexing, and slicing reachable from input. Error paths that leak resources or leave state inconsistent.
- **Memory safety:** each `unsafe` block upholds the invariants stated in its `// SAFETY:` comment. FFI boundaries validate what they receive.
- **Races:** check-then-use on shared state and on the filesystem.
- **Secrets and exposure:** credentials and personal data in logs, errors, or debug output. Constant-time comparison and a suitable randomness source where they matter.
- **Filesystem and processes:** path traversal, symbolic links, permissions of created files, temporary files, and command construction.
- **Defaults:** safe behavior by default, with dangerous behavior requiring explicit opt-in.
- **Supply chain and automation:** dependencies and features in manifests, build scripts, workflow and token permissions, and untrusted pull request data in workflows.

Confirm a finding with a local failing test or reproduction where practical. When a finding rests on reasoning alone, say so when reporting it.

## Classify findings

A security finding is a weakness that lets an attacker affect confidentiality, integrity, or availability, given the project's trust boundaries. A defect without attacker leverage is an ordinary issue, not an audit finding. Record a hardening opportunity as a `low` finding only when it needs a decision; otherwise leave it out.

Assign each finding a severity by its impact and how readily an attacker can trigger it, not by the effort to fix it:

- `critical`: code execution, memory corruption, or broad data exposure, in a default configuration without special access.
- `high`: denial of service, bypass of a security control, or limited data exposure, in a default configuration.
- `medium`: significant impact that requires a non-default configuration, particular deployment conditions, or substantial attacker resources.
- `low`: limited impact, defense in depth, or conditions that are unlikely in practice.

Establish whether each finding affects a released version: reproduce it on the latest release of each supported version, or confirm that the vulnerable code is present in those release tags.

Before reporting a finding, check whether it is already known: accepted in a `// SECURITY:` comment or design requirement, recorded in an earlier audit with an open issue, or the subject of an unpublished advisory, including draft advisories you can access. Do not report a known finding again unless its stated assumption no longer holds or its impact has changed; in that case, report it as a new finding that names the earlier one.

## Decide each outcome

Each finding has one outcome:

- **Advisory:** the weakness affects a released version and meets the advisory threshold in `base-security-advisory`. Follow that skill, and record the advisory identifier.
- **Fixed:** the weakness is fixed in a pull request. This suits code that has not been released, and released weaknesses below the advisory threshold.
- **Accepted:** the maintainers decide not to fix it. Add a `// SECURITY:` comment where the risk lives, stating the assumption that makes it acceptable, so a later reviewer can tell whether it still holds. When the same assumption applies across the project, such as "configuration files are trusted", capture it once in `design/*.md` with `base-design-author`, and refer to it from the comments.
- **Tracked:** the weakness remains open in a public issue. This suits only findings that the maintainers judge are not exploitable in released versions and are safe to discuss publicly.

Use the language's comment syntax, keeping the `SECURITY:` marker:

```rust
// SECURITY: Include depth is not limited because configuration files are trusted input; untrusted configuration would need a depth limit.
fn load_includes(path: &Path) -> Result<Configuration, Error> {
```

Accepting a risk, choosing the advisory process, and opening a public issue are maintainer decisions. Propose an outcome with its evidence; do not choose one on the maintainers' behalf.

## Record the audit

Create `security/audit-{date}-{sha}.md`, where `{date}` is the day the review finished in `YYYY-MM-DD` form and `{sha}` is the first 12 characters of the revision. Give the full revision on the `Revision:` line:

```markdown
# Audit

Revision: `fc81a68c1e2b5d0c9a7e4f3b2a1d0c9b8a7f6e5d` on `main`.

- `src/limits.rs` reviewed with findings:
  - low: error message includes the configured limit file path; tracked in #63.
- `src/parser.rs` reviewed with findings:
  - high: unbounded header count allows memory exhaustion; advisory GHSA-xxxx-xxxx-xxxx.
  - low: header count limit not applied to trailers; fixed in #61.
  - low: unbounded include depth; accepted, configuration is trusted (`// SECURITY:` at `load_includes`).
- `src/request.rs` reviewed with no findings.
```

Follow this format exactly so the records remain searchable and machine-readable:

- List one entry per file, by path relative to the repository root, in byte order as `git ls-tree` lists them.
- End each entry with either `reviewed with no findings.` or `reviewed with findings:` followed by a nested list.
- Write each finding as `{severity}: {description}; {outcome}.`, where the outcome is `advisory {identifier}`, `fixed in #{number}`, `accepted, {reason}`, or `tracked in #{number}`. Do not use `; ` within the description.
- Record a finding that spans files under the file where it is best fixed or explained, and name the other files in its description.
- Keep the identifier an advisory had when the record merged; the advisory record lists its later aliases.

Do not edit an audit record after it merges. A later outcome, such as an issue being fixed, is recorded when the file is next audited: a later audit reviews the file again and records its findings then. A finding from outside an audit, such as a vulnerability report, is likewise recorded when its file is next audited in full.

## Protect undisclosed findings

An audit record that names a file and a severity tells an attacker where to look. While any finding in an audit has an unpublished advisory, keep the whole record where only the maintainers can read it, and land it after the advisory is published. Advisory comments are also visible to the advisory's collaborators, including external reporters, so do not keep a record there that mentions other unpublished advisories.

A fix for a related finding, one that shares a file, code path, or mechanism with an unpublished advisory, also points at the vulnerability. Hold it, or include it in the advisory's fix. Fixes, `// SECURITY:` comments, and issues for unrelated findings can land in the meantime.

## Report

Summarize the audit for the user: the revision, the files reviewed, the files skipped as current, each finding with its proposed outcome, findings that rest on reasoning alone, and anything in scope that was not reviewed. Once outcomes are decided, land the audit record following the project's pull request workflow.
