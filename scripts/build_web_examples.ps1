# Create directory if it doesn't exist
[void](New-Item -ItemType Directory -Path ".\docs\examples\assets" -ErrorAction SilentlyContinue)

# Copy assets to docs/examples directory
Copy-Item -Path ".\examples\assets" -Destination ".\docs\examples" -Recurse -ErrorAction SilentlyContinue

# Initialize document body
$doc_body = "<ul>`n"

# Function to compile each example
function Compile {
    param (
        [string]$example
    )

    .\scripts\web_example.ps1 $example --release --no-assets

    $url = "examples/${example}.html"
    $image = "examples/images/${example}.jpg"
    $doc_body += "`n<li><a href=`"$url`"><div class=`"example-image`"><img src=`"$image`" alt=`"$example`"></div><div class=`"example-link`">$example</div></a></li>"
}

# Loop through each .rs file in examples directory
Get-ChildItem -Path ".\examples\*.rs" | ForEach-Object {
    Compile $_.Basename
}

# Wait for compilation to finish
#Wait-Process -Name "web_example"

$doc_body += "`n</ul>"

# Copy docs.html to index.html and replace body placeholder
Copy-Item -Path ".\scripts\docs.html" -Destination ".\docs\index.html"
$index = (Get-Content ".\scripts\docs.html") -replace "{{ BODY }}", $doc_body
$index | Set-Content -Path ".\docs\index.html"
