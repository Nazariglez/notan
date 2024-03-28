param(
    [string]$1,
    [string]$2,
    [string]$3
)

[void](New-Item -ItemType Directory -Path ".\docs\examples\$1" -Force)

$env:RUSTFLAGS = "--cfg=web_sys_unstable_apis"
$features = "glyph,egui,text,extra,audio,links,drop_files,clipboard,save_file,texture_to_file"

if ($2 -eq "--release") {
    # Build release version
    cargo build --target wasm32-unknown-unknown --release --example $1 --features=$features
    wasm-bindgen .\target\wasm32-unknown-unknown\release\examples\$1.wasm --out-dir .\docs\examples\$1 --no-modules --browser
    wasm-opt -O -o .\docs\examples\$1\${1}_bg.wasm .\docs\examples\$1\${1}_bg.wasm

    if ($3 -eq "--gzip") {
        # Gzip the wasm files
        Compress-Archive -Path ".\docs\examples\$1\$1_bg.wasm" -DestinationPath ".\docs\examples\$1\$1_bg.wasm.gz" -Force
        Compress-Archive -Path ".\docs\examples\$1\$1.js" -DestinationPath ".\docs\examples\$1\$1.js.gz" -Force
    }
}
else {
    # Build debug version
    cargo build --target wasm32-unknown-unknown --example $1 --features=$features
    wasm-bindgen .\target\wasm32-unknown-unknown\debug\examples\$1.wasm --out-dir .\docs\examples\$1 --no-modules --browser --keep-debug --debug
}

# Copy example.html and replace placeholder with example name
Copy-Item .\scripts\example.html .\docs\examples\$1.html -Force
$index = Get-Content .\scripts\example.html | ForEach-Object { $_ -replace '{{ EXAMPLE }}', $1 }
$index | Set-Content .\docs\examples\$1.html -Force

if ($3 -ne "--no-assets") {
    # Copy assets folder
    Copy-Item .\examples\assets .\docs\examples -Recurse -Force
}
