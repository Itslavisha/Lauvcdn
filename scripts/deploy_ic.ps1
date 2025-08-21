param(
    [string]$Network = "ic",
    [string]$AdminPrincipal = "$(dfx identity get-principal)"
)

# Ensure identities and wallet are configured for mainnet before running

dfx deploy --network $Network registry

dfx deploy --network $Network storage_us

dfx deploy --network $Network storage_eu

dfx deploy --network $Network router

dfx deploy --network $Network billing

dfx deploy --network $Network analytics

# Initialize billing admin (first time)
try {
    dfx canister --network $Network call billing init "(principal '$AdminPrincipal')"
} catch {
    Write-Host "Billing init may already be set: $_"
}
