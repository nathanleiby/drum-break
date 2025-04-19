#!/bin/sh
git ls-files |   entr -r timeout -k 5 0 ./dev-wasm.sh

# TODO: it would be nice if clicking "run game" weren't required for faster dev loop.. can i avoid that
# also -- can plug into "storybook" idea of previewing various UI screens, component mindset
