# Ensure WSL Ubuntu is installed and accessible
# Usage: .\scripts\run_full_stack.ps1

$WSL_DISTRO = "Ubuntu"

Write-Host "Bootstrapping DFX and deploying canisters in WSL ($WSL_DISTRO)..."
$cmd = "/bin/bash -lc 'chmod +x /mnt/d/internship/lauv/scripts/bootstrap_wsl.sh && /mnt/d/internship/lauv/scripts/bootstrap_wsl.sh'"
$out = wsl -d $WSL_DISTRO $cmd
Write-Host $out

try {
  $json = $out | ConvertFrom-Json
} catch {
  Write-Host "Failed to parse canister IDs. Output was:" -ForegroundColor Yellow
  Write-Host $out
  throw
}

$env:VITE_STORAGE_US_CANISTER_ID = $json.storage_us
Write-Host "Using STORAGE_US_CANISTER_ID=$($env:VITE_STORAGE_US_CANISTER_ID)"

Push-Location frontend
npm i
npm run dev
Pop-Location
