param(
    [string]$Network = "ic",
    [string]$AdminPrincipal = "$(dfx identity get-principal)"
)

# Ensure identities and wallet are configured for mainnet before running

dfx deploy --network $Network registry

dfx deploy --network $Network storage

# Configure storage
$REG = dfx canister --network $Network id registry
dfx canister --network $Network call storage set_registry "(principal '$REG')"