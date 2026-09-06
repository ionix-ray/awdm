//! Main entry point for the AI Model Dashboard frontend

use dioxus::prelude::*;
use shared::{DashboardData, Model, ValidationStatus};

mod config;
mod components;
mod pages;
mod utils;

fn main() {
    // Initialize panic hook for better error messages in WASM
    console_error_panic_hook::set_once();
    
    // Initialize logger
    dioxus_logger::init(log::LevelFilter::Info).expect("failed to init logger");

    log::info!("Starting AI Model Dashboard...");

    launch(App);
}

/// Root application component
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/css/main.css") }
        
        div { class: "app-container",
            Header {}
            
            main { class: "main-content",
                Router::<Route> {}
            }
            
            Footer {}
        }
    }
}

/// Application routes
#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[layout(HomePageLayout)]
        #[route("/")]
        Home {},
        #[route("/models")]
        ModelsList {},
        #[route("/model/:id")]
        ModelDetail { id: String },
        #[route("/providers")]
        Providers {},
        #[route("/about")]
        About {},
    #[end_layout]
    
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

/// Home page layout with navigation
#[component]
fn HomePageLayout() -> Element {
    rsx! {
        Outlet::<Route> {}
    }
}

/// Header component with navigation
#[component]
fn Header() -> Element {
    rsx! {
        header { class: "app-header",
            div { class: "header-content",
                Link { to: Route::Home {},
                    div { class: "logo",
                        svg_icon("ai-brain")
                        span { "AI Model Dashboard" }
                    }
                }
                
                nav { class: "main-nav",
                    Link { to: Route::Home {}, "Home" }
                    Link { to: Route::ModelsList {}, "Models" }
                    Link { to: Route::Providers {}, "Providers" }
                    Link { to: Route::About {}, "About" }
                }
                
                div { class: "header-actions",
                    SearchBar {}
                }
            }
        }
    }
}

/// Footer component
#[component]
fn Footer() -> Element {
    let current_year = chrono::Utc::now().year();
    
    rsx! {
        footer { class: "app-footer",
            div { class: "footer-content",
                p { 
                    "© {current_year} AI Model Intelligence Dashboard. "
                    "Built with Rust + Dioxus."
                }
                div { class: "footer-links",
                    a { href: "https://github.com/ionix-ray/ai-model-dashboard",
                        "GitHub"
                    }
                    a { href: "#", "Documentation" }
                    a { href: "#", "API" }
                }
                div { class: "last-updated",
                    LastUpdatedDisplay {}
                }
            }
        }
    }
}

/// Home page component
#[component]
fn Home() -> Element {
    rsx! {
        div { class: "home-page",
            HeroSection {}
            StatsOverview {}
            RecentModels { limit: 6 }
            FeatureHighlights {}
        }
    }
}

/// Models list page with search and filter
#[component]
fn ModelsList() -> Element {
    rsx! {
        div { class: "models-page",
            h1 { class: "page-title", "AI Models" }
            
            div { class: "filters-section",
                FilterPanel {}
            }
            
            div { class: "models-grid",
                ModelsGrid {}
            }
            
            div { class: "pagination-section",
                Pagination {}
            }
        }
    }
}

/// Model detail page
#[component]
fn ModelDetail(id: String) -> Element {
    rsx! {
        div { class: "model-detail-page",
            BackButton {}
            ModelCard { model_id: id.clone(), detailed: true }
            ModelBenchmarks { model_id: id.clone() }
            ModelNews { model_id: id }
        }
    }
}

/// Providers page
#[component]
fn Providers() -> Element {
    rsx! {
        div { class: "providers-page",
            h1 { class: "page-title", "AI Providers" }
            ProvidersGrid {}
        }
    }
}

/// About page
#[component]
fn About() -> Element {
    rsx! {
        div { class: "about-page",
            h1 { class: "page-title", "About" }
            div { class: "about-content",
                h2 { "Purpose" }
                p { 
                    "This dashboard provides real-time intelligence on AI model releases, "
                    "free tiers, and provider updates for developers and researchers."
                }
                
                h2 { "Features" }
                ul { class: "feature-list",
                    li { "Daily automated scanning of Hugging Face, GitHub, and official blogs" }
                    li { "AI-validated data using OpenCode and other providers" }
                    li { "Free tier tracking with clear usage limits" }
                    li { "Advanced search and filtering" }
                    li { "Community-verified benchmarks" }
                }
                
                h2 { "Technology" }
                p { 
                    "Built with Rust and Dioxus for blazing-fast performance. "
                    "Hosted on GitHub Pages with automated daily updates."
                }
            }
        }
    }
}

/// Not found page
#[component]
fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        div { class: "not-found-page",
            h1 { "404 - Page Not Found" }
            p { "The page you're looking for doesn't exist." }
            Link { to: Route::Home {}, "Return Home" }
        }
    }
}

// Stub components - to be implemented in separate files
#[component]
fn HeroSection() -> Element { rsx! { div { class: "hero-section", "Hero content here" } } }
#[component]
fn StatsOverview() -> Element { rsx! { div { class: "stats-overview", "Stats here" } } }
#[component]
fn RecentModels(limit: usize) -> Element { rsx! { div { "Recent models" } } }
#[component]
fn FeatureHighlights() -> Element { rsx! { div { "Features" } } }
#[component]
fn FilterPanel() -> Element { rsx! { div { class: "filter-panel", "Filters" } } }
#[component]
fn ModelsGrid() -> Element { rsx! { div { class: "models-grid", "Models grid" } } }
#[component]
fn Pagination() -> Element { rsx! { div { class: "pagination", "Pagination controls" } } }
#[component]
fn BackButton() -> Element { rsx! { button { class: "back-button", "← Back" } } }
#[component]
fn ModelCard(model_id: String, detailed: bool) -> Element { 
    rsx! { div { class: "model-card", "Model card for {model_id}" } } 
}
#[component]
fn ModelBenchmarks(model_id: String) -> Element { rsx! { div { "Benchmarks" } } }
#[component]
fn ModelNews(model_id: String) -> Element { rsx! { div { "News" } } }
#[component]
fn ProvidersGrid() -> Element { rsx! { div { "Providers grid" } } }
#[component]
fn SearchBar() -> Element { rsx! { input { placeholder: "Search models..." } } }
#[component]
fn LastUpdatedDisplay() -> Element { rsx! { span { "Last updated: Loading..." } } }

/// Helper function to render Carbon Design System icons
fn svg_icon(name: &str) -> Element {
    // Simplified - will use actual Carbon icons in implementation
    rsx! {
        svg { 
            class: "carbon-icon",
            width: "24",
            height: "24",
            viewBox: "0 0 32 32",
            fill: "currentColor",
            path { d: "M16 2a14 14 0 1 0 14 14A14 14 0 0 0 16 2zm0 26a12 12 0 1 1 12-12 12 12 0 0 1-12 12z" }
        }
    }
}
