#!/usr/bin/env bash
# generate-model-json.sh
# Joins registry intent + fetched metadata into unified model-status.json
# Adds computed fields for insights and analytics

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=/dev/null
. "$SCRIPT_DIR/lib-tracker.sh"

validate_dirs || exit 1

log_info "Generating model-status.json..."

python3 - "$REG_DIR" "$META_DIR" "$CONFIG_FILE" "$JSON_OUT" << 'PY'
import json
import sys
import os
import glob
from datetime import datetime, timezone

reg_dir, meta_dir, config_file, out_path = sys.argv[1:5]
now = datetime.now(timezone.utc)

# Load config
config = {}
if os.path.exists(config_file):
    with open(config_file) as f:
        config = json.load(f)

def load_json(path):
    with open(path) as f:
        return json.load(f)

def key(model_id):
    return model_id.replace("/", "__")

models = []
registry_files = sorted(glob.glob(os.path.join(reg_dir, "*.json")))

for rf in registry_files:
    intent = load_json(rf)
    model_id = intent.get("modelId", "")
    if not model_id:
        continue
    
    k = key(model_id)
    mf = os.path.join(meta_dir, f"{k}.json")
    metadata = load_json(mf) if os.path.exists(mf) else {}
    
    # Merge: metadata wins (fresh data), intent provides tracking info
    m = dict(intent)
    m.update(metadata)
    
    # Compute insights
    release_date_str = m.get("releaseDate", "")
    days_since_release = None
    if release_date_str:
        try:
            release_date = datetime.fromisoformat(release_date_str.replace("Z", "+00:00"))
            days_since_release = (now - release_date).days
        except Exception:
            pass
    
    # Calculate relevance score
    relevance_score = 0
    validation_flags = m.get("validationFlags", {})
    
    # Community verification boost
    if validation_flags.get("communityVerified"):
        relevance_score += 20
    
    # Official announcement boost
    if validation_flags.get("officialAnnouncement"):
        relevance_score += 25
    
    # Freshness boost (released in last 30 days)
    if days_since_release is not None and days_since_release < 30:
        relevance_score += 30
    elif days_since_release is not None and days_since_release < 90:
        relevance_score += 15
    
    # Free tier boost
    if m.get("isFree"):
        relevance_score += 15
    
    # Open weights boost
    availability = m.get("availability", {})
    if availability.get("openWeights"):
        relevance_score += 10
    
    # Confidence score from validation
    confidence = validation_flags.get("confidenceScore", 50)
    relevance_score = min(relevance_score + (confidence // 10), 100)
    
    # Detect new releases (within 7 days)
    is_new_release = days_since_release is not None and days_since_release <= 7
    
    # Check for free tier availability
    has_free_tier = m.get("isFree") or (m.get("freeTier", {}).get("available"))
    
    m["_computed"] = {
        "relevanceScore": relevance_score,
        "daysSinceRelease": days_since_release,
        "isNewRelease": is_new_release,
        "hasFreeTier": has_free_tier,
        "generatedAt": now.isoformat()
    }
    
    models.append(m)

# Sort by relevance score (highest first), then by newness
models.sort(key=lambda x: (-x.get("_computed", {}).get("relevanceScore", 0), 
                           -(x.get("_computed", {}).get("isNewRelease", False))))

# Build summary statistics
total_models = len(models)
free_models = sum(1 for m in models if m.get("_computed", {}).get("hasFreeTier"))
new_releases = sum(1 for m in models if m.get("_computed", {}).get("isNewRelease"))
high_confidence = sum(1 for m in models if m.get("validationFlags", {}).get("confidenceScore", 0) >= 80)

providers = {}
categories = {}
for m in models:
    provider = m.get("provider", "Unknown")
    providers[provider] = providers.get(provider, 0) + 1
    
    for cat in m.get("categories", []):
        categories[cat] = categories.get(cat, 0) + 1

output = {
    "metadata": {
        "generatedAt": now.isoformat(),
        "version": config.get("version", 1),
        "schedule": config.get("schedule", "0 */6 * * *"),
        "sources": list(set(
            src for m in models 
            for src in m.get("sources", [])
        ))
    },
    "summary": {
        "totalModels": total_models,
        "freeModels": free_models,
        "newReleasesLast7Days": new_releases,
        "highConfidenceModels": high_confidence,
        "uniqueProviders": len(providers),
        "topCategories": dict(sorted(categories.items(), key=lambda x: -x[1])[:10])
    },
    "providers": providers,
    "models": models
}

with open(out_path, "w") as f:
    json.dump(output, f, indent=2, default=str)
    f.write("\n")

print(f"Generated {out_path}: {total_models} models")
print(f"  - Free models: {free_models}")
print(f"  - New releases (7d): {new_releases}")
print(f"  - High confidence: {high_confidence}")
PY

log_info "JSON generation complete"
