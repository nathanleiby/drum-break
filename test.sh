#!/bin/sh
git ls-files | RUST_BACKTRACE=1 RUST_LOG=info entr cargo test
