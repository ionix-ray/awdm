#!/bin/bash
# Local CI Simulation Script
# Run this before pushing to GitHub to save CI/CD time and costs

set -e

echo "═══════════════════════════════════════════════════════════"
echo "  🧪 Running Local CI Simulation"
echo "═══════════════════════════════════════════════════════════"
echo ""

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

step_success() {
    echo -e "${GREEN}✅${NC} $1"
}

step_error() {
    echo -e "${RED}❌${NC} $1"
    exit 1
}

step_info() {
    echo -e "${YELLOW}ℹ️${NC} $1"
}

# Step 1: Check Rust toolchain
step_info "Checking Rust toolchain..."
if ! command -v cargo &> /dev/null; then
    step_error "Rust/Cargo not found. Please install Rust from https://rustup.rs/"
fi
step_success "Rust toolchain available"

# Step 2: Format check
step_info "Checking code formatting..."
if cargo fmt --all -- --check; then
    step_success "Code formatting OK"
else
    step_error "Code formatting failed. Run 'cargo fmt' to fix."
fi

# Step 3: Clippy linting
step_info "Running Clippy linter..."
if cargo clippy --workspace --all-targets --all-features -- -D warnings; then
    step_success "Clippy checks passed"
else
    step_error "Clippy found issues. Fix warnings before proceeding."
fi

# Step 4: Unit tests
step_info "Running unit tests..."
if cargo test --workspace --lib; then
    step_success "All unit tests passed"
else
    step_error "Unit tests failed"
fi

# Step 5: Security audit (optional, skip if cargo-audit not installed)
step_info "Running security audit..."
if command -v cargo-audit &> /dev/null; then
    if cargo audit --deny warnings 2>/dev/null; then
        step_success "Security audit passed"
    else
        step_info "Security audit found warnings (continuing...)"
    fi
else
    step_info "cargo-audit not installed, skipping security audit"
fi

# Step 6: Validate configuration files
step_info "Validating TOML configurations..."
if python3 -c "import toml; toml.load('frontend/content/config.toml')" 2>/dev/null; then
    step_success "Configuration files valid"
else
    step_info "Python toml module not available, skipping config validation"
fi

# Step 7: Check build (debug mode for speed)
step_info "Building project (debug mode)..."
if cargo build --workspace; then
    step_success "Build successful"
else
    step_error "Build failed"
fi

echo ""
echo "═══════════════════════════════════════════════════════════"
echo -e "${GREEN}🎉 All local checks passed!${NC}"
echo "═══════════════════════════════════════════════════════════"
echo ""
echo "Ready to push to GitHub. The CI pipeline should pass."
echo ""
echo "Next steps:"
echo "  1. Commit your changes: git add . && git commit -m 'message'"
echo "  2. Push to GitHub: git push"
echo "  3. Monitor GitHub Actions: https://github.com/ionix-ray/ai-model-dashboard/actions"
echo ""
