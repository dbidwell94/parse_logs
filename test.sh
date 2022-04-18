#! /bin/bash

sudo ufw enable
$HOME/.cargo/bin/cargo build
chown -R nobody:nogroup target
$HOME/.cargo/bin/cargo test
