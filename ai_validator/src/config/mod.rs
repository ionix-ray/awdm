//! Configuration module for loading and managing config.toml

use crate::{AppConfig, ValidationError, Result};
use std::fs;
use std::path::Path;

/// Load configuration from a TOML file
pub fn load_config<P: AsRef<Path>>(config_path: P) -> Result<AppConfig> {
    let content = fs::read_to_string(config_path.as_ref())
        .map_err(|e| ValidationError::ConfigError(format!("Failed to read config file: {}", e)))?;
    
    let config: AppConfig = toml::from_str(&content)
        .map_err(|e| ValidationError::ConfigError(format!("Failed to parse TOML: {}", e)))?;
    
    Ok(config)
}

/// Load configuration from environment or default path
pub fn load_default_config() -> Result<AppConfig> {
    let paths = [
        "config.toml",
        "/workspace/config.toml",
        "./config.toml",
        "../config.toml",
    ];
    
    for path in &paths {
        if Path::new(path).exists() {
            return load_config(path);
        }
    }
    
    Err(ValidationError::ConfigError(
        "No configuration file found. Please provide config.toml".to_string()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config_from_file() {
        // This test will pass when config.toml exists
        let result = load_default_config();
        assert!(result.is_ok(), "Config should load successfully");
    }
}
