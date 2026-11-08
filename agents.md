# Agent instructions

Run `cargo bake agent:context:install` to install dependency context and skills, and use `cargo bake agent:context:list` and `cargo bake agent:context:show` to discover guidance. Follow the Base conventions and relevant installed skills under `.agents/`.

Before planning or changing behavior, read applicable `design/*.md`, and assess changes against them before opening or updating a pull request. This crate's own skills are authored in `context/audit.md` and `context/advisory.md`. The installed `base-security-audit` and `base-security-advisory` skills come from the published `base-security` release that `base-project` depends on, so they can lag behind `context/`. Edit the source skill documents, and use their installed names in consuming projects.

Use `base-security-audit` when reviewing code for security issues, and record each review in `security/audit-*.md`. Use `base-security-advisory` for vulnerability reports and for findings that may warrant an advisory. Before reporting a security finding, read the relevant `// SECURITY:` comments and `design/*.md`.

The private `bake/` package uses `base-project` for development and release tasks. Run `cargo bake --regenerate` after changing task dependencies. Use `cargo bake test` for the workspace tests and `cargo bake test:coverage --all-targets true` for the standard coverage check. The tests package the crate and verify that a consumer can discover and install both skills from the packaged contents.
