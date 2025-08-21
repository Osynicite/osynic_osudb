# Development Guide

This guide outlines the development workflow and quality checks for the project.

## CI/CD Pipeline

The project includes an optimized CI/CD pipeline designed for efficiency and energy conservation:

### Main CI Pipeline (`.github/workflows/ci.yml`)
- **Quick Checks**: Combines compilation, formatting, and clippy checks in a single job
- **Cross-platform Testing**: Only runs on Windows/macOS when explicitly needed
- **Beta Rust Testing**: Only runs on main branch pushes
- **Documentation**: Only generates on main branch pushes
- **Coverage**: Only runs when commit message contains `[coverage]`
- **Path-based Filtering**: Skips CI for documentation-only changes

### Release Pipeline (`.github/workflows/release.yml`)
- **Triggered on Tags**: Only runs for version releases
- **Full Testing Matrix**: Comprehensive testing across all platforms and Rust versions
- **Complete Coverage**: Generates full documentation and coverage reports

### Maintenance Pipeline (`.github/workflows/maintenance.yml`)
- **Weekly Schedule**: Runs dependency audits and checks weekly
- **Security Audits**: Automated vulnerability scanning
- **Dependency Updates**: Checks for outdated packages
- **License Validation**: Ensures license compliance

## Energy-Efficient Development

### Local Development Workflow

#### Quick Feedback Loop (Recommended for daily development)
```bash
# PowerShell
.\dev.ps1 quick

# Unix/Linux/macOS
make quick
```

This runs the essential checks quickly:
1. Compilation check
2. Code formatting check
3. Clippy analysis

#### Pre-commit Checks
```bash
# PowerShell
.\dev.ps1 pre-commit

# Unix/Linux/macOS
make pre-commit
```

This runs quick checks plus tests - recommended before committing.

#### Full CI Simulation
```bash
# PowerShell
.\dev.ps1 all

# Unix/Linux/macOS
make all
```

Only use this when you need to simulate the complete CI pipeline locally.

## Local Development

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Available Commands

#### Using PowerShell Script (Windows)

```powershell
# Quick development cycle
.\dev.ps1 quick        # Fast checks (check + fmt + clippy)
.\dev.ps1 pre-commit   # Pre-commit checks (quick + test)

# Individual checks
.\dev.ps1 check        # Compile check
.\dev.ps1 fmt          # Format code
.\dev.ps1 fmt-check    # Check formatting
.\dev.ps1 clippy       # Run clippy
.\dev.ps1 test         # Run tests with all features
.\dev.ps1 test-fast    # Run tests without features (faster)

# Complete workflow
.\dev.ps1 all          # Run all checks (equivalent to CI)

# Maintenance
.\dev.ps1 clean        # Clean build artifacts
.\dev.ps1 update       # Update dependencies
.\dev.ps1 audit        # Security audit
.\dev.ps1 docs         # Generate documentation
```

#### Using Makefile (Unix/Linux/macOS)

```bash
# Quick development cycle
make quick        # Fast checks (check + fmt + clippy)
make pre-commit   # Pre-commit checks (quick + test)

# Individual checks
make check        # Compile check
make fmt          # Format code
make fmt-check    # Check formatting
make clippy       # Run clippy
make test         # Run tests with all features
make test-fast    # Run tests without features (faster)

# Complete workflow
make all          # Run all checks (equivalent to CI)

# Maintenance
make clean        # Clean build artifacts
make update       # Update dependencies
make audit        # Security audit
make docs         # Generate documentation
```

## Triggering Full CI

### Cross-platform Testing
Add the `test-all` label to your PR to trigger Windows and macOS testing.

### Coverage Reports
Include `[coverage]` in your commit message to generate code coverage reports.

### Documentation Generation
Documentation is automatically generated on pushes to the main branch.

## Code Quality Standards

### Formatting
- Use `rustfmt` with default settings
- All code must be formatted before committing
- Run `cargo fmt --all` to format your code

### Linting
- Use `clippy` for static analysis
- All clippy warnings are treated as errors in CI
- Fix clippy warnings or use `#[allow(...)]` with justification

### Testing
- Write tests for new functionality
- Use `make test-fast` for quick iteration during development
- Use `make test` for full feature testing before committing

### Security
- Run `cargo audit` regularly to check for vulnerabilities
- Dependencies are automatically audited weekly via CI

## Efficient Development Workflow

### Daily Development
1. Make your changes
2. Run `make quick` or `.\dev.ps1 quick` for fast feedback
3. Fix any issues
4. Iterate quickly

### Before Committing
1. Run `make pre-commit` or `.\dev.ps1 pre-commit`
2. Fix any failures
3. Commit your changes

### Before Releasing
1. Run `make all` or `.\dev.ps1 all` locally
2. Create a tag to trigger the full release pipeline
3. Monitor the release workflow

## Performance Tips

- Use `test-fast` during development for quicker iteration
- Only run full cross-platform tests when necessary
- Use shared cache keys in CI for faster builds
- Leverage path-based filtering to skip unnecessary CI runs

## Troubleshooting

### Common Issues

1. **Compilation errors**: Run `cargo clean` and try again
2. **Formatting issues**: Run `cargo fmt --all` to auto-fix
3. **Clippy warnings**: Address warnings or add `#[allow(...)]` with justification
4. **Test failures**: Check test output and fix the underlying issues

### Getting Help

If you encounter issues:

1. Check the CI logs for detailed error messages
2. Refer to the [Rust documentation](https://doc.rust-lang.org/)
3. Check [clippy lint descriptions](https://rust-lang.github.io/rust-clippy/)

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes using the efficient development workflow
4. Run pre-commit checks locally
5. Submit a pull request

All pull requests must pass the CI pipeline before merging. Use labels and commit message tags to control CI behavior when needed.
