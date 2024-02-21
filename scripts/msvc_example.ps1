param(
    [string]$1,
    [string]$2,
    [string]$3
)

$features = "glyph,egui,text,extra,audio,links,drop_files,clipboard,save_file,texture_to_file"
if ($2 -eq "--release") {
    # Build release version
    cargo build --target x86_64-pc-windows-msvc --release --example $1 --features=$features
    Copy-Item .\target\x86_64-pc-windows-msvc\release\examples\$1.exe .\docs\msvc_examples\$1.exe
}
else {
    # Build debug version
    cargo build --target x86_64-pc-windows-msvc --example $1 --features=$features
    Copy-Item .\target\x86_64-pc-windows-msvc\debug\examples\$1.exe .\docs\msvc_examples\$1.exe
}

if ($3 -ne "--no-assets") {
    # Copy assets folder
    Copy-Item .\examples\assets .\docs\msvc_examples -Recurse -Force
}
