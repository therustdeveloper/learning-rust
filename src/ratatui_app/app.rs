//! src/ratatui_app/app.rs
//!
//! Dashboard state: which module/section is selected and the last captured output.

use super::registry::{self, Section, TuiModule};
use crossterm::event::KeyCode;
use gag::BufferRedirect;
use ratatui::widgets::{ListState, ScrollbarState};
use std::io::{self, Read, Write};

/// Captured stdout of the last module or section that was run.
pub struct Output {
    pub source: &'static str,
    pub text: String,
}

pub struct App {
    pub modules: Vec<&'static TuiModule>,
    pub module_list: ListState,
    pub module_scroll: ScrollbarState,
    pub section_list: ListState,
    pub output: Option<Output>,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        let modules = registry::modules();
        let mut app = Self {
            module_scroll: ScrollbarState::new(modules.len()),
            modules,
            module_list: ListState::default(),
            section_list: ListState::default(),
            output: None,
            should_quit: false,
        };
        app.select_module(0);
        app
    }

    pub fn selected_module(&self) -> Option<&'static TuiModule> {
        self.module_list
            .selected()
            .and_then(|i| self.modules.get(i).copied())
    }

    pub fn selected_section(&self) -> Option<&'static Section> {
        let sections = self.selected_module()?.sections();
        sections.get(self.section_list.selected()?)
    }

    fn has_sections(&self) -> bool {
        self.selected_module()
            .is_some_and(|module| !module.sections().is_empty())
    }

    pub fn on_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Down | KeyCode::Char('j') if self.has_sections() => self.step_section(1),
            KeyCode::Up | KeyCode::Char('k') if self.has_sections() => self.step_section(-1),
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Right | KeyCode::Char('l') => {
                self.step_module(1)
            }
            KeyCode::Up | KeyCode::Char('k') | KeyCode::Left | KeyCode::Char('h') => {
                self.step_module(-1)
            }
            KeyCode::Home => self.select_module(0),
            KeyCode::End => self.select_module(self.modules.len().saturating_sub(1)),
            KeyCode::Enter => self.run_selected(),
            KeyCode::Char('c') | KeyCode::Backspace => self.output = None,
            _ => {}
        }
    }

    fn select_module(&mut self, index: usize) {
        if index >= self.modules.len() {
            return;
        }
        self.module_list.select(Some(index));
        self.module_scroll = self.module_scroll.position(index);
        self.section_list
            .select((!self.modules[index].sections().is_empty()).then_some(0));
        self.output = None;
    }

    fn step_module(&mut self, delta: isize) {
        let current = self.module_list.selected().unwrap_or(0);
        if let Some(next) = wrap(current, delta, self.modules.len()) {
            self.select_module(next);
        }
    }

    fn step_section(&mut self, delta: isize) {
        let len = self
            .selected_module()
            .map_or(0, |module| module.sections().len());
        let current = self.section_list.selected().unwrap_or(0);
        self.section_list.select(wrap(current, delta, len));
    }

    fn run_selected(&mut self) {
        let Some(module) = self.selected_module() else {
            return;
        };
        let (source, run) = match self.selected_section() {
            Some(section) => (section.name, section.run),
            None if module.sections().is_empty() => (module.name(), module.run),
            None => return,
        };
        self.output = Some(Output {
            source,
            text: capture_stdout(run),
        });
    }
}

/// Moves `current` by `delta` within `0..len`, wrapping at both ends.
fn wrap(current: usize, delta: isize, len: usize) -> Option<usize> {
    (len > 0).then(|| (current as isize + delta).rem_euclid(len as isize) as usize)
}

/// Runs `run` and returns everything it printed to stdout.
fn capture_stdout(run: fn()) -> String {
    let mut buffer = match BufferRedirect::stdout() {
        Ok(buffer) => buffer,
        Err(error) => return format!("Unable to capture output: {error}"),
    };
    run();
    let _ = io::stdout().flush();

    let mut text = String::new();
    let _ = buffer.read_to_string(&mut text);
    if text.is_empty() {
        "No output produced by this module.".to_string()
    } else {
        text
    }
}
