//! AI Provider module for dynamic provider selection and API calls

use crate::{ProviderConfig, PromptConfig, ValidationError, Result};
use std::collections::HashMap;

pub struct AIProviderManager {
    providers: Vec<ProviderConfig>,
    current_provider_index: usize,
}

impl AIProviderManager {
    pub fn new(providers: Vec<ProviderConfig>) -> Self {
        // Sort by priority and filter enabled providers
        let mut sorted_providers = providers;
        sorted_providers.sort_by_key(|p| p.priority);
        sorted_providers.retain(|p| p.enabled);
        
        Self {
            providers: sorted_providers,
            current_provider_index: 0,
        }
    }

    /// Get the best available provider based on priority
    pub fn get_best_provider(&self) -> Option<&ProviderConfig> {
        self.providers.first()
    }

    /// Get a provider by name
    pub fn get_provider(&self, name: &str) -> Option<&ProviderConfig> {
        self.providers.iter().find(|p| p.name == name)
    }

    /// Select next provider in fallback chain
    pub fn select_next_provider(&mut self) -> Option<&ProviderConfig> {
        if self.providers.is_empty() {
            return None;
        }
        
        self.current_provider_index = (self.current_provider_index + 1) % self.providers.len();
        self.providers.get(self.current_provider_index)
    }

    /// Build prompt from template with variables
    pub fn build_prompt(&self, prompt_config: &PromptConfig, variables: &HashMap<String, String>) -> String {
        let mut prompt = prompt_config.user_prompt_template.clone();
        
        for (key, value) in variables {
            prompt = prompt.replace(&format!("{{{}}}", key), value);
        }
        
        prompt
    }

    /// Validate that output matches expected JSON schema
    pub fn validate_ai_output(&self, output: &str, expected_fields: &[&str]) -> Result<()> {
        let parsed: serde_json::Value = serde_json::from_str(output)
            .map_err(|e| ValidationError::AIApiError(format!("Invalid JSON from AI: {}", e)))?;
        
        if let Some(obj) = parsed.as_object() {
            for field in expected_fields {
                if !obj.contains_key(*field) {
                    return Err(ValidationError::AIApiError(
                        format!("Missing expected field: {}", field)
                    ));
                }
            }
            Ok(())
        } else {
            Err(ValidationError::AIApiError("AI output is not a JSON object".to_string()))
        }
    }

    /// Estimate token usage for budget control
    pub fn estimate_tokens(&self, input: &str) -> usize {
        // Rough estimation: ~4 characters per token
        input.len() / 4
    }

    /// Check if within budget limits
    pub fn check_budget(&self, estimated_tokens: usize, token_limit: u64) -> bool {
        (estimated_tokens as u64) <= token_limit
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ProviderConfig;

    #[test]
    fn test_provider_selection() {
        let providers = vec![
            ProviderConfig {
                name: "provider1".to_string(),
                provider_type: "openai-compatible".to_string(),
                base_url: "https://api.test1.com".to_string(),
                api_key_env: "KEY1".to_string(),
                enabled: true,
                priority: 2,
            },
            ProviderConfig {
                name: "provider2".to_string(),
                provider_type: "openai-compatible".to_string(),
                base_url: "https://api.test2.com".to_string(),
                api_key_env: "KEY2".to_string(),
                enabled: true,
                priority: 1,
            },
        ];

        let manager = AIProviderManager::new(providers);
        let best = manager.get_best_provider().unwrap();
        
        assert_eq!(best.name, "provider2"); // Lower priority number = higher priority
        assert_eq!(best.priority, 1);
    }

    #[test]
    fn test_prompt_building() {
        let prompt_config = PromptConfig {
            name: "test".to_string(),
            mode: "zen".to_string(),
            temperature: 0.5,
            max_tokens: 100,
            system_prompt: "You are helpful".to_string(),
            user_prompt_template: "Verify model: {model_name} from {provider}".to_string(),
        };

        let mut variables = HashMap::new();
        variables.insert("model_name".to_string(), "GPT-4".to_string());
        variables.insert("provider".to_string(), "OpenAI".to_string());

        let manager = AIProviderManager::new(vec![]);
        let prompt = manager.build_prompt(&prompt_config, &variables);

        assert!(prompt.contains("GPT-4"));
        assert!(prompt.contains("OpenAI"));
    }

    #[test]
    fn test_token_estimation() {
        let manager = AIProviderManager::new(vec![]);
        let tokens = manager.estimate_tokens("This is a test string with about 10 tokens");
        
        // Should be roughly 40/4 = 10 tokens
        assert!(tokens > 5 && tokens < 15);
    }
}
