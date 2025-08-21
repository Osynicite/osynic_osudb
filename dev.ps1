# Development script for Windows PowerShell
param(
    [Parameter(Position=0)]
    [string]$Command = "help"
)

function Show-Help {
    Write-Host "Available commands:" -ForegroundColor Green
    Write-Host "  quick      - Run quick checks (check + fmt-check + clippy)" -ForegroundColor Cyan
    Write-Host "  check      - Run cargo check" -ForegroundColor Yellow
    Write-Host "  fmt        - Format the code" -ForegroundColor Yellow
    Write-Host "  fmt-check  - Check formatting" -ForegroundColor Yellow
    Write-Host "  clippy     - Run clippy" -ForegroundColor Yellow
    Write-Host "  test       - Run tests" -ForegroundColor Yellow
    Write-Host "  test-fast  - Run tests without features" -ForegroundColor Yellow
    Write-Host "  all        - Run all checks (equivalent to CI)" -ForegroundColor Yellow
    Write-Host "  pre-commit - Run pre-commit checks (quick + test)" -ForegroundColor Cyan
    Write-Host "  clean      - Clean build artifacts" -ForegroundColor Yellow
    Write-Host "  update     - Update dependencies" -ForegroundColor Yellow
    Write-Host "  release    - Build for release" -ForegroundColor Yellow
    Write-Host "  docs       - Generate documentation" -ForegroundColor Yellow
    Write-Host "  audit      - Run security audit" -ForegroundColor Yellow
}

function Run-Check {
    Write-Host "Running cargo check..." -ForegroundColor Blue
    cargo check --all-features
}

function Run-Format {
    Write-Host "Formatting code..." -ForegroundColor Blue
    cargo fmt --all
}

function Run-FormatCheck {
    Write-Host "Checking code formatting..." -ForegroundColor Blue
    cargo fmt --all -- --check
}

function Run-Clippy {
    Write-Host "Running clippy..." -ForegroundColor Blue
    cargo clippy --all-features -- -D warnings
}

function Run-Test {
    Write-Host "Running tests..." -ForegroundColor Blue
    cargo test --all-features
}

function Run-TestFast {
    Write-Host "Running fast tests..." -ForegroundColor Blue
    cargo test
}

function Run-Quick {
    Write-Host "Running quick checks..." -ForegroundColor Magenta
    
    Write-Host "1/3 Checking compilation..." -ForegroundColor Blue
    cargo check --all-features
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    
    Write-Host "2/3 Checking code formatting..." -ForegroundColor Blue
    cargo fmt --all -- --check
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    
    Write-Host "3/3 Running clippy..." -ForegroundColor Blue
    cargo clippy --all-features -- -D warnings
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    
    Write-Host "Quick checks passed!" -ForegroundColor Green
}

function Run-PreCommit {
    Write-Host "Running pre-commit checks..." -ForegroundColor Magenta
    
    Run-Quick
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    
    Write-Host "Running tests..." -ForegroundColor Blue
    cargo test --all-features
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    
    Write-Host "Pre-commit checks passed!" -ForegroundColor Green
}

function Run-All {
    Write-Host "Running all checks..." -ForegroundColor Blue
    Run-Check
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    
    Run-FormatCheck
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    
    Run-Clippy
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    
    Run-Test
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    
    Write-Host "All checks passed!" -ForegroundColor Green
}

function Run-Clean {
    Write-Host "Cleaning build artifacts..." -ForegroundColor Blue
    cargo clean
}

function Run-Update {
    Write-Host "Updating dependencies..." -ForegroundColor Blue
    cargo update
}

function Run-Release {
    Write-Host "Building for release..." -ForegroundColor Blue
    cargo build --release
}

function Run-Docs {
    Write-Host "Generating documentation..." -ForegroundColor Blue
    cargo doc --no-deps --open
}

function Run-Audit {
    Write-Host "Running security audit..." -ForegroundColor Blue
    # 检查是否安装了 cargo-audit
    if (!(Get-Command "cargo-audit" -ErrorAction SilentlyContinue)) {
        Write-Host "Installing cargo-audit..." -ForegroundColor Yellow
        cargo install cargo-audit
    }
    cargo audit
}

switch ($Command.ToLower()) {
    "quick" { Run-Quick }
    "check" { Run-Check }
    "fmt" { Run-Format }
    "fmt-check" { Run-FormatCheck }
    "clippy" { Run-Clippy }
    "test" { Run-Test }
    "test-fast" { Run-TestFast }
    "pre-commit" { Run-PreCommit }
    "all" { Run-All }
    "clean" { Run-Clean }
    "update" { Run-Update }
    "release" { Run-Release }
    "docs" { Run-Docs }
    "audit" { Run-Audit }
    "help" { Show-Help }
    default { 
        Write-Host "Unknown command: $Command" -ForegroundColor Red
        Show-Help 
    }
}
