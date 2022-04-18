#! /bin/bash

ufw enable
$HOME/.cargo/bin/cargo test
chown -R nobody:nogroup target