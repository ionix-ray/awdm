#!/usr/bin/env bash
# simulate-github-action.sh
# Simulates the GitHub Actions pipeline locally to save CI/CD time and catch errors early
# Usage: ./simulate-github-action.sh [--force-refresh] [--zen-mode] [--skip-ai]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$ROOT_DIR"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Flags
FORCE_REFRESH=false
AI_MODE="go"
SKIP_AI=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --force-refresh)
            FORCE_REFRESH=true
            shift
            ;;
        --zen-mode)
            AI_MODE="zen"
            shift
            ;;
        --skip-ai)
            SKIP_AI=true
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --force-refresh    Clear cache and fetch fresh data"
            echo "  --zen-mode         Use minimal AI validation (faster)"
            echo "  --skip-ai          Skip AI validation step entirely"
            echo "  -h, --help         Show this help message"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            exit 1
            ;;
    esac
done

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  AI Model Tracker - Local Simulation ${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# Track timing
START_TIME=$(date +%s)

# Step 1: Validate Registry
echo -e "${YELLOW}[Step 1/5] Validating registry...${NC}"
if bash .github/scripts/validate-registry.sh; then
    echo -e "${GREEN}✓ Registry validation passed${NC}"
else
    echo -e "${RED}✗ Registry validation failed${NC}"
    exit 1
fi
echo ""

# Step 2: Fetch Model Data
echo -e "${YELLOW}[Step 2/5] Fetching model data...${NC}"
if [[ "$FORCE_REFRESH" == "true" ]]; then
    echo "Force refresh enabled, clearing cache..."
    rm -rf .cache
fi

export HF_API_BASE="https://huggingface.co/api"
export HF_RATE_LIMIT_PER_HOUR=50

if bash .github/scripts/fetch-model-data.sh; then
    echo -e "${GREEN}✓ Data fetch completed${NC}"
else
    echo -e "${RED}✗ Data fetch failed${NC}"
    exit 1
fi
echo ""

# Step 3: AI Validation (optional)
if [[ "$SKIP_AI" != "true" ]]; then
    echo -e "${YELLOW}[Step 3/5] Running AI validation (mode: $AI_MODE)...${NC}"
    
    # Check if token is available
    if [[ -z "${OPENCODE_API_TOKEN:-}" ]]; then
        echo -e "${YELLOW}⚠ OPENCODE_API_TOKEN not set, skipping AI validation${NC}"
        echo "   Set it with: export OPENCODE_API_TOKEN=your_token"
    else
        export AI_MODE
        export CONFIG_FILE="tracker/config.toml"
        
        if bash .github/scripts/ai-validation/ai-validate-models.sh tracker/registry; then
            echo -e "${GREEN}✓ AI validation completed${NC}"
        else
            echo -e "${YELLOW}⚠ AI validation had issues (non-fatal)${NC}"
        fi
    fi
else
    echo -e "${YELLOW}[Step 3/5] Skipping AI validation (as requested)${NC}"
fi
echo ""

# Step 4: Generate JSON
echo -e "${YELLOW}[Step 4/5] Generating model-status.json...${NC}"
if bash .github/scripts/generate-model-json.sh; then
    echo -e "${GREEN}✓ JSON generation completed${NC}"
    
    # Validate generated JSON
    if command -v python3 &> /dev/null; then
        echo "Validating JSON schema..."
        if python3 -c "import json; json.load(open('public/model-status.json'))"; then
            echo -e "${GREEN}✓ Generated JSON is valid${NC}"
            
            # Show summary
            echo ""
            echo -e "${BLUE}Generated Data Summary:${NC}"
            python3 -c "
import json
with open('public/model-status.json') as f:
    data = json.load(f)
    
print(f\"  Total Models: {len(data.get('models', []))}\")
print(f\"  Free Models: {sum(1 for m in data.get('models', []) if m.get('isFree', False))}\")
print(f\"  Last Updated: {data.get('metadata', {}).get('lastUpdated', 'N/A')}\")
print(f\"  Sources: {', '.join(data.get('metadata', {}).get('sources', []))}\")
"
        else
            echo -e "${RED}✗ Generated JSON is invalid${NC}"
            exit 1
        fi
    fi
else
    echo -e "${RED}✗ JSON generation failed${NC}"
    exit 1
fi
echo ""

# Step 5: Build Frontend (if pnpm available)
echo -e "${YELLOW}[Step 5/5] Building frontend...${NC}"
if command -v pnpm &> /dev/null; then
    cd frontend
    
    echo "Installing dependencies..."
    if pnpm install --frozen-lockfile; then
        echo "Building..."
        if pnpm run build; then
            echo -e "${GREEN}✓ Frontend build completed${NC}"
        else
            echo -e "${RED}✗ Frontend build failed${NC}"
            cd ..
            exit 1
        fi
    else
        echo -e "${YELLOW}⚠ Frontend dependencies installation failed (may need setup)${NC}"
    fi
    
    cd ..
else
    echo -e "${YELLOW}⚠ pnpm not found, skipping frontend build${NC}"
    echo "   Install pnpm: curl -fsSL https://get.pnpm.io/install.sh | sh -"
fi
echo ""

# Calculate elapsed time
END_TIME=$(date +%s)
ELAPSED=$((END_TIME - START_TIME))

echo -e "${BLUE}========================================${NC}"
echo -e "${GREEN}✓ Simulation completed successfully!${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
echo "Elapsed time: ${ELAPSED}s"
echo ""
echo "Estimated GitHub Actions runtime: ~$((ELAPSED + 60))s (includes checkout, setup overhead)"
echo ""
echo "Output files:"
echo "  - public/model-status.json"
echo "  - tracker/metadata/*.json"
echo "  - tracker/registry/*.json"
echo ""

if [[ -d "frontend/dist" ]]; then
    echo "Frontend build:"
    echo "  - frontend/dist/"
fi

echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo "1. Review the generated JSON: cat public/model-status.json | jq"
echo "2. Test locally: cd frontend && pnpm dev"
echo "3. Commit and push to trigger GitHub Actions"
echo ""
