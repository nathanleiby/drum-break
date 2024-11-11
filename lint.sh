#!/bin/sh
git ls-files | entr cargo clippy --all --all-features --tests -- -D warnings
