#!/usr/bin/env bash
# lib-tracker.sh
# Shared library functions for AI Model Tracker

set -euo pipefail

# Default paths (can be overridden via environment)
export TRACKER_DIR="${TRACKER_DIR:-tracker}"
export CONFIG_FILE="${TRACKER_DIR}/config.json"
export REG_DIR="${TRACKER_DIR}/registry"
export META_DIR="${TRACKER_DIR}/metadata"
export SCHEMA_DIR="${TRACKER_DIR}/schemas"
export JSON_OUT="${JSON_OUT:-public/model-status.json}"
export HTML_OUT="${HTML_OUT:-public/index.html}"

# Logging helpers
log_info() {
  echo "[INFO] $*"
}

log_warn() {
  echo "[WARN] $*" >&2
}

log_error() {
  echo "[ERROR] $*" >&2
}

# Validate that required directories exist
validate_dirs() {
  local dirs=("$REG_DIR" "$META_DIR" "$SCHEMA_DIR")
  for dir in "${dirs[@]}"; do
    if [[ ! -d "$dir" ]]; then
      log_error "Required directory $dir missing"
      return 1
    fi
  done
  return 0
}

# Get model key from modelId (replace / with __)
model_key() {
  local model_id="$1"
  echo "${model_id//\//__}"
}

# Check if jq is available
check_jq() {
  if ! command -v jq &> /dev/null; then
    log_error "jq is required but not installed"
    return 1
  fi
}

# Check if python3 is available
check_python() {
  if ! command -v python3 &> /dev/null; then
    log_error "python3 is required but not installed"
    return 1
  fi
}

# Validate JSON against schema using Python
validate_json_schema() {
  local json_file="$1"
  local schema_file="$2"
  
  python3 - "$json_file" "$schema_file" << 'PY'
import json
import sys
try:
    import jsonschema
except ImportError:
    print("::error::jsonschema not installed", file=sys.stderr)
    sys.exit(2)

json_file, schema_file = sys.argv[1:3]
try:
    with open(json_file) as f:
        data = json.load(f)
    with open(schema_file) as f:
        schema = json.load(f)
    jsonschema.validate(data, schema)
    print(f"✓ {json_file} validates against schema")
    sys.exit(0)
except json.JSONDecodeError as e:
    print(f"::error::{json_file}: Invalid JSON - {e}", file=sys.stderr)
    sys.exit(1)
except jsonschema.ValidationError as e:
    print(f"::error::{json_file}: Schema validation failed - {e.message}", file=sys.stderr)
    sys.exit(1)
PY
}
