#!/bin/bash
cargo build --target wasm32-unknown-unknown --release --example $1
mkdir -p output
wasm-bindgen ./target/wasm32-unknown-unknown/release/examples/$1.wasm --out-dir output --no-modules
cp example.html output/$1.html
index=$(sed "s/{{ EXAMPLE }}/${1}/g" "example.html")
echo "${index}" > output/$1.html