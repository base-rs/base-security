# `base-security`

Security audit and advisory records for projects, captured in `security/` and maintained by agent skills for reviewing code and handling vulnerabilities.

## Motivation

"Review this file for security issues" is easy to ask and hard to track. Without a record, the same unchanged code is reviewed repeatedly, other code is never reviewed, and accepted risks are reported again by each new reviewer. Vulnerabilities need a careful path from private report to public advisory, without disclosing them before users can update.

`base-security` defines two kinds of tracked record. An audit records which files were reviewed at which revision, with an outcome for each finding, so the next audit can review only what changed. An advisory records a published vulnerability, with its identifiers and affected versions in front matter.

## Usage

Add `base-security` to the development tooling dependencies resolved by Agent Context. In a Rust project with a private Bake package:

```sh
cargo add --manifest-path bake/Cargo.toml base-security
cargo bake agent:context:install --package base-security
```

The install command is provided by `bake-agent-context`, which `base-project` includes. `base-security` provides guidance and skills, with no runtime API or Bake tasks of its own. The record formats and skills are language-independent.

The package provides two self-contained skills:

- [Security Audit](context/audit.md), installed as `base-security-audit`: find files that are due for review, review them against trust boundaries and common weaknesses, decide an outcome for each finding, and record the audit in `security/audit-{date}-{sha}.md`. Accepted risks are explained in `// SECURITY:` comments where they live, so later reviews do not report them again.
- [Security Advisory](context/advisory.md), installed as `base-security-advisory`: triage a report, prepare a fix privately, release it, publish the GitHub Security Advisory, and record it in `security/advisory-{date}-{slug}.md`.

An audit record looks like this:

```markdown
# Audit

Revision: `fc81a68c1e2b5d0c9a7e4f3b2a1d0c9b8a7f6e5d` on `main`.

- `src/parser.rs` reviewed with findings:
  - low: header count limit not applied to trailers; fixed in #61.
  - low: unbounded include depth; accepted, configuration is trusted (`// SECURITY:` at `load_includes`).
- `src/request.rs` reviewed with no findings.
```

Keep records in the project's tracked `security/` directory. Installing this package installs the shared procedure; it does not create records. Enable private vulnerability reporting on the repository so reporters can submit vulnerabilities without opening public issues.

Add this instruction to the project's agent guidance:

> Use `base-security-audit` when reviewing code for security issues, and record each review in `security/audit-*.md`. Use `base-security-advisory` for vulnerability reports and for findings that may warrant an advisory. Before reporting a security finding, read the relevant `// SECURITY:` comments and `design/*.md`.

## Releasing

Prepare a release with `cargo bake cargo:version:patch` (or `minor`, `major`, or `bump --version X.Y.Z`), commit it with a `Release: vVERSION` trailer, then run `cargo bake cargo:release` and open a pull request. After review and merge, GitHub Actions publishes the release when the configured `crates-io` environment approves it, then creates or updates the matching GitHub Release from `releases.md`. Follow the shared [Releasing skill](https://github.com/base-rs/base-project/blob/main/context/releasing.md) for the standard process.

## Releases

<!-- bake-readme:releases:start -->

See [releases.md](releases.md) for the full release history.

### v0.1.0

- Publish as `base-security` in the `base-rs` organization.
- Add the `base-security-audit` skill, which reviews files at a revision on the default branch and records outcomes in `security/audit-{date}-{sha}.md`, with `// SECURITY:` comments for accepted risks.
- Add the `base-security-advisory` skill, which takes a vulnerability from report to private fix, release, and published advisory, and records it in `security/advisory-{date}-{slug}.md`.

<!-- bake-readme:releases:end -->

## See Also

- [`base-project`](https://github.com/base-rs/base-project).
- [`base-design`](https://github.com/base-rs/base-design).
- [`bake-agent-context`](https://github.com/base-rs/bake-agent-context).

## Contributing

Please open an issue or pull request on [GitHub](https://github.com/base-rs/base-security). Report vulnerabilities privately through [GitHub Security Advisories](https://github.com/base-rs/base-security/security/advisories/new).

### Agent Context

Run `cargo bake agent:context:install` to install shared context and skills. Use `cargo bake agent:context:list` and `cargo bake agent:context:show` to discover relevant guides, follow `agents.md` if present, and apply skills under `.agents/skills/`. The installer preserves repository-owned `agents.md`; it does not create or regenerate that file.
