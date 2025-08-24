param(
    [string]$AdminPrincipal = "$(dfx identity get-principal)"
)

Write-Host "Deploying Lauv CDN locally..." -ForegroundColor Green

# Stop any existing replica
try { dfx stop | Out-Null } catch {}

# Start fresh local replica
dfx start --clean --background

# Deploy canisters
Write-Host "Deploying registry canister..." -ForegroundColor Yellow
dfx deploy registry

Write-Host "Deploying storage canister..." -ForegroundColor Yellow
dfx deploy storage

# Configure storage to use registry
$REGISTRY_ID = dfx canister id registry
Write-Host "Configuring storage to use registry: $REGISTRY_ID" -ForegroundColor Yellow
dfx canister call storage set_registry "(principal \`"$REGISTRY_ID\`")"

# Output canister IDs for frontend
Write-Host "`nDeployment complete! 🎉" -ForegroundColor Green
Write-Host "Canister IDs for frontend .env:" -ForegroundColor Cyan
Write-Host "VITE_REGISTRY_CANISTER_ID=$REGISTRY_ID"
Write-Host "VITE_STORAGE_CANISTER_ID=$(dfx canister id storage)"
Write-Host "`nLocal CDN is ready at: http://127.0.0.1:4943" -ForegroundColor Green