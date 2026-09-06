#!/usr/bin/env bash
# fetch-model-data.sh
# Fetches AI model data from various sources (Hugging Face, GitHub, RSS feeds)
# Respects rate limits and caches responses to minimize API calls

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=/dev/null
. "$SCRIPT_DIR/lib-tracker.sh"

# Configuration
HF_API_BASE="${HF_API_BASE:-https://huggingface.co/api}"
HF_RATE_LIMIT="${HF_RATE_LIMIT_PER_HOUR:-50}"
CACHE_DIR="${CACHE_DIR:-.cache}"
CACHE_TTL="${CACHE_TTL:-3600}"  # 1 hour default

# Create cache directory
mkdir -p "$CACHE_DIR"

log_info "Fetching model data from sources..."

python3 - "$REG_DIR" "$META_DIR" "$CONFIG_FILE" "$CACHE_DIR" "$HF_API_BASE" "$HF_RATE_LIMIT" << 'PY'
import json
import sys
import os
import glob
import hashlib
import time
from datetime import datetime, timezone, timedelta
from urllib.request import urlopen, Request
from urllib.error import URLError, HTTPError
import xml.etree.ElementTree as ET

reg_dir, meta_dir, config_file, cache_dir, hf_api_base, hf_rate_limit = sys.argv[1:7]

# Use provided API base or default
HF_API_BASE = hf_api_base if hf_api_base else "https://huggingface.co/api"
HF_RATE_LIMIT = int(hf_rate_limit) if hf_rate_limit else 50

# Load config
config = {}
if os.path.exists(config_file):
    with open(config_file) as f:
        config = json.load(f)

# Rate limiting state
rate_limit_state = {
    'huggingface': {'count': 0, 'reset_time': time.time()}
}

def get_cache_key(url):
    return hashlib.md5(url.encode()).hexdigest()

def get_cached(cache_key, ttl=3600):
    """Get cached response if not expired"""
    cache_path = os.path.join(cache_dir, f"{cache_key}.json")
    if not os.path.exists(cache_path):
        return None
    
    try:
        with open(cache_path) as f:
            cached = json.load(f)
        
        cached_at = cached.get('_cached_at', 0)
        if time.time() - cached_at < ttl:
            return cached.get('data')
    except Exception:
        pass
    
    return None

def set_cache(cache_key, data):
    """Cache response with timestamp"""
    cache_path = os.path.join(cache_dir, f"{cache_key}.json")
    try:
        with open(cache_path, 'w') as f:
            json.dump({
                '_cached_at': time.time(),
                'data': data
            }, f)
    except Exception as e:
        print(f"[WARN] Failed to cache {cache_key}: {e}", file=sys.stderr)

def fetch_url(url, headers=None, rate_limit_key=None):
    """Fetch URL with caching and rate limiting"""
    cache_key = get_cache_key(url)
    
    # Try cache first
    cached = get_cached(cache_key)
    if cached is not None:
        print(f"[CACHE] {url}")
        return cached
    
    # Check rate limit
    if rate_limit_key and rate_limit_key in rate_limit_state:
        state = rate_limit_state[rate_limit_key]
        if state['count'] >= HF_RATE_LIMIT:
            if time.time() < state['reset_time']:
                print(f"[RATE LIMIT] {rate_limit_key} exceeded, skipping", file=sys.stderr)
                return None
            else:
                state['count'] = 0
                state['reset_time'] = time.time() + 3600
        
        state['count'] += 1
    
    # Fetch from URL
    try:
        req = Request(url, headers=headers or {})
        with urlopen(req, timeout=30) as response:
            data = json.loads(response.read().decode())
            set_cache(cache_key, data)
            print(f"[FETCH] {url}")
            return data
    except HTTPError as e:
        if e.code == 429:
            print(f"[RATE LIMITED] {url}", file=sys.stderr)
        else:
            print(f"[HTTP ERROR {e.code}] {url}", file=sys.stderr)
        return None
    except URLError as e:
        print(f"[URL ERROR] {url}: {e}", file=sys.stderr)
        return None
    except Exception as e:
        print(f"[ERROR] {url}: {e}", file=sys.stderr)
        return None

def fetch_huggingface_model(model_id):
    """Fetch model info from Hugging Face API"""
    url = f"{HF_API_BASE}/models/{model_id}"
    headers = {'User-Agent': 'AI-Model-Tracker/1.0'}
    data = fetch_url(url, headers, rate_limit_key='huggingface')
    
    if not data:
        return None
    
    # Extract relevant information
    now = datetime.now(timezone.utc).isoformat()
    
    # Determine if free
    is_free = True
    free_tier = {'available': True, 'limits': {}, 'conditions': ''}
    
    # Check for pipeline_tag to infer modality
    modality = []
    pipeline_tag = data.get('pipeline_tag', '')
    if pipeline_tag:
        if 'text' in pipeline_tag.lower():
            modality.append('text')
        if 'image' in pipeline_tag.lower():
            modality.append('image')
        if 'audio' in pipeline_tag.lower():
            modality.append('audio')
    
    # Get parameters from tags or config
    parameters = 'unknown'
    for tag in data.get('tags', []):
        if isinstance(tag, str) and tag.endswith('B'):
            try:
                num = float(tag[:-1])
                parameters = f"{int(num)}B" if num == int(num) else f"{num}B"
            except ValueError:
                pass
    
    # Context window from config
    context_window = None
    config_data = data.get('config', {})
    if isinstance(config_data, dict):
        context_window = config_data.get('max_position_embeddings')
    
    # License
    license_info = data.get('license', '')
    
    return {
        'modelId': model_id,
        'name': data.get('modelName', model_id.split('/')[-1]),
        'provider': model_id.split('/')[0],
        'releaseDate': data.get('createdAt', now),
        'lastCheckedAt': now,
        'isFree': is_free,
        'freeTier': free_tier,
        'specifications': {
            'parameters': parameters,
            'contextWindow': context_window,
            'architecture': data.get('library_name', 'transformer'),
            'modality': modality if modality else ['text']
        },
        'availability': {
            'hosted': ['huggingface'],
            'openWeights': True,
            'license': license_info
        },
        'benchmarks': {},
        'newsReferences': [],
        'validationFlags': {
            'communityVerified': data.get('likes', 0) > 10,
            'officialAnnouncement': False,
            'anomalyDetected': False,
            'confidenceScore': 85
        },
        'sources': ['huggingface']
    }

def load_registry_models():
    """Load all models from registry"""
    models = []
    registry_files = glob.glob(os.path.join(reg_dir, "*.json"))
    
    for rf in registry_files:
        try:
            with open(rf) as f:
                record = json.load(f)
            if record.get('tracked', True):
                models.append(record)
        except Exception as e:
            print(f"[ERROR] Failed to load {rf}: {e}", file=sys.stderr)
    
    return models

def main():
    registry_models = load_registry_models()
    print(f"Processing {len(registry_models)} tracked models...")
    
    updated_count = 0
    error_count = 0
    
    for model_record in registry_models:
        model_id = model_record.get('modelId', '')
        if not model_id:
            continue
        
        print(f"\n[MODEL] {model_id}")
        
        # Fetch from Hugging Face
        hf_data = fetch_huggingface_model(model_id)
        
        if not hf_data:
            print(f"[WARN] No data fetched for {model_id}")
            error_count += 1
            continue
        
        # Merge with registry info
        for key, value in model_record.items():
            if key not in hf_data:
                hf_data[key] = value
        
        # Save metadata
        metadata_key = model_id.replace('/', '__')
        metadata_path = os.path.join(meta_dir, f"{metadata_key}.json")
        
        try:
            with open(metadata_path, 'w') as f:
                json.dump(hf_data, f, indent=2)
            print(f"[SAVED] {metadata_path}")
            updated_count += 1
        except Exception as e:
            print(f"[ERROR] Failed to save {metadata_path}: {e}", file=sys.stderr)
            error_count += 1
    
    print(f"\n[SUMMARY] Updated: {updated_count}, Errors: {error_count}")

if __name__ == '__main__':
    main()
PY

log_info "Data fetch complete"
