cargo build --target wasm32-unknown-unknown --release
mkdir -p generated
wasm-bindgen ./target/wasm32-unknown-unknown/release/nae.wasm --out-dir generated --no-modules
cp index.html generated