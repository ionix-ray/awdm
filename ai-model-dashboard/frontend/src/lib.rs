//! Frontend library for AI Model Dashboard
//! 
//! Re-exports all modules for use in tests and the main binary.

pub mod config;
pub mod components;
pub mod pages;
pub mod utils;

// Re-export commonly used items
pub use config::{load_config, AppConfig, AiProvider, ValidationPrompt, PromptMode};
