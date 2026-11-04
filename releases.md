# Releases

## v0.1.0

- Publish as `base-security` in the `base-rs` organization.
- Add the `base-security-audit` skill, which reviews files at a revision on the default branch and records outcomes in `security/audit-{date}-{sha}.md`, with `// SECURITY:` comments for accepted risks.
- Add the `base-security-advisory` skill, which takes a vulnerability from report to private fix, release, and published advisory, and records it in `security/advisory-{date}-{slug}.md`.
