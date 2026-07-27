#!/usr/bin/env bash

set -euo pipefail
shopt -s nullglob

count=0

for src in ./src/as/*; do
  [[ -d "$src" ]] || continue
  echo "AUDIT_BUILD_START ${src#./src/}"
  (
    cd "$src"
    npm ci
    npm run build
  )
  echo "AUDIT_BUILD_OK ${src#./src/}"
  ((count += 1))
done

for src in ./src/rust/*; do
  [[ -d "$src" ]] || continue
  echo "AUDIT_BUILD_START ${src#./src/}"
  (
    cd "$src"
    aidoku package
  )
  echo "AUDIT_BUILD_OK ${src#./src/}"
  ((count += 1))
done

echo "AUDIT_BUILD_TOTAL $count"
