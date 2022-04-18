#!/bin/bash
set -e
set -o pipefail
sudo ufw enable
source $HOME/.cargo/env
export RUSTFLAGS="-Cinstrument-coverage"
cargo build
LLVM_PROFILE_FILE="your_name-%p-%m.profraw" cargo test --verbose --color always
grcov . --binary-path ./target/debug/ -s . -t lcov --branch --ignore-not-existing --ignore "/*" -o coverage/coverage.lcov
chown -R nobody:nogroup ./coverage
chmod -R 777 ./coverage