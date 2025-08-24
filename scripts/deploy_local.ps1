param(
    [string]$AdminPrincipal = "$(dfx identity get-principal)"
)

# Start local replica
try { dfx stop | Out-Null } catch {}

dfx start --clean --background

# Deploy canisters
$canisters = @('registry','storage')
foreach ($c in $canisters) {
    dfx deploy $c
}

# Configure canister relationships
$REG = dfx canister id registry
dfx canister call storage set_registry "(principal \`"$REG\`")"

# Output canister IDs for frontend
Write-Host "Canister IDs:"
Write-Host "VITE_REGISTRY_CANISTER_ID=$(dfx canister id registry)"
Write-Host "VITE_STORAGE_CANISTER_ID=$(dfx canister id storage)"
