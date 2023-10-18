#!/usr/bin/env bash
set -e

rm -rf src/
svd2rust -i ATSAM3X8E.svd
form -i lib.rs -o ./src/
rm -f lib.rs
cargo fmt
cargo build
