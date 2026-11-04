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

1. Read the report, and any fix it proposes, recording the commit you reviewed. List the project's advisories with `gh api --paginate 'repos/{owner}/{repo}/security-advisories'`, filtering with `?state=triage`, `draft`, or `published`, and search reports in every state for the same component and mechanism; see [Duplicate reports](#duplicate-reports). A report matching a published advisory is either a duplicate or a regression or bypass that needs a new advisory. Read, without commenting on, open pull requests and issues that touch the same code, and note how they interact with a fix.
2. Reproduce the issue on the default branch and on the latest release of each supported version. Verify the affected range rather than relying on the reporter's. Test variants beyond the reporter's input: a report often demonstrates the most severe form, and whether ordinary, well-formed input reaches the same path changes the impact.
3. Assess it against the advisory threshold: an attacker can cause a concrete confidentiality, integrity, or availability impact in a configuration the project supports and users plausibly run, given the project's documented trust boundaries, in a released version. A weakness below the threshold, or one only in unreleased code, is fixed or tracked through `base-security-audit` instead, and a report of one is closed with a short explanation and an invitation to open an ordinary issue or pull request. Thank the reporter either way.
4. Present the reproduction, affected versions, proposed severity, and recommendation to the maintainers, who accept or close the report.

### Duplicate reports

The same issue is often reported more than once, sometimes months apart. Propose one report as the canonical advisory: normally the earliest, unless a later one is materially better because its reporter is responsive, it includes a fix and tests, or it describes the scope accurately where the earlier one does not. Say which you chose and why.

Propose closing the others as duplicates, with a short comment thanking each reporter, explaining that another report covers the same issue, and saying they will be credited on the published advisory. Reporters cannot see each other's advisories, so do not link or identify them. Credit every reporter on the canonical advisory, and carry over any reproduction, affected version, or impact detail it lacks, limited to this issue, since the canonical reporter can read it. From then on, handle the set as one issue, with one fix, description, CVE, and severity.

## Prepare the fix

Once the report is accepted, the fix is prepared in the advisory's temporary private fork, which a maintainer creates if the reporter has not. Keep the fix minimal and local to the vulnerable path, and follow the style of earlier security fixes in the same files for naming, the placement of limits, and tests. Add regression tests that fail without the fix and pass with it, using the reproduction from the report rather than only the general mechanism.

Merging from a temporary private fork does not run status checks or enforce branch protection rules. Run the project's formatting, lint, test, and coverage checks locally, and record the results. Review a fix written by a reporter line by line, including any changes to build scripts, manifests, and workflows, as untrusted code.

GitHub merges all open pull requests in the fork together, and only one may target the default branch. Prepare the fix as one pull request.

A pull request in a temporary private fork may not reflect later pushes to its branch; this has been observed for pull requests opened by reporters. After pushing, confirm the pull request's head commit with `gh api repos/{owner}/{repo}-ghsa-xxxx-xxxx-xxxx/pulls/{number} --jq .head.sha`. If it did not move, propose a remedy to the maintainers: changing the pull request's base branch and back, or opening a new pull request from a new branch and closing the stale one. Propose merging only when the pull request's head is the commit the local checks ran on; a stale head would merge, and disclose, an untested fix.

Look for the same weakness elsewhere: other parsers, limits, or paths that share the mechanism. Fix variants with the same root cause in the same pull request; handle unrelated weaknesses as separate advisories.

Releases are published from the default branch. Patching an older release line needs a process the maintainers define for that project.

## Describe the advisory

Write the title as one sentence naming the mechanism and its impact, with identifiers in backticks, for example "Unbounded header count in `Parser` allows memory exhaustion." Structure the description as:

```markdown
## Summary

What an attacker sends, the affected type or function, and the consequence.

## Details

How the vulnerable code behaves, why existing limits do not apply, and how affected versions differ. Distinguish the issue from any similar published advisory.

## Impact

Who is affected, by default or only with an optional component enabled; what an attacker can achieve and whether authentication is required; and what the attack does not require, such as malformed input or large payloads.

## Mitigation

- Update to {patched version}, which {fix in one line}.
- Any configuration or deployment workaround.
- Any mitigation at an application boundary, such as a reverse proxy limit.
```

Mitigation always starts with the update. Match the tone and length of the project's published advisories. Before replacing a reporter's description and title, preserve the original in an advisory comment.

Propose a severity with a CVSS 3.1 vector and a one-line justification for each metric, and compare it with the project's published advisories of the same impact class. Reuse the vector of an earlier advisory with the same mechanism unless there is a reason to differ. As a reference, unauthenticated, low-complexity resource exhaustion that is unbounded and held for the lifetime of a request scores Availability High; a cost that sustained requests can use to deny service to legitimate users also scores Availability High, while partial or degraded availability scores Availability Low; and impacts that need an uncommon configuration or high attack complexity score lower. The advisory's severity follows its CVSS score, which can differ from the severity an audit assigned; explain the difference when it does. GitHub's interface calls the `medium` level "Moderate".

Credit people with GitHub's credit types: the reporter as `reporter`, the fix author as `remediation_developer`, a maintainer who reviewed a reporter's fix as `remediation_reviewer`, and the coordinating maintainer as `coordinator`. GitHub records one type per person, so prefer each person's highest-effort role: a reporter who wrote the fix is a `remediation_developer`. Follow the project's published advisories where their convention differs.

Set the affected products on the draft: the crates.io ecosystem, the package, the affected range, and the planned patched version for each release line. List unsupported lines that remain vulnerable without a patched version. GitHub requires this before a CVE can be requested, so propose requesting the CVE as soon as it is set, and confirm on the advisory page that the request registered, since the API shows no pending state. GitHub aims to review requests within days, but assignment can take longer, and it does not block the release.

## Release and publish

Merging the fix discloses it, so prepare everything before merging: the fix, its release notes entry, the version bump, and the advisory description and affected products. Agreeing an embargo with other parties, and sharing advisory details outside the advisory, are maintainer decisions. While an embargo applies, do not merge the fix, publish the advisory, or submit it elsewhere until it ends.

1. Once a maintainer merges the fix from the advisory page, open the release pull request immediately, following `base-project-releasing`. The commit GitHub creates when merging from the advisory cannot be relied on to carry a `Release:` trailer, so the release needs its own pull request. Describe the fix in `releases.md` in one line at the level of the advisory title, without exploit details, and link the advisory, using the CVE as link text when it is assigned and the GHSA identifier otherwise.
2. Ask the maintainers to approve the release publication as soon as checks pass, keeping the interval between disclosure and release short.
3. After the patched crate is available on crates.io, confirm the affected products match what was released.
4. Propose publication to the maintainers once the description, severity, and credits are final. Publishing deletes the temporary private fork.
5. Propose submitting the advisory to the [RustSec Advisory Database](https://github.com/rustsec/advisory-db) so `cargo audit` reports it. Yanking affected versions breaks dependent builds, so propose it only when the risk justifies that.
6. When a CVE is assigned after the advisory is published, update the link text of the `releases.md` entry to the CVE and add it to the `aliases` of the advisory record, in one follow-up pull request, keeping the same link target. The GitHub Release body is generated only at release time, so propose that a maintainer edit it to match. GitHub attaches the CVE to the published advisory itself.

### Several advisories in one release

Security fixes are often batched so each release carries all of them. When several advisories are in flight:

- Triage, fix, and describe each advisory independently, with its own private fork, fix, description, CVE, and severity. Do not combine unrelated fixes in one pull request.
- Prepare the version bump in advance, and open the release pull request after the last fix in the batch merges, with one `releases.md` entry per advisory.
- Each temporary private fork starts from the default branch as it was when the fork was created. After each fix merges, update the remaining fork branches onto the default branch, rerun the local checks, and confirm each pull request's head again before proposing the next merge.
- Set the same patched version on every advisory in the batch, and propose publishing them together once the release is available.
- If one fix is not ready, the maintainers decide whether to hold the release or ship without it; a half-finished fix should not delay the others indefinitely. Before shipping without it, check that the shipped fixes and their release notes do not point at the held vulnerability through a shared file, code path, or mechanism; if they do, hold both.
- Each merged fix is disclosed, so merge the batch's fixes close together, immediately before the release.

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
