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

# Initialize billing admin
try {
    dfx canister call billing init "(principal '$AdminPrincipal')"
} catch {
    Write-Host "Billing init may already be set: $_"
}
