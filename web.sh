cargo build --target wasm32-unknown-unknown
mkdir -p generated
wasm-bindgen ./target/wasm32-unknown-unknown/debug/nae.wasm --out-dir generated --no-modules
cp index.html generated