# Fibonacci Benchmark Suite - Generate Report
# PowerShell Script for Windows

param(
    [string]$ReportDir = ""
)

$ErrorActionPreference = "Stop"

Write-Host "[REPORT] Fibonacci Report Generator" -ForegroundColor Cyan
Write-Host "====================================" -ForegroundColor Cyan
Write-Host ""

# Determine report directory
if (-not $ReportDir) {
    $timestamp = Get-Date -Format "yyyy-MM-dd_HH-mm-ss"
    $ReportDir = "reports\$timestamp"
}
$dataDir = "$ReportDir\data"

Write-Host "[DIR] Report directory: $ReportDir" -ForegroundColor Yellow
New-Item -ItemType Directory -Force -Path $dataDir | Out-Null

# Build release first
Write-Host "[BUILD] Building in release mode..." -ForegroundColor Yellow
cargo build --release -p fib-viz
if ($LASTEXITCODE -ne 0) {
    Write-Host "[ERROR] Build failed!" -ForegroundColor Red
    exit 1
}

# Run fib-viz to generate CSV data
Write-Host "[VIZ] Generating visualization data..." -ForegroundColor Yellow
cargo run --release -p fib-viz
if ($LASTEXITCODE -ne 0) {
    Write-Host "[ERROR] Visualization generation failed!" -ForegroundColor Red
    exit 1
}

# Move generated CSVs to report directory
Write-Host "[COPY] Moving data files..." -ForegroundColor Yellow
if (Test-Path "results") {
    Copy-Item -Path "results\*" -Destination $dataDir -Recurse -Force
    Write-Host "   [OK] Data files copied to $dataDir" -ForegroundColor Green
} else {
    Write-Host "   [WARN] No results directory found" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "[DONE] Report generation complete!" -ForegroundColor Green
Write-Host "   Data saved to: $dataDir" -ForegroundColor Cyan
Write-Host ""

# Copy HTML templates with UTF-8 encoding preservation
Write-Host "[HTML] Copying HTML report templates..." -ForegroundColor Yellow
if (Test-Path "templates") {
    # Function to copy files preserving UTF-8 encoding
    function Copy-FileWithUTF8 {
        param(
            [string]$SourcePath,
            [string]$DestPath
        )
        $content = [System.IO.File]::ReadAllText($SourcePath, [System.Text.Encoding]::UTF8)
        $destDir = Split-Path -Parent $DestPath
        if (-not (Test-Path $destDir)) {
            New-Item -ItemType Directory -Force -Path $destDir | Out-Null
        }
        [System.IO.File]::WriteAllText($DestPath, $content, [System.Text.Encoding]::UTF8)
    }
    
    # Copy all template files preserving UTF-8
    Get-ChildItem -Path "templates" -Recurse -File | ForEach-Object {
        $relativePath = $_.FullName.Substring((Resolve-Path "templates").Path.Length + 1)
        $destPath = Join-Path $ReportDir $relativePath
        Copy-FileWithUTF8 -SourcePath $_.FullName -DestPath $destPath
    }
    Write-Host "   [OK] HTML templates copied to $ReportDir" -ForegroundColor Green
} else {
    Write-Host "   [WARN] No templates directory found" -ForegroundColor Yellow
}

# Embed JSON data into HTML for offline viewing
Write-Host "[EMBED] Embedding data into HTML for offline viewing..." -ForegroundColor Yellow
$htmlPath = "$ReportDir\report.html"
if ((Test-Path $htmlPath) -and (Test-Path $dataDir)) {
    $htmlContent = Get-Content $htmlPath -Raw -Encoding UTF8
    
    # Read JSON files with UTF-8 encoding
    $complexityData = ""
    $binetData = ""
    $goldenData = ""
    
    if (Test-Path "$dataDir\complexity_comparison.json") {
        $complexityData = (Get-Content "$dataDir\complexity_comparison.json" -Raw -Encoding UTF8).Trim()
    }
    if (Test-Path "$dataDir\binet_accuracy.json") {
        $binetData = (Get-Content "$dataDir\binet_accuracy.json" -Raw -Encoding UTF8).Trim()
    }
    if (Test-Path "$dataDir\golden_ratio_convergence.json") {
        $goldenData = (Get-Content "$dataDir\golden_ratio_convergence.json" -Raw -Encoding UTF8).Trim()
    }
    
    # Replace placeholder with timestamp
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $htmlContent = $htmlContent -replace "{{TIMESTAMP}}", $timestamp
    
    # Build the data script with proper string concatenation
    # Embed JSON as strings, they will be parsed by JavaScript
    $dataScript = "<script>`n// Embedded data for offline viewing`n"
    $dataScript += "window.EMBEDDED_DATA = {`n"
    
    # Escape JSON strings for JavaScript template literals
    # Need to escape: backticks, backslashes, and dollar signs
    function Escape-ForTemplateLiteral {
        param([string]$text)
        if ([string]::IsNullOrWhiteSpace($text)) { return "" }
        # Use String.Replace for predictable escaping
        # Escape in order: backslashes first, then backticks, then dollar signs
        $escaped = $text.Replace('\', '\\').Replace('`', '\`').Replace('$', '\$')
        return $escaped
    }
    
    $bt = '`'  # backtick for template literals
    if ($complexityData) {
        $escapedComplexity = Escape-ForTemplateLiteral $complexityData
        $dataScript += "    complexity: $bt$escapedComplexity$bt,`n"
    } else {
        $dataScript += "    complexity: `"`",`n"
    }
    if ($binetData) {
        $escapedBinet = Escape-ForTemplateLiteral $binetData
        $dataScript += "    binet: $bt$escapedBinet$bt,`n"
    } else {
        $dataScript += "    binet: `"`",`n"
    }
    if ($goldenData) {
        $escapedGolden = Escape-ForTemplateLiteral $goldenData
        $dataScript += "    golden: $bt$escapedGolden$bt`n"
    } else {
        $dataScript += "    golden: `"`"`n"
    }
    $dataScript += "};`n</script>"
    
    # Insert before closing </head> tag
    $htmlContent = $htmlContent -replace "</head>", "$dataScript`n</head>"
    
    Set-Content -Path $htmlPath -Value $htmlContent -Encoding UTF8
    Write-Host "   [OK] Data embedded into HTML" -ForegroundColor Green
}

Write-Host ""
Write-Host "Generated files:" -ForegroundColor Yellow
Get-ChildItem $dataDir -File | ForEach-Object { Write-Host "   - data\$($_.Name)" -ForegroundColor White }
Get-ChildItem $ReportDir -File -ErrorAction SilentlyContinue | ForEach-Object { Write-Host "   - $($_.Name)" -ForegroundColor White }

Write-Host ""
Write-Host "To view the report, open:" -ForegroundColor Cyan
Write-Host "   $ReportDir\report.html" -ForegroundColor White
