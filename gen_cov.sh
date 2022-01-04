echo "*** Build tests... ***"
export LLVM_PROFILE_FILE="your_name-%p-%m.profraw"
export CARGO_INCREMENTAL=0
# export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
# export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests"
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off"
# export RUSTDOCFLAGS="-Cpanic=abort"
cargo test
echo "*** Generate test coverage report... ***"
# do not cover widgets drawing code:
IGNORED="*/widget_draw.rs"
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing --ignore $IGNORED -o ./target/debug/coverage/
[[ $? -eq 0 ]] && echo -e "\x1B]8;;$(pwd)/target/debug/coverage/index.html\x07Coverage report ready\x1B]8;;\x07" || echo "ERROR"
