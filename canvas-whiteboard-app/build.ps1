$ErrorActionPreference = "Stop"

# Ensure wasm-pack is installed
if (-not (Get-Command wasm-pack -ErrorAction SilentlyContinue)) {
    Write-Host "==> wasm-pack not found. Installing..."
    cargo install wasm-pack
    if ($LASTEXITCODE -ne 0) { throw "Failed to install wasm-pack" }
}

# Ensure wasm32 target is available
$targets = rustup target list --installed
if ($targets -notcontains "wasm32-unknown-unknown") {
    Write-Host "==> Adding wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
    if ($LASTEXITCODE -ne 0) { throw "Failed to add wasm32 target" }
}

Write-Host "==> Building wasm package..."
wasm-pack build --target web --out-dir pkg
if ($LASTEXITCODE -ne 0) { throw "wasm-pack build failed" }

Write-Host "==> Done. Serve the www/ directory to run the app."
Write-Host "    Example: cd www; python -m http.server 8080"
