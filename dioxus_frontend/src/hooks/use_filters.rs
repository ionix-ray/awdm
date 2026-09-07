//! Hook for filter functionality

use dioxus::prelude::*;
use crate::store::FilterOptions;

/// Custom hook for managing filters
pub fn use_filters() -> UseFiltersReturn {
    let mut filters = use_signal(FilterOptions::default);
    
    UseFiltersReturn { filters }
}

#[derive(Clone, Copy)]
pub struct UseFiltersReturn {
    pub filters: Signal<FilterOptions>,
}

impl UseFiltersReturn {
    pub fn toggle_free_only(&self) {
        let mut f = self.filters.write();
        f.free_only = !f.free_only;
    }
    
    pub fn set_free_only(&self, value: bool) {
        let mut f = self.filters.write();
        f.free_only = value;
    }
    
    pub fn toggle_provider(&self, provider: String) {
        let mut f = self.filters.write();
        if f.providers.contains(&provider) {
            f.providers.retain(|p| p != &provider);
        } else {
            f.providers.push(provider);
        }
    }
    
    pub fn set_min_confidence(&self, value: u8) {
        let mut f = self.filters.write();
        f.min_confidence = value;
    }
    
    pub fn clear_all(&self) {
        let mut f = self.filters.write();
        *f = FilterOptions::default();
    }
    
    pub fn get_filters(&self) -> FilterOptions {
        self.filters.read().clone()
    }
    
    pub fn is_active(&self) -> bool {
        let f = self.filters.read();
        f.free_only || !f.providers.is_empty() || f.min_confidence > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_filter_options_default() {
        let filters = FilterOptions::default();
        assert!(!filters.free_only);
        assert!(filters.providers.is_empty());
        assert_eq!(filters.min_confidence, 0);
    }
}
