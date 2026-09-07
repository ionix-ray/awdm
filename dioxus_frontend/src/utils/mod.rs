//! Utility functions for the dashboard

use wasm_bindgen::JsCast;
use web_sys::{window, Document, Window};

/// Get the browser window
pub fn get_window() -> Option<Window> {
    window()
}

/// Get the document
pub fn get_document() -> Option<Document> {
    window().and_then(|win| win.document().ok())
}

/// Log to browser console
pub fn log_info(message: &str) {
    if let Some(window) = get_window() {
        let _ = window.console().log1(&message.into());
    }
}

/// Log error to browser console
pub fn log_error(message: &str) {
    if let Some(window) = get_window() {
        let _ = window.console().error1(&message.into());
    }
}

/// Format date for display
pub fn format_date(iso_date: &str) -> String {
    match chrono::NaiveDate::parse_from_str(iso_date, "%Y-%m-%d") {
        Ok(date) => date.format("%B %d, %Y").to_string(),
        Err(_) => iso_date.to_string(),
    }
}

/// Calculate relative time (e.g., "2 days ago")
pub fn relative_time(iso_date: &str) -> String {
    match chrono::NaiveDate::parse_from_str(iso_date, "%Y-%m-%d") {
        Ok(date) => {
            let today = chrono::Local::now().date_naive();
            let days = (today - date).num_days();
            
            match days {
                0 => "Today".to_string(),
                1 => "Yesterday".to_string(),
                d if d < 7 => format!("{} days ago", d),
                d if d < 30 => format!("{} weeks ago", d / 7),
                d if d < 365 => format!("{} months ago", d / 30),
                d => format!("{} years ago", d / 365),
            }
        }
        Err(_) => iso_date.to_string(),
    }
}

/// Sanitize HTML to prevent XSS
pub fn sanitize_html(input: &str) -> String {
    // Basic HTML entity encoding
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// Check if running in WASM environment
pub fn is_wasm() -> bool {
    cfg!(target_arch = "wasm32")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_date() {
        let formatted = format_date("2024-01-15");
        assert!(formatted.contains("2024"));
    }
    
    #[test]
    fn test_sanitize_html() {
        let malicious = "<script>alert('xss')</script>";
        let sanitized = sanitize_html(malicious);
        assert!(!sanitized.contains("<script>"));
        assert!(sanitized.contains("&lt;script&gt;"));
    }
    
    #[test]
    fn test_relative_time_today() {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let relative = relative_time(&today);
        assert_eq!(relative, "Today");
    }
}
