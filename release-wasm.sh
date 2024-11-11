# !/bin/bash

# exit on error
set -e

rustup target add wasm32-unknown-unknown
./wasm-bindgen-macroquad.sh --release drum-break
rm -rf github_pages

## TODO: Figure out how to properly handle resources, like audio files and images
# cp -r res dist/
cp -r dist github_pages
