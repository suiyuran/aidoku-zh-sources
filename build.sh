#!/bin/bash

for src in ./src/as/*; do
  (
    cd "$src"

    if [ "$(cat ./res/source.json | grep 'deprecated' | awk '{print $2}')" == "true" ]; then
      rm -rf ./build/package.aix
    else
      npm run build
    fi
  )
done

for src in ./src/rust/*; do
  (
    cd "$src"

    if [ "$(cat ./res/source.json | grep 'deprecated' | awk '{print $2}')" == "true" ]; then
      rm -rf ./package.aix
    else
      ./build.sh -a
    fi
  )
done

aidoku build ./src/**/*.aix
