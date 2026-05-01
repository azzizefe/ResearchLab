# AnyDesk Pivot Detector - Environment Setup Script (Windows)

Write-Host "Setting up AnyDesk Pivot Detector development environment..." -ForegroundColor Cyan

# 1. Rust Toolchain
if (!(Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "Rust not found. Please install it from https://rustup.rs/" -ForegroundColor Red
    exit 1
}
rustup update stable
rustup component add clippy rustfmt

# 2. Node.js & pnpm
if (!(Get-Command node -ErrorAction SilentlyContinue)) {
    Write-Host "Node.js not found. Please install it." -ForegroundColor Red
    exit 1
}
if (!(Get-Command pnpm -ErrorAction SilentlyContinue)) {
    Write-Host "Installing pnpm..." -ForegroundColor Yellow
    npm install -g pnpm
}

# 3. Cargo Tools
Write-Host "Installing Cargo tools (nextest, tarpaulin, audit, deny)..." -ForegroundColor Yellow
cargo install cargo-nextest cargo-tarpaulin cargo-audit cargo-deny cargo-tauri

# 4. Frontend Dependencies
Write-Host "Installing frontend dependencies..." -ForegroundColor Yellow
cd frontend
pnpm install
cd ..

# 5. Environment
if (!(Test-Path .env)) {
    Write-Host "Creating .env from example..." -ForegroundColor Yellow
    Copy-Item .env.example .env
}

# 6. Git Hooks
if (Get-Command pre-commit -ErrorAction SilentlyContinue) {
    Write-Host "Installing pre-commit hooks..." -ForegroundColor Yellow
    pre-commit install
}

Write-Host "Setup complete! Use 'make dev' or 'cargo tauri dev' to start." -ForegroundColor Green
