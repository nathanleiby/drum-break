# !/bin/bash
cargo build --release
mkdir -p release
cp target/release/macroix ./release/macroix
chmod 700 ./release/macroix
cp -r res/ ./release/res/
version="$(cat VERSION)"
zip -r release-$version.zip release
