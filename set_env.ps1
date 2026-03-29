# MicronOS Build Environment Setup
# Esegui QUESTO PRIMA di ogni cargo build/test
# Metti questo nel tuo PowerShell profile per non doverlo eseguire ogni volta

# Tutto su D: (SSD esterno)
$env:CARGO_HOME = "D:\cargo_home"
$env:CARGO_TARGET_DIR = "D:\os\micronos\target"

# Crea directory se non esistono
if (!(Test-Path "D:\cargo_home")) { 
    New-Item -ItemType Directory -Path "D:\cargo_home" -Force | Out-Null 
}
if (!(Test-Path "D:\os\micronos\target")) { 
    New-Item -ItemType Directory -Path "D:\os\micronos\target" -Force | Out-Null 
}

# Per non usare mai ~/.cargo su C:
$env:RUSTUP_HOME = "D:\cargo_home"

Write-Host ""
Write-Host "======================================" -ForegroundColor Cyan
Write-Host " MicronOS Build Environment" -ForegroundColor Cyan
Write-Host "======================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "CARGO_HOME:       $env:CARGO_HOME" -ForegroundColor Yellow
Write-Host "CARGO_TARGET_DIR: $env:CARGO_TARGET_DIR" -ForegroundColor Yellow
Write-Host "RUSTUP_HOME:      $env:RUSTUP_HOME" -ForegroundColor Yellow
Write-Host ""
Write-Host "TUTTO su D: (SSD esterno)" -ForegroundColor Green
Write-Host "C: (HDD) NON toccato" -ForegroundColor Green
Write-Host ""
