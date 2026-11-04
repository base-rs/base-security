# Stable Record Format

## Requirement

The `security/audit-{date}-{sha}.md` and `security/advisory-{date}-{slug}.md` formats must remain readable by simple line-oriented tools. An audit entry names one file and its outcome on a single line, with findings in a nested list using the fixed outcome forms. An advisory record carries its identifiers, package, severity, and version requirements in front matter.

Changing the filename pattern, entry wording, outcome forms, or front matter fields is a breaking change for projects that keep records. Such a change requires a migration path for existing records and a corresponding release note.

## Rationale

Records accumulate in consuming projects over years. Agents and scripts use them to find files due for review and to locate advisories, so the format must stay stable and parseable without a dedicated parser.

## Scope

The record formats defined by `base-security-audit` and `base-security-advisory`, including the example commands in those skills.

## Validation

`bake/tests/audit.rs` checks that the audit examples in the skill and readme follow the stated rules, and runs the due-file command from `base-security-audit` against a fixture repository. The fixture covers unreviewed, changed, and unchanged files, two audits on the same day, a path mentioned only in a finding description, paths with spaces, leading spaces, and non-ASCII characters, a file added after the checkout, and running from a subdirectory. Inspect the advisory example against the stated front matter fields.
