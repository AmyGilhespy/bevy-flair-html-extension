# justfile
# `just build[-all]` -> dev, or `just build[-all] profile=release` -> release

profile := "debug"

[private]
default: list

# List all recipes.
list:
    just --list

# Build with `just build` -> dev, or `just build profile=release` -> release
build:
    clear
    cargo build {{ if profile == "release" { "--release" } else { "" } }}

# Build with `just build-all` -> dev, or `just build-all profile=release` -> release
build-all: cross-linux cross-windows cross-macos-x64 cross-macos-aarch64

# Clear cache, delete temps, and I think it also deletes the built files.
clean:
    cargo clean

# Reformat the code as defined in the style guide.
fmt:
    cargo fmt

# Add all unignored files to git and make a commit with MESSAGE as the commit message.
commit *MESSAGE="Unlabeled commit.": ci
    git add .
    git commit -m "{{ MESSAGE }}"

# Push commits to remote repository.
push:
    git push -u origin main

# Pull commits from remote repository.
pull:
    git pull

# Run `cargo check`, `cargo fmt --check`, `cargo clippy -- -D warnings` and same with `-W clippy::pedantic`, `cargo test`, and `cargo build`.
ci:
    clear
    cargo check
    cargo fmt --check
    cargo clippy -- -D warnings
    cargo clippy -- -D warnings -W clippy::pedantic
    cargo test
    cargo build

publish-dry-run: ci
    cargo publish -p bevy-flair-html-extension --dry-run

publish-for-real: publish-dry-run
    cargo publish -p bevy-flair-html-extension

# Added target with `rustup target add x86_64-unknown-linux-gnu`
[private]
cross-linux:
    cargo build --target x86_64-unknown-linux-gnu {{ if profile == "release" { "--release" } else { "" } }}

# Added target with `rustup target add x86_64-pc-windows-gnu`
[private]
cross-windows:
    cargo build --target x86_64-pc-windows-gnu {{ if profile == "release" { "--release" } else { "" } }}

# Added target with `rustup target add x86_64-apple-darwin`
[private]
cross-macos-x64:
    cargo build --target x86_64-apple-darwin {{ if profile == "release" { "--release" } else { "" } }}

# Added target with `rustup target add aarch64-apple-darwin`
[private]
cross-macos-aarch64:
    cargo build --target aarch64-apple-darwin {{ if profile == "release" { "--release" } else { "" } }}
