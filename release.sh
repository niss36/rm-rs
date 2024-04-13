if ! (git diff --exit-code --quiet); then
    echo "You have unsaved changes, please commit or stash them first."
    exit 1
fi

tag="$1"

if [[ "$tag" != v?*.?*.?* ]]; then
    echo "Usage: release.sh v<X.Y.Z>"
    exit 1
fi

version="${tag#v}"

echo "📦 Preparing to release $tag"

# bump version number
msg="# managed by release.sh - do not edit manually"
sed -E -i '' "s/^version = \".+\" $msg$/version = \"$version\" $msg/" Cargo.toml
sed -E -i '' "s/\"version\": \".+\"/\"version\": \"$version\"/" npm/rm-rs/package.json
sed -E -i '' "s/\"(rm-rs-.+)\": \".+\"/\"\1\": \"$version\"/g" npm/rm-rs/package.json

# update lockfiles
cargo check
cd npm && bash generate-packages.sh && cd ..
pnpm install

git add --all
git commit -m "chore(release): prepare release $tag"

git tag "$tag"

echo "🚀 Done!"
