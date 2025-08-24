# Full stack development script for Lauv CDN
# Deploys canisters in WSL and starts frontend

param(
    [string]$WSLDistro = ""
)

Write-Host "🚀 Starting Lauv CDN full stack..." -ForegroundColor Green

# Auto-detect Ubuntu WSL distro if not specified
if (-not $WSLDistro) {
    $distros = (wsl -l -q) -split "`n" | ForEach-Object { $_.Trim() } | Where-Object { $_ -ne '' }
    $WSLDistro = $distros | Where-Object { $_ -like 'Ubuntu*' } | Select-Object -First 1
    if (-not $WSLDistro) { 
        Write-Error '❌ No Ubuntu WSL distro found. Install with: wsl --install -d Ubuntu'
        exit 1 
    }
}

Write-Host "🐧 Using WSL distro: $WSLDistro" -ForegroundColor Cyan

# Bootstrap and deploy in WSL
Write-Host "📡 Bootstrapping DFX and deploying canisters..." -ForegroundColor Yellow
$bootstrapCmd = "/bin/bash -lc 'chmod +x /mnt/d/internship/lauv/scripts/bootstrap_wsl.sh && /mnt/d/internship/lauv/scripts/bootstrap_wsl.sh'"
$output = wsl -d $WSLDistro $bootstrapCmd

Write-Host $output

# Parse canister IDs from JSON output
try {
    $canisterInfo = $output | ConvertFrom-Json
    $env:VITE_REGISTRY_CANISTER_ID = $canisterInfo.registry
    $env:VITE_STORAGE_CANISTER_ID = $canisterInfo.storage
    
    Write-Host "✅ Canister IDs configured:" -ForegroundColor Green
    Write-Host "   Registry: $($env:VITE_REGISTRY_CANISTER_ID)" -ForegroundColor Cyan
    Write-Host "   Storage: $($env:VITE_STORAGE_CANISTER_ID)" -ForegroundColor Cyan
} catch {
    Write-Host "⚠️  Failed to parse canister IDs. Output was:" -ForegroundColor Yellow
    Write-Host $output
    throw "Could not extract canister IDs from deployment output"
}

# Update frontend .env file
$envContent = @"
VITE_STORAGE_CANISTER_ID=$($env:VITE_STORAGE_CANISTER_ID)
VITE_REGISTRY_CANISTER_ID=$($env:VITE_REGISTRY_CANISTER_ID)
VITE_DFX_NETWORK=local
VITE_HOST=http://127.0.0.1:4943
"@

$envContent | Out-File -FilePath "frontend/.env.local" -Encoding UTF8
Write-Host "📝 Updated frontend/.env.local" -ForegroundColor Green

# Start frontend
Write-Host "🌐 Starting frontend development server..." -ForegroundColor Yellow
Push-Location frontend
try {
    if (-not (Test-Path "node_modules")) {
        Write-Host "📦 Installing frontend dependencies..." -ForegroundColor Yellow
        npm install
    }
    Write-Host "🎯 Frontend ready at http://localhost:5173" -ForegroundColor Green
    npm run dev
} finally {
    Pop-Location
}