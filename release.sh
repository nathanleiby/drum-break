# !/bin/bash
cargo build --release
mkdir -p release
cp target/release/macroix ./release/macroix
chmod 700 ./release/macroix
cp -r res/ ./release/res/
version="$(cat VERSION)"
zip_file=release-$version.zip
zip -r $zip_file release

gh release create --prerelease --notes "" $version $zip_file
