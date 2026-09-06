//! AI Validator Library for Model Intelligence Dashboard
//! 
//! This library provides:
//! - Configuration loading from config.toml
//! - AI provider integration (OpenCode, Orca Router, etc.)
//! - Schema validation and atomic file operations
//! - Dioxus web components for frontend

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod config;
pub mod validator;
pub mod ai_provider;
pub mod models;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub general: GeneralConfig,
    pub ai: AIConfig,
    pub sources: SourcesConfig,
    pub validation: ValidationConfig,
    pub security: SecurityConfig,
    pub github_actions: GitHubActionsConfig,
    pub frontend: FrontendConfig,
    pub monitoring: MonitoringConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub name: String,
    pub version: String,
    pub description: String,
    pub github_pages_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub mode: String,
    pub default_model: String,
    pub token_usage_limit: u64,
    pub budget_limit_usd: f64,
    pub providers: Vec<ProviderConfig>,
    pub prompts: Vec<PromptConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub name: String,
    #[serde(rename = "type")]
    pub provider_type: String,
    pub base_url: String,
    pub api_key_env: String,
    pub enabled: bool,
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptConfig {
    pub name: String,
    pub mode: String,
    pub temperature: f64,
    pub max_tokens: u32,
    pub system_prompt: String,
    pub user_prompt_template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourcesConfig {
    pub update_frequency_hours: u32,
    pub max_concurrent_requests: u32,
    pub retry_attempts: u32,
    pub timeout_seconds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    pub schema_strict_mode: bool,
    pub require_cross_source_verification: bool,
    pub min_confidence_threshold: u32,
    pub auto_reject_anomalies: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub enable_csp: bool,
    pub sanitize_html: bool,
    pub validate_json_schema: bool,
    pub atomic_writes: bool,
    pub backup_enabled: bool,
    pub max_backups: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubActionsConfig {
    pub schedule: String,
    pub timeout_minutes: u32,
    pub concurrency_group: String,
    pub cache_enabled: bool,
    pub artifact_retention_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontendConfig {
    pub framework: String,
    pub output_dir: String,
    pub wasm_opt_level: String,
    pub cdn_prefix: String,
    pub carbon_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub clarity_project_id: String,
    pub enable_analytics: bool,
    pub log_level: String,
}

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Validation error: {0}")]
    ValidationFailed(String),
    #[error("AI API error: {0}")]
    AIApiError(String),
    #[error("File operation error: {0}")]
    FileError(String),
    #[error("Schema error: {0}")]
    SchemaError(String),
}

pub type Result<T> = std::result::Result<T, ValidationError>;

// WASM bindings for frontend use
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn init_wasm() {
    console_error_panic_hook::set_once();
    log::info!("WASM module initialized");
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn validate_model_data(json_data: &str) -> Result<String> {
    use crate::validator::DataValidator;
    
    let validator = DataValidator::new();
    match validator.validate_json(json_data).await {
        Ok(validated) => Ok(serde_json::to_string(&validated).unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e))),
        Err(e) => Ok(serde_json::json!({"error": e.to_string()}).to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_deserialization() {
        // Basic test to ensure config structures work
        let config = GeneralConfig {
            name: "Test".to_string(),
            version: "1.0.0".to_string(),
            description: "Test Desc".to_string(),
            github_pages_url: "https://test.com".to_string(),
        };
        assert_eq!(config.name, "Test");
    }
}
