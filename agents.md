# Agent instructions

Run `cargo bake agent:context:install` to install dependency context and skills, and use `cargo bake agent:context:list` and `cargo bake agent:context:show` to discover guidance. Follow the Base conventions and relevant installed skills under `.agents/`.

Before planning or changing behavior, read applicable `design/*.md`, and assess changes against them before opening or updating a pull request. This crate's own skills are authored in `context/audit.md` and `context/advisory.md`. The installer excludes workspace members, so it does not install them into this checkout; edit the source skill documents, and use their installed names, `base-security-audit` and `base-security-advisory`, in consuming projects.

The private `bake/` package uses `base-project` for development and release tasks. Run `cargo bake --regenerate` after changing task dependencies. Use `cargo bake test` for the workspace tests and `cargo bake test:coverage --all-targets true` for the standard coverage check. The tests package the crate and verify that a consumer can discover and install both skills from the packaged contents.
