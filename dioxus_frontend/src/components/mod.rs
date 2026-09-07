//! Components module for Dioxus frontend

mod error_boundary;
mod model_card;

pub use error_boundary::ErrorBoundary;
pub use model_card::ModelCard;

// Re-export main components from lib.rs
pub use crate::{Header, Footer, StatsBar, SearchBar, FilterPanel, ModelGrid, Pagination};
