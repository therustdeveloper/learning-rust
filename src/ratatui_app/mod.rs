//! Ratatui TUI Application - Module Browser
//!
//! A professional terminal user interface for browsing all available learning modules
//! in this repository. Navigate with arrow keys, press Enter to view details, and 'q' to quit.
//!
//! Modules appear here by registering themselves with [`tui_module!`](crate::tui_module).

mod app;
mod dashboard;
mod registry;
mod ui;

pub use registry::{Category, Content, Section, TuiModule};

/// Main entry point following the repository's index pattern
pub fn index(show: bool) {
    if show {
        if let Err(e) = dashboard::run_app() {
            eprintln!("Error running Ratatui app: {}", e);
        }
    }
}
