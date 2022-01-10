#
# SPDX-License-Identifier: Apache-2.0 OR MIT
#
# Copyright (C) 2021 Shun Sakai
#

alias all := default

# Run default recipe
default: build

# Build a package
@build:
    cargo build

# Check a package
@check:
    cargo check

# Run tests
@test:
    cargo test

# Run the code formatter
@fmt:
    cargo fmt

# Run the linter
@clippy:
    cargo clippy -- -D warnings

# Run the linter for GitHub Actions workflow files
@lint-github-actions:
    actionlint

# Run the code formatter for the README
@fmt-readme:
    npx prettier -w README.md

# Run the linter for the README
@lint-readme:
    npx markdownlint README.md
