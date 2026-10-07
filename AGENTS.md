# AGENTS.md

## Project
Git commit-msg hook: extracts the ticket key from the branch name (e.g. `QUEUE-123`) and appends it to the commit message. Rust 1.85 (edition 2024). `make help` lists the targets.

## Done
Done = `make check` prints `ALL CHECKS PASSED`. Run it before every PR.

## Invariants
- Ship one self-contained binary; the OS is the only runtime dependency.
- Add a `Cargo.toml` dependency only for a clear need.
- Keep the release profile size-optimised (strip, opt-level=z, lto, codegen-units=1, panic=abort).
- Read the commit hash for `--version` from the value `build.rs` bakes in at build time.
- CLI contract, fixed: argv[1] is the commit message file (passed by git); `--validate` takes a branch name as argv[2].
- Tests run hermetic: no real git repository, no network. Follow the pattern in `tests/cli.rs` and `src/*/mod.rs`.

## Workflow
- One feature at a time; start the next only once the first is `make check` green.
- Keep refactors out of an unverified feature.
- One logical unit per commit.
- Cut tagged releases from a clean `master` with `bin/dev/tag-release.sh <version>`.
