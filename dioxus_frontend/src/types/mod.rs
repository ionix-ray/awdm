//! Type definitions for the AI Model Dashboard

use serde::{Deserialize, Serialize};

/// Main model data structure matching JSON schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub release_date: String,
    pub is_free: bool,
    pub free_tier: FreeTier,
    pub specifications: Specifications,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub benchmarks: Option<Benchmarks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability: Option<Availability>,
    #[serde(default)]
    pub news_references: Vec<NewsReference>,
    pub validation_flags: ValidationFlags,
}

/// Free tier information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreeTier {
    pub available: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<FreeTierLimits>,
    #[serde(default)]
    pub conditions: String,
}

/// Free tier limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreeTierLimits {
    #[serde(rename = "rpm")]
    pub rpm: u32,
    #[serde(rename = "rpd")]
    pub rpd: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens: Option<u32>,
}

/// Model specifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Specifications {
    pub parameters: String,
    pub context_window: u32,
    pub architecture: String,
    #[serde(default)]
    pub modality: Vec<String>,
}

/// Benchmark scores
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Benchmarks {
    pub source: String,
    pub scores: std::collections::HashMap<String, f64>,
}

/// Availability information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Availability {
    #[serde(default)]
    pub hosted: Vec<String>,
    pub open_weights: bool,
    #[serde(default)]
    pub license: String,
}

/// News reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsReference {
    pub title: String,
    pub url: String,
    pub source: String,
    pub published_at: String,
    #[serde(default)]
    pub sentiment: String,
}

/// Validation flags from AI verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationFlags {
    #[serde(default)]
    pub community_verified: bool,
    #[serde(default)]
    pub official_announcement: bool,
    #[serde(default)]
    pub anomaly_detected: bool,
    #[serde(default = "default_confidence")]
    pub confidence: u8,
}

fn default_confidence() -> u8 {
    50
}

/// Metadata for the dataset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub last_updated: String,
    pub sources: Vec<String>,
    pub validation_status: String,
}

/// Complete API response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelData {
    pub metadata: Metadata,
    pub models: Vec<Model>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_model_deserialization() {
        let json = r#"{
            "id": "test-1",
            "name": "Test Model",
            "provider": "TestProvider",
            "release_date": "2024-01-01",
            "is_free": true,
            "free_tier": {
                "available": true,
                "conditions": ""
            },
            "specifications": {
                "parameters": "7B",
                "context_window": 4096,
                "architecture": "Transformer",
                "modality": ["text"]
            },
            "validation_flags": {
                "community_verified": true,
                "official_announcement": true,
                "anomaly_detected": false,
                "confidence": 95
            },
            "news_references": []
        }"#;
        
        let model: Model = serde_json::from_str(json).unwrap();
        assert_eq!(model.name, "Test Model");
        assert!(model.is_free);
        assert_eq!(model.validation_flags.confidence, 95);
    }
    
    #[test]
    fn test_default_confidence() {
        let json = r#"{
            "id": "test-2",
            "name": "Test",
            "provider": "Test",
            "release_date": "2024-01-01",
            "is_free": false,
            "free_tier": {"available": false},
            "specifications": {
                "parameters": "1B",
                "context_window": 2048,
                "architecture": "Transformer",
                "modality": []
            },
            "validation_flags": {}
        }"#;
        
        let model: Model = serde_json::from_str(json).unwrap();
        assert_eq!(model.validation_flags.confidence, 50); // default
    }
}
