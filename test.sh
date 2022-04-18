#! /bin/bash

sudo ufw enable
source $HOME/.cargo/env
export RUSTFLAGS="-Cinstrument-coverage"
cargo build
LLVM_PROFILE_FILE="./profraw/your_name-%p-%m.profraw" cargo test --verbose
grcov . --binary-path ./target/debug/ -s . -t html --branch --ignore-not-existing --ignore "/*" -o coverage