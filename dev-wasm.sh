# !/bin/bash

# exit on error
set -e

rustup target add wasm32-unknown-unknown
./wasm-bindgen-macroquad.sh drum-break

## TODO: Figure out how to properly
# cp -r res dist/

pushd dist/
basic-http-server -a 0.0.0.0:4001 .
popd
