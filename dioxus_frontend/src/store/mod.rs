//! Store module for state management using Dioxus signals

use dioxus::prelude::*;
use crate::types::{Model, ModelData};

/// Application state
#[derive(Clone, Debug)]
pub struct AppState {
    pub models: Vec<Model>,
    pub loading: bool,
    pub error: Option<String>,
    pub search_query: String,
    pub filters: FilterOptions,
    pub current_page: usize,
    pub items_per_page: usize,
}

/// Filter options
#[derive(Clone, Debug, Default)]
pub struct FilterOptions {
    pub free_only: bool,
    pub providers: Vec<String>,
    pub modalities: Vec<String>,
    pub min_confidence: u8,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            models: Vec::new(),
            loading: true,
            error: None,
            search_query: String::new(),
            filters: FilterOptions::default(),
            current_page: 1,
            items_per_page: 12,
        }
    }
}

/// Global application state signal
pub type AppStateSignal = Signal<AppState>;

/// Initialize the app state and fetch data
pub async fn initialize_app_state() -> Result<ModelData, String> {
    // Fetch model data from generated JSON
    let response = gloo_net::http::Request::get("/model-status.json")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch data: {}", e))?;
    
    if !response.ok() {
        return Err(format!("HTTP Error: {}", response.status()));
    }
    
    let data: ModelData = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    
    Ok(data)
}

/// Filter models based on current filters and search
pub fn filter_models(models: &[Model], query: &str, filters: &FilterOptions) -> Vec<&Model> {
    models
        .iter()
        .filter(|model| {
            // Search filter
            if !query.is_empty() {
                let search_lower = query.to_lowercase();
                if !model.name.to_lowercase().contains(&search_lower)
                    && !model.provider.to_lowercase().contains(&search_lower)
                {
                    return false;
                }
            }
            
            // Free tier filter
            if filters.free_only && !model.is_free {
                return false;
            }
            
            // Provider filter
            if !filters.providers.is_empty() 
                && !filters.providers.contains(&model.provider) 
            {
                return false;
            }
            
            // Confidence filter
            if model.validation_flags.confidence < filters.min_confidence {
                return false;
            }
            
            true
        })
        .collect()
}

/// Paginate filtered results
pub fn paginate_models<'a>(models: &[&'a Model], page: usize, per_page: usize) -> Vec<&'a Model> {
    let start = (page - 1) * per_page;
    let end = std::cmp::min(start + per_page, models.len());
    
    if start >= models.len() {
        return Vec::new();
    }
    
    models[start..end].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{FreeTier, Specifications, ValidationFlags};
    
    fn create_test_model(name: &str, provider: &str, is_free: bool, confidence: u8) -> Model {
        Model {
            id: format!("test-{}", name),
            name: name.to_string(),
            provider: provider.to_string(),
            release_date: "2024-01-01".to_string(),
            is_free,
            free_tier: FreeTier { available: is_free, limits: None, conditions: String::new() },
            specifications: Specifications {
                parameters: "7B".to_string(),
                context_window: 4096,
                architecture: "Transformer".to_string(),
                modality: vec!["text".to_string()],
            },
            benchmarks: None,
            availability: None,
            news_references: vec![],
            validation_flags: ValidationFlags {
                community_verified: true,
                official_announcement: true,
                anomaly_detected: false,
                confidence,
            },
        }
    }
    
    #[test]
    fn test_filter_by_search_query() {
        let models = vec![
            create_test_model("GPT-4", "OpenAI", false, 95),
            create_test_model("Llama-3", "Meta", true, 90),
            create_test_model("Gemma-2", "Google", true, 85),
        ];
        
        let filters = FilterOptions::default();
        let filtered = filter_models(&models, "llama", &filters);
        
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Llama-3");
    }
    
    #[test]
    fn test_filter_free_only() {
        let models = vec![
            create_test_model("GPT-4", "OpenAI", false, 95),
            create_test_model("Llama-3", "Meta", true, 90),
            create_test_model("Gemma-2", "Google", true, 85),
        ];
        
        let filters = FilterOptions {
            free_only: true,
            ..Default::default()
        };
        
        let filtered = filter_models(&models, "", &filters);
        
        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().all(|m| m.is_free));
    }
    
    #[test]
    fn test_filter_by_confidence() {
        let models = vec![
            create_test_model("High-Conf", "Provider1", true, 95),
            create_test_model("Med-Conf", "Provider2", true, 70),
            create_test_model("Low-Conf", "Provider3", true, 50),
        ];
        
        let filters = FilterOptions {
            min_confidence: 80,
            ..Default::default()
        };
        
        let filtered = filter_models(&models, "", &filters);
        
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "High-Conf");
    }
    
    #[test]
    fn test_pagination() {
        let models: Vec<Model> = (0..25)
            .map(|i| create_test_model(&format!("Model-{}", i), "Provider", true, 90))
            .collect();
        
        let model_refs: Vec<&Model> = models.iter().collect();
        
        let page1 = paginate_models(&model_refs, 1, 10);
        let page2 = paginate_models(&model_refs, 2, 10);
        let page3 = paginate_models(&model_refs, 3, 10);
        
        assert_eq!(page1.len(), 10);
        assert_eq!(page2.len(), 10);
        assert_eq!(page3.len(), 5);
    }
}
