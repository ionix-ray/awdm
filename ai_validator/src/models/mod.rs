//! Data models for AI Model Intelligence Dashboard

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Main data structure for the dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardData {
    pub metadata: Metadata,
    pub models: Vec<Model>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub last_updated: DateTime<Utc>,
    pub sources: Vec<String>,
    pub validation_status: String,
    pub data_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub release_date: DateTime<Utc>,
    pub is_free: bool,
    pub free_tier: Option<FreeTier>,
    pub specifications: Specifications,
    pub benchmarks: Option<Benchmarks>,
    pub availability: Availability,
    pub news_references: Vec<NewsReference>,
    pub validation_flags: ValidationFlags,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreeTier {
    pub available: bool,
    pub limits: Option<FreeLimits>,
    pub conditions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreeLimits {
    pub rpm: Option<u32>,      // Requests per minute
    pub rpd: Option<u32>,      // Requests per day
    pub tokens: Option<u64>,   // Token limit
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Specifications {
    pub parameters: Option<String>,
    pub context_window: Option<u32>,
    pub architecture: Option<String>,
    pub modality: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Benchmarks {
    pub source: String,
    pub scores: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Availability {
    pub hosted: Vec<String>,
    pub open_weights: bool,
    pub license: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsReference {
    pub title: String,
    pub url: String,
    pub source: String,
    pub published_at: DateTime<Utc>,
    pub sentiment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationFlags {
    pub community_verified: bool,
    pub official_announcement: bool,
    pub anomaly_detected: bool,
    pub confidence_score: u32,
}

impl DashboardData {
    pub fn new() -> Self {
        Self {
            metadata: Metadata {
                last_updated: Utc::now(),
                sources: vec![],
                validation_status: "pending".to_string(),
                data_hash: None,
            },
            models: vec![],
        }
    }

    pub fn add_model(&mut self, model: Model) {
        self.models.push(model);
    }

    pub fn model_count(&self) -> usize {
        self.models.len()
    }

    pub fn free_models_count(&self) -> usize {
        self.models.iter().filter(|m| m.is_free).count()
    }

    pub fn recent_releases(&self, days: u32) -> Vec<&Model> {
        use chrono::Duration;
        let cutoff = Utc::now() - Duration::days(days as i64);
        self.models
            .iter()
            .filter(|m| m.release_date >= cutoff)
            .collect()
    }
}

impl Default for DashboardData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dashboard_data_creation() {
        let data = DashboardData::new();
        assert_eq!(data.model_count(), 0);
        assert_eq!(data.metadata.validation_status, "pending");
    }

    #[test]
    fn test_model_addition() {
        let mut data = DashboardData::new();
        let model = Model {
            id: "test-1".to_string(),
            name: "Test Model".to_string(),
            provider: "TestProvider".to_string(),
            release_date: Utc::now(),
            is_free: true,
            free_tier: Some(FreeTier {
                available: true,
                limits: Some(FreeLimits {
                    rpm: Some(60),
                    rpd: Some(1000),
                    tokens: None,
                }),
                conditions: "No credit card required".to_string(),
            }),
            specifications: Specifications {
                parameters: Some("7B".to_string()),
                context_window: Some(4096),
                architecture: Some("Transformer".to_string()),
                modality: vec!["text".to_string()],
            },
            benchmarks: None,
            availability: Availability {
                hosted: vec!["HuggingFace".to_string()],
                open_weights: true,
                license: "MIT".to_string(),
            },
            news_references: vec![],
            validation_flags: ValidationFlags {
                community_verified: false,
                official_announcement: false,
                anomaly_detected: false,
                confidence_score: 50,
            },
        };

        data.add_model(model);
        assert_eq!(data.model_count(), 1);
        assert!(data.free_models_count() > 0);
    }
}
