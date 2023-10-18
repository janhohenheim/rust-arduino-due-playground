#!/usr/bin/env bash

set -e
cargo objcopy --bin rust-arduino-due-playground --target thumbv7m-none-eabi --release -- --output-target=binary image.bin
