#!/usr/bin/env bash
set -euo pipefail

echo "🚀 Bootstrapping Lauv CDN in WSL..."

# Check if we're in WSL
if ! command -v sudo >/dev/null 2>&1; then
  echo "❌ Please run in a Debian/Ubuntu WSL distro." >&2
  exit 1
fi

# Install system dependencies
echo "📦 Installing system dependencies..."
sudo apt update
sudo apt install -y curl build-essential pkg-config libssl-dev git jq

# Install Rust if not present
if ! command -v cargo >/dev/null 2>&1; then
  echo "🦀 Installing Rust..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source "$HOME/.cargo/env"
fi

# Add wasm target
echo "🎯 Adding wasm32 target..."
rustup target add wasm32-unknown-unknown || true

# Install DFX if not present
if ! command -v dfx >/dev/null 2>&1; then
  echo "🔧 Installing DFX..."
  sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
fi

echo "✅ DFX version: $(dfx --version)"

# Navigate to project directory
cd /mnt/d/internship/lauv

# Start local replica
echo "🌐 Starting local IC replica..."
dfx start --clean --background

# Deploy canisters
echo "📡 Deploying registry canister..."
dfx deploy registry

echo "💾 Deploying storage canister..."
dfx deploy storage

# Configure storage
REG=$(dfx canister id registry)
echo "🔗 Configuring storage to use registry: $REG"
dfx canister call storage set_registry "(principal \"$REG\")"

# Output canister IDs as JSON for PowerShell consumption
echo "🎉 Deployment complete!"
jq -n \
  --arg registry "$REG" \
  --arg storage "$(dfx canister id storage)" \
  '{registry: $registry, storage: $storage}'