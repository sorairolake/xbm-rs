# SPDX-FileCopyrightText: 2024 Shun Sakai
#
# SPDX-License-Identifier: Apache-2.0 OR MIT

alias lint := clippy

# Run default recipe
_default:
    just -l

# Build a package
build:
    cargo build

# Remove generated artifacts
clean:
    cargo clean

# Check a package
check:
    cargo check

# Run tests
test:
    cargo test

# Run benchmarks
bench:
    cargo +nightly bench

# Run the formatter
fmt:
    cargo fmt

# Run the formatter with options
fmt-with-options:
    cargo +nightly fmt

# Run the linter
clippy:
    cargo clippy -- -D warnings

# Apply lint suggestions
clippy-fix:
    cargo +nightly clippy --fix --allow-dirty --allow-staged -- -D warnings

# Build the package documentation
doc $RUSTDOCFLAGS="--cfg docsrs":
    cargo +nightly doc --all-features

# Run the linter for GitHub Actions workflow files
lint-github-actions:
    actionlint -verbose

# Run the formatter for the README
fmt-readme:
    npx prettier -w README.md

# Increment the version
bump part:
    bump-my-version bump {{ part }}
    cargo set-version --bump {{ part }}
