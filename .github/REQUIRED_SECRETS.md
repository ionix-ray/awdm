# Required GitHub Secrets

## Setup Instructions

1. Go to your repository on GitHub
2. Click **Settings** → **Secrets and variables** → **Actions**
3. Click **New repository secret**
4. Add the following secrets:

## Required Secrets

### OPENCODE_API_TOKEN
- **Name**: `OPENCODE_API_TOKEN`
- **Value**: Your OpenCode API token
- **Description**: API token for AI validation and enrichment
- **Required**: Yes (for AI validation features)

## Optional Secrets

### HUGGINGFACE_TOKEN
- **Name**: `HF_API_TOKEN`
- **Value**: Your Hugging Face API token
- **Description**: For accessing private models or higher rate limits
- **Required**: No (public API works without it)

## Environment Variables (Optional)

These can be set as repository variables or in the workflow:

| Variable | Default | Description |
|----------|---------|-------------|
| `AI_MODE` | `go` | AI validation mode: `zen` or `go` |
| `AI_MAX_RETRIES` | `3` | Maximum retry attempts for AI calls |
| `AI_TIMEOUT` | `30` | Timeout in seconds for AI API calls |
| `CONFIG_FILE` | `tracker/config.toml` | Path to configuration file |

## Verification

After adding secrets, you can verify they're working by:

1. Going to **Actions** tab
2. Selecting **AI Model Tracker** workflow
3. Clicking **Run workflow**
4. Setting `skip_ai_validation` to `false`
5. Monitoring the "AI Validation and Enrichment" step

If the secret is missing or invalid, the workflow will log a warning but continue without AI validation.

## Security Notes

- ✅ Secrets are encrypted and never exposed in logs
- ✅ Only accessible to workflows in this repository
- ✅ Can be rotated anytime without code changes
- ✅ Consider using environment-specific secrets for staging/production
