# Private Disclosure

## Requirement

Guidance shipped by this crate must keep the details, location, and severity of an undisclosed vulnerability out of public places: issues, pull requests, commits, branches, CI runs, release notes, and audit or advisory records. Publication follows the availability of a patched release, and the interval between a public fix and its release is kept short.

Decisions with external effect, including accepting a report, accepting a risk, publishing an advisory, opening a public issue, and yanking a release, remain with the maintainers. Guidance may direct an agent to propose them with evidence, not to take them.

## Rationale

Audit records and advisories make security work visible and repeatable. Recording them carelessly would disclose vulnerabilities before users can update, which is worse than keeping no record.

## Scope

The `base-security-audit` and `base-security-advisory` skills, and the `security/` record formats they define.

## Validation

Trace the guidance through concrete scenarios: an audit finds a vulnerability in a released version alongside unrelated findings; a report arrives with a proposed fix in a temporary private fork; a fix merges before its release; a CVE is assigned after publication. Confirm that no step places undisclosed details in a public location, and that each external action is presented as a maintainer decision.
