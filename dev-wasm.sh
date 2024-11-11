# !/bin/bash

# exit on error
set -e

rustup target add wasm32-unknown-unknown
./wasm-bindgen-macroquad.sh drum-break

# copy assets
cp -r assets dist/

# run
pushd dist/
basic-http-server -a 0.0.0.0:4001 .
popd
