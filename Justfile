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

# build debug; eg: `just build-dbg -v`
build-dbg *ARGS:
    cargo build
    ls -hsk target/debug/demo1

# build release
build-rel *ARGS:
    cargo build --release {{ARGS}}
    ls -hsk target/release/demo1

#
run-dbg:
    clear
    cargo run

run-rel:
    clear
    cargo run --release

# find out what functions takes most of the space in the library
bloat-lib:
    cargo bloat --release --filter rtwins --no-relative-size -n 50

# find out what crates takes most of the space in the executable
bloat-demo:
    cargo bloat --release --crates

# expand macros in demo <module>.rs
expand-demo *ARGS:
    cargo expand --bin demo1 {{ARGS}}

# expand macros in twins library <module>.rs
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
    @cargo tarpaulin -V; [ $? -eq 0 ] || cargo install cargo-tarpaulin
    cargo tarpaulin --out html --output-dir target/ --skip-clean --exclude-files "tests/*"

# vulnerabilities check
audit:
    cargo audit

# format using nightly (due to unstable rustfmt options)
fmt:
    cargo +nightly fmt

# debug locally
gdb:
    # rust-gdb -tui -ex "b main" target/debug/demo1 -ex "r"
    cgdb -ex "b main" target/debug/demo1 -ex "r"

# debug remote
gdbremote:
    rust-gdb -tui -ex "target remote :6789" -ex "b main" -ex "c"

# gdb server
gdbserve:
    gdbserver :6789 target/debug/demo1
