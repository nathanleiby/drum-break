# !/bin/bash

# exit on error
set -e

rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/debug/macroix.wasm .
