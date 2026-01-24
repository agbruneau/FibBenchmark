# Fibonacci Benchmark Suite - Run All Benchmarks
# PowerShell Script for Windows

param(
    [string]$Filter = "",
    [switch]$SaveBaseline
)

$ErrorActionPreference = "Stop"

Write-Host "[BENCH] Fibonacci Benchmark Suite" -ForegroundColor Cyan
Write-Host "==================================" -ForegroundColor Cyan
Write-Host ""

# Create timestamp for report directory
$timestamp = Get-Date -Format "yyyy-MM-dd_HH-mm-ss"
$reportDir = "reports\$timestamp"
$criterionDir = "$reportDir\criterion"

Write-Host "[DIR] Creating report directory: $reportDir" -ForegroundColor Yellow
New-Item -ItemType Directory -Force -Path $criterionDir | Out-Null

# Build release first
Write-Host "[BUILD] Building in release mode..." -ForegroundColor Yellow
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "[ERROR] Build failed!" -ForegroundColor Red
    exit 1
}

# Run benchmarks
Write-Host "[RUN] Running benchmarks..." -ForegroundColor Yellow
$benchArgs = @()
if ($Filter) {
    $benchArgs += "--", $Filter
}
if ($SaveBaseline) {
    $benchArgs += "--", "--save-baseline", "main"
}

cargo bench @benchArgs
if ($LASTEXITCODE -ne 0) {
    Write-Host "[ERROR] Benchmarks failed!" -ForegroundColor Red
    exit 1
}

# Copy Criterion reports
Write-Host "[COPY] Copying Criterion reports..." -ForegroundColor Yellow
if (Test-Path "target\criterion\report") {
    Copy-Item -Path "target\criterion\report\*" -Destination $criterionDir -Recurse -Force
    Write-Host "   [OK] Criterion reports copied to $criterionDir" -ForegroundColor Green
} else {
    Write-Host "   [WARN] No Criterion reports found" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "[DONE] Benchmarks complete!" -ForegroundColor Green
Write-Host "   Reports saved to: $reportDir" -ForegroundColor Cyan
