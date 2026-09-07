//! Hook for fetching and managing model data

use dioxus::prelude::*;
use crate::store::{AppState, AppStateSignal, initialize_app_state};
use crate::types::ModelData;

/// Custom hook to fetch and manage model data
pub fn use_model_data() -> UseModelDataReturn {
    let mut state = use_signal(AppState::default);
    
    // Fetch data on mount
    use_effect(move || {
        spawn(async move {
            state.write().loading = true;
            
            match initialize_app_state().await {
                Ok(data) => {
                    let mut state = state.write();
                    state.models = data.models;
                    state.loading = false;
                    state.error = None;
                }
                Err(e) => {
                    let mut state = state.write();
                    state.loading = false;
                    state.error = Some(e);
                }
            }
        });
    });
    
    UseModelDataReturn {
        signal: state,
    }
}

#[derive(Clone, Copy)]
pub struct UseModelDataReturn {
    pub signal: AppStateSignal,
}

impl UseModelDataReturn {
    pub fn loading(&self) -> bool {
        self.signal.read().loading
    }
    
    pub fn error(&self) -> Option<String> {
        self.signal.read().error.clone()
    }
    
    pub fn models(&self) -> Vec<crate::types::Model> {
        self.signal.read().models.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_use_model_data_initial_state() {
        // Test that initial state has loading=true and empty models
        let state = AppState::default();
        assert!(state.loading);
        assert!(state.models.is_empty());
        assert!(state.error.is_none());
    }
}
