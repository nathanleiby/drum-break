# !/bin/bash

# exit on error
set -e

rustup target add wasm32-unknown-unknown
./wasm-bindgen-macroquad.sh --release drum-break
rm -rf github_pages

# copy assets
cp -r assets dist/

# put it into a folder that will be committed and pushed
cp -r dist github_pages
