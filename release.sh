# !/bin/bash

rustup target add x86_64-apple-darwin
cargo install universal2
cargo universal2
mkdir -p release
cp target/universal2-apple-darwin/macroix ./release/macroix
chmod 700 ./release/macroix
cp -r res/ ./release/res/
version="$(cat VERSION)"
zip_file=release-$version.zip
zip -r $zip_file release

gh release create --prerelease --notes "" $version $zip_file
