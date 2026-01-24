# Setup Go Environment for FibBenchmark (Windows PowerShell)
# This script helps set up the Go + CGO environment for the Rust-Go bridge

Write-Host "🔧 FibBenchmark - Go Setup Script" -ForegroundColor Cyan
Write-Host "==================================" -ForegroundColor Cyan
Write-Host ""

# Check if Go is installed
Write-Host "📦 Checking Go installation..." -ForegroundColor Yellow
$goVersion = go version 2>$null
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Go found: $goVersion" -ForegroundColor Green
} else {
    Write-Host "❌ Go not found. Please install Go from https://golang.org/dl/" -ForegroundColor Red
    Write-Host ""
    Write-Host "Installation options:" -ForegroundColor Yellow
    Write-Host "  1. Download from https://golang.org/dl/"
    Write-Host "  2. Via Chocolatey: choco install golang"
    Write-Host "  3. Via winget: winget install -e --id GoLang.Go"
    exit 1
}

# Check if GCC is installed (required for CGO)
Write-Host ""
Write-Host "📦 Checking GCC installation (required for CGO)..." -ForegroundColor Yellow
$gccVersion = gcc --version 2>$null
if ($LASTEXITCODE -eq 0) {
    $gccFirstLine = ($gccVersion | Select-Object -First 1)
    Write-Host "✅ GCC found: $gccFirstLine" -ForegroundColor Green
} else {
    Write-Host "❌ GCC not found. CGO requires GCC (MinGW-w64 on Windows)." -ForegroundColor Red
    Write-Host ""
    Write-Host "Installation options:" -ForegroundColor Yellow
    Write-Host "  1. Via Chocolatey: choco install mingw"
    Write-Host "  2. Download from https://www.mingw-w64.org/"
    Write-Host "  3. Via MSYS2: https://www.msys2.org/"
    Write-Host ""
    Write-Host "After installing, make sure gcc is in your PATH." -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Note: Without GCC, fib-go will use a Rust stub implementation." -ForegroundColor Cyan
}

# Check CGO_ENABLED
Write-Host ""
Write-Host "📦 Checking CGO configuration..." -ForegroundColor Yellow
$cgoEnabled = $env:CGO_ENABLED
if ($cgoEnabled -eq "1") {
    Write-Host "✅ CGO_ENABLED=1" -ForegroundColor Green
} elseif ($cgoEnabled -eq "0") {
    Write-Host "⚠️  CGO_ENABLED=0 (CGO is disabled)" -ForegroundColor Yellow
    Write-Host "   To enable: `$env:CGO_ENABLED = '1'" -ForegroundColor Yellow
} else {
    Write-Host "ℹ️  CGO_ENABLED not set (defaults to enabled if GCC is available)" -ForegroundColor Cyan
}

# Try to build the Go library
Write-Host ""
Write-Host "🔨 Testing Go library build..." -ForegroundColor Yellow

$goDir = Join-Path $PSScriptRoot "..\crates\fib-go\go"
$outDir = Join-Path $PSScriptRoot "..\target\go-test"

if (!(Test-Path $outDir)) {
    New-Item -ItemType Directory -Path $outDir | Out-Null
}

$libPath = Join-Path $outDir "libfibgo.a"

Push-Location $goDir
try {
    $env:CGO_ENABLED = "1"
    $result = go build -buildmode=c-archive -o $libPath fib.go 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Go library built successfully!" -ForegroundColor Green
        Write-Host "   Output: $libPath" -ForegroundColor Gray
        
        # Clean up test build
        Remove-Item $libPath -ErrorAction SilentlyContinue
        Remove-Item ($libPath -replace '\.a$', '.h') -ErrorAction SilentlyContinue
    } else {
        Write-Host "❌ Go library build failed:" -ForegroundColor Red
        Write-Host $result -ForegroundColor Red
    }
} finally {
    Pop-Location
}

# Summary
Write-Host ""
Write-Host "📋 Summary" -ForegroundColor Cyan
Write-Host "─────────────────────────────────────" -ForegroundColor Cyan

if ($goVersion -and ($gccVersion -or $LASTEXITCODE -eq 0)) {
    Write-Host "✅ Environment is ready for native Go bridge!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Next steps:" -ForegroundColor Yellow
    Write-Host "  1. cargo build -p fib-go"
    Write-Host "  2. cargo run --bin fib-bench -- compare-go -n 1000"
} else {
    Write-Host "⚠️  Using Rust stub (native Go not available)" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "The Rust stub provides the same API but without actual Go code." -ForegroundColor Cyan
    Write-Host "Install GCC/MinGW-w64 to enable native Go compilation." -ForegroundColor Cyan
}

Write-Host ""
