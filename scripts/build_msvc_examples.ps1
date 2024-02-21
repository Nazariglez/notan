# Create directory if it doesn't exist
[void](New-Item -ItemType Directory -Path ".\docs\msvc_examples\examples\assets" -ErrorAction SilentlyContinue)

# Copy assets to docs/examples directory
Copy-Item -Path ".\examples\assets" -Destination ".\docs\msvc_examples" -Recurse -ErrorAction SilentlyContinue

# Function to compile each example
function Compile {
    param (
        [string]$example
    )

    .\scripts\msvc_example.ps1 $example --release --no-assets
}

# Loop through each .rs file in examples directory
Get-ChildItem -Path ".\examples\*.rs" | ForEach-Object {
    Compile $_.Basename
}
