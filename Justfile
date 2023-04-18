# `Just a command runner` script
# Web: https://github.com/casey/just
# Man: https://just.systems/man/en/
#
# Generate Bash completion script:
# `just --completions bash > ~/.local/share/bash-completion/completions/just`
# and reload console

# by default (no params), list the recipes
default:
    @just --list

alias b  := build-dbg
alias br := build-rel
alias r  := run-dbg
alias rr := run-rel
alias t  := test
alias nx := nxtest

# build debug; eg: `just build-dbg `
build-dbg:
    cargo build --example demo_full
    cargo size --example demo_full -- -B

# build release
build-rel:
    cargo build --release --example demo_full
    cargo size --release --example demo_full -- -B

#
run-dbg:
    clear
    cargo run --example demo_full

run-rel:
    clear
    cargo run --release --example demo_full

# find out what functions takes most of the space in the library
bloat-lib:
    cargo bloat --release --example demo_full --filter rtwins --no-relative-size -n 50

# find out what crates takes most of the space in the executable
bloat-demo:
    cargo bloat --release --example demo_full --crates

# expand macros in demo <module>.rs
expand-demo *ARGS:
    cargo expand --example demo_full {{ARGS}}

# expand macros in twins module
expand-lib *ARGS:
    cargo expand --lib {{ARGS}}

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
    @cargo tarpaulin -V > /dev/null; [ $? -eq 0 ] || cargo install cargo-tarpaulin
    cargo tarpaulin --out html --output-dir target/ --skip-clean --exclude-files "tests/*"

# vulnerabilities check
audit:
    @cargo audit --version > /dev/null; [ $? -eq 0 ] || cargo install cargo-audit
    cargo audit

# format using nightly (due to unstable rustfmt options)
fmt:
    cargo +nightly fmt

# run Clippy linter
clip:
    cargo clippy --no-deps

# debug locally
gdb:
    # rust-gdb -tui -ex "b main" target/debug/examples/demo_full -ex "r"
    cgdb -ex "b main" target/debug/examples/demo_full -ex "r"

# debug remote
gdb-remote:
    rust-gdb -tui -ex "target remote :6789" -ex "b main" -ex "c"

# gdb server
gdb-server:
    gdbserver :6789 target/debug/examples/demo_full

# project dependencies tree
tree:
    cargo tree
