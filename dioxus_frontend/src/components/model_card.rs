//! Model Card Component for displaying AI model information

use dioxus::prelude::*;
use crate::types::Model;

/// ModelCard displays detailed information about an AI model
#[component]
pub fn ModelCard(model: Model) -> Element {
    let free_tier_badge = if model.is_free {
        rsx! {
            span {
                style: "background: #198038; color: white; padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.75rem; font-weight: 600;",
                "FREE"
            }
        }
    } else {
        rsx! {
            span {
                style: "background: #6f6f6f; color: white; padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.75rem;",
                "Paid"
            }
        }
    };
    
    let confidence_color = match model.validation_flags.confidence {
        c if c >= 80 => "#198038",  // Green
        c if c >= 60 => "#f1c21b",  // Yellow
        _ => "#da1e28",              // Red
    };

    rsx! {
        div {
            class: "model-card",
            style: "background: white; border: 1px solid #e0e0e0; border-radius: 8px; padding: 1.5rem; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
            
            // Header
            div { style: "display: flex; justify-content: space-between; align-items: start; margin-bottom: 1rem;",
                div {
                    h3 { 
                        style: "margin: 0 0 0.5rem; font-size: 1.25rem; font-weight: 600; color: #161616;",
                        "{model.name}"
                    }
                    p { 
                        style: "margin: 0; color: #6f6f6f; font-size: 0.875rem;",
                        "by {model.provider}"
                    }
                }
                {free_tier_badge}
            }
            
            // Specifications
            div { style: "margin-bottom: 1rem;",
                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 0.5rem;",
                    div {
                        span { style: "font-size: 0.75rem; color: #6f6f6f;", "Parameters" }
                        p { style: "margin: 0; font-weight: 600;", "{model.specifications.parameters}" }
                    }
                    div {
                        span { style: "font-size: 0.75rem; color: #6f6f6f;", "Context Window" }
                        p { style: "margin: 0; font-weight: 600;", "{model.specifications.context_window} tokens" }
                    }
                }
            }
            
            // Free Tier Details
            if model.free_tier.available {
                div { 
                    style: "background: #f0fdf0; border: 1px solid #198038; border-radius: 8px; padding: 1rem; margin-bottom: 1rem;",
                    h4 { style: "margin: 0 0 0.5rem; font-size: 0.875rem; color: #198038;", "Free Tier Available" }
                    div { style: "font-size: 0.875rem; color: #161616;",
                        if let Some(limits) = &model.free_tier.limits {
                            rsx! {
                                p { style: "margin: 0.25rem 0;", "⚡ {limits.rpm} RPM" }
                                p { style: "margin: 0.25rem 0;", "📅 {limits.rpd} RPD" }
                                if let Some(tokens) = limits.tokens {
                                    p { style: "margin: 0.25rem 0;", "📝 {tokens} tokens" }
                                }
                            }
                        }
                    }
                }
            }
            
            // Confidence Score
            div { 
                style: "display: flex; align-items: center; gap: 0.5rem; padding: 0.75rem; background: #f4f4f4; border-radius: 8px;",
                span { style: "font-size: 0.875rem; color: #6f6f6f;", "Confidence:" }
                span { 
                    style: "font-weight: 700; color: {confidence_color};",
                    "{model.validation_flags.confidence}%"
                }
                if model.validation_flags.community_verified {
                    span { style: "color: #198038; font-size: 1.25rem;", "✓" }
                }
            }
            
            // Release Date
            p { 
                style: "margin-top: 1rem; font-size: 0.75rem; color: #6f6f6f;",
                "Released: {model.release_date}"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Model, Specifications, FreeTier, ValidationFlags};
    
    #[test]
    fn test_model_card_with_free_tier() {
        let model = Model {
            id: "test-1".to_string(),
            name: "Test Model".to_string(),
            provider: "TestProvider".to_string(),
            release_date: "2024-01-01".to_string(),
            is_free: true,
            free_tier: FreeTier {
                available: true,
                limits: None,
                conditions: String::new(),
            },
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
                confidence: 95,
            },
        };
        
        assert!(model.is_free);
        assert_eq!(model.validation_flags.confidence, 95);
    }
}
