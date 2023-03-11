#!/usr/bin/env bash

cargo build --release
cargo doc --release
ln -s ./target/release/hanoi ./hanoi
