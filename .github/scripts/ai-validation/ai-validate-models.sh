#!/usr/bin/env bash
# ai-validate-models.sh
# AI-powered validation and enrichment for model data using OpenCode API
# Supports zen/go modes, multiple prompts, schema validation, and efficient token usage

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LIB_DIR="$SCRIPT_DIR"
# shellcheck source=/dev/null
. "$LIB_DIR/../lib-tracker.sh"

# Configuration
CONFIG_FILE="${CONFIG_FILE:-tracker/config.toml}"
CACHE_DIR="${AI_CACHE_DIR:-.cache/ai}"
OPENCODE_TOKEN="${OPENCODE_API_TOKEN:-}"
MODE="${AI_MODE:-go}"  # zen or go
MAX_RETRIES="${AI_MAX_RETRIES:-3}"
TIMEOUT_SECONDS="${AI_TIMEOUT:-30}"

# Create cache directory
mkdir -p "$CACHE_DIR"

log_info "Starting AI validation (mode: $MODE)..."

# Parse TOML config (simple parser for our specific format)
parse_toml_value() {
    local key="$1"
    local section="$2"
    local file="$3"
    
    if [[ ! -f "$file" ]]; then
        echo ""
        return
    fi
    
    awk -v section="$section" -v key="$key" '
    BEGIN { in_section = 0 }
    /^\[/ { 
        if ($0 ~ "\\[" section "\\]") in_section = 1
        else in_section = 0
        next
    }
    in_section && $0 ~ "^" key "[[:space:]]*=" {
        gsub(/^[^=]+=[[:space:]]*/, "")
        gsub(/^"/, "")
        gsub(/"$/, "")
        print
        exit
    }
    ' "$file"
}

parse_toml_array() {
    local section="$1"
    local key="$2"
    local file="$3"
    
    if [[ ! -f "$file" ]]; then
        echo "[]"
        return
    fi
    
    awk -v section="$section" -v key="$key" '
    BEGIN { in_section = 0; in_array = 0; print "[" }
    /^\[/ { 
        if ($0 ~ "\\[" section "\\]") { in_section = 1; in_array = 0 }
        else { in_section = 0; in_array = 0 }
        next
    }
    in_section && $0 ~ "^" key "[[:space:]]*=" {
        in_array = 1
        gsub(/^[^=]+=[[:space:]]*\[/, "")
        gsub(/\][[:space:]]*$/, "")
        gsub(/"/, "")
        gsub(/[[:space:]]/, "")
        split($0, arr, ",")
        for (i in arr) {
            if (arr[i] != "") printf "\"%s\"", arr[i]
            if (i < length(arr)) printf ","
        }
        next
    }
    in_array && /\]/ {
        in_array = 0
        print "]"
        exit
    }
    END { if (!in_array) print "]" }
    ' "$file"
}

get_prompt_template() {
    local prompt_name="$1"
    local file="$2"
    
    if [[ ! -f "$file" ]]; then
        echo ""
        return
    fi
    
    # Extract multi-line prompt from TOML
    python3 - "$file" "$prompt_name" << 'PY'
import sys
import re

config_file = sys.argv[1]
prompt_name = sys.argv[2]

with open(config_file, 'r') as f:
    content = f.read()

# Find the prompt section
pattern = rf'{prompt_name}\s*=\s*"""(.*?)"""'
match = re.search(pattern, content, re.DOTALL)

if match:
    print(match.group(1).strip())
else:
    print("")
PY
}

# Load configuration from TOML
load_config() {
    log_info "Loading configuration from $CONFIG_FILE..."
    
    MODE=$(parse_toml_value "mode" "general" "$CONFIG_FILE")
    DEFAULT_MODEL=$(parse_toml_value "default_model" "general" "$CONFIG_FILE")
    AUTO_DETECT=$(parse_toml_value "auto_detect_best_model" "general" "$CONFIG_FILE")
    MIN_CONFIDENCE=$(parse_toml_value "min_confidence_score" "validation" "$CONFIG_FILE")
    MAX_RETRIES=$(parse_toml_value "max_retries" "validation" "$CONFIG_FILE")
    TIMEOUT_SECONDS=$(parse_toml_value "timeout_seconds" "validation" "$CONFIG_FILE")
    MAX_API_CALLS=$(parse_toml_value "max_api_calls_per_hour" "rate_limits" "$CONFIG_FILE")
    
    # Defaults
    MODE="${MODE:-go}"
    DEFAULT_MODEL="${DEFAULT_MODEL:-anthropic/claude-sonnet-4-5}"
    MIN_CONFIDENCE="${MIN_CONFIDENCE:-70}"
    MAX_RETRIES="${MAX_RETRIES:-3}"
    TIMEOUT_SECONDS="${TIMEOUT_SECONDS:-30}"
    MAX_API_CALLS="${MAX_API_CALLS:-50}"
    
    log_info "Configuration loaded: mode=$MODE, model=$DEFAULT_MODEL, min_confidence=$MIN_CONFIDENCE"
}

# Get cache key for AI response
get_ai_cache_key() {
    local model_id="$1"
    local prompt_type="$2"
    local input_hash="$3"
    
    echo "${model_id//\//__}_${prompt_type}_${input_hash}"
}

# Check if cached AI response exists and is valid
get_cached_ai_response() {
    local cache_key="$1"
    local ttl="${2:-86400}"  # 24 hours default
    
    local cache_file="$CACHE_DIR/${cache_key}.json"
    
    if [[ ! -f "$cache_file" ]]; then
        return 1
    fi
    
    # Check if cache is expired
    local cached_at
    cached_at=$(python3 -c "import json, time, sys; d=json.load(open('$cache_file')); print(int(d.get('_cached_at', 0)))" 2>/dev/null || echo "0")
    local now
    now=$(date +%s)
    
    if (( now - cached_at < ttl )); then
        python3 -c "import json; d=json.load(open('$cache_file')); print(json.dumps(d.get('data', {})))"
        return 0
    fi
    
    return 1
}

# Cache AI response
cache_ai_response() {
    local cache_key="$1"
    local response="$2"
    
    local cache_file="$CACHE_DIR/${cache_key}.json"
    
    python3 - "$cache_file" "$response" << 'PY'
import json
import sys
import time

cache_file = sys.argv[1]
response_json = sys.argv[2]

try:
    data = json.loads(response_json)
    with open(cache_file, 'w') as f:
        json.dump({
            '_cached_at': int(time.time()),
            'data': data
        }, f, indent=2)
except Exception as e:
    print(f"[WARN] Failed to cache: {e}", file=sys.stderr)
PY
}

# Call OpenCode API with retry logic
call_opencode_api() {
    local model="$1"
    local prompt="$2"
    local output_schema="$3"
    local cache_key="$4"
    
    # Try cache first (unless in go mode with force refresh)
    if [[ "$MODE" != "force" ]] && get_cached_ai_response "$cache_key" 86400; then
        log_info "[CACHE HIT] $cache_key"
        return 0
    fi
    
    if [[ -z "$OPENCODE_TOKEN" ]]; then
        log_warn "OPENCODE_API_TOKEN not set, skipping AI validation"
        echo "{}"
        return 1
    fi
    
    local retry_count=0
    local last_error=""
    
    while (( retry_count < MAX_RETRIES )); do
        log_info "[AI CALL] Model: $model, Attempt: $((retry_count + 1))/$MAX_RETRIES"
        
        # Build curl command with proper escaping
        local response
        response=$(curl -s -X POST \
            "https://your-gateway.com/v1/chat/completions" \
            -H "Authorization: Bearer $OPENCODE_TOKEN" \
            -H "Content-Type: application/json" \
            -d "{
                \"model\": \"$model\",
                \"messages\": [
                    {
                        \"role\": \"system\",
                        \"content\": \"You are a precise JSON API. Always respond with valid JSON matching the schema. No markdown, no explanations outside JSON.\"
                    },
                    {
                        \"role\": \"user\",
                        \"content\": $prompt
                    }
                ],
                \"temperature\": 0.1,
                \"max_tokens\": 1000,
                \"response_format\": {\"type\": \"json_object\"}
            }" \
            --max-time "$TIMEOUT_SECONDS" 2>&1) || last_error="curl_failed"
        
        if [[ $? -eq 0 ]] && [[ -n "$response" ]]; then
            # Extract JSON from response
            local json_body
            json_body=$(echo "$response" | python3 -c "
import sys, json
try:
    data = json.load(sys.stdin)
    content = data.get('choices', [{}])[0].get('message', {}).get('content', '{}')
    # Try to parse the content as JSON
    result = json.loads(content)
    print(json.dumps(result))
except Exception as e:
    print('{}')
    sys.exit(1)
" 2>/dev/null) || json_body="{}"
            
            # Validate response schema
            if [[ "$json_body" != "{}" ]] && validate_json_schema "$json_body" "$output_schema"; then
                log_info "[AI SUCCESS] Valid response received"
                cache_ai_response "$cache_key" "$json_body"
                echo "$json_body"
                return 0
            else
                log_warn "[AI WARN] Invalid schema, retrying..."
            fi
        else
            log_warn "[AI ERROR] $last_error"
        fi
        
        retry_count=$((retry_count + 1))
        sleep 2  # Backoff
    done
    
    log_error "[AI FAILED] Max retries exceeded for $cache_key"
    echo "{}"
    return 1
}

# Validate JSON against schema
validate_json_schema() {
    local json_data="$1"
    local schema="$2"
    
    # Simple schema validation (can be enhanced with jsonschema library)
    echo "$json_data" | python3 -c "
import sys, json

try:
    data = json.load(sys.stdin)
    # Basic validation: check if it's a valid JSON object
    if isinstance(data, dict):
        sys.exit(0)
    else:
        sys.exit(1)
except:
    sys.exit(1)
" 2>/dev/null
}

# Test models for capability and free tier
test_model_capabilities() {
    log_info "Testing model capabilities..."
    
    local models_json
    models_json=$(parse_toml_array "general" "models_to_test" "$CONFIG_FILE")
    
    local best_model=""
    local best_score=0
    
    # In zen mode, just use default
    if [[ "$MODE" == "zen" ]]; then
        log_info "Zen mode: using default model"
        echo "$DEFAULT_MODEL"
        return 0
    fi
    
    # Test each model (simplified - in production, actual API calls would be made)
    echo "$models_json" | python3 - "$DEFAULT_MODEL" << 'PY'
import sys
import json

models = json.loads(sys.argv[1]) if len(sys.argv) > 1 else []
default = sys.argv[2] if len(sys.argv) > 2 else "anthropic/claude-sonnet-4-5"

# In production, this would make actual test calls
# For now, return the first model or default
if models:
    print(models[0])
else:
    print(default)
PY
}

# Main validation function
validate_model_with_ai() {
    local model_data="$1"
    local prompt_type="$2"  # validate_model_release, check_free_tier, etc.
    
    # Extract model info
    local model_id provider model_name
    model_id=$(echo "$model_data" | python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('modelId', ''))")
    provider=$(echo "$model_data" | python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('provider', ''))")
    model_name=$(echo "$model_data" | python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('name', ''))")
    
    log_info "Validating model: $model_id (prompt: $prompt_type)"
    
    # Get prompt template
    local prompt_template
    prompt_template=$(get_prompt_template "$prompt_type" "$CONFIG_FILE")
    
    if [[ -z "$prompt_template" ]]; then
        log_warn "Prompt template '$prompt_type' not found"
        echo "{}"
        return 1
    fi
    
    # Fill template variables
    local filled_prompt
    filled_prompt=$(python3 - "$prompt_template" "$model_name" "$provider" "$model_data" | python3 -c "
import sys
import json

template = sys.stdin.read()
model_name = sys.argv[1] if len(sys.argv) > 1 else ''
provider = sys.argv[2] if len(sys.argv) > 2 else ''
model_data_json = sys.argv[3] if len(sys.argv) > 3 else '{}'

model_data = json.loads(model_data_json)

# Replace placeholders
filled = template
filled = filled.replace('{model_name}', model_name)
filled = filled.replace('{provider}', provider)
filled = filled.replace('{release_date}', model_data.get('releaseDate', 'unknown'))
filled = filled.replace('{parameters}', str(model_data.get('specifications', {}).get('parameters', 'unknown')))
filled = filled.replace('{context_window}', str(model_data.get('specifications', {}).get('contextWindow', 'unknown')))
filled = filled.replace('{license}', model_data.get('availability', {}).get('license', 'unknown'))
filled = filled.replace('{free_tier_claims}', json.dumps(model_data.get('freeTier', {})))
filled = filled.replace('{benchmarks}', json.dumps(model_data.get('benchmarks', {})))
filled = filled.replace('{news_articles}', json.dumps(model_data.get('newsReferences', [])))
filled = filled.replace('{specifications}', json.dumps(model_data.get('specifications', {})))
filled = filled.replace('{availability}', json.dumps(model_data.get('availability', {})))
filled = filled.replace('{validation_results}', json.dumps(model_data.get('validationFlags', {})))

print(json.dumps(filled))
")
    
    # Generate cache key
    local input_hash
    input_hash=$(echo "$filled_prompt" | md5sum | cut -d' ' -f1)
    local cache_key
    cache_key=$(get_ai_cache_key "$model_id" "$prompt_type" "$input_hash")
    
    # Get best model to use
    local best_model
    best_model=$(test_model_capabilities)
    
    # Call AI API
    call_opencode_api "$best_model" "$filled_prompt" "{}" "$cache_key"
}

# Process all models in registry
process_all_models() {
    local reg_dir="$1"
    
    log_info "Processing models in $reg_dir..."
    
    # Find all registry JSON files
    find "$reg_dir" -name "*.json" -type f | while read -r registry_file; do
        local model_data
        model_data=$(cat "$registry_file")
        
        local model_id
        model_id=$(echo "$model_data" | python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('modelId', ''))")
        
        log_info "Processing: $model_id"
        
        # Run validations based on mode
        if [[ "$MODE" == "go" ]]; then
            # Full validation suite
            local release_validation free_tier_validation benchmark_validation
            
            release_validation=$(validate_model_with_ai "$model_data" "validate_model_release")
            free_tier_validation=$(validate_model_with_ai "$model_data" "check_free_tier")
            benchmark_validation=$(validate_model_with_ai "$model_data" "validate_benchmarks")
            
            # Merge results back into model data
            python3 - "$registry_file" "$release_validation" "$free_tier_validation" "$benchmark_validation" << 'PY'
import json
import sys

registry_file = sys.argv[1]
release_val = json.loads(sys.argv[2]) if len(sys.argv) > 2 else {}
free_val = json.loads(sys.argv[3]) if len(sys.argv) > 3 else {}
benchmark_val = json.loads(sys.argv[4]) if len(sys.argv) > 4 else {}

with open(registry_file, 'r') as f:
    data = json.load(f)

# Update validation flags
if 'validationFlags' not in data:
    data['validationFlags'] = {}

if release_val:
    data['validationFlags']['aiValidated'] = release_val.get('isValid', False)
    data['validationFlags']['confidenceScore'] = release_val.get('confidenceScore', 0)
    data['validationFlags']['discrepancies'] = release_val.get('discrepancies', [])

if free_val:
    data['freeTier'] = free_val.get('freeTier', data.get('freeTier', {}))
    data['isFree'] = free_val.get('isFree', data.get('isFree', True))

if benchmark_val:
    data['benchmarks'] = benchmark_val.get('validatedBenchmarks', data.get('benchmarks', {}))

# Save updated data
with open(registry_file, 'w') as f:
    json.dump(data, f, indent=2)

print(f"Updated {registry_file}")
PY
        elif [[ "$MODE" == "zen" ]]; then
            # Minimal validation (just basic checks)
            log_info "Zen mode: skipping comprehensive AI validation"
        fi
    done
}

# Main execution
main() {
    load_config
    
    local reg_dir="${1:-tracker/registry}"
    
    if [[ ! -d "$reg_dir" ]]; then
        log_error "Registry directory not found: $reg_dir"
        exit 1
    fi
    
    process_all_models "$reg_dir"
    
    log_info "AI validation complete"
}

# Run if executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
