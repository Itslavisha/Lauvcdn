#!/usr/bin/env bash
set -euo pipefail

if ! command -v sudo >/dev/null 2>&1; then
  echo "Please run in a Debian/Ubuntu WSL distro." >&2
  exit 1
fi

sudo apt update
sudo apt install -y curl build-essential pkg-config libssl-dev git jq

if ! command -v cargo >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source "$HOME/.cargo/env"
fi

rustup target add wasm32-unknown-unknown || true

if ! command -v dfx >/dev/null 2>&1; then
  sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
fi

dfx --version

cd /mnt/d/internship/lauv

dfx start --clean --background

dfx deploy registry

dfx deploy storage_us

dfx deploy router

REG=$(dfx canister id registry)
dfx canister call storage_us set_registry "(principal \"$REG\")"
dfx canister call storage_us set_region '("us")'
dfx canister call router set_registry "(principal \"$REG\")"

# Output canister IDs as JSON
jq -n --arg storage_us "$(dfx canister id storage_us)" '{storage_us: $storage_us}'
