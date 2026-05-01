#!/bin/bash
# AnyDesk Pivot Detector - Environment Setup Script (Linux/macOS)

echo "Setting up AnyDesk Pivot Detector development environment..."

# 1. Rust Toolchain
if ! command -v cargo &> /dev/null; then
    echo "Rust not found. Please install it from https://rustup.rs/"
    exit 1
fi
rustup update stable
rustup component add clippy rustfmt

# 2. Node.js & pnpm
if ! command -v node &> /dev/null; then
    echo "Node.js not found. Please install it."
    exit 1
fi
if ! command -v pnpm &> /dev/null; then
    echo "Installing pnpm..."
    npm install -g pnpm
fi

# 3. Cargo Tools
echo "Installing Cargo tools..."
cargo install cargo-nextest cargo-tarpaulin cargo-audit cargo-deny cargo-tauri

# 4. Frontend Dependencies
echo "Installing frontend dependencies..."
cd frontend && pnpm install && cd ..

# 5. Environment
if [ ! -f .env ]; then
    echo "Creating .env from example..."
    cp .env.example .env
fi

# 6. Git Hooks
if command -v pre-commit &> /dev/null; then
    echo "Installing pre-commit hooks..."
    pre-commit install
fi

echo "Setup complete!"
