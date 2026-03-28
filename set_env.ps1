# MicronOS Build Environment Setup
# Run this BEFORE building to use external SSD (D:)

$env:CARGO_TARGET_DIR = "D:\os\micronos\target"

# Create directory if it doesn't exist
if (!(Test-Path "D:\os\micronos\target")) { 
    New-Item -ItemType Directory -Path "D:\os\micronos\target" | Out-Null 
}

Write-Host ""
Write-Host "======================================" -ForegroundColor Cyan
Write-Host " MicronOS Build Environment" -ForegroundColor Cyan
Write-Host "======================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "CARGO_TARGET_DIR: $env:CARGO_TARGET_DIR" -ForegroundColor Yellow
Write-Host ""
Write-Host "Build artifacts on D: (SSD esterno)" -ForegroundColor Green
Write-Host "C: (HDD primario) NON usato per build" -ForegroundColor Green
Write-Host ""
