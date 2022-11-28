#!/bin/bash
mkdir -p ./docs/examples/$1

# web_sys_unstable_apis is required to enable the web_sys clipboard API
# https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Clipboard.html
export RUSTFLAGS=--cfg=web_sys_unstable_apis
features=glyph,egui,text,extra,audio,links,drop_files,clipboard,save_file,texture_to_file

if [[ $2 == '--release' ]];
then
  cargo build --target wasm32-unknown-unknown --release --example $1 --features=$features
  wasm-bindgen ./target/wasm32-unknown-unknown/release/examples/$1.wasm --out-dir ./docs/examples/$1 --no-modules --browser
  wasm-opt -O -o ./docs/examples/$1/$1_bg.wasm ./docs/examples/$1/$1_bg.wasm

  if [[ $3 == '--gzip' ]];
  then
    gzip -f ./docs/examples/$1/$1_bg.wasm
    gzip -f ./docs/examples/$1/$1.js
  fi

else
  cargo build --target wasm32-unknown-unknown --example $1 --features=$features
  wasm-bindgen ./target/wasm32-unknown-unknown/debug/examples/$1.wasm --out-dir ./docs/examples/$1 --no-modules --browser --keep-debug --debug
fi

cp ./scripts/example.html ./docs/examples/$1.html
index=$(sed "s/{{ EXAMPLE }}/${1}/g" "./scripts/example.html")
echo "${index}" > ./docs/examples/$1.html

if [[ $3 != '--no-assets' ]];
then
  cp -R ./examples/assets ./docs/examples
fi
