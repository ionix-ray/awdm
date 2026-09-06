//! TOML parser for the configuration file.

use serde::Deserialize;

/// Top-level config from TOML.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AppConfig {
    pub default: DefaultConfig,
    pub ai_providers: Vec<AiProvider>,
    pub validation_prompts: Vec<ValidationPrompt>,
    pub data_sources: Vec<DataSource>,
    pub schedule: ScheduleConfig,
    pub model_selection: ModelSelection,
}

/// Default settings
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DefaultConfig {
    pub mode: PromptMode,
}

/// AI Provider configuration
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AiProvider {
    pub name: String,
    pub base_url: String,
    pub api_key_env: String,
    pub models: Vec<String>,
}

/// Validation prompt configuration
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ValidationPrompt {
    pub name: String,
    pub mode: PromptMode,
    pub prompt_template: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub output_schema: String,
}

/// Prompt execution mode
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PromptMode {
    Zen,
    Go,
}

/// Data source configuration
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DataSource {
    pub name: String,
    pub url: String,
    pub scrape_interval_hours: u32,
    pub requires_auth: bool,
}

/// Schedule configuration
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ScheduleConfig {
    pub cron_expression: String,
    pub timezone: String,
    pub max_runs_per_day: u32,
}

/// Model selection strategy
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ModelSelection {
    pub enabled: bool,
    pub check_availability: bool,
    pub fallback_order: Vec<String>,
    pub timeout_seconds: u32,
    pub max_retries: u32,
}

/// Parse the configuration TOML string into structured config.
pub fn parse_config(toml_str: &str) -> Result<AppConfig, String> {
    toml::from_str(toml_str).map_err(|e| format!("Failed to parse config TOML: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL_CONFIG: &str = r##"
[default]
mode = "zen"

[[ai_providers]]
name = "test_provider"
base_url = "https://api.test.com/v1"
api_key_env = "TEST_KEY"
models = ["test-model-1"]

[[validation_prompts]]
name = "test_prompt"
mode = "zen"
prompt_template = "Test prompt"
max_tokens = 500
temperature = 0.3
output_schema = "test_schema"

[[data_sources]]
name = "test_source"
url = "https://example.com/api"
scrape_interval_hours = 6
requires_auth = false

[schedule]
cron_expression = "0 6 * * *"
timezone = "UTC"
max_runs_per_day = 2

[model_selection]
enabled = true
check_availability = true
fallback_order = ["test_provider"]
timeout_seconds = 30
max_retries = 3
"##;

    #[test]
    fn test_parse_minimal_config() {
        let config = parse_config(MINIMAL_CONFIG).unwrap();
        assert_eq!(config.default.mode, PromptMode::Zen);
        assert_eq!(config.ai_providers.len(), 1);
        assert_eq!(config.validation_prompts.len(), 1);
        assert_eq!(config.data_sources.len(), 1);
    }

    #[test]
    fn test_parse_ai_provider() {
        let config = parse_config(MINIMAL_CONFIG).unwrap();
        let provider = &config.ai_providers[0];
        assert_eq!(provider.name, "test_provider");
        assert_eq!(provider.base_url, "https://api.test.com/v1");
        assert_eq!(provider.models.len(), 1);
    }

    #[test]
    fn test_parse_validation_prompt() {
        let config = parse_config(MINIMAL_CONFIG).unwrap();
        let prompt = &config.validation_prompts[0];
        assert_eq!(prompt.name, "test_prompt");
        assert_eq!(prompt.mode, PromptMode::Zen);
        assert_eq!(prompt.max_tokens, 500);
    }

    #[test]
    fn test_parse_schedule() {
        let config = parse_config(MINIMAL_CONFIG).unwrap();
        assert_eq!(config.schedule.cron_expression, "0 6 * * *");
        assert_eq!(config.schedule.timezone, "UTC");
        assert_eq!(config.schedule.max_runs_per_day, 2);
    }

    #[test]
    fn test_parse_model_selection() {
        let config = parse_config(MINIMAL_CONFIG).unwrap();
        assert!(config.model_selection.enabled);
        assert_eq!(config.model_selection.fallback_order.len(), 1);
        assert_eq!(config.model_selection.timeout_seconds, 30);
    }

    #[test]
    fn test_parse_invalid_toml_returns_error() {
        let result = parse_config("not valid toml {{{");
        assert!(result.is_err());
    }

    #[test]
    fn test_go_mode_parsing() {
        let toml = r##"
[default]
mode = "go"

[[ai_providers]]
name = "x"
base_url = "x"
api_key_env = "x"
models = ["x"]

[[validation_prompts]]
name = "x"
mode = "go"
prompt_template = "x"
max_tokens = 1
temperature = 0.1
output_schema = "x"

[[data_sources]]
name = "x"
url = "x"
scrape_interval_hours = 1
requires_auth = false

[schedule]
cron_expression = "x"
timezone = "x"
max_runs_per_day = 1

[model_selection]
enabled = false
check_availability = false
fallback_order = []
timeout_seconds = 1
max_retries = 1
"##;
        let config = parse_config(toml).unwrap();
        assert_eq!(config.default.mode, PromptMode::Go);
    }
}
