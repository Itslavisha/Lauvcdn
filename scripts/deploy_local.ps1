param(
    [string]$AdminPrincipal = "$(dfx identity get-principal)"
)

# Start local replica
try { dfx stop | Out-Null } catch {}

dfx start --clean --background

# Deploy canisters
$canisters = @('registry','storage_us','storage_eu','router','billing','analytics')
foreach ($c in $canisters) {
    dfx deploy $c
}

# Configure canister relationships
$REG = dfx canister id registry
dfx canister call storage_us set_registry "(principal \`"$REG\`")"
dfx canister call storage_us set_region '("us")'
dfx canister call storage_eu set_region '("eu")' 2>$null || Write-Host "storage_eu doesn't have set_region method"
dfx canister call router set_registry "(principal \`"$REG\`")" 2>$null || Write-Host "router set_registry failed"

# Initialize billing admin
try {
    dfx canister call billing recharge "(principal \`"$AdminPrincipal\`", 1000000000000)" 
} catch {
    Write-Host "Billing recharge failed: $_"
}

# Output canister IDs for frontend
Write-Host "Canister IDs:"
Write-Host "VITE_REGISTRY_CANISTER_ID=$(dfx canister id registry)"
Write-Host "VITE_STORAGE_US_CANISTER_ID=$(dfx canister id storage_us)"
Write-Host "VITE_STORAGE_EU_CANISTER_ID=$(dfx canister id storage_eu)"
Write-Host "VITE_ROUTER_CANISTER_ID=$(dfx canister id router)"
Write-Host "VITE_BILLING_CANISTER_ID=$(dfx canister id billing)"
Write-Host "VITE_ANALYTICS_CANISTER_ID=$(dfx canister id analytics)"
