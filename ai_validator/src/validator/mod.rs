//! Data validation module with schema validation and anomaly detection

use crate::{ValidationError, Result};
use serde_json::Value;
use sha2::{Sha256, Digest};

pub struct DataValidator {
    strict_mode: bool,
}

impl DataValidator {
    pub fn new() -> Self {
        Self { strict_mode: true }
    }

    pub fn with_strict_mode(strict: bool) -> Self {
        Self { strict_mode: strict }
    }

    /// Validate JSON data against expected schema
    pub async fn validate_json(&self, json_data: &str) -> Result<Value> {
        let parsed: Value = serde_json::from_str(json_data)
            .map_err(|e| ValidationError::SchemaError(format!("Invalid JSON: {}", e)))?;
        
        // Basic schema validation
        self.validate_structure(&parsed)?;
        
        // Check for anomalies
        if self.strict_mode {
            self.detect_anomalies(&parsed)?;
        }
        
        Ok(parsed)
    }

    /// Validate the basic structure of model data
    fn validate_structure(&self, data: &Value) -> Result<()> {
        let obj = data.as_object()
            .ok_or_else(|| ValidationError::ValidationFailed("Root must be an object".to_string()))?;
        
        // Check required fields for model intelligence data
        if !obj.contains_key("metadata") {
            return Err(ValidationError::ValidationFailed("Missing 'metadata' field".to_string()));
        }
        
        if !obj.contains_key("models") {
            return Err(ValidationError::ValidationFailed("Missing 'models' field".to_string()));
        }
        
        Ok(())
    }

    /// Detect potential anomalies in the data
    fn detect_anomalies(&self, data: &Value) -> Result<()> {
        // Simple anomaly detection - can be enhanced with AI
        if let Some(models) = data.get("models").and_then(|m| m.as_array()) {
            for model in models {
                if let Some(obj) = model.as_object() {
                    // Check for suspicious benchmark scores
                    if let Some(benchmarks) = obj.get("benchmarks") {
                        if let Some(scores) = benchmarks.get("scores") {
                            if let Some(score_obj) = scores.as_object() {
                                for (_, score_value) in score_obj {
                                    if let Some(score) = score_value.as_f64() {
                                        if score > 100.0 || score < 0.0 {
                                            return Err(ValidationError::ValidationFailed(
                                                format!("Anomaly detected: Invalid benchmark score {}", score)
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }

    /// Generate a hash for data integrity verification
    pub fn generate_hash(&self, data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

impl Default for DataValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_valid_json() {
        let validator = DataValidator::new();
        let valid_json = r#"{
            "metadata": {"lastUpdated": "2024-01-01"},
            "models": []
        }"#;
        
        let result = validator.validate_json(valid_json).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_invalid_json_structure() {
        let validator = DataValidator::new();
        let invalid_json = r#"{"data": []}"#;
        
        let result = validator.validate_json(invalid_json).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_hash_generation() {
        let validator = DataValidator::new();
        let hash1 = validator.generate_hash("test data");
        let hash2 = validator.generate_hash("test data");
        let hash3 = validator.generate_hash("different data");
        
        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }
}
