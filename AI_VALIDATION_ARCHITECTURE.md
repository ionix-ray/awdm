# AI Validation Architecture

## Overview

This document describes the AI-powered validation and enrichment system for the AI Model Tracker, integrating OpenCode API for intelligent fact-checking while maintaining cost efficiency through caching and mode-based execution.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                    GitHub Actions Pipeline                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐      │
│  │   Validate   │───▶│   Fetch      │───▶│   AI         │      │
│  │   Registry   │    │   Data       │    │   Validate   │      │
│  └──────────────┘    └──────────────┘    └──────────────┘      │
│                            │                    │                │
│                            ▼                    ▼                │
│                     ┌──────────────┐    ┌──────────────┐        │
│                     │   Hugging    │    │   OpenCode   │        │
│                     │   Face API   │    │   API        │        │
│                     └──────────────┘    └──────────────┘        │
│                            │                    │                │
│                            ▼                    ▼                │
│                     ┌──────────────┐    ┌──────────────┐        │
│                     │   Cache      │    │   AI Cache   │        │
│                     │   (.cache)   │    │   (.cache/ai)│        │
│                     └──────────────┘    └──────────────┘        │
│                            │                    │                │
└────────────────────────────┼────────────────────┼────────────────┘
                             │                    │
                             ▼                    ▼
                      ┌─────────────────────────────────┐
                      │      Registry & Metadata        │
                      │      (JSON Files)               │
                      └─────────────────────────────────┘
                                       │
                                       ▼
                              ┌─────────────────┐
                              │  Generate JSON  │
                              │  (model-status) │
                              └─────────────────┘
                                       │
                                       ▼
                              ┌─────────────────┐
                              │  Build Frontend │
                              │  (pnpm + Vite)  │
                              └─────────────────┘
                                       │
                                       ▼
                              ┌─────────────────┐
                              │  Deploy to      │
                              │  GitHub Pages   │
                              └─────────────────┘
```

## Configuration System

### config.toml Structure

The `tracker/config.toml` file controls all AI validation behavior:

```toml
[general]
mode = "go"  # or "zen"
default_model = "anthropic/claude-sonnet-4-5"
auto_detect_best_model = true
models_to_test = [...]

[opencode]
server_port = 4096
api_token_env = "OPENCODE_API_TOKEN"
base_url = "https://your-gateway.com"
autoupdate = true

[prompts]
# Five specialized prompts for different validation tasks
validate_model_release = """..."""
check_free_tier = """..."""
validate_benchmarks = """..."""
analyze_news_sentiment = """..."""
generate_model_summary = """..."""

[validation]
enable_schema_validation = true
min_confidence_score = 70
max_retries = 3
timeout_seconds = 30

[rate_limits]
max_api_calls_per_hour = 50
max_concurrent_requests = 3
request_delay_ms = 1000

[caching]
enabled = true
ttl_seconds = 86400
cache_dir = ".cache/ai"
```

## Execution Modes

### Zen Mode (Cost-Effective)
- Minimal AI calls
- Uses default model only
- Skips non-critical validations
- Faster execution (~5 minutes)
- Best for routine updates

### Go Mode (Comprehensive)
- Full validation suite
- Tests multiple models for best capability
- All five prompt types executed
- Cross-validation enabled
- Slower but thorough (~20 minutes)
- Best for major updates or new models

## GitHub Actions Integration

### Workflow Inputs

When triggering manually via `workflow_dispatch`:

```yaml
inputs:
  force_refresh:
    description: 'Force refresh all data (ignore cache)'
    type: boolean
    default: false
    
  ai_mode:
    description: 'AI validation mode: zen (minimal) or go (comprehensive)'
    type: choice
    options:
      - zen
      - go
    default: 'go'
    
  skip_ai_validation:
    description: 'Skip AI validation step'
    type: boolean
    default: false
```

### Environment Variables

Required secrets:
- `OPENCODE_API_TOKEN`: Your OpenCode API token

Optional environment variables:
- `AI_MODE`: Override mode (zen/go)
- `AI_MAX_RETRIES`: Max retry attempts (default: 3)
- `AI_TIMEOUT`: API timeout in seconds (default: 30)
- `CONFIG_FILE`: Path to config.toml (default: tracker/config.toml)

## Prompt Templates

### 1. validate_model_release
**Purpose**: Verify model existence, release date, specifications, and license

**Input Variables**:
- `{model_name}`
- `{provider}`
- `{release_date}`
- `{parameters}`
- `{context_window}`
- `{license}`

**Output Schema**:
```json
{
  "isValid": boolean,
  "confidenceScore": number,
  "verifiedFields": ["field1", "field2"],
  "discrepancies": [{"field": "string", "expected": "string", "actual": "string"}],
  "anomalies": ["description"],
  "sources": ["url"]
}
```

### 2. check_free_tier
**Purpose**: Analyze pricing and free tier availability

**Input Variables**:
- `{model_name}`
- `{provider}`
- `{free_tier_claims}`

**Output Schema**:
```json
{
  "isFree": boolean,
  "freeTier": {
    "available": boolean,
    "limits": {"rpm": number, "rpd": number, "tokens": number},
    "conditions": "string"
  },
  "paidTiers": [...],
  "competitorComparison": [...],
  "confidenceScore": number
}
```

### 3. validate_benchmarks
**Purpose**: Cross-reference benchmark scores with official leaderboards

**Input Variables**:
- `{model_name}`
- `{benchmarks}`

**Output Schema**:
```json
{
  "validatedBenchmarks": {...},
  "discrepancies": [...],
  "missingBenchmarks": [...],
  "methodologyIssues": [...],
  "overallConfidence": number
}
```

### 4. analyze_news_sentiment
**Purpose**: Analyze news articles and announcements

**Input Variables**:
- `{model_name}`
- `{news_articles}`

**Output Schema**:
```json
{
  "overallSentiment": "positive|neutral|negative",
  "articles": [...],
  "controversialClaims": [...],
  "consensusView": "string"
}
```

### 5. generate_model_summary
**Purpose**: Create concise, informative summaries

**Input Variables**:
- `{model_name}`
- `{specifications}`
- `{benchmarks}`
- `{availability}`
- `{validation_results}`

**Output Schema**:
```json
{
  "summary": "string (max 300 chars)",
  "highlights": [...],
  "limitations": [...],
  "useCases": [...],
  "targetAudience": "developers|researchers|enterprise|general"
}
```

## Caching Strategy

### Two-Tier Caching

1. **API Response Cache** (`.cache/`)
   - Stores raw API responses from HuggingFace
   - TTL: 1 hour
   - Key: MD5 hash of URL

2. **AI Response Cache** (`.cache/ai/`)
   - Stores processed AI validation results
   - TTL: 24 hours
   - Key: `{model_id}_{prompt_type}_{input_hash}`

### Cache Invalidation

Cache is cleared when:
- `force_refresh` input is true
- Manual workflow dispatch with force option
- Cache TTL expired
- Schema validation fails repeatedly

## Error Handling

### Retry Logic
- Maximum 3 retries per AI call
- Exponential backoff (2s delay between retries)
- Graceful degradation on failure

### Failure Scenarios

| Scenario | Behavior |
|----------|----------|
| Missing API token | Skip AI validation, continue with basic data |
| API timeout | Retry up to max_retries, then skip |
| Invalid schema | Retry with same prompt, log warning |
| Rate limit exceeded | Queue remaining requests, resume next run |
| Low confidence score (<60) | Flag for manual review, don't auto-merge |

## Security Considerations

### Credential Management
- ✅ API tokens stored in GitHub Secrets
- ✅ Never exposed in logs or frontend
- ✅ Environment variable injection only in CI/CD

### Input Sanitization
- ✅ All user inputs validated against schema
- ✅ JSON output parsed and sanitized
- ✅ No shell injection vulnerabilities

### OWASP Top 10 Mitigation
- ✅ A01 (Broken Access Control): GitHub token permissions scoped minimally
- ✅ A02 (Cryptographic Failures): HTTPS for all API calls
- ✅ A03 (Injection): Parameterized queries, input validation
- ✅ A05 (Security Misconfiguration): Strict CSP headers in frontend
- ✅ A07 (XSS): React escapes outputs by default

## Local Simulation

Run the full pipeline locally before pushing:

```bash
# Basic simulation
./scripts/simulate-github-action.sh

# Force refresh (clear cache)
./scripts/simulate-github-action.sh --force-refresh

# Zen mode (faster, minimal AI)
./scripts/simulate-github-action.sh --zen-mode

# Skip AI validation entirely
./scripts/simulate-github-action.sh --skip-ai

# Show help
./scripts/simulate-github-action.sh --help
```

## Cost Optimization

### Estimated API Costs (per run)

| Mode | AI Calls | Tokens/Call | Total Tokens | Est. Cost* |
|------|----------|-------------|--------------|------------|
| Zen  | 3        | ~500        | ~1,500       | $0.002     |
| Go   | 15       | ~500        | ~7,500       | $0.01      |

*Based on Claude Sonnet pricing ($3/1M input tokens)

### Monthly Estimates (2 runs/day)
- Zen mode: ~$0.12/month
- Go mode: ~$0.60/month
- Mixed (avg): ~$0.36/month

### Optimization Strategies
1. **Caching**: 24-hour cache reduces redundant calls by ~80%
2. **Zen mode**: Use for routine updates
3. **Selective validation**: Only validate new/changed models
4. **Batch processing**: Group multiple models in single call (future)

## Monitoring & Observability

### Logs
All AI validation steps are logged with structured format:
```
[INFO] Starting AI validation (mode: go)...
[INFO] Loading configuration from tracker/config.toml...
[INFO] Configuration loaded: mode=go, model=anthropic/claude-sonnet-4-5
[INFO] Processing models in tracker/registry...
[INFO] Validating model: google/gemma-2-2b-it (prompt: validate_model_release)
[CACHE HIT] google__gemma-2-2b-it_validate_model_release_abc123
[AI CALL] Model: anthropic/claude-sonnet-4-5, Attempt: 1/3
[AI SUCCESS] Valid response received
[SAVED] Updated tracker/registry/google__gemma-2-2b-it.json
[INFO] AI validation complete
```

### Metrics to Track
- AI validation success rate
- Average confidence scores
- Cache hit ratio
- API call count per run
- Execution time per model

## Future Enhancements

1. **Cross-Validation**: Run same prompt on multiple models, compare results
2. **Anomaly Detection**: ML-based outlier detection for benchmarks
3. **Trend Analysis**: Track changes over time, flag significant shifts
4. **Community Verification**: Integrate user feedback loop
5. **Multi-Language Support**: Validate non-English sources

## Troubleshooting

### Common Issues

**Issue**: AI validation skipped
- **Cause**: Missing `OPENCODE_API_TOKEN` secret
- **Fix**: Add secret in repository settings → Secrets and variables → Actions

**Issue**: Repeated schema validation failures
- **Cause**: Model output format changed
- **Fix**: Update prompt template to enforce stricter JSON output

**Issue**: High API costs
- **Cause**: Too many go mode runs
- **Fix**: Switch to zen mode for routine updates, use caching

**Issue**: Slow execution
- **Cause**: Many models to validate
- **Fix**: Enable selective validation, increase concurrency limit

## References

- [OpenCode API Documentation](https://opencode.ai/docs)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Carbon Design System](https://carbondesignsystem.com/)
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
