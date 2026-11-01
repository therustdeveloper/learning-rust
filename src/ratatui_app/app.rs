//! src/ratatui_app/app.rs
//!
//! Dashboard state: which module/section is selected and the last captured output.

use super::registry::{self, Section, TuiModule};
use crossterm::event::KeyCode;
use gag::BufferRedirect;
use ratatui::widgets::{ListState, ScrollbarState};
use std::io::{self, Read, Write};

/// Panel that receives ↑/↓; `Tab`/`→`/`l` and `Shift+Tab`/`←`/`h` cycle through them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Focus {
    Modules,
    Sections,
    Output,
}

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
    /// First visible line of the output panel; clamped by the renderer.
    pub output_scroll: u16,
    pub focus: Focus,
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
            output_scroll: 0,
            focus: Focus::Modules,
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

    /// Panels on screen, left to right then top to bottom; the submodule list
    /// only exists for modules with sections.
    fn panels(&self) -> &'static [Focus] {
        if self.has_sections() {
            &[Focus::Modules, Focus::Sections, Focus::Output]
        } else {
            &[Focus::Modules, Focus::Output]
        }
    }

    fn cycle_focus(&mut self, delta: isize) {
        let panels = self.panels();
        let current = panels.iter().position(|&p| p == self.focus).unwrap_or(0);
        if let Some(next) = wrap(current, delta, panels.len()) {
            self.focus = panels[next];
        }
    }

    fn clear_output(&mut self) {
        self.output = None;
        self.output_scroll = 0;
    }

    pub fn on_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Tab | KeyCode::Right | KeyCode::Char('l') => self.cycle_focus(1),
            KeyCode::BackTab | KeyCode::Left | KeyCode::Char('h') => self.cycle_focus(-1),
            KeyCode::Down | KeyCode::Char('j') => self.step(1),
            KeyCode::Up | KeyCode::Char('k') => self.step(-1),
            KeyCode::Home => self.jump(false),
            KeyCode::End => self.jump(true),
            KeyCode::Enter => self.run_selected(),
            KeyCode::Char('c') | KeyCode::Backspace => self.clear_output(),
            _ => {}
        }
    }

    /// Moves within the focused panel: selection in lists, one line in the output.
    fn step(&mut self, delta: isize) {
        match self.focus {
            Focus::Modules => self.step_module(delta),
            Focus::Sections => self.step_section(delta),
            Focus::Output if delta > 0 => self.output_scroll = self.output_scroll.saturating_add(1),
            Focus::Output => self.output_scroll = self.output_scroll.saturating_sub(1),
        }
    }

    /// Goes to the first or last entry of the focused panel.
    fn jump(&mut self, to_end: bool) {
        match self.focus {
            Focus::Modules => self.select_module(if to_end {
                self.modules.len().saturating_sub(1)
            } else {
                0
            }),
            Focus::Sections => {
                let len = self.selected_module().map_or(0, |m| m.sections().len());
                self.section_list
                    .select((len > 0).then(|| if to_end { len - 1 } else { 0 }));
            }
            // The renderer clamps the scroll to the last full page.
            Focus::Output => self.output_scroll = if to_end { u16::MAX } else { 0 },
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
        self.clear_output();
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
            None => match module.run {
                Some(run) if module.sections().is_empty() => (module.name(), run),
                _ => return,
            },
        };
        self.output = Some(Output {
            source,
            text: capture_stdout(run),
        });
        self.output_scroll = 0;
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

#[cfg(test)]
mod tests {
    use super::*;

    fn app_on(module: &str) -> App {
        let mut app = App::new();
        while app.selected_module().map(|m| m.name()) != Some(module) {
            app.on_key(KeyCode::Down);
        }
        app
    }

    #[test]
    fn forward_keys_cycle_panels_clockwise() {
        for key in [KeyCode::Tab, KeyCode::Right, KeyCode::Char('l')] {
            let mut app = app_on("basics");
            let mut seen = Vec::new();
            for _ in 0..3 {
                app.on_key(key);
                seen.push(app.focus);
            }
            assert_eq!(seen, [Focus::Sections, Focus::Output, Focus::Modules]);
        }
    }

    #[test]
    fn backward_keys_cycle_panels_counter_clockwise() {
        for key in [KeyCode::BackTab, KeyCode::Left, KeyCode::Char('h')] {
            let mut app = app_on("basics");
            let mut seen = Vec::new();
            for _ in 0..3 {
                app.on_key(key);
                seen.push(app.focus);
            }
            assert_eq!(seen, [Focus::Output, Focus::Sections, Focus::Modules]);
        }
    }

    #[test]
    fn up_down_move_within_focused_panel() {
        let mut app = app_on("basics");
        let module = app.module_list.selected();
        app.on_key(KeyCode::Char('l'));
        app.on_key(KeyCode::Char('j'));
        assert_eq!(app.section_list.selected(), Some(1));
        assert_eq!(app.module_list.selected(), module);
        app.on_key(KeyCode::Up);
        assert_eq!(app.section_list.selected(), Some(0));
    }

    #[test]
    fn modules_without_sections_skip_the_submodule_panel() {
        let mut app = App::new();
        while app.has_sections() {
            app.on_key(KeyCode::Down);
        }
        app.on_key(KeyCode::Tab);
        assert_eq!(app.focus, Focus::Output);
        app.on_key(KeyCode::Tab);
        assert_eq!(app.focus, Focus::Modules);
    }
}
