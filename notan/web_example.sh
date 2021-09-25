#!/bin/bash
mkdir -p output/examples
mkdir -p output/$1
if [[ $2 == '--release' ]];
then
  cargo build --target wasm32-unknown-unknown --release --example $1 --all-features
  wasm-bindgen ./target/wasm32-unknown-unknown/release/examples/$1.wasm --out-dir output/$1 --no-modules --browser
else
  cargo build --target wasm32-unknown-unknown --example $1 --all-features
  wasm-bindgen ./target/wasm32-unknown-unknown/debug/examples/$1.wasm --out-dir output/$1 --no-modules --browser --keep-debug --debug
fi

cp example.html output/$1.html
index=$(sed "s/{{ EXAMPLE }}/${1}/g" "example.html")
echo "${index}" > output/$1.html
cp -R examples/assets output/examples/assets