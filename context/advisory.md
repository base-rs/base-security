---
name: base-security-advisory
description: Take a vulnerability from a private report or audit finding through a private fix, release, and published advisory, then record it in security/advisory-*.md. Use when triaging a security report, deciding whether a finding needs an advisory, preparing a vulnerability fix, or publishing and recording an advisory.
metadata:
  type: skill
---

# Security Advisory

Use this skill when a vulnerability report arrives, or when an audit finds a weakness that affects a released version. The GitHub Security Advisory is the authoritative record and the place for private coordination. After publication, the project records a copy in `security/advisory-{date}-{slug}.md`.

Reports arrive through private vulnerability reporting. If the repository does not have it enabled, propose enabling it to the maintainers.

## Keep it private until release

Until the advisory is published, do not reveal the vulnerability, its location, or its severity in public issues, pull requests, commits, branches, CI runs, release notes, or audit records. Coordinate in the advisory, and work in its temporary private fork or in a private temporary directory created with `mktemp -d`. Run code supplied by a reporter, including reproductions, only in an isolated environment such as a container or virtual machine.

Accepting or closing a report, merging the fix, approving the release, and publishing the advisory are maintainer decisions. Prepare each step and propose it with its evidence; do not push, comment, or change anything on GitHub unless a maintainer asks for that step.

## Triage the report

1. Read the report. Search open advisories for reports of the same component and mechanism. Propose one canonical advisory, normally the earliest unless a later report is materially better, and credit every reporter on it.
2. Reproduce the issue on the default branch and on the latest release of each supported version. Verify the affected range rather than relying on the reporter's.
3. Assess it against the advisory threshold: an attacker can cause a concrete confidentiality, integrity, or availability impact in a configuration the project supports and users plausibly run, given the project's documented trust boundaries, in a released version. A weakness below the threshold, or one only in unreleased code, is fixed or tracked through `base-security-audit` instead, and a report of one is closed with a short explanation and an invitation to open an ordinary issue or pull request. Thank the reporter either way.
4. Present the reproduction, affected versions, proposed severity, and recommendation to the maintainers, who accept or close the report.

## Prepare the fix

Once the report is accepted, the fix is prepared in the advisory's temporary private fork, which a maintainer creates if the reporter has not. Keep the fix minimal and local to the vulnerable path. Add regression tests that fail without the fix and pass with it, using the reproduction from the report rather than only the general mechanism.

Merging from a temporary private fork does not run status checks or enforce branch protection rules. Run the project's formatting, lint, test, and coverage checks locally, and record the results. Review a fix written by a reporter line by line, including any changes to build scripts, manifests, and workflows, as untrusted code.

GitHub merges all open pull requests in the fork together, and only one may target the default branch. Prepare the fix as one pull request.

Look for the same weakness elsewhere: other parsers, limits, or paths that share the mechanism. Fix variants with the same root cause in the same pull request; handle unrelated weaknesses as separate advisories.

Releases are published from the default branch. Patching an older release line needs a process the maintainers define for that project.

## Describe the advisory

Write the title as one sentence naming the mechanism and its impact, with identifiers in backticks, for example "Unbounded header count in `Parser` allows memory exhaustion." Structure the description as:

```markdown
## Summary

What an attacker sends, the affected type or function, and the consequence.

## Details

How the vulnerable code behaves, why existing limits do not apply, and how affected versions differ.

## Impact

Who is affected, what an attacker can achieve, and what the attack does not require.

## Mitigation

- Update to {patched version}, which {fix in one line}.
- Any configuration or deployment workaround.
```

Before replacing a reporter's description, preserve the original in an advisory comment. Propose a severity with a CVSS vector and a one-line justification for each metric. GitHub's interface calls the `medium` level "Moderate". Credit the reporter, the fix author, and any reviewer of a fix the reporter wrote.

Set the affected products on the draft: the crates.io ecosystem, the package, the affected range, and the planned patched version for each release line. List unsupported lines that remain vulnerable without a patched version. GitHub requires this before a CVE can be requested, so propose requesting the CVE as soon as it is set. GitHub aims to review requests within days, but assignment can take longer, and it does not block the release.

## Release and publish

Merging the fix discloses it, so prepare everything before merging: the fix, its release notes entry, the version bump, and the advisory description and affected products.

1. Once a maintainer merges the fix from the advisory page, open the release pull request immediately, following `base-project-releasing`. The commit GitHub creates when merging from the advisory cannot be relied on to carry a `Release:` trailer, so the release needs its own pull request. Describe the fix in `releases.md` in one line at the level of the advisory title, without exploit details, and link the advisory, using the CVE as link text when it is assigned and the GHSA identifier otherwise.
2. Ask the maintainers to approve the release publication as soon as checks pass, keeping the interval between disclosure and release short.
3. After the patched crate is available on crates.io, confirm the affected products match what was released.
4. Propose publication to the maintainers once the description, severity, and credits are final. Publishing deletes the temporary private fork.
5. Propose submitting the advisory to the [RustSec Advisory Database](https://github.com/rustsec/advisory-db) so `cargo audit` reports it. Yanking affected versions breaks dependent builds, so propose it only when the risk justifies that.

## Record the advisory

After publication, add `security/advisory-{date}-{slug}.md`, where `{date}` is the publication date in `YYYY-MM-DD` form and `{slug}` is a short lowercase hyphenated form of the title, for example `advisory-2026-11-10-unbounded-header-count.md`. Copy the content from the published advisory, which remains authoritative; when the advisory changes, for example when a CVE is assigned or a further release line is patched, update the file to match.

```markdown
---
id: GHSA-xxxx-xxxx-xxxx
aliases: [CVE-2026-12345, RUSTSEC-2026-0042]
package: base-http1
severity: high
cvss: CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:N/I:N/A:H
patched: [">=0.2.3"]
unaffected: ["<0.1.0"]
---

# Unbounded header count in `Parser` allows memory exhaustion.

## Summary
## Details
## Impact
## Mitigation
## References
```

The front matter describes the published advisory:

- `id`: the GHSA identifier of the advisory.
- `aliases`: other identifiers, such as CVE and RustSec identifiers, as they are assigned.
- `package`: the affected crate. When an advisory affects several crates, add one file per crate with the same `id`, including the crate name in each slug.
- `severity`: `low`, `medium`, `high`, or `critical`, matching the published advisory.
- `cvss`: the published CVSS vector, when the advisory has one.
- `patched` and `unaffected`: semantic version requirements, with the same meaning as in RustSec advisories. Versions that match neither are affected.

A RustSec submission uses its own TOML format, with a `RUSTSEC-` identifier and the GHSA and CVE identifiers as aliases; the version requirements and a CVSS 3.x vector carry over unchanged.

Under References, link the published advisory, the fix, and the release. Land any audit record that refers to the advisory with or after this file, following `base-security-audit`.

## Check before reporting a step as done

- Every claim about an affected or patched version was reproduced on that version.
- The project's checks were run locally on each fix, and the results are recorded.
- Nothing about the vulnerability became public before the fix was ready to release.
- The advisory was published only after the patched crate was available, and the record matches it.
