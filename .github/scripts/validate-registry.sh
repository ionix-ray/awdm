#!/usr/bin/env bash
# validate-registry.sh
# Validates tracker config, registry records, and metadata against JSON schemas
# Usage: ./validate-registry.sh [--live]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=/dev/null
. "$SCRIPT_DIR/lib-tracker.sh"

LIVE="${1:-}"

# Check dependencies
check_jq || exit 1
check_python || exit 1
validate_dirs || exit 1

log_info "Validating registry..."

python3 - "$CONFIG_FILE" "$REG_DIR" "$META_DIR" "$SCHEMA_DIR" << 'PY'
import json
import sys
import os
import glob

try:
    import jsonschema
except ImportError:
    print("::error::jsonschema not installed. Run: pip install jsonschema", file=sys.stderr)
    sys.exit(2)

config_file, reg_dir, meta_dir, schema_dir = sys.argv[1:5]
errors = []
warnings = []

def load_json(path):
    with open(path) as f:
        return json.load(f)

# Load schemas
try:
    config_schema = load_json(os.path.join(schema_dir, "config.schema.json"))
    model_schema = load_json(os.path.join(schema_dir, "model-record.schema.json"))
    metadata_schema = load_json(os.path.join(schema_dir, "metadata-record.schema.json"))
except Exception as e:
    print(f"::error::Failed to load schemas: {e}", file=sys.stderr)
    sys.exit(1)

# Validate config.json
if os.path.exists(config_file):
    try:
        config = load_json(config_file)
        jsonschema.validate(config, config_schema)
        print(f"✓ config.json validates")
    except json.JSONDecodeError as e:
        errors.append(f"config.json: Invalid JSON - {e}")
    except jsonschema.ValidationError as e:
        errors.append(f"config.json: {e.message}")
else:
    warnings.append("config.json not found, using defaults")

# Validate registry records
model_ids = set()
registry_files = sorted(glob.glob(os.path.join(reg_dir, "*.json")))

for rf in registry_files:
    basename = os.path.basename(rf)
    try:
        record = load_json(rf)
    except json.JSONDecodeError as e:
        errors.append(f"{basename}: Invalid JSON - {e}")
        continue
    
    try:
        jsonschema.validate(record, model_schema)
        print(f"✓ {basename} validates")
    except jsonschema.ValidationError as e:
        # Treat additionalProperties as warning, not error
        if e.validator == 'additionalProperties':
            warnings.append(f"{basename}: {e.message}")
            print(f"⚠ {basename} validates (with warnings)")
        else:
            errors.append(f"{basename}: {e.message}")
            continue
    
    # Check for duplicate modelIds
    model_id = record.get("modelId", "")
    if model_id in model_ids:
        errors.append(f"{basename}: Duplicate modelId '{model_id}'")
    model_ids.add(model_id)
    
    # Validate modelId matches filename pattern
    expected_key = model_id.replace("/", "__")
    expected_filename = f"{expected_key}.json"
    if basename != expected_filename:
        warnings.append(f"{basename}: Filename should be {expected_filename}")

# Validate metadata records
metadata_files = sorted(glob.glob(os.path.join(meta_dir, "*.json")))

for mf in metadata_files:
    basename = os.path.basename(mf)
    try:
        metadata = load_json(mf)
    except json.JSONDecodeError as e:
        errors.append(f"{basename}: Invalid JSON - {e}")
        continue
    
    try:
        jsonschema.validate(metadata, metadata_schema)
        print(f"✓ {basename} validates")
    except jsonschema.ValidationError as e:
        errors.append(f"{basename}: {e.message}")
        continue
    
    # Check that metadata corresponds to a registered model
    model_id = metadata.get("modelId", "")
    if model_id and model_id not in model_ids:
        warnings.append(f"{basename}: No matching registry record for modelId '{model_id}'")

# Summary
print()
if warnings:
    print(f"Warnings ({len(warnings)}):")
    for w in warnings:
        print(f"  ⚠ {w}")

if errors:
    print(f"Errors ({len(errors)}):")
    for e in errors:
        print(f"  ✗ {e}")
    print()
    print("::error::Registry validation failed")
    sys.exit(1)
else:
    print(f"✓ Registry validation passed ({len(registry_files)} models, {len(metadata_files)} metadata records)")
    sys.exit(0)
PY
