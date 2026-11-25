# Team Whole Lotta Panic

Advanced Programming Course Project  
A.Y. 2026/2027

## Members

- Ernesto Beltrami
- Tommaso Martini
- Marcu Chereji
- Alessandro Roncato

## Workspace

The repository is a Cargo workspace. The root `Cargo.toml` lists the members, and each member has its own `Cargo.toml` and `src/`. The workspace shares one `Cargo.lock` and one `target/` directory.

| Package | Role |
| --- | --- |
| `admiralty` | Schedules turns and coordinates communication. |
| `cartographer` | Displays the map and ships. Its binary runs the current 2D preview. |
| `chart` | Owns the game state and applies the rules. |
| `harness` | Provides the entry point for assembling and running the game. |
| `islands` | Holds the team's Island implementations. |
| `ship` | Holds the team's Ship implementation. |

Run Cargo commands from the repository root. Use `-p` to select one package or `--workspace` to check or test every member.

```sh
cargo check -p cartographer
cargo test -p cartographer
cargo run -p cartographer
cargo test --workspace
```

If the team adds a package, create it from the root with `cargo new --lib --edition 2024 --vcs none <name>` for a library, or use `--bin` for an executable. Verify that it appears in the root `workspace.members` list. The `--vcs none` flag keeps version control at the repository root.

## Dependencies

Add a dependency under `[dependencies]` in the `Cargo.toml` of the package that imports it. For example, `eframe` belongs in [`cartographer/Cargo.toml`](cartographer/Cargo.toml). If a dependency is used only by tests, put it under `[dev-dependencies]` in that package's manifest.

For another package in this repository, use a relative path. For example, when the harness starts using the Chart, add this to the harness manifest:

```toml
[dependencies]
chart = { path = "../chart" }
```

When two packages exchange types from the course's shared code, use the same Git URL and revision for the shared dependency. After changing dependencies, run `cargo check` and include the root `Cargo.lock` if it changed.

## Before committing

Configure your editor to run `rustfmt` on save. From the workspace root, run:

```sh
cargo fmt --all
cargo fmt --all --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -W clippy::pedantic -D warnings
```

`cargo fmt` applies formatting and `--check` verifies it. `--locked` makes Cargo use the checked-in dependency resolution without updating the lockfile. The Clippy command includes test targets and fails on warnings.

Review the diff before staging. Keep each commit focused on one change and check what you staged with `git diff --cached`. For example, `feat(cartographer): draw ships at the map corners` describes one change that can be reviewed on its own.

## Contributions

Contributions will be documented during development.
