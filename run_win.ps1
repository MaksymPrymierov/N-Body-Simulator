# Check if rustfmt is installed
Write-Host "||Checking if rustfmt is installed..." -ForegroundColor Yellow
if (-not (Get-Command rustfmt -ErrorAction SilentlyContinue)) {
    Write-Host "||rustfmt is not installed. Please run 'rustup component add rustfmt' to install it." -ForegroundColor Red
    exit 1
}

# Clippy
# MEMO: Uknown behaviour, adjust as we go.
Write-Host "||Running clippy..." -ForegroundColor Yellow
cargo clippy --all-targets --all-features


# Build the project
Write-Host "||Building debug build..." -ForegroundColor Yellow
cargo run
if ($LASTEXITCODE -eq 0) {
    Write-Host "||Build successful." -ForegroundColor Green -BackgroundColor Green
} else {
    Write-Host "||Build failed." -ForegroundColor Red -BackgroundColor Red
    exit 1
}

# Format Rust code
Write-Host "||Formatting Rust code..." -ForegroundColor Yellow
cargo fmt --all -- --check
if ($LASTEXITCODE -eq 0) {
    Write-Host "||Formatting successful." -ForegroundColor Green
} else {
    Write-Host "||Formatting Failed." -ForegroundColor Red -BackgroundColor Red
    exit 1
}
