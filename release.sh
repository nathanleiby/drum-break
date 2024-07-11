# !/bin/bash

# exit on error
set -e

# prep
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
mkdir -p release

# build: x86
target=x86_64-apple-darwin
final_binary_name=macroix-x86
cargo build --release --target $target
cp target/x86_64-apple-darwin/release/macroix ./release/$final_binary_name
chmod 700 ./release/$final_binary_name

# build: arm
cargo build --release --target aarch64-apple-darwin
cp target/aarch64-apple-darwin/release/macroix ./release/macroix-aarch64
chmod 700 ./release/macroix-aarch64

# add assets
cp -r res/ ./release/res/

# zip it up and ship the release
version="$(cat VERSION)"
zip_file=release-$version.zip
zip -r $zip_file release

gh release create $version $zip_file --prerelease --notes "$version"
