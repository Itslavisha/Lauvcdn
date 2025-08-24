# Lauv: Decentralized CDN on ICP

This repo contains ICP canisters (Rust), a React frontend, and SDKs for Lauv's on‑chain CDN.

## Prereqs
- DFX CLI, Rust toolchain, Node.js

## Local development
- Start a local replica: `dfx start --clean --background`
- Deploy canisters: `dfx deploy`
- Frontend (once scaffolded): `cd frontend && npm i && npm run dev`

## Canisters
- registry: File metadata and permissions
- storage: File storage with chunking and HTTP access

See `scripts/deploy_local.ps1` for commands and `dfx.json` for canister config.
