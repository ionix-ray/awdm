//! Configuration loader for AI Model Dashboard
//! 
//! Loads and parses the TOML configuration file at compile time.

pub mod parser;

pub use parser::{parse_config, AppConfig, AiProvider, ValidationPrompt, PromptMode, DataSource};

/// The TOML source, embedded at compile time.
const CONFIG_TOML: &str = include_str!("../../content/config.toml");

/// Load and parse the configuration. Panics if the TOML is malformed.
pub fn load_config() -> AppConfig {
    parse_config(CONFIG_TOML)
        .expect("config.toml must be valid — check frontend/content/config.toml")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_loads_successfully() {
        let config = load_config();
        assert!(!config.ai_providers.is_empty());
        assert!(!config.validation_prompts.is_empty());
    }

    #[test]
    fn test_default_mode_is_zen() {
        let config = load_config();
        assert_eq!(config.default.mode, PromptMode::Zen);
    }

    #[test]
    fn test_opencode_provider_exists() {
        let config = load_config();
        let opencode = config.ai_providers.iter()
            .find(|p| p.name == "opencode");
        assert!(opencode.is_some());
    }
}
