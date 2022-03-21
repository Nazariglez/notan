#!/bin/bash

# this script needs `scrot` installed on a linux machine
run() {
  cargo run --example "$f" --all-features
}

finish() {
  scrot "./docs/examples/images/$f.jpg" -u -d 30
  path="target/debug/examples/$f"
  p=$(ps -ef | awk -v path="$path" '$8==path {print $2}')
  kill "$p"
}

mkdir -p ./docs/examples/images
for f in ./examples/*.rs; do
  f=$f
  f=${f/\.\/examples\//""}
  f=${f/.rs/""}
  # take a screenshot if it doesn't exists
  if [ ! -f ./docs/examples/images/"$f".jpg ]; then
    run "$f" &
    finish "$f"
  fi
done

