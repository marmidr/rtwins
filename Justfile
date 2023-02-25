# `Just a command runner` script
# Web: https://github.com/casey/just
# Man: https://just.systems/man/en/
#
# Bash: put the completion script under ~/.local/share/bash-completion/completions/
#

# by default (no params), list the recipes
default:
    @just --list

alias b  := build-dbg
alias br := build-rel
alias r  := run-dbg
alias rr := run-rel
alias t  := test
alias nx := nxtest

# build debug; eg: `just build-dbg -v`
build-dbg *ARGS:
    cargo build
    ls -hsk target/debug/eva

# build release
build-rel *ARGS:
    cargo build --release {{ARGS}}
    ls -hsk target/release/eva

#
run-dbg:
    clear
    cargo run

run-rel:
    clear
    cargo run --release

# default tests runner
test:
    clear
    cargo test

# run tests using `nextest`
nxtest:
    clear
    cargo nextest run

# code coverage using tarpaulin
cover-tarp:
    @cargo tarpaulin -V; [ $? -eq 0 ] || cargo install cargo-tarpaulin
    cargo tarpaulin --out html --output-dir target/ --skip-clean --exclude-files "tests/*"

# vulnerabilities check
audit:
    cargo audit

# format using nightly (due to unstable rustfmt options)
fmt:
    cargo +nightly fmt
