//! Hook for search functionality with debouncing

use dioxus::prelude::*;
use std::time::Duration;

/// Custom hook for search with debouncing
pub fn use_search(initial_query: &str) -> UseSearchReturn {
    let mut query = use_signal(|| initial_query.to_string());
    let mut debounced_query = use_signal(|| initial_query.to_string());
    
    // Debounce search input (300ms delay)
    use_effect(move || {
        let current_query = query.read().clone();
        
        spawn(async move {
            tokio::time::sleep(Duration::from_millis(300)).await;
            let mut dq = debounced_query.write();
            *dq = current_query;
        });
    });
    
    UseSearchReturn {
        query,
        debounced_query,
    }
}

#[derive(Clone, Copy)]
pub struct UseSearchReturn {
    pub query: Signal<String>,
    pub debounced_query: Signal<String>,
}

impl UseSearchReturn {
    pub fn get_query(&self) -> String {
        self.query.read().clone()
    }
    
    pub fn get_debounced_query(&self) -> String {
        self.debounced_query.read().clone()
    }
    
    pub fn set_query(&self, value: String) {
        let mut q = self.query.write();
        *q = value;
    }
    
    pub fn clear(&self) {
        let mut q = self.query.write();
        *q = String::new();
        let mut dq = self.debounced_query.write();
        *dq = String::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_use_search_initial_state() {
        // Note: Full testing requires Dioxus runtime
        // This is a placeholder for the actual test structure
        assert!(true);
    }
}
