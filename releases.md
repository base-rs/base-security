# Releases

## Unreleased

- Derive the `base-security-audit` and `base-security-advisory` skill names from the package and filename, instead of repeating them in front matter. Installed names are unchanged.

## v0.1.1

- Extend `base-security-advisory` with handling duplicate reports, batching several advisories into one release, the behavior of temporary private fork pull requests, consistent severity and credit types, embargoes, and updating release notes when a CVE is assigned.

## v0.1.0

- Publish as `base-security` in the `base-rs` organization.
- Add the `base-security-audit` skill, which reviews files at a revision on the default branch and records outcomes in `security/audit-{date}-{sha}.md`, with `// SECURITY:` comments for accepted risks.
- Add the `base-security-advisory` skill, which takes a vulnerability from report to private fix, release, and published advisory, and records it in `security/advisory-{date}-{slug}.md`.
