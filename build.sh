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
  local package_path python_bin
  package_path="$(realpath "$1")"
  if python3 -c "import zipfile" >/dev/null 2>&1; then
    python_bin="python3"
  elif python -c "import zipfile" >/dev/null 2>&1; then
    python_bin="python"
  else
    echo "Python 3 is required to add legacy package icons." >&2
    return 1
  fi
  "$python_bin" - "$package_path" <<'PY'
import copy
import os
import sys
import tempfile
import zipfile

package_path = sys.argv[1]
with zipfile.ZipFile(package_path, "r") as archive:
    entries = [
        (info, archive.read(info))
        for info in archive.infolist()
        if info.filename != "Payload/Icon.png"
    ]

icon = next((data for info, data in entries if info.filename == "Payload/icon.png"), None)
if icon is None:
    raise SystemExit(f"{package_path}: Payload/icon.png is missing")

fd, temporary_path = tempfile.mkstemp(
    prefix=".aidoku-icon-", suffix=".aix", dir=os.path.dirname(package_path)
)
os.close(fd)
try:
    with zipfile.ZipFile(temporary_path, "w") as archive:
        for info, data in entries:
            archive.writestr(info, data)
        legacy_info = copy.copy(
            next(info for info, _ in entries if info.filename == "Payload/icon.png")
        )
        legacy_info.filename = "Payload/Icon.png"
        archive.writestr(legacy_info, icon)
    os.replace(temporary_path, package_path)
finally:
    if os.path.exists(temporary_path):
        os.unlink(temporary_path)
PY
}

for src in ./src/as/*; do
  [[ -d "$src" ]] || continue
  if is_deprecated "$src"; then
    continue
  fi

  (
    cd "$src"
    npm ci
    npm run build
  )
  package="$src/build/package.aix"
  packages+=("$package")
done

for src in ./src/rust/*; do
  [[ -d "$src" ]] || continue
  if is_deprecated "$src"; then
    continue
  fi

  (
    cd "$src"
    aidoku package
  )
  package="$src/package.aix"
  packages+=("$package")
done

if ((${#packages[@]} == 0)); then
  echo "No active source packages were built." >&2
  exit 1
fi

# The current Aidoku CLI emits the canonical v2 object and its compact form.
rm -rf -- "$V2_OUTPUT"
aidoku build "${packages[@]}" --output "$V2_OUTPUT" --name "$SOURCE_LIST_NAME"

# `aidoku build` rewrites every package, so compatibility files added before
# this point are discarded. Add the legacy runner's case-sensitive icon name to
# the final packages that are actually published.
for package in "$V2_OUTPUT"/sources/*.aix; do
  add_legacy_icon "$package"
done

# Keep package/icon locations used by the long-standing legacy directory URL.
mkdir -p public/sources public/icons
cp -f "$V2_OUTPUT"/sources/*.aix public/sources/
cp -f "$V2_OUTPUT"/icons/*.png public/icons/

# Retained legacy packages include deprecated sources that are intentionally no
# longer present in the v2 list. Repair their case-sensitive icon entry too, so
# existing installs (notably 漫畫狗) no longer show a blank icon.
for package in public/sources/*.aix; do
  if unzip -Z1 "$package" | grep -qx "Payload/icon.png"; then
    add_legacy_icon "$package"
  fi
done

# Preserve public/index.json and public/index.min.json as legacy arrays. Current
# clients should use public/v2/index.min.json, which avoids the legacy warning.
node scripts/build-indexes.mjs \
  "$V2_OUTPUT/index.min.json" \
  public/index.json \
  public/index.min.json
