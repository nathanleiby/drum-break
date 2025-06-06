# !/bin/bash

# exit on error
set -e

# prep
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
mkdir -p release

# build: x86
target=x86_64-apple-darwin
final_binary_name=drum-break-x86
cargo build --release --target $target
cp target/x86_64-apple-darwin/release/drum-break ./release/$final_binary_name
chmod 700 ./release/$final_binary_name

# build: arm
cargo build --release --target aarch64-apple-darwin
cp target/aarch64-apple-darwin/release/drum-break ./release/drum-break-aarch64
chmod 700 ./release/drum-break-aarch64

# add assets
cp -r assets/ ./release/assets/

# zip it up and ship the release
version="$(cat VERSION)"
zip_file=release-$version.zip
zip -r $zip_file release

echo "About to upload release version '$version' to Github ..."
read -p "Are you sure? (y/n) " -n 1 -r
echo    # (optional) move to a new line
if ! [[ $REPLY =~ ^[Yy]$ ]]
then
    echo "Canceled release."
    exit 0;
fi

gh release create $version $zip_file --prerelease --notes "$version"
