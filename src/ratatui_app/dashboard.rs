//! src/ratatui_app/dashboard.rs
//!
//! Event loop of the module browser. Modules are not listed here: each one
//! registers itself with `crate::tui_module!` in its own `mod.rs`.

use super::{app::App, ui};
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::DefaultTerminal;
use std::io;

/// Run the TUI application
pub fn run_app() -> io::Result<()> {
    ratatui::run(|terminal| event_loop(terminal, &mut App::new()))
}

fn event_loop(terminal: &mut DefaultTerminal, app: &mut App) -> io::Result<()> {
    while !app.should_quit {
        terminal.draw(|frame| ui::draw(frame, app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                app.on_key(key.code);
            }
        }
    }
    Ok(())
}
