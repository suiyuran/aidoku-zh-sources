#!/usr/bin/env bash

set -euo pipefail
shopt -s nullglob

SOURCE_LIST_NAME="Aidoku 中文图源"
V2_OUTPUT="public/v2"
packages=()

is_deprecated() {
  node -e '
    const fs = require("node:fs");
    const source = JSON.parse(fs.readFileSync(process.argv[1], "utf8"));
    process.exit(source.deprecated === true ? 0 : 1);
  ' "$1/res/source.json"
}

# These sources still use Aidoku's legacy WASM ABI. The current CLI and v2
# runner use Payload/icon.png, while Aidoku's legacy compatibility runner reads
# Payload/Icon.png. Publish both entries until the source ABI is migrated.
add_legacy_icon() {
  local package_path stage_dir
  package_path="$(realpath "$1")"
  stage_dir="$(mktemp -d)"
  unzip -q "$package_path" -d "$stage_dir"
  cp "$stage_dir/Payload/icon.png" "$stage_dir/Payload/Icon.png"
  (
    cd "$stage_dir"
    zip -q "$package_path" Payload/Icon.png
  )
  rm -rf -- "$stage_dir"
}

for src in ./src/as/*; do
  if is_deprecated "$src"; then
    continue
  fi

  (
    cd "$src"
    npm ci
    npm run build
  )
  package="$src/build/package.aix"
  add_legacy_icon "$package"
  packages+=("$package")
done

for src in ./src/rust/*; do
  if is_deprecated "$src"; then
    continue
  fi

  (
    cd "$src"
    aidoku package
  )
  package="$src/package.aix"
  add_legacy_icon "$package"
  packages+=("$package")
done

if ((${#packages[@]} == 0)); then
  echo "No active source packages were built." >&2
  exit 1
fi

# The current Aidoku CLI emits the canonical v2 object and its compact form.
rm -rf -- "$V2_OUTPUT"
aidoku build "${packages[@]}" --output "$V2_OUTPUT" --name "$SOURCE_LIST_NAME"

# Keep package/icon locations used by the long-standing legacy directory URL.
mkdir -p public/sources public/icons
cp -f "$V2_OUTPUT"/sources/*.aix public/sources/
cp -f "$V2_OUTPUT"/icons/*.png public/icons/

# Preserve public/index.json and public/index.min.json as legacy arrays. Current
# clients should use public/v2/index.min.json, which avoids the legacy warning.
node scripts/build-indexes.mjs \
  "$V2_OUTPUT/index.min.json" \
  public/index.json \
  public/index.min.json
