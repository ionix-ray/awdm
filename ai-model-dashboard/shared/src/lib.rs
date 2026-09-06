//! Shared types for AI Model Dashboard
//! 
//! These types are used by both the data pipeline (GitHub Actions)
//! and the frontend (Dioxus) to ensure type safety across the stack.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

// ============================================================================
// Core Data Models
// ============================================================================

/// Main dashboard data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardData {
    pub metadata: Metadata,
    pub models: Vec<Model>,
}

/// Metadata about the data generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub last_updated: DateTime<Utc>,
    pub sources: Vec<String>,
    pub validation_status: ValidationStatus,
    pub total_models: usize,
    pub free_models_count: usize,
    pub new_releases_7d: usize,
    pub high_confidence_count: usize,
    pub unique_providers: Vec<String>,
}

/// Overall validation status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ValidationStatus {
    FullyValidated,
    PartiallyValidated,
    PendingValidation,
    ValidationFailed,
}

/// Individual AI model entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub release_date: DateTime<Utc>,
    pub is_free: bool,
    pub free_tier: Option<FreeTier>,
    pub specifications: ModelSpecifications,
    pub benchmarks: Option<Benchmarks>,
    pub availability: Availability,
    pub news_references: Vec<NewsReference>,
    pub validation_flags: ValidationFlags,
    pub confidence_score: f32,
    pub priority_level: PriorityLevel,
}

/// Free tier information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreeTier {
    pub available: bool,
    pub limits: FreeTierLimits,
    pub conditions: String,
}

/// Rate limits for free tier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreeTierLimits {
    pub rpm: Option<u32>,      // Requests per minute
    pub rpd: Option<u32>,      // Requests per day
    pub tokens: Option<u64>,   // Token limit
}

/// Model technical specifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSpecifications {
    pub parameters: Option<String>,
    pub context_window: Option<u32>,
    pub architecture: Option<String>,
    pub modality: Vec<ModelModality>,
    pub license: String,
}

/// Supported modalities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ModelModality {
    Text,
    Image,
    Audio,
    Video,
    Code,
    Multimodal,
}

/// Benchmark performance data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Benchmarks {
    pub source: String,
    pub scores: serde_json::Value,
    pub evaluated_at: Option<DateTime<Utc>>,
}

/// Model availability information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Availability {
    pub hosted_on: Vec<String>,
    pub open_weights: bool,
    pub huggingface_id: Option<String>,
    pub github_repo: Option<String>,
}

/// News reference about the model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsReference {
    pub title: String,
    pub url: String,
    pub source: String,
    pub published_at: DateTime<Utc>,
    pub sentiment: Sentiment,
}

/// Sentiment analysis result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Sentiment {
    Positive,
    Neutral,
    Negative,
}

/// Validation flags for trust scoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationFlags {
    pub community_verified: bool,
    pub official_announcement: bool,
    pub anomaly_detected: bool,
    pub cross_source_verified: bool,
}

/// Priority level for display
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PriorityLevel {
    Low,
    Medium,
    High,
    Critical,
}

// ============================================================================
// Configuration Models (for TOML configs)
// ============================================================================

/// AI Provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiProvider {
    pub name: String,
    pub base_url: String,
    pub api_key_env: String,
    pub models: Vec<String>,
}

/// Validation prompt configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationPrompt {
    pub name: String,
    pub mode: PromptMode,
    pub prompt_template: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub output_schema: String,
}

/// Prompt execution mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PromptMode {
    Zen,   // Cost-effective, faster
    Go,    // Maximum capability, thorough
}

/// Complete configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub default_mode: PromptMode,
    pub ai_providers: Vec<AiProvider>,
    pub validation_prompts: Vec<ValidationPrompt>,
    pub data_sources: Vec<DataSource>,
    pub schedule: ScheduleConfig,
}

/// Data source configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSource {
    pub name: String,
    pub url: String,
    pub scrape_interval_hours: u32,
    pub requires_auth: bool,
}

/// Schedule configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleConfig {
    pub cron_expression: String,
    pub timezone: String,
    pub max_runs_per_day: u32,
}

// ============================================================================
// API Response Types
// ============================================================================

/// Generic API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(msg: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg.into()),
        }
    }
}

// ============================================================================
// Search and Filter Types
// ============================================================================

/// Search query parameters
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchQuery {
    pub query: Option<String>,
    pub providers: Vec<String>,
    pub modalities: Vec<ModelModality>,
    pub free_only: bool,
    pub open_weights_only: bool,
    pub min_confidence: Option<f32>,
    pub sort_by: SortField,
    pub sort_order: SortOrder,
    pub page: u32,
    pub page_size: u32,
}

/// Sortable fields
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SortField {
    #[default]
    ReleaseDate,
    Name,
    Provider,
    ConfidenceScore,
}

/// Sort order
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    #[default]
    Desc,
    Asc,
}

/// Paginated search results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResults {
    pub items: Vec<Model>,
    pub total: usize,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

// ============================================================================
// Helper Implementations
// ============================================================================

impl Model {
    /// Create a new model with default values
    pub fn new(id: String, name: String, provider: String) -> Self {
        Self {
            id,
            name,
            provider,
            release_date: Utc::now(),
            is_free: false,
            free_tier: None,
            specifications: ModelSpecifications {
                parameters: None,
                context_window: None,
                architecture: None,
                modality: vec![ModelModality::Text],
                license: "Unknown".to_string(),
            },
            benchmarks: None,
            availability: Availability {
                hosted_on: vec![],
                open_weights: false,
                huggingface_id: None,
                github_repo: None,
            },
            news_references: vec![],
            validation_flags: ValidationFlags {
                community_verified: false,
                official_announcement: false,
                anomaly_detected: false,
                cross_source_verified: false,
            },
            confidence_score: 0.5,
            priority_level: PriorityLevel::Medium,
        }
    }
}

impl Default for ValidationStatus {
    fn default() -> Self {
        Self::PendingValidation
    }
}

impl Default for PriorityLevel {
    fn default() -> Self {
        Self::Medium
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_creation() {
        let model = Model::new(
            "test-1".to_string(),
            "Test Model".to_string(),
            "TestProvider".to_string(),
        );
        
        assert_eq!(model.id, "test-1");
        assert_eq!(model.name, "Test Model");
        assert_eq!(model.provider, "TestProvider");
        assert_eq!(model.confidence_score, 0.5);
        assert_eq!(model.priority_level, PriorityLevel::Medium);
    }

    #[test]
    fn test_api_response_ok() {
        let response: ApiResponse<String> = ApiResponse::ok("data".to_string());
        assert!(response.success);
        assert_eq!(response.data, Some("data".to_string()));
        assert!(response.error.is_none());
    }

    #[test]
    fn test_api_response_err() {
        let response: ApiResponse<String> = ApiResponse::err("error message");
        assert!(!response.success);
        assert!(response.data.is_none());
        assert_eq!(response.error, Some("error message".to_string()));
    }

    #[test]
    fn test_serialization() {
        let model = Model::new(
            "test-1".to_string(),
            "Test Model".to_string(),
            "TestProvider".to_string(),
        );
        
        let json = serde_json::to_string(&model).unwrap();
        assert!(json.contains("\"name\":\"Test Model\""));
        
        let deserialized: Model = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.name, model.name);
    }
}
