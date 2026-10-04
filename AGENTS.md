# AGENTS.md

## Project
Git commit-msg hook that extracts a ticket/issue number from the branch name and appends it to the commit message. Rust 1.85 (edition 2024), no external runtime dependencies beyond the system allocator.

## Commands
- Build (debug): `make build`
- Build (release): `make release`
- Test: `make test`
- Full verification: `make check`
- Install from source: `make install`
- Find outdated dependencies: `make outdated`
- Tag a release: `bin/dev/tag-release.sh <version>`

## Hard Constraints (MUST)
- Dependencies managed via `Cargo.toml`. Do not add new ones without a clear need.
- The binary must remain a single self-contained executable — no runtime dependencies beyond what the OS provides.
- Release builds use the size-optimised profile: strip, opt-level=z, lto, codegen-units=1, panic=abort. Do not relax these.
- The commit hash is baked into the binary at build time via `build.rs`. Do not shell out to git for `--version`.
- CLI contract: the first positional argument is always the commit message file (passed by git). `--validate` takes a branch name as the second argument. Do not change this interface.
- Never write tests that require a real git repository or network access. Use the existing pattern from `tests/cli.rs` and `src/*/mod.rs`.

## Definition of Done
Feature is done = `make check` green.
"Code written" is not done.

## Rules
- One feature at a time. Do not start a second until the first passes verification.
- No drive-by refactoring while the main feature is unverified.
- Before PR: run `make check`.
- Atomic commits — one logical unit of work per commit.
- Tagged releases are cut from `master` with a clean working tree. Use `bin/dev/tag-release.sh`.

## Where to Look for Details
- `src/main.rs` — entry point, argument dispatch.
- `src/environment/` — CLI argument parsing, env vars, git config reading.
- `src/ticket_number/` — extraction of ticket key from branch name (e.g. `QUEUE-123`).
- `src/patch_commit_msg/` — logic for inserting/appending the ticket line into the commit message.
- `src/process/` — thin wrapper around `std::process::Command`.
- `src/file/` — file read/write helpers.
- `tests/cli.rs` — integration tests exercising the binary end-to-end.
- `bin/dev/` — development scripts (build, test, release).
- `bin/install/` — install and download scripts for end users.
- `bin/support/` — maintenance scripts (outdated deps).
