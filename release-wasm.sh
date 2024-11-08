# !/bin/bash

# exit on error
set -e

rustup target add wasm32-unknown-unknown
./wasm-bindgen-macroquad.sh --release macroix
rm -rf github_pages
cp -r dist github_pages
