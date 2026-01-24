# Fibonacci Benchmark Suite - Run All
# PowerShell Script for Windows
# Orchestrator script that runs benchmarks and generates reports

param(
    [string]$Filter = "",
    [switch]$SkipBenchmarks,
    [switch]$SkipReport
)

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

Write-Host "[RUN] Fibonacci Benchmark Suite - Full Run" -ForegroundColor Magenta
Write-Host "===========================================" -ForegroundColor Magenta
Write-Host ""

# Create shared timestamp for this run
$timestamp = Get-Date -Format "yyyy-MM-dd_HH-mm-ss"
$reportDir = "reports\$timestamp"

Write-Host "[TIME] Run timestamp: $timestamp" -ForegroundColor Cyan
Write-Host "[DIR] Report directory: $reportDir" -ForegroundColor Cyan
Write-Host ""

# Step 1: Run benchmarks
if (-not $SkipBenchmarks) {
    Write-Host "========================================" -ForegroundColor Gray
    Write-Host "Step 1/2: Running Benchmarks" -ForegroundColor Yellow
    Write-Host "========================================" -ForegroundColor Gray
    & "$ScriptDir\run_benchmarks.ps1" -Filter $Filter
    Write-Host ""
}

# Step 2: Generate report
if (-not $SkipReport) {
    Write-Host "========================================" -ForegroundColor Gray
    Write-Host "Step 2/2: Generating Report" -ForegroundColor Yellow
    Write-Host "========================================" -ForegroundColor Gray
    & "$ScriptDir\generate_report.ps1" -ReportDir $reportDir
    Write-Host ""
}

Write-Host "========================================" -ForegroundColor Gray
Write-Host "[DONE] All tasks complete!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Gray
Write-Host ""
Write-Host "Reports available at:" -ForegroundColor Cyan
Write-Host "   $reportDir" -ForegroundColor White
Write-Host ""
Write-Host "To view Criterion HTML report:" -ForegroundColor Yellow
Write-Host "   start $reportDir\criterion\index.html" -ForegroundColor White
