//! AI Model Intelligence Dashboard - Dioxus Frontend
//! 
//! A high-performance WASM-based dashboard for tracking AI model releases,
//! free tiers, and updates using Dioxus framework.

use dioxus::prelude::*;
use log::info;
use wasm_bindgen::prelude::*;

mod components;
mod hooks;
mod store;
mod types;
mod utils;

use components::{App, ErrorBoundary};

#[wasm_bindgen(start)]
pub fn run_app() {
    // Initialize logger
    wasm_logger::init(wasm_logger::Config::default());
    
    info!("🚀 Initializing AI Model Intelligence Dashboard");
    
    // Set panic hook for better error messages
    console_error_panic_hook::set_once();
    
    // Launch the Dioxus app
    dioxus_web::launch(App);
}

/// Root App Component
#[component]
fn App() -> Element {
    rsx! {
        ErrorBoundary {
            div { class: "app-container",
                Header {}
                main {
                    StatsBar {}
                    SearchBar {}
                    FilterPanel {}
                    ModelGrid {}
                    Pagination {}
                }
                Footer {}
            }
        }
    }
}

#[component]
fn Header() -> Element {
    rsx! {
        header {
            style: "background: linear-gradient(135deg, #0f62fe 0%, #0043ce 100%); color: white; padding: 2rem 0; margin-bottom: 2rem;",
            div { style: "max-width: 1400px; margin: 0 auto; padding: 0 2rem;",
                h1 { 
                    style: "margin: 0 0 0.5rem; font-size: 2rem; font-weight: 700;",
                    "AI Model Intelligence Dashboard"
                }
                p { 
                    style: "margin: 0; font-size: 1rem; opacity: 0.9; max-width: 600px;",
                    "Real-time tracking of AI model releases, free tiers, and updates for developers and researchers"
                }
            }
        }
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        footer {
            style: "background: #161616; color: #ffffff; padding: 2rem; margin-top: 4rem;",
            div { style: "max-width: 1400px; margin: 0 auto; text-align: center;",
                p { style: "margin: 0 0 0.5rem; font-size: 0.875rem;",
                    "AI Model Intelligence Dashboard"
                }
                p { style: "margin: 0; font-size: 0.75rem; opacity: 0.7;",
                    "Built with Dioxus (Rust/WASM) and Carbon Design System"
                }
                p { style: "margin: 0.5rem 0 0; font-size: 0.75rem; opacity: 0.5;",
                    "Data updated via GitHub Actions • Hosted on GitHub Pages"
                }
            }
        }
    }
}

#[component]
fn StatsBar() -> Element {
    rsx! {
        div { class: "stats-bar", style: "display: flex; gap: 1rem; margin-bottom: 2rem; flex-wrap: wrap;",
            // Stats will be populated from store
            div { class: "stat-card skeleton", style: "flex: 1; min-width: 200px; padding: 1rem; background: #f4f4f4; border-radius: 8px;",
                "Loading stats..."
            }
        }
    }
}

#[component]
fn SearchBar() -> Element {
    let mut search_query = use_signal(|| String::new());
    
    rsx! {
        div { style: "margin-bottom: 1.5rem;",
            input {
                r#type: "text",
                placeholder: "Search models...",
                value: "{search_query}",
                oninput: move |e| search_query.set(e.value()),
                style: "width: 100%; max-width: 400px; padding: 0.75rem; border: 1px solid #ccc; border-radius: 8px; font-size: 1rem;",
            }
        }
    }
}

#[component]
fn FilterPanel() -> Element {
    rsx! {
        div { class: "filter-panel", style: "margin-bottom: 1.5rem; padding: 1rem; background: #f4f4f4; border-radius: 8px;",
            h3 { style: "margin: 0 0 1rem; font-size: 1.25rem;", "Filters" }
            // Filters will be implemented with multi-select
            div { "Filter options coming soon..." }
        }
    }
}

#[component]
fn ModelGrid() -> Element {
    rsx! {
        div { class: "model-grid",
            // Models will be rendered from store
            div { class: "skeleton", style: "height: 300px;", "Loading models..." }
            div { class: "skeleton", style: "height: 300px;", "Loading models..." }
            div { class: "skeleton", style: "height: 300px;", "Loading models..." }
        }
    }
}

#[component]
fn Pagination() -> Element {
    rsx! {
        div { style: "margin-top: 2rem; display: flex; justify-content: center; gap: 0.5rem;",
            button { "Previous" }
            span { "Page 1 of N" }
            button { "Next" }
        }
    }
}
