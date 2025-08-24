param(
    [string]$Network = "ic",
    [string]$AdminPrincipal = "$(dfx identity get-principal)"
)

Write-Host "🚀 Deploying Lauv CDN to Internet Computer..." -ForegroundColor Green
Write-Host "Network: $Network" -ForegroundColor Cyan
Write-Host "Admin Principal: $AdminPrincipal" -ForegroundColor Cyan

# Ensure we have the right identity and wallet configured
Write-Host "⚠️  Make sure you have:" -ForegroundColor Yellow
Write-Host "   1. Configured your identity for mainnet deployment" -ForegroundColor Yellow
Write-Host "   2. Sufficient cycles in your wallet" -ForegroundColor Yellow
Write-Host "   3. Reviewed the deployment configuration" -ForegroundColor Yellow

Read-Host "Press Enter to continue or Ctrl+C to abort"

# Deploy registry
Write-Host "📡 Deploying registry canister..." -ForegroundColor Yellow
dfx deploy --network $Network registry

# Deploy storage
Write-Host "💾 Deploying storage canister..." -ForegroundColor Yellow
dfx deploy --network $Network storage

# Configure storage to use registry
$REGISTRY_ID = dfx canister --network $Network id registry
Write-Host "🔗 Configuring storage to use registry: $REGISTRY_ID" -ForegroundColor Yellow
dfx canister --network $Network call storage set_registry "(principal '$REGISTRY_ID')"

# Output final information
Write-Host "`n🎉 Deployment to $Network complete!" -ForegroundColor Green
Write-Host "Canister IDs:" -ForegroundColor Cyan
Write-Host "  Registry: $REGISTRY_ID"
Write-Host "  Storage: $(dfx canister --network $Network id storage)"

if ($Network -eq "ic") {
    Write-Host "`n🌐 Your CDN is live on the Internet Computer!" -ForegroundColor Green
    Write-Host "Access via: https://$(dfx canister --network $Network id storage).ic0.app" -ForegroundColor Cyan
}