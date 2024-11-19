#!/bin/sh
git ls-files |   entr -r timeout -k 5 0 ./dev-wasm.sh
