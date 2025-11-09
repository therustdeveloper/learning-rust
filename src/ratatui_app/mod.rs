//! Ratatui TUI Application - Module Browser
//!
//! A professional terminal user interface for browsing all available learning modules
//! in this repository. Navigate with arrow keys, press Enter to view details, and 'q' to quit.

mod dashboard;

/// Main entry point following the repository's index pattern
pub fn index(show: bool) {
    if show {
        if let Err(e) = dashboard::run_app() {
            eprintln!("Error running ratatui app: {}", e);
        }
    }
}
