#!/bin/bash

for src in ./src/as/*; do
  (
    cd "$src"
    npm run build
  )
done

for src in ./src/rust/*; do
  (
    cd "$src"
    ./build.sh -a
  )
done

aidoku build ./src/**/*.aix
