//! Dashboard - Main TUI implementation
//!
//! Contains the terminal user interface implementation for browsing learning modules.

// ToDo:
// refactor this massive code into a modular strategy
// each module must handle its own TUI configuration
// I want that everything reacts as an injected code

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use gag::BufferRedirect;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, Clear, List, ListItem, ListState, Padding, Paragraph, Scrollbar,
        ScrollbarOrientation, ScrollbarState, Wrap,
    },
    Frame, Terminal,
};
use std::io::{self, Read};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc, Arc,
};
use std::thread::{self, JoinHandle};

/// Submodule information structure
#[derive(Clone, Debug)]
struct Submodule {
    name: String,
    description: String,
}

/// Module information structure
#[derive(Clone)]
struct Module {
    name: String,
    description: String,
    category: String,
    submodules: Vec<Submodule>,
}

/// Application state
struct App {
    modules: Vec<Module>,
    list_state: ListState,
    scroll_state: ScrollbarState,
    selected_module: Option<Module>,
    module_output: Option<String>,
    output_scroll: u16,
    // Webserver threading support
    webserver_thread: Option<JoinHandle<()>>,
    webserver_shutdown: Arc<AtomicBool>,
    webserver_output_rx: Option<mpsc::Receiver<String>>,
    webserver_running: bool,
    webserver_output: String, // Keep webserver output separate
    // Futures threading support
    futures_thread: Option<JoinHandle<()>>,
    futures_shutdown: Arc<AtomicBool>,
    futures_output_rx: Option<mpsc::Receiver<String>>,
    futures_running: bool,
    futures_output: String, // Keep futures output separate
    // Network threading support
    network_thread: Option<JoinHandle<()>>,
    network_shutdown: Arc<AtomicBool>,
    network_output_rx: Option<mpsc::Receiver<String>>,
    network_running: bool,
    network_output: String, // Keep network output separate
    // Message passing threading support
    message_passing_thread: Option<JoinHandle<()>>,
    message_passing_shutdown: Arc<AtomicBool>,
    message_passing_output_rx: Option<mpsc::Receiver<String>>,
    message_passing_running: bool,
    message_passing_output: String, // Keep message_passing output separate
    // User input state
    input_mode: bool,
    input_buffer: String,
    input_prompt: String,
    user_input_result: String,
    // Submodule state
    submodule_list_state: ListState,
    selected_submodule_index: Option<usize>,
    submodule_output: Option<String>,
    submodule_scroll: u16,
}

impl App {
    fn new() -> Self {
        let modules = get_all_modules();
        let total_items = modules.len();

        let mut list_state = ListState::default();
        list_state.select(Some(0));

        let selected_module = modules.first().cloned();

        Self {
            modules,
            list_state,
            scroll_state: ScrollbarState::new(total_items),
            selected_module,
            module_output: None,
            output_scroll: 0,
            webserver_thread: None,
            webserver_shutdown: Arc::new(AtomicBool::new(false)),
            webserver_output_rx: None,
            webserver_running: false,
            webserver_output: String::new(),
            futures_thread: None,
            futures_shutdown: Arc::new(AtomicBool::new(false)),
            futures_output_rx: None,
            futures_running: false,
            futures_output: String::new(),
            network_thread: None,
            network_shutdown: Arc::new(AtomicBool::new(false)),
            network_output_rx: None,
            network_running: false,
            network_output: String::new(),
            message_passing_thread: None,
            message_passing_shutdown: Arc::new(AtomicBool::new(false)),
            message_passing_output_rx: None,
            message_passing_running: false,
            message_passing_output: String::new(),
            input_mode: false,
            input_buffer: String::new(),
            input_prompt: String::new(),
            user_input_result: String::new(),
            submodule_list_state: ListState::default(),
            selected_submodule_index: None,
            submodule_output: None,
            submodule_scroll: 0,
        }
    }

    fn select_first_submodule(&mut self) {
        if let Some(module) = &self.selected_module {
            if !module.submodules.is_empty() {
                self.submodule_list_state.select(Some(0));
                self.selected_submodule_index = Some(0);
                self.submodule_output = None;
                self.submodule_scroll = 0;
            }
        }
    }

    fn next_submodule(&mut self) {
        if let Some(module) = &self.selected_module {
            if module.submodules.is_empty() {
                return;
            }

            let i = match self.submodule_list_state.selected() {
                Some(i) => {
                    if i >= module.submodules.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.submodule_list_state.select(Some(i));
            self.selected_submodule_index = Some(i);
            self.submodule_scroll = 0;
        }
    }

    fn previous_submodule(&mut self) {
        if let Some(module) = &self.selected_module {
            if module.submodules.is_empty() {
                return;
            }

            let i = match self.submodule_list_state.selected() {
                Some(i) => {
                    if i == 0 {
                        module.submodules.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.submodule_list_state.select(Some(i));
            self.selected_submodule_index = Some(i);
            self.submodule_scroll = 0;
        }
    }

    fn next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.modules.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i);
        self.selected_module = self.modules.get(i).cloned();

        // Reset submodule state and select first submodule if module has submodules
        self.submodule_list_state.select(None);
        self.selected_submodule_index = None;
        self.submodule_output = None;
        self.submodule_scroll = 0;
        if let Some(module) = &self.selected_module {
            if !module.submodules.is_empty() {
                self.select_first_submodule();
            }
        }

        // Handle long-running modules when navigating
        if let Some(module) = &self.selected_module {
            if self.webserver_running && module.name == "webserver" {
                // Navigated back to webserver - show output
                self.module_output = Some(self.webserver_output.clone());
            } else if self.futures_running && module.name == "futures" {
                // Navigated back to futures - show output
                self.module_output = Some(self.futures_output.clone());
            } else if self.network_running && module.name == "network" {
                // Navigated back to network - show output
                self.module_output = Some(self.network_output.clone());
            } else if self.message_passing_running && module.name == "message_passing" {
                // Navigated back to message_passing - show output
                self.module_output = Some(self.message_passing_output.clone());
            } else if !self.webserver_running
                && !self.futures_running
                && !self.network_running
                && !self.message_passing_running
            {
                // No background tasks, clear output
                self.module_output = None;
            } else {
                // Navigated away from running module - clear output but keep module running
                self.module_output = None;
            }
        } else {
            self.module_output = None;
        }
        self.output_scroll = 0;
    }

    fn previous(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.modules.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i);
        self.selected_module = self.modules.get(i).cloned();

        // Reset submodule state and select first submodule if module has submodules
        self.submodule_list_state.select(None);
        self.selected_submodule_index = None;
        self.submodule_output = None;
        self.submodule_scroll = 0;
        if let Some(module) = &self.selected_module {
            if !module.submodules.is_empty() {
                self.select_first_submodule();
            }
        }

        // Handle long-running modules when navigating
        if let Some(module) = &self.selected_module {
            if self.webserver_running && module.name == "webserver" {
                // Navigated back to webserver - show output
                self.module_output = Some(self.webserver_output.clone());
            } else if self.futures_running && module.name == "futures" {
                // Navigated back to futures - show output
                self.module_output = Some(self.futures_output.clone());
            } else if self.network_running && module.name == "network" {
                // Navigated back to network - show output
                self.module_output = Some(self.network_output.clone());
            } else if self.message_passing_running && module.name == "message_passing" {
                // Navigated back to message_passing - show output
                self.module_output = Some(self.message_passing_output.clone());
            } else if !self.webserver_running
                && !self.futures_running
                && !self.network_running
                && !self.message_passing_running
            {
                // No background tasks, clear output
                self.module_output = None;
            } else {
                // Navigated away from running module - clear output but keep module running
                self.module_output = None;
            }
        } else {
            self.module_output = None;
        }
        self.output_scroll = 0;
    }

    /// Sanitize output by removing control characters that could interfere with TUI
    fn sanitize_output(s: &str) -> String {
        s.chars()
            .filter(|c| !c.is_control() || *c == '\n' || *c == '\t')
            .collect()
    }

    /// Check if webserver has new output and update display
    fn check_webserver_output(&mut self) {
        if let Some(rx) = &self.webserver_output_rx {
            // Collect all available messages without blocking
            while let Ok(msg) = rx.try_recv() {
                // Sanitize the message to prevent terminal corruption
                let clean_msg = Self::sanitize_output(&msg);
                if !clean_msg.is_empty() {
                    self.webserver_output.push_str(&clean_msg);
                    if !clean_msg.ends_with('\n') {
                        self.webserver_output.push('\n');
                    }
                }
            }
        }

        // Check if thread has finished
        if self.webserver_running {
            if let Some(handle) = &self.webserver_thread {
                if handle.is_finished() {
                    self.webserver_running = false;
                    // Take the handle to join it
                    if let Some(handle) = self.webserver_thread.take() {
                        let _ = handle.join();
                    }
                    self.webserver_output_rx = None;

                    // Append completion message
                    self.webserver_output.push_str("\n[Webserver stopped]\n");
                }
            }
        }

        // Update module_output if we're viewing webserver
        if let Some(module) = &self.selected_module {
            if module.name == "webserver" && !self.webserver_output.is_empty() {
                self.module_output = Some(self.webserver_output.clone());
            }
        }
    }

    /// Stop the running webserver
    fn stop_webserver(&mut self) {
        if self.webserver_running {
            // Signal shutdown
            self.webserver_shutdown.store(true, Ordering::Relaxed);

            // Wait for thread to finish (with timeout to prevent hanging)
            if let Some(handle) = self.webserver_thread.take() {
                let _ = handle.join();
            }

            self.webserver_running = false;
            self.webserver_output_rx = None;

            // Update output
            self.webserver_output
                .push_str("\n[Webserver stopped by user]\n");

            // Update module_output if we're viewing webserver
            if let Some(module) = &self.selected_module {
                if module.name == "webserver" {
                    self.module_output = Some(self.webserver_output.clone());
                }
            }

            // Reset shutdown flag for next run
            self.webserver_shutdown.store(false, Ordering::Relaxed);
        }
    }

    /// Start the webserver in a background thread
    fn start_webserver(&mut self) {
        // Clear previous webserver output
        self.webserver_output.clear();
        self.webserver_output.push_str("[Starting webserver...]\n");

        let (tx, rx) = mpsc::channel();
        let shutdown_signal = self.webserver_shutdown.clone();

        let handle = thread::spawn(move || {
            crate::webserver::web::run_with_shutdown(tx, shutdown_signal);
        });

        self.webserver_thread = Some(handle);
        self.webserver_output_rx = Some(rx);
        self.webserver_running = true;
        self.module_output = Some(self.webserver_output.clone());
    }

    /// Check if futures has new output and update display
    fn check_futures_output(&mut self) {
        if let Some(rx) = &self.futures_output_rx {
            // Collect all available messages without blocking
            while let Ok(msg) = rx.try_recv() {
                // Sanitize the message to prevent terminal corruption
                let clean_msg = Self::sanitize_output(&msg);
                if !clean_msg.is_empty() {
                    self.futures_output.push_str(&clean_msg);
                    if !clean_msg.ends_with('\n') {
                        self.futures_output.push('\n');
                    }
                }
            }
        }

        // Check if thread has finished
        if self.futures_running {
            if let Some(handle) = &self.futures_thread {
                if handle.is_finished() {
                    self.futures_running = false;
                    // Take the handle to join it
                    if let Some(handle) = self.futures_thread.take() {
                        let _ = handle.join();
                    }
                    self.futures_output_rx = None;

                    // Append completion message
                    self.futures_output
                        .push_str("\n[Futures examples completed]\n");
                }
            }
        }

        // Update module_output if we're viewing futures
        if let Some(module) = &self.selected_module {
            if module.name == "futures" && !self.futures_output.is_empty() {
                self.module_output = Some(self.futures_output.clone());
            }
        }
    }

    /// Stop the running futures
    fn stop_futures(&mut self) {
        if self.futures_running {
            // Signal shutdown
            self.futures_shutdown.store(true, Ordering::Relaxed);

            // Wait for thread to finish (with timeout to prevent hanging)
            if let Some(handle) = self.futures_thread.take() {
                let _ = handle.join();
            }

            self.futures_running = false;
            self.futures_output_rx = None;

            // Update output
            self.futures_output
                .push_str("\n[Futures stopped by user]\n");

            // Update module_output if we're viewing futures
            if let Some(module) = &self.selected_module {
                if module.name == "futures" {
                    self.module_output = Some(self.futures_output.clone());
                }
            }

            // Reset shutdown flag for next run
            self.futures_shutdown.store(false, Ordering::Relaxed);
        }
    }

    /// Start the futures in a background thread
    fn start_futures(&mut self) {
        // Clear previous futures output
        self.futures_output.clear();
        self.futures_output
            .push_str("[Starting futures examples...]\n");

        let (tx, rx) = mpsc::channel();
        let shutdown_signal = self.futures_shutdown.clone();

        let handle = thread::spawn(move || {
            crate::futures::definition::run_with_shutdown(tx, shutdown_signal);
        });

        self.futures_thread = Some(handle);
        self.futures_output_rx = Some(rx);
        self.futures_running = true;
        self.module_output = Some(self.futures_output.clone());
    }

    /// Check if network has new output and update display
    fn check_network_output(&mut self) {
        if let Some(rx) = &self.network_output_rx {
            // Collect all available messages without blocking
            while let Ok(msg) = rx.try_recv() {
                // Sanitize the message to prevent terminal corruption
                let clean_msg = Self::sanitize_output(&msg);
                if !clean_msg.is_empty() {
                    self.network_output.push_str(&clean_msg);
                    if !clean_msg.ends_with('\n') {
                        self.network_output.push('\n');
                    }
                }
            }
        }

        // Check if thread has finished
        if self.network_running {
            if let Some(handle) = &self.network_thread {
                if handle.is_finished() {
                    self.network_running = false;
                    // Take the handle to join it
                    if let Some(handle) = self.network_thread.take() {
                        let _ = handle.join();
                    }
                    self.network_output_rx = None;

                    // Append completion message
                    self.network_output.push_str("\n[Network server stopped]\n");
                }
            }
        }

        // Update module_output if we're viewing network
        if let Some(module) = &self.selected_module {
            if module.name == "network" && !self.network_output.is_empty() {
                self.module_output = Some(self.network_output.clone());
            }
        }
    }

    /// Stop the running network server
    fn stop_network(&mut self) {
        if self.network_running {
            // Signal shutdown
            self.network_shutdown.store(true, Ordering::Relaxed);

            // Wait for thread to finish (with timeout to prevent hanging)
            if let Some(handle) = self.network_thread.take() {
                let _ = handle.join();
            }

            self.network_running = false;
            self.network_output_rx = None;

            // Update output
            self.network_output
                .push_str("\n[Network server stopped by user]\n");

            // Update module_output if we're viewing network
            if let Some(module) = &self.selected_module {
                if module.name == "network" {
                    self.module_output = Some(self.network_output.clone());
                }
            }

            // Reset shutdown flag for next run
            self.network_shutdown.store(false, Ordering::Relaxed);
        }
    }

    /// Start the network server in a background thread
    fn start_network(&mut self) {
        // Clear previous network output
        self.network_output.clear();
        self.network_output
            .push_str("[Starting network server...]\n");

        let (tx, rx) = mpsc::channel();
        let shutdown_signal = self.network_shutdown.clone();

        let handle = thread::spawn(move || {
            crate::network::run_with_shutdown(tx, shutdown_signal);
        });

        self.network_thread = Some(handle);
        self.network_output_rx = Some(rx);
        self.network_running = true;
        self.module_output = Some(self.network_output.clone());
    }

    /// Check if message_passing has new output and update display
    fn check_message_passing_output(&mut self) {
        if let Some(rx) = &self.message_passing_output_rx {
            // Collect all available messages without blocking
            while let Ok(msg) = rx.try_recv() {
                // Sanitize the message to prevent terminal corruption
                let clean_msg = Self::sanitize_output(&msg);
                if !clean_msg.is_empty() {
                    self.message_passing_output.push_str(&clean_msg);
                    if !clean_msg.ends_with('\n') {
                        self.message_passing_output.push('\n');
                    }
                }
            }
        }

        // Check if thread has finished
        if self.message_passing_running {
            if let Some(handle) = &self.message_passing_thread {
                if handle.is_finished() {
                    self.message_passing_running = false;
                    // Take the handle to join it
                    if let Some(handle) = self.message_passing_thread.take() {
                        let _ = handle.join();
                    }
                    self.message_passing_output_rx = None;

                    // Append completion message
                    self.message_passing_output
                        .push_str("\n[Message passing demo completed]\n");
                }
            }
        }

        // Update module_output if we're viewing message_passing
        if let Some(module) = &self.selected_module {
            if module.name == "message_passing" && !self.message_passing_output.is_empty() {
                self.module_output = Some(self.message_passing_output.clone());
            }
        }
    }

    /// Stop the running message_passing demo
    fn stop_message_passing(&mut self) {
        if self.message_passing_running {
            // Signal shutdown
            self.message_passing_shutdown.store(true, Ordering::Relaxed);

            // Wait for thread to finish (with timeout to prevent hanging)
            if let Some(handle) = self.message_passing_thread.take() {
                let _ = handle.join();
            }

            self.message_passing_running = false;
            self.message_passing_output_rx = None;

            // Update output
            self.message_passing_output
                .push_str("\n[Message passing stopped by user]\n");

            // Update module_output if we're viewing message_passing
            if let Some(module) = &self.selected_module {
                if module.name == "message_passing" {
                    self.module_output = Some(self.message_passing_output.clone());
                }
            }

            // Reset shutdown flag for next run
            self.message_passing_shutdown
                .store(false, Ordering::Relaxed);
        }
    }

    /// Start the message_passing demo in a background thread
    fn start_message_passing(&mut self) {
        // Clear previous message_passing output
        self.message_passing_output.clear();
        self.message_passing_output
            .push_str("[Starting message passing demo...]\n");

        let (tx, rx) = mpsc::channel();
        let shutdown_signal = self.message_passing_shutdown.clone();

        let handle = thread::spawn(move || {
            crate::message_passing::definition::run_with_shutdown(tx, shutdown_signal);
        });

        self.message_passing_thread = Some(handle);
        self.message_passing_output_rx = Some(rx);
        self.message_passing_running = true;
        self.module_output = Some(self.message_passing_output.clone());
    }

    /// Start user input mode with a popup
    fn start_user_input(&mut self, prompt: &str) {
        self.input_mode = true;
        self.input_buffer.clear();
        self.input_prompt = prompt.to_string();
    }

    /// Handle character input in input mode
    fn handle_input_char(&mut self, c: char) {
        if self.input_mode {
            self.input_buffer.push(c);
        }
    }

    /// Handle backspace in input mode
    fn handle_input_backspace(&mut self) {
        if self.input_mode {
            self.input_buffer.pop();
        }
    }

    /// Process the user input when Enter is pressed
    fn submit_user_input(&mut self) {
        if self.input_mode {
            let input = self.input_buffer.trim().to_lowercase();

            // Process power state input
            let result = match input.as_str() {
                "off" => "turning off",
                "sleep" => "sleeping",
                "reboot" => "rebooting",
                "shutdown" => "shutting down",
                "hibernate" => "hibernating",
                _ => "invalid power state",
            };

            self.user_input_result = format!(
                "--- User Input Activity\n\nEnter new power state: {}\n\nResult: {}",
                self.input_buffer.trim(),
                result
            );

            self.module_output = Some(self.user_input_result.clone());
            self.input_mode = false;
            self.input_buffer.clear();
        }
    }

    /// Cancel input mode
    fn cancel_input(&mut self) {
        self.input_mode = false;
        self.input_buffer.clear();
        self.user_input_result = "Input cancelled".to_string();
        self.module_output = Some(self.user_input_result.clone());
    }

    fn run_module(&mut self) {
        if let Some(module) = &self.selected_module {
            self.output_scroll = 0;

            // Special handling for webserver module
            if module.name == "webserver" {
                if self.webserver_running {
                    // Already running - refresh and show current output
                    self.check_webserver_output();
                    // Ensure the output is displayed
                    self.module_output = Some(self.webserver_output.clone());
                    return;
                } else {
                    // Start webserver in background thread
                    self.start_webserver();
                    return;
                }
            }

            // Special handling for futures module
            if module.name == "futures" {
                if self.futures_running {
                    // Already running - refresh and show current output
                    self.check_futures_output();
                    // Ensure the output is displayed
                    self.module_output = Some(self.futures_output.clone());
                    return;
                } else {
                    // Start futures in background thread
                    self.start_futures();
                    return;
                }
            }

            // Special handling for network module
            if module.name == "network" {
                if self.network_running {
                    // Already running - refresh and show current output
                    self.check_network_output();
                    // Ensure the output is displayed
                    self.module_output = Some(self.network_output.clone());
                    return;
                } else {
                    // Start network in background thread
                    self.start_network();
                    return;
                }
            }

            // Special handling for message_passing module
            if module.name == "message_passing" {
                if self.message_passing_running {
                    // Already running - refresh and show current output
                    self.check_message_passing_output();
                    // Ensure the output is displayed
                    self.module_output = Some(self.message_passing_output.clone());
                    return;
                } else {
                    // Start message_passing in background thread
                    self.start_message_passing();
                    return;
                }
            }

            // Special handling for user_input module - use popup instead of stdin
            if module.name == "user_input" {
                self.start_user_input(
                    "Enter new power state (off/sleep/reboot/shutdown/hibernate):",
                );
                return;
            }

            // Capture stdout for other modules
            let mut buf = BufferRedirect::stdout().unwrap();

            // Call the module's index function
            match module.name.as_str() {
                "add_millimeters_to_meters" => crate::add_millimeters_to_meters::index(true),
                "advanced_match" => crate::advanced_match::index(true),
                "algorithms" => crate::algorithms::index(true),
                "api_mutex" => crate::api_mutex::index(true),
                "arrays" => crate::arrays::index(true),
                "automatic_clean" => crate::automatic_clean::index(true),
                "basics" => crate::basics::index(true),
                "bit_patterns" => crate::bit_patterns::index(true),
                "boxes" => crate::boxes::index(true),
                "channels" => crate::channels::index(true),
                "closures" => crate::closures::index(true),
                "collections" => crate::collections::index(true),
                "compiler_directives" => crate::compiler_directives::index(true),
                "concurrency" => crate::concurrency::index(true),
                "cow_type" => crate::cow_type::index(true),
                "cubesats" => crate::cubesats::index(true),
                "custom_smart_pointer" => crate::custom_smart_pointer::index(true),
                "data_collections" => crate::data_collections::index(true),
                "decisions" => crate::decisions::index(true),
                "deconstructing_float" => crate::deconstructing_float::index(true),
                "destructuring_structs" => crate::destructuring_structs::index(true),
                "doc" => crate::doc::index(true),
                "draw" => crate::draw::index(true),
                "enums" => crate::enums::index(true),
                "errors" => crate::errors::index(true),
                "exercises" => crate::exercises::index(true),
                "expressions" => crate::expressions::index(true),
                "extern_functions" => crate::extern_functions::index(true),
                "external_crates" => crate::external_crates::index(true),
                "files" => crate::files::index(true),
                "for_loops" => crate::for_loops::index(true),
                "formatted_print" => crate::formatted_print::index(true),
                "functions" => crate::functions::index(true),
                "futures" => crate::futures::index(true),
                "generics" => crate::generics::index(true),
                "get_type" => crate::get_type::index(true),
                "guessing_game" => crate::guessing_game::index(true),
                "hashmaps" => crate::hashmaps::index(true),
                "hello_world" => crate::hello_world::index(true),
                "inspecting_endianness" => crate::inspecting_endianness::index(true),
                "interior_mutability" => crate::interior_mutability::index(true),
                "interpreting_float" => crate::interpreting_float::index(true),
                "iterators" => crate::iterators::index(true),
                "joinhandle" => crate::joinhandle::index(true),
                "lifetimes" => crate::lifetimes::index(true),
                "loops" => crate::loops::index(true),
                "machine_learning" => crate::machine_learning::index(true),
                "macros" => crate::macros::index(true),
                "mandelbrot" => crate::mandelbrot::index(true),
                "matches" => crate::matches::index(true),
                "message_passing" => crate::message_passing::index(true),
                "messenger" => crate::messenger::index(true),
                "methods" => crate::methods::index(true),
                "modules" => crate::modules::index(true),
                "move_vars" => crate::move_vars::index(true),
                "mutable_reference" => crate::mutable_reference::index(true),
                "network" => crate::network::index(true),
                "new_type" => crate::new_type::index(true),
                "numbers" => crate::numbers::index(true),
                "object_oriented" => crate::object_oriented::index(true),
                "open_file" => crate::open_file::index(true),
                "operator_overloading" => crate::operator_overloading::index(true),
                "optionals" => crate::optionals::index(true),
                "ownership" => crate::ownership::index(true),
                "patterns" => crate::patterns::index(true),
                "post" => crate::post::index(true),
                "print_var_type" => crate::print_var_type::index(true),
                "random_numbers" => crate::random_numbers::index(true),
                "ranges" => crate::ranges::index(true),
                "rc_t" => crate::rc_t::index(true),
                "references" => crate::references::index(true),
                "results" => crate::results::index(true),
                "rust_atomics_and_locks" => crate::rust_atomics_and_locks::index(true),
                "shadowing" => crate::shadowing::index(true),
                "slices" => crate::slices::index(true),
                "static_variable" => crate::static_variable::index(true),
                "strings" => crate::strings::index(true),
                "structs" => crate::structs::index(true),
                "system_calls" => crate::system_calls::index(true),
                "testing" => crate::testing::index(true),
                "threads" => crate::threads::index(true),
                "traits" => crate::traits::index(true),
                "tree_data_structure" => crate::tree_data_structure::index(true),
                "tuples" => crate::tuples::index(true),
                "type_aliases" => crate::type_aliases::index(true),
                "typestate_pattern" => crate::typestate_pattern::index(true),
                "unsafe_code" => crate::unsafe_code::index(true),
                "unsafe_trait" => crate::unsafe_trait::index(true),
                "user_input" => crate::user_input::index(true),
                "variables" => crate::variables::index(true),
                "vectors" => crate::vectors::index(true),
                "webserver" => crate::webserver::index(true),
                "while_loop" => crate::while_loop::index(true),
                "zero_cost_abstraction" => crate::zero_cost_abstraction::index(true),
                _ => {}
            }

            // Get the captured output
            let mut output = String::new();
            buf.read_to_string(&mut output).unwrap();
            drop(buf);

            self.module_output = Some(if output.is_empty() {
                "No output produced by this module.".to_string()
            } else {
                output
            });
        }
    }

    fn run_submodule(&mut self) {
        if let Some(module) = &self.selected_module {
            if let Some(idx) = self.selected_submodule_index {
                if idx < module.submodules.len() {
                    self.submodule_scroll = 0;

                    // Capture stdout for submodule
                    let mut buf = BufferRedirect::stdout().unwrap();

                    // Call the submodule's index function
                    match (module.name.as_str(), module.submodules[idx].name.as_str()) {
                        ("algorithms", "bubble_sort") => {
                            crate::algorithms::bubble_sort::index(true)
                        }
                        ("algorithms", "cocktail_sort") => {
                            crate::algorithms::cocktail_sort::index(true)
                        }
                        ("algorithms", "comb_sort") => crate::algorithms::comb_sort::index(true),
                        ("algorithms", "insertion_sort") => {
                            crate::algorithms::insertion_sort::index(true)
                        }
                        ("algorithms", "merge_sort") => crate::algorithms::merge_sort::index(true),
                        ("algorithms", "quick_sort") => crate::algorithms::quick_sort::index(true),
                        ("algorithms", "selection_sort") => {
                            crate::algorithms::selection_sort::index(true)
                        }
                        ("api_mutex", "definition") => crate::api_mutex::definition::index(true),
                        ("api_mutex", "mutexes") => crate::api_mutex::mutexes::index(true),
                        ("api_mutex", "thread_parking") => {
                            crate::api_mutex::thread_parking::index(true)
                        }
                        ("arrays", "definition") => crate::arrays::definition::index(true),
                        ("arrays", "predictability") => crate::arrays::predictability::index(true),
                        ("basics", "arrays") => crate::basics::arrays::index(true),
                        ("basics", "booleans") => crate::basics::booleans::index(true),
                        ("basics", "borrowing") => crate::basics::borrowing::index(true),
                        ("basics", "branches") => crate::basics::branches::index(true),
                        ("basics", "characters") => crate::basics::characters::index(true),
                        ("basics", "clone") => crate::basics::clone::index(true),
                        ("basics", "compounds") => crate::basics::compounds::index(true),
                        ("basics", "constants") => crate::basics::constants::index(true),
                        ("basics", "control_flow") => crate::basics::control_flow::index(true),
                        ("basics", "convert_values") => crate::basics::convert_values::index(true),
                        ("basics", "floats") => crate::basics::floats::index(true),
                        ("basics", "functions") => crate::basics::functions::index(true),
                        ("basics", "if_let") => crate::basics::if_let::index(true),
                        ("basics", "integers") => crate::basics::integers::index(true),
                        ("basics", "loops") => crate::basics::loops::index(true),
                        ("basics", "match_statements") => {
                            crate::basics::match_statements::index(true)
                        }
                        ("basics", "num_operations") => crate::basics::num_operations::index(true),
                        ("basics", "recursion") => crate::basics::recursion::index(true),
                        ("basics", "scalars") => crate::basics::scalars::index(true),
                        ("basics", "scopes") => crate::basics::scopes::index(true),
                        ("basics", "shadowing") => crate::basics::shadowing::index(true),
                        ("basics", "slices") => crate::basics::slices::index(true),
                        ("basics", "type_casting") => crate::basics::type_casting::index(true),
                        ("basics", "underscores") => crate::basics::underscores::index(true),
                        ("basics", "variables") => crate::basics::variables::index(true),
                        ("boxes", "definition") => crate::boxes::definition::index(true),
                        ("boxes", "dereference") => crate::boxes::dereference::index(true),
                        ("closures", "closures_inside_methods") => {
                            crate::closures::closures_inside_methods::index(true)
                        }
                        ("closures", "definition") => crate::closures::definition::index(true),
                        ("closures", "demo") => crate::closures::demo::index(true),
                        ("closures", "map_collect") => crate::closures::map_collect::index(true),
                        ("closures", "map_combinator") => {
                            crate::closures::map_combinator::index(true)
                        }
                        ("closures", "process_closure_thread") => {
                            crate::closures::process_closure_thread::index(true)
                        }
                        ("closures", "simple_closure") => {
                            crate::closures::simple_closure::index(true)
                        }
                        ("closures", "vars_outside_closure") => {
                            crate::closures::vars_outside_closure::index(true)
                        }
                        ("collections", "hashmaps") => crate::collections::hashmaps::index(true),
                        ("collections", "vectors") => crate::collections::vectors::index(true),
                        ("concurrency", "definition") => {
                            crate::concurrency::definition::index(true)
                        }
                        ("concurrency", "message_passing") => {
                            crate::concurrency::message_passing::index(true)
                        }
                        ("concurrency", "scoped_threads") => {
                            crate::concurrency::scoped_threads::index(true)
                        }
                        ("concurrency", "sharing_state_concurrently") => {
                            crate::concurrency::sharing_state_concurrently::index(true)
                        }
                        ("data_collections", "definition") => {
                            crate::data_collections::definition::index(true)
                        }
                        ("data_collections", "shipping_box") => {
                            crate::data_collections::shipping_box::index(true)
                        }
                        ("enums", "cards") => crate::enums::cards::index(true),
                        ("enums", "catching_multiple_values") => {
                            crate::enums::catching_multiple_values::index(true)
                        }
                        ("enums", "definition") => crate::enums::definition::index(true),
                        ("enums", "demo") => crate::enums::demo::index(true),
                        ("enums", "emulator") => crate::enums::emulator::index(true),
                        ("enums", "enum_multi_array") => {
                            crate::enums::enum_multi_array::index(true)
                        }
                        ("enums", "enum_with_data") => crate::enums::enum_with_data::index(true),
                        ("enums", "enum_with_match") => crate::enums::enum_with_match::index(true),
                        ("enums", "enums_and_functions") => {
                            crate::enums::enums_and_functions::index(true)
                        }
                        ("enums", "enums_with_enums") => {
                            crate::enums::enums_with_enums::index(true)
                        }
                        ("enums", "event") => crate::enums::event::index(true),
                        ("enums", "german_months") => crate::enums::german_months::index(true),
                        ("enums", "hold_data") => crate::enums::hold_data::index(true),
                        ("enums", "match_exact_value") => {
                            crate::enums::match_exact_value::index(true)
                        }
                        ("enums", "matches") => crate::enums::matches::index(true),
                        ("enums", "options") => crate::enums::options::index(true),
                        ("enums", "payment_method_type") => {
                            crate::enums::payment_method_type::index(true)
                        }
                        ("enums", "result_enum") => crate::enums::result_enum::index(true),
                        ("errors", "activity_custom_errors") => {
                            crate::errors::activity_custom_errors::index(true);
                        }
                        ("errors", "closures") => crate::errors::closures::index(true),
                        ("errors", "custom_errors") => {
                            crate::errors::custom_errors::index(true);
                        }
                        ("errors", "demo_custom_error") => {
                            crate::errors::demo_custom_error::index(true);
                        }
                        ("errors", "error_handling") => crate::errors::error_handling::index(true),
                        ("errors", "error_propagation") => {
                            crate::errors::error_propagation::index(true)
                        }
                        ("exercises", "ex03") => crate::exercises::ex03::index(true),
                        ("exercises", "ex08") => crate::exercises::ex08::index(true),
                        ("external_crates", "activity") => {
                            crate::external_crates::activity::index(true)
                        }
                        ("external_crates", "demo") => crate::external_crates::demo::index(true),
                        ("functions", "blocks_in_functions") => {
                            crate::functions::blocks_in_functions::index(true)
                        }
                        ("functions", "definition") => crate::functions::definition::index(true),
                        ("functions", "func_return_option") => {
                            crate::functions::func_return_option::index(true)
                        }
                        ("generics", "activity") => crate::generics::activity::index(true),
                        ("generics", "conveyor_belt") => {
                            crate::generics::conveyor_belt::index(true)
                        }
                        ("generics", "definition") => crate::generics::definition::index(true),
                        ("generics", "demo") => crate::generics::demo::index(true),
                        ("generics", "generic_add") => crate::generics::generic_add::index(true),
                        ("generics", "generic_enums") => {
                            crate::generics::generic_enums::index(true)
                        }
                        ("generics", "generic_functions") => {
                            crate::generics::generic_functions::index(true)
                        }
                        ("generics", "generic_implementation") => {
                            crate::generics::generic_implementation::index(true)
                        }
                        ("generics", "generic_structs") => {
                            crate::generics::generic_structs::index(true)
                        }
                        ("generics", "generic_structures") => {
                            crate::generics::generic_structures::index(true)
                        }
                        ("generics", "generic_with_where") => {
                            crate::generics::generic_with_where::index(true)
                        }
                        ("generics", "introduction") => crate::generics::introduction::index(true),
                        ("generics", "multiple_generics") => {
                            crate::generics::multiple_generics::index(true)
                        }
                        ("generics", "question_operator") => {
                            crate::generics::question_operator::index(true)
                        }
                        ("generics", "vehicle") => crate::generics::vehicle::index(true),
                        ("hashmaps", "activity") => crate::hashmaps::activity::index(true),
                        ("hashmaps", "definition") => crate::hashmaps::definition::index(true),
                        ("hashmaps", "furniture") => crate::hashmaps::furniture::index(true),
                        ("hashmaps", "hashmap_entry_method") => {
                            crate::hashmaps::hashmap_entry_method::index(true)
                        }
                        ("hashmaps", "hashmap_get") => crate::hashmaps::hashmap_get::index(true),
                        ("hashmaps", "hashmap_loop") => crate::hashmaps::hashmap_loop::index(true),
                        ("hashmaps", "hashmap_old_values") => {
                            crate::hashmaps::hashmap_old_values::index(true)
                        }
                        ("iterators", "activity") => crate::iterators::activity::index(true),
                        ("iterators", "definition") => crate::iterators::definition::index(true),
                        ("iterators", "demo") => crate::iterators::demo::index(true),
                        ("iterators", "extend") => crate::iterators::extend::index(true),
                        ("iterators", "filter_map") => crate::iterators::filter_map::index(true),
                        ("iterators", "find_position") => {
                            crate::iterators::find_position::index(true)
                        }
                        ("iterators", "fold") => crate::iterators::fold::index(true),
                        ("iterators", "functional_style") => {
                            crate::iterators::functional_style::index(true)
                        }
                        ("iterators", "impl_iterator") => {
                            crate::iterators::impl_iterator::index(true)
                        }
                        ("iterators", "iter_in_collections") => {
                            crate::iterators::iter_in_collections::index(true)
                        }
                        ("iterators", "iterator_next") => {
                            crate::iterators::iterator_next::index(true)
                        }
                        ("iterators", "mapping_and_filtering") => {
                            crate::iterators::mapping_and_filtering::index(true)
                        }
                        ("iterators", "nonfunctional_style") => {
                            crate::iterators::nonfunctional_style::index(true)
                        }
                        ("iterators", "partition") => crate::iterators::partition::index(true),
                        ("iterators", "result_option_ok") => {
                            crate::iterators::result_option_ok::index(true)
                        }
                        ("iterators", "rev") => crate::iterators::rev::index(true),
                        ("iterators", "scan_hash_table") => {
                            crate::iterators::scan_hash_table::index(true)
                        }
                        ("iterators", "skip") => crate::iterators::skip::index(true),
                        ("iterators", "without_for") => crate::iterators::without_for::index(true),
                        ("lifetimes", "activity") => crate::lifetimes::activity::index(true),
                        ("lifetimes", "anonymous_lifetime") => {
                            crate::lifetimes::anonymous_lifetime::index(true)
                        }
                        ("lifetimes", "compare_activity") => {
                            crate::lifetimes::compare_activity::index(true)
                        }
                        ("lifetimes", "definition") => crate::lifetimes::definition::index(true),
                        ("lifetimes", "demo") => crate::lifetimes::demo::index(true),
                        ("lifetimes", "example") => crate::lifetimes::example::index(true),
                        ("lifetimes", "multiple_lifetimes") => {
                            crate::lifetimes::multiple_lifetimes::index(true)
                        }
                        ("lifetimes", "named_lifetime") => {
                            crate::lifetimes::named_lifetime::index(true)
                        }
                        ("loops", "definition") => crate::loops::definition::index(true),
                        ("loops", "returning_values_from_loops") => {
                            crate::loops::returning_values_from_loops::index(true)
                        }
                        ("machine_learning", "reading_csv_file") => {
                            let _ = crate::machine_learning::reading_csv_file::index(true);
                        }
                        ("macros", "create_vector") => crate::macros::create_vector::index(true),
                        ("macros", "definition") => crate::macros::definition::index(true),
                        ("matches", "catch_all") => crate::matches::catch_all::index(true),
                        ("matches", "definition") => crate::matches::definition::index(true),
                        ("matches", "if_let") => crate::matches::if_let::index(true),
                        ("matches", "match_control_flow") => {
                            crate::matches::match_control_flow::index(true)
                        }
                        ("matches", "match_guard") => crate::matches::match_guard::index(true),
                        ("matches", "match_vector") => crate::matches::match_vector::index(true),
                        ("matches", "multiple_placeholders") => {
                            crate::matches::multiple_placeholders::index(true)
                        }
                        ("matches", "pattern_match") => crate::matches::pattern_match::index(true),
                        ("matches", "placeholder") => crate::matches::placeholder::index(true),
                        ("matches", "while_let") => crate::matches::while_let::index(true),
                        ("messenger", "definition") => crate::messenger::definition::index(true),
                        ("methods", "associated_functions") => {
                            crate::methods::associated_functions::index(true)
                        }
                        ("methods", "definition") => crate::methods::definition::index(true),
                        ("methods", "multiple_impl_blocks") => {
                            crate::methods::multiple_impl_blocks::index(true)
                        }
                        ("methods", "parameters") => crate::methods::parameters::index(true),
                        ("modules", "inline_modules_activity") => {
                            crate::modules::inline_modules_activity::index(true)
                        }
                        ("modules", "inline_modules_demo") => {
                            crate::modules::inline_modules_demo::index(true)
                        }
                        ("modules", "mod_basics") => crate::modules::mod_basics::index(true),
                        ("new_type", "activity") => crate::new_type::activity::index(true),
                        ("new_type", "definition") => crate::new_type::definition::index(true),
                        ("new_type", "demo") => crate::new_type::demo::index(true),
                        ("optionals", "activity") => crate::optionals::activity::index(true),
                        ("optionals", "definition") => crate::optionals::definition::index(true),
                        ("optionals", "unwrap_or_else") => {
                            crate::optionals::unwrap_or_else::index(true)
                        }
                        ("ownership", "borrowing") => crate::ownership::borrowing::index(true),
                        ("ownership", "dereference_operator") => {
                            crate::ownership::dereference_operator::index(true)
                        }
                        ("ownership", "functions") => crate::ownership::functions::index(true),
                        ("ownership", "mutable_borrowing") => {
                            crate::ownership::mutable_borrowing::index(true)
                        }
                        ("ownership", "mutable_references") => {
                            crate::ownership::mutable_references::index(true)
                        }
                        ("ownership", "ownership_arrays") => {
                            crate::ownership::ownership_arrays::index(true)
                        }
                        ("ownership", "references_borrowing") => {
                            crate::ownership::references_borrowing::index(true)
                        }
                        ("ownership", "return_multiple_values") => {
                            crate::ownership::return_multiple_values::index(true)
                        }
                        ("ownership", "return_values_and_scope") => {
                            crate::ownership::return_values_and_scope::index(true)
                        }
                        ("ownership", "returning_ownership") => {
                            crate::ownership::returning_ownership::index(true)
                        }
                        ("ownership", "string_clone_heap") => {
                            crate::ownership::string_clone_heap::index(true)
                        }
                        ("ownership", "string_type") => crate::ownership::string_type::index(true),
                        ("references", "basic_reference") => {
                            crate::references::basic_reference::index(true)
                        }
                        ("references", "definition") => crate::references::definition::index(true),
                        ("references", "mut_reference") => {
                            crate::references::mut_reference::index(true)
                        }
                        ("references", "ref_of_references") => {
                            crate::references::ref_of_references::index(true)
                        }
                        ("results", "activity") => crate::results::activity::index(true),
                        ("results", "definition") => crate::results::definition::index(true),
                        ("results", "question_mark_operator") => {
                            crate::results::question_mark_operator::index(true)
                        }
                        ("slices", "array_slices") => crate::slices::array_slices::index(true),
                        ("slices", "definition") => crate::slices::definition::index(true),
                        ("slices", "mutable_slice") => crate::slices::mutable_slice::index(true),
                        ("slices", "reference_array") => {
                            crate::slices::reference_array::index(true)
                        }
                        ("strings", "borrowed_string") => {
                            crate::strings::borrowed_string::index(true)
                        }
                        ("strings", "concatenation") => crate::strings::concatenation::index(true),
                        ("strings", "definition") => crate::strings::definition::index(true),
                        ("strings", "iterate") => crate::strings::iterate::index(true),
                        ("strings", "matches") => crate::strings::matches::index(true),
                        ("strings", "string_functions") => {
                            crate::strings::string_functions::index(true)
                        }
                        ("strings", "string_is_encoded") => {
                            crate::strings::string_is_encoded::index(true)
                        }
                        ("strings", "string_literals") => {
                            crate::strings::string_literals::index(true)
                        }
                        ("strings", "substrings") => crate::strings::substrings::index(true),
                        ("structs", "builder_pattern") => {
                            crate::structs::builder_pattern::index(true)
                        }
                        ("structs", "coffee") => crate::structs::coffee::index(true),
                        ("structs", "definition") => crate::structs::definition::index(true),
                        ("structs", "file") => crate::structs::file::index(true),
                        ("structs", "human") => crate::structs::human::index(true),
                        ("structs", "integrate_struct_enum_match") => {
                            crate::structs::integrate_struct_enum_match::index(true)
                        }
                        ("structs", "methods") => crate::structs::methods::index(true),
                        ("structs", "person") => crate::structs::person::index(true),
                        ("structs", "queue") => crate::structs::queue::index(true),
                        ("structs", "struct_update_syntax") => {
                            crate::structs::struct_update_syntax::index(true)
                        }
                        ("structs", "structs_owned") => crate::structs::structs_owned::index(true),
                        ("structs", "tuple_structs") => crate::structs::tuple_structs::index(true),
                        ("structs", "unit_struct") => crate::structs::unit_struct::index(true),
                        ("system_calls", "c_call") => crate::system_calls::c_call::index(true),
                        ("system_calls", "raw_syscall") => {
                            crate::system_calls::raw_syscall::index(true)
                        }
                        ("testing", "activity") => crate::testing::activity::index(true),
                        ("testing", "definition") => crate::testing::definition::index(true),
                        ("testing", "demo") => crate::testing::demo::index(true),
                        ("threads", "definition") => crate::threads::definition::index(true),
                        ("threads", "message_passing_between_threads") => {
                            crate::threads::message_passing_between_threads::index(true)
                        }
                        ("threads", "move_data_between_threads") => {
                            crate::threads::move_data_between_threads::index(true)
                        }
                        ("threads", "multithreading") => {
                            crate::threads::multithreading::index(true)
                        }
                        ("threads", "mutexes") => crate::threads::mutexes::index(true),
                        ("threads", "preventing_race_condition") => {
                            crate::threads::preventing_race_condition::index(true)
                        }
                        ("threads", "scoped_threads") => {
                            crate::threads::scoped_threads::index(true)
                        }
                        ("threads", "sending_multiple_values") => {
                            crate::threads::sending_multiple_values::index(true)
                        }
                        ("threads", "sharing_data_between_threads") => {
                            crate::threads::sharing_data_between_threads::index(true)
                        }
                        ("threads", "threads_in_array") => {
                            crate::threads::threads_in_array::index(true)
                        }
                        ("traits", "activity") => crate::traits::activity::index(true),
                        ("traits", "activity_trait_objects") => {
                            crate::traits::activity_trait_objects::index(true)
                        }
                        ("traits", "complex_trait") => crate::traits::complex_trait::index(true),
                        ("traits", "copy_trait") => crate::traits::copy_trait::index(true),
                        ("traits", "default_trait") => crate::traits::default_trait::index(true),
                        ("traits", "definition") => crate::traits::definition::index(true),
                        ("traits", "demo") => crate::traits::demo::index(true),
                        ("traits", "demo_trait_objets") => {
                            crate::traits::demo_trait_objets::index(true)
                        }
                        ("traits", "derives") => crate::traits::derives::index(true),
                        ("traits", "from_trait") => crate::traits::from_trait::index(true),
                        ("traits", "impl_display") => crate::traits::impl_display::index(true),
                        ("traits", "media_aggregator") => {
                            crate::traits::media_aggregator::index(true)
                        }
                        ("traits", "returning_traits_with_dyn") => {
                            crate::traits::returning_traits_with_dyn::index(true)
                        }
                        ("traits", "simple_trait") => crate::traits::simple_trait::index(true),
                        ("traits", "with_generics") => crate::traits::with_generics::index(true),
                        ("tuples", "definition") => crate::tuples::definition::index(true),
                        ("tuples", "single_tuple") => crate::tuples::single_tuple::index(true),
                        ("type_aliases", "demo") => crate::type_aliases::demo::index(true),
                        ("type_aliases", "long_types") => {
                            crate::type_aliases::long_types::index(true)
                        }
                        ("typestate_pattern", "activity") => {
                            crate::typestate_pattern::activity::index(true)
                        }
                        ("typestate_pattern", "demo") => {
                            crate::typestate_pattern::demo::index(true)
                        }
                        ("user_input", "activity") => crate::user_input::activity::index(true),
                        ("user_input", "demo") => crate::user_input::demo::index(true),
                        ("variables", "constants") => crate::variables::constants::index(true),
                        ("variables", "vars") => crate::variables::vars::index(true),
                        ("vectors", "add_remove") => crate::vectors::add_remove::index(true),
                        ("vectors", "create_vector") => crate::vectors::create_vector::index(true),
                        ("vectors", "definition") => crate::vectors::definition::index(true),
                        ("vectors", "get_method") => crate::vectors::get_method::index(true),
                        ("vectors", "reading_vector") => {
                            crate::vectors::reading_vector::index(true)
                        }
                        ("vectors", "sort_vector") => crate::vectors::sort_vector::index(true),
                        ("vectors", "vector_capacity") => {
                            crate::vectors::vector_capacity::index(true)
                        }
                        ("while_loop", "definition") => crate::while_loop::definition::index(true),
                        ("while_loop", "while_let") => crate::while_loop::while_let::index(true),
                        _ => {
                            println!(
                                "Submodule {} in {} not yet implemented",
                                module.submodules[idx].name, module.name
                            );
                        }
                    }

                    // Get captured output
                    let mut output = String::new();
                    buf.read_to_string(&mut output).unwrap();

                    // Store output
                    self.submodule_output = Some(output);
                }
            }
        }
    }
}

/// Run the TUI application
pub fn run_app() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();

    // Main event loop
    let result = loop {
        terminal.draw(|f| ui(f, &mut app))?;

        // Check for long-running module output updates
        if app.webserver_running {
            app.check_webserver_output();
        }
        if app.futures_running {
            app.check_futures_output();
        }
        if app.network_running {
            app.check_network_output();
        }
        if app.message_passing_running {
            app.check_message_passing_output();
        }

        // Use a timeout for event polling to allow periodic updates
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    // Handle input mode separately
                    if app.input_mode {
                        match key.code {
                            KeyCode::Char(c) => {
                                app.handle_input_char(c);
                            }
                            KeyCode::Backspace => {
                                app.handle_input_backspace();
                            }
                            KeyCode::Enter => {
                                app.submit_user_input();
                            }
                            KeyCode::Esc => {
                                app.cancel_input();
                            }
                            _ => {}
                        }
                    } else {
                        // Normal mode key handling
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => {
                                // Stop long-running modules before quitting
                                if app.webserver_running {
                                    app.stop_webserver();
                                }
                                if app.futures_running {
                                    app.stop_futures();
                                }
                                if app.network_running {
                                    app.stop_network();
                                }
                                if app.message_passing_running {
                                    app.stop_message_passing();
                                }
                                break Ok(());
                            }
                            KeyCode::Down | KeyCode::Char('j') => {
                                // Check if current module has submodules
                                let has_submodules = app
                                    .selected_module
                                    .as_ref()
                                    .map(|m| !m.submodules.is_empty())
                                    .unwrap_or(false);

                                if has_submodules {
                                    app.next_submodule();
                                } else {
                                    app.next();
                                }
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                // Check if current module has submodules
                                let has_submodules = app
                                    .selected_module
                                    .as_ref()
                                    .map(|m| !m.submodules.is_empty())
                                    .unwrap_or(false);

                                if has_submodules {
                                    app.previous_submodule();
                                } else {
                                    app.previous();
                                }
                            }
                            KeyCode::Enter => {
                                // Check if current module has submodules
                                let has_submodules = app
                                    .selected_module
                                    .as_ref()
                                    .map(|m| !m.submodules.is_empty())
                                    .unwrap_or(false);

                                if has_submodules {
                                    app.run_submodule();
                                } else {
                                    app.run_module();
                                }
                            }
                            KeyCode::Char('c') | KeyCode::Backspace => {
                                // Clear submodule output
                                if app.submodule_output.is_some() {
                                    app.submodule_output = None;
                                    app.submodule_scroll = 0;
                                }
                            }
                            KeyCode::Char('s') | KeyCode::Char('x') => {
                                // Stop long-running modules if running
                                let mut stopped_something = false;
                                if app.webserver_running {
                                    app.stop_webserver();
                                    stopped_something = true;
                                }
                                if app.futures_running {
                                    app.stop_futures();
                                    stopped_something = true;
                                }
                                if app.network_running {
                                    app.stop_network();
                                    stopped_something = true;
                                }
                                if app.message_passing_running {
                                    app.stop_message_passing();
                                    stopped_something = true;
                                }
                                if stopped_something {
                                    // Force a redraw to ensure clean UI after stopping
                                    terminal.clear()?;
                                }
                            }
                            KeyCode::Left | KeyCode::Char('h') => {
                                // Always navigate between modules with left/right
                                app.previous();
                            }
                            KeyCode::Right | KeyCode::Char('l') => {
                                // Always navigate between modules with left/right
                                app.next();
                            }
                            KeyCode::Home => {
                                app.list_state.select(Some(0));
                                app.scroll_state = app.scroll_state.position(0);
                                app.selected_module = app.modules.first().cloned();
                            }
                            KeyCode::End => {
                                let last = app.modules.len() - 1;
                                app.list_state.select(Some(last));
                                app.scroll_state = app.scroll_state.position(last);
                                app.selected_module = app.modules.get(last).cloned();
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    };

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

/// Render the UI
fn ui(f: &mut Frame, app: &mut App) {
    let size = f.area();

    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(10),   // Main content
            Constraint::Length(3), // Footer
        ])
        .split(size);

    // Header
    render_header(f, chunks[0]);

    // Main content area - split into module list and details
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40), // Module list
            Constraint::Percentage(60), // Details panel
        ])
        .split(chunks[1]);

    // Render module list
    render_module_list(f, app, main_chunks[0]);

    // Render details panel
    render_details_panel(f, app, main_chunks[1]);

    // Footer
    render_footer(f, chunks[2], app);

    // Render input popup if in input mode
    if app.input_mode {
        render_input_popup(f, app);
    }
}

/// Render the header section
fn render_header(f: &mut Frame, area: Rect) {
    let title = Paragraph::new("Learning Rust - Module Browser")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .padding(Padding::horizontal(1)),
        );
    f.render_widget(title, area);
}

/// Render the module list
fn render_module_list(f: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app
        .modules
        .iter()
        .enumerate()
        .map(|(i, module)| {
            let _category_color = match module.category.as_str() {
                "Basics" => Color::Green,
                "Intermediate" => Color::Yellow,
                "Advanced" => Color::Red,
                "Concurrency" => Color::Magenta,
                "Data Structures" => Color::Blue,
                "System" => Color::Cyan,
                _ => Color::White,
            };

            let content = Line::from(vec![
                Span::styled(
                    format!("{:>3}. ", i + 1),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(&module.name, Style::default().fg(Color::White)),
            ]);

            ListItem::new(content).style(Style::default())
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(format!(" Modules ({}) ", app.modules.len()))
                .title_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .padding(Padding::horizontal(1)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    f.render_stateful_widget(list, area, &mut app.list_state);

    // Render scrollbar
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));
    f.render_stateful_widget(
        scrollbar,
        area.inner(ratatui::layout::Margin {
            vertical: 1,
            horizontal: 0,
        }),
        &mut app.scroll_state,
    );
}

/// Render details panel with submodules (split vertically)
fn render_details_with_submodules(f: &mut Frame, app: &App, area: Rect, module: &Module) {
    // Split the details area vertically
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40), // Submodule list
            Constraint::Percentage(60), // Submodule output
        ])
        .split(area);

    // Render submodule list
    let items: Vec<ListItem> = module
        .submodules
        .iter()
        .enumerate()
        .map(|(i, submodule)| {
            let content = Line::from(vec![
                Span::styled(
                    format!("{:>2}. ", i + 1),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(&submodule.name, Style::default().fg(Color::White)),
            ]);
            ListItem::new(content).style(Style::default())
        })
        .collect();

    let submodule_list = List::new(items)
        .block(
            Block::default()
                .title(format!(
                    " {} Submodules ({}) ",
                    module.name,
                    module.submodules.len()
                ))
                .title_style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .padding(Padding::horizontal(1)),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(
        submodule_list,
        chunks[0],
        &mut app.submodule_list_state.clone(),
    );

    // Render submodule output or instructions
    let (output_title, output_content) = if let Some(output) = &app.submodule_output {
        let title = if let Some(idx) = app.selected_submodule_index {
            format!(" Output: {} ", module.submodules[idx].name)
        } else {
            " Output ".to_string()
        };
        (title, output.clone())
    } else {
        let instructions = if let Some(idx) = app.selected_submodule_index {
            format!(
                "Submodule: {}\n\nDescription: {}\n\nPress Enter to run this submodule",
                module.submodules[idx].name, module.submodules[idx].description
            )
        } else {
            "Use ↑↓ arrows to navigate submodules\nPress Enter to run selected submodule"
                .to_string()
        };
        (" Submodule Details ".to_string(), instructions)
    };

    let output_paragraph = Paragraph::new(output_content)
        .block(
            Block::default()
                .title(output_title)
                .title_style(
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .padding(Padding::uniform(1)),
        )
        .wrap(Wrap { trim: false })
        .scroll((app.submodule_scroll, 0));

    f.render_widget(output_paragraph, chunks[1]);
}

/// Render the details panel
fn render_details_panel(f: &mut Frame, app: &App, area: Rect) {
    // Check if selected module has submodules
    if let Some(module) = &app.selected_module {
        if !module.submodules.is_empty() {
            render_details_with_submodules(f, app, area, module);
            return;
        }
    }

    // Original rendering for modules without submodules
    let (title, content) = if let Some(output) = &app.module_output {
        // Show module output
        let mut title = if let Some(module) = &app.selected_module {
            format!(" Output: {} ", module.name)
        } else {
            " Output ".to_string()
        };

        // Add running indicator for long-running modules
        if app.webserver_running {
            title = " Output: webserver [RUNNING] ".to_string();
        } else if app.futures_running {
            title = " Output: futures [RUNNING] ".to_string();
        } else if app.network_running {
            title = " Output: network [RUNNING] ".to_string();
        } else if app.message_passing_running {
            title = " Output: message_passing [RUNNING] ".to_string();
        }

        (title, Paragraph::new(output.as_str()))
    } else if let Some(module) = &app.selected_module {
        // Show module details
        let category_style = match module.category.as_str() {
            "Basics" => Style::default().fg(Color::Green),
            "Intermediate" => Style::default().fg(Color::Yellow),
            "Advanced" => Style::default().fg(Color::Red),
            "Concurrency" => Style::default().fg(Color::Magenta),
            "Data Structures" => Style::default().fg(Color::Blue),
            "System" => Style::default().fg(Color::Cyan),
            _ => Style::default().fg(Color::White),
        };

        let text = Text::from(vec![
            Line::from(vec![
                Span::styled("Module: ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    &module.name,
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Category: ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    &module.category,
                    category_style.add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "Description:",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(module.description.as_str()),
            Line::from(""),
            Line::from(vec![
                Span::styled("Location: ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format!("src/{}/", &module.name),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::ITALIC),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Press ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    "Enter",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    if module.name == "webserver" && app.webserver_running {
                        " - webserver is running, press to view output"
                    } else if module.name == "webserver" {
                        " to start webserver in background"
                    } else if module.name == "futures" && app.futures_running {
                        " - futures is running, press to view output"
                    } else if module.name == "futures" {
                        " to start futures in background"
                    } else if module.name == "network" && app.network_running {
                        " - network server is running, press to view output"
                    } else if module.name == "network" {
                        " to start network server in background"
                    } else {
                        " to run this module"
                    },
                    Style::default().fg(Color::DarkGray),
                ),
            ]),
        ]);

        // Add stop instruction if long-running modules are active
        let mut text_lines = text.lines.clone();
        if (module.name == "webserver" && app.webserver_running)
            || (module.name == "futures" && app.futures_running)
            || (module.name == "network" && app.network_running)
        {
            let module_name = if module.name == "webserver" {
                "webserver"
            } else if module.name == "futures" {
                "futures"
            } else {
                "network"
            };
            text_lines.push(Line::from(vec![
                Span::styled("Press ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    "s",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::styled(" or ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    "x",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!(" to stop the {}", module_name),
                    Style::default().fg(Color::DarkGray),
                ),
            ]));
        }

        (
            " Details ".to_string(),
            Paragraph::new(Text::from(text_lines)),
        )
    } else {
        (
            " Details ".to_string(),
            Paragraph::new("No module selected"),
        )
    };

    let block = Block::default()
        .title(title)
        .title_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .padding(Padding::uniform(1));

    f.render_widget(content.block(block).wrap(Wrap { trim: true }), area);
}

/// Render the footer section
fn render_footer(f: &mut Frame, area: Rect, app: &App) {
    // Check if current module has submodules
    let has_submodules = app
        .selected_module
        .as_ref()
        .map(|m| !m.submodules.is_empty())
        .unwrap_or(false);

    let mut help_spans = vec![];

    if has_submodules {
        // Show submodule navigation keys
        help_spans.extend(vec![
            Span::styled(
                " ←/h ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("Prev Module  "),
            Span::styled(
                "→/l ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("Next Module  "),
            Span::styled(
                "↑/k ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("Prev Sub  "),
            Span::styled(
                "↓/j ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("Next Sub  "),
            Span::styled(
                "Enter ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("Run  "),
        ]);

        // Add clear key if there's submodule output
        if app.submodule_output.is_some() {
            help_spans.extend(vec![
                Span::styled(
                    "c/⌫ ",
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("Clear  "),
            ]);
        }
    } else {
        // Show regular module navigation keys
        help_spans.extend(vec![
            Span::styled(
                " ↑/k ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("Up  "),
            Span::styled(
                "↓/j ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("Down  "),
            Span::styled(
                "Enter ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("Run  "),
        ]);
    }

    // Add stop key if any long-running module is active
    if app.webserver_running || app.futures_running || app.network_running {
        help_spans.push(Span::styled(
            "s/x ",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ));
        help_spans.push(Span::raw("Stop  "));
    }

    help_spans.extend(vec![
        Span::styled(
            "Home ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("First  "),
        Span::styled(
            "End ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("Last  "),
        Span::styled(
            "q/ESC ",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Span::raw("Quit"),
    ]);

    let help_text = Line::from(help_spans);

    let footer = Paragraph::new(help_text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .padding(Padding::horizontal(1)),
        );
    f.render_widget(footer, area);
}

/// Render input popup modal
fn render_input_popup(f: &mut Frame, app: &App) {
    let area = f.area();

    // Calculate popup size (centered, 60% width, 30% height)
    let popup_width = (area.width * 60) / 100;
    let popup_height = 9; // Fixed height for the input box
    let popup_x = (area.width - popup_width) / 2;
    let popup_y = (area.height - popup_height) / 2;

    let popup_area = Rect {
        x: popup_x,
        y: popup_y,
        width: popup_width,
        height: popup_height,
    };

    // Create the popup content
    let text = Text::from(vec![
        Line::from(""),
        Line::from(Span::styled(
            &app.input_prompt,
            Style::default().fg(Color::Cyan),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "> ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(&app.input_buffer, Style::default().fg(Color::White)),
            Span::styled(
                "_",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::SLOW_BLINK),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Press Enter to submit, ESC to cancel",
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        )),
    ]);

    let popup = Paragraph::new(text).block(
        Block::default()
            .title(" User Input ")
            .title_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .padding(Padding::uniform(1))
            .style(Style::default().bg(Color::Black)),
    );

    // Clear the popup area first to create a clean background
    f.render_widget(Clear, popup_area);
    f.render_widget(popup, popup_area);
}

/// Get all modules with their metadata
fn get_all_modules() -> Vec<Module> {
    vec![
        Module {
            name: "add_millimeters_to_meters".to_string(),
            description: "Unit conversion example demonstrating basic arithmetic operations"
                .to_string(),
            category: "Basics".to_string(),
            submodules: vec![],
        },
        Module {
            name: "advanced_match".to_string(),
            description: "Advanced pattern matching techniques and match guards".to_string(),
            category: "Intermediate".to_string(),
            submodules: vec![],
        },
        Module {
            name: "algorithms".to_string(),
            description: "Common algorithms implementation in Rust".to_string(),
            category: "Intermediate".to_string(),
            submodules: vec![
                Submodule {
                    name: "bubble_sort".to_string(),
                    description: "Bubble sort algorithm implementation".to_string(),
                },
                Submodule {
                    name: "cocktail_sort".to_string(),
                    description: "Cocktail sort algorithm implementation".to_string(),
                },
                Submodule {
                    name: "comb_sort".to_string(),
                    description: "Comb sort algorithm implementation".to_string(),
                },
                Submodule {
                    name: "insertion_sort".to_string(),
                    description: "Insertion sort algorithm implementation".to_string(),
                },
                Submodule {
                    name: "merge_sort".to_string(),
                    description: "Merge sort algorithm implementation".to_string(),
                },
                Submodule {
                    name: "quick_sort".to_string(),
                    description: "Quick sort algorithm implementation".to_string(),
                },
                Submodule {
                    name: "selection_sort".to_string(),
                    description: "Selection sort algorithm implementation".to_string(),
                },
            ],
        },
        Module {
            name: "api_mutex".to_string(),
            description: "API examples using Mutex for safe concurrent access".to_string(),
            category: "Concurrency".to_string(),
            submodules: vec![
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "mutexes".to_string(),
                    description: "Mutexes".to_string(),
                },
                Submodule {
                    name: "thread_parking".to_string(),
                    description: "Thread Parking".to_string(),
                },
            ],
        },
        Module {
            name: "algorithms".to_string(),
            description: "Fixed-size array operations and manipulations".to_string(),
            category: "Basics".to_string(),
            submodules: vec![
                Submodule {
                    name: "bubble_sort".to_string(),
                    description: "Bubble Sort".to_string(),
                },
                Submodule {
                    name: "cocktail_sort".to_string(),
                    description: "Cocktail Sort".to_string(),
                },
                Submodule {
                    name: "comb_sort".to_string(),
                    description: "Comb Sort".to_string(),
                },
                Submodule {
                    name: "insertion_sort".to_string(),
                    description: "Insertion Sort".to_string(),
                },
                Submodule {
                    name: "merge_sort".to_string(),
                    description: "Merge Sort".to_string(),
                },
                Submodule {
                    name: "quick_sort".to_string(),
                    description: "Quick Sort".to_string(),
                },
                Submodule {
                    name: "selection_sort".to_string(),
                    description: "Selection Sort".to_string(),
                },
            ],
        },
        Module {
            name: "arrays".to_string(),
            description: "Fundamental Rust Arrays".to_string(),
            category: "Basics".to_string(),
            submodules: vec![
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "predictability".to_string(),
                    description: "Predictability".to_string(),
                },
            ],
        },
        Module {
            name: "automatic_clean".to_string(),
            description: "Automatic resource cleanup using RAII pattern".to_string(),
            category: "Intermediate".to_string(),
            submodules: vec![],
        },
        Module {
            name: "basics".to_string(),
            description: "Fundamental Rust concepts and syntax".to_string(),
            category: "Basics".to_string(),
            submodules: vec![
                Submodule {
                    name: "arrays".to_string(),
                    description: "Arrays".to_string(),
                },
                Submodule {
                    name: "booleans".to_string(),
                    description: "Booleans".to_string(),
                },
                Submodule {
                    name: "borrowing".to_string(),
                    description: "Borrowing".to_string(),
                },
                Submodule {
                    name: "branches".to_string(),
                    description: "Branches".to_string(),
                },
                Submodule {
                    name: "characters".to_string(),
                    description: "Characters".to_string(),
                },
                Submodule {
                    name: "clone".to_string(),
                    description: "Clone".to_string(),
                },
                Submodule {
                    name: "compounds".to_string(),
                    description: "Compounds".to_string(),
                },
                Submodule {
                    name: "constants".to_string(),
                    description: "Constants".to_string(),
                },
                Submodule {
                    name: "control_flow".to_string(),
                    description: "Control Flow".to_string(),
                },
                Submodule {
                    name: "convert_values".to_string(),
                    description: "Convert Values".to_string(),
                },
                Submodule {
                    name: "floats".to_string(),
                    description: "Floats".to_string(),
                },
                Submodule {
                    name: "functions".to_string(),
                    description: "Functions".to_string(),
                },
                Submodule {
                    name: "if_let".to_string(),
                    description: "If Let".to_string(),
                },
                Submodule {
                    name: "integers".to_string(),
                    description: "Integers".to_string(),
                },
                Submodule {
                    name: "loops".to_string(),
                    description: "Loops".to_string(),
                },
                Submodule {
                    name: "match_statements".to_string(),
                    description: "Match Statements".to_string(),
                },
                Submodule {
                    name: "num_operations".to_string(),
                    description: "Num Operations".to_string(),
                },
                Submodule {
                    name: "recursion".to_string(),
                    description: "Recursion".to_string(),
                },
                Submodule {
                    name: "scalars".to_string(),
                    description: "Scalars".to_string(),
                },
                Submodule {
                    name: "scopes".to_string(),
                    description: "Scopes".to_string(),
                },
                Submodule {
                    name: "shadowing".to_string(),
                    description: "Shadowing".to_string(),
                },
                Submodule {
                    name: "slices".to_string(),
                    description: "Slices".to_string(),
                },
                Submodule {
                    name: "type_casting".to_string(),
                    description: "Type Casting".to_string(),
                },
                Submodule {
                    name: "underscores".to_string(),
                    description: "Underscores".to_string(),
                },
                Submodule {
                    name: "variables".to_string(),
                    description: "Variables".to_string(),
                },
            ],
        },
        Module {
            name: "bit_patterns".to_string(),
            description: "Bit manipulation and binary patterns".to_string(),
            category: "Advanced".to_string(),
            submodules: vec![
                Submodule {
                    name: "constants".to_string(),
                    description: "Constants".to_string(),
                },
                Submodule {
                    name: "vars".to_string(),
                    description: "Vars".to_string(),
                },
            ],
        },
        Module {
            name: "boxes".to_string(),
            description: "Heap allocation with Box smart pointer".to_string(),
            category: "Intermediate".to_string(),
            submodules: vec![
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "dereference".to_string(),
                    description: "Dereference".to_string(),
                },
            ],
        },
        Module {
            name: "channels".to_string(),
            description: "Message passing between threads using channels".to_string(),
            category: "Concurrency".to_string(),
            submodules: vec![],
        },
        Module {
            name: "closures".to_string(),
            description: "Anonymous functions and closure captures".to_string(),
            category: "Intermediate".to_string(),
            submodules: vec![
                Submodule {
                    name: "closures_inside_methods".to_string(),
                    description: "Closures Inside Methods".to_string(),
                },
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "demo".to_string(),
                    description: "Demo".to_string(),
                },
                Submodule {
                    name: "map_collect".to_string(),
                    description: "Map Collect".to_string(),
                },
                Submodule {
                    name: "map_combinator".to_string(),
                    description: "Map Combinator".to_string(),
                },
                Submodule {
                    name: "process_closure_thread".to_string(),
                    description: "Process Closure Thread".to_string(),
                },
                Submodule {
                    name: "simple_closure".to_string(),
                    description: "Simple Closure".to_string(),
                },
                Submodule {
                    name: "vars_outside_closure".to_string(),
                    description: "Vars Outside Closure".to_string(),
                },
            ],
        },
        Module {
            name: "collections".to_string(),
            description: "Standard library collection types".to_string(),
            category: "Data Structures".to_string(),
            submodules: vec![
                Submodule {
                    name: "hashmaps".to_string(),
                    description: "Hashmaps".to_string(),
                },
                Submodule {
                    name: "vectors".to_string(),
                    description: "Vectors".to_string(),
                },
            ],
        },
        Module {
            name: "compiler_directives".to_string(),
            description: "Conditional compilation and attributes".to_string(),
            category: "Advanced".to_string(),
            submodules: vec![
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "furniture".to_string(),
                    description: "Furniture".to_string(),
                },
                Submodule {
                    name: "hashmap_entry_method".to_string(),
                    description: "Hashmap Entry Method".to_string(),
                },
                Submodule {
                    name: "hashmap_get".to_string(),
                    description: "Hashmap Get".to_string(),
                },
                Submodule {
                    name: "hashmap_loop".to_string(),
                    description: "Hashmap Loop".to_string(),
                },
                Submodule {
                    name: "hashmap_old_values".to_string(),
                    description: "Hashmap Old Values".to_string(),
                },
            ],
        },
        Module {
            name: "concurrency".to_string(),
            description: "Concurrent programming patterns".to_string(),
            category: "Concurrency".to_string(),
            submodules: vec![
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "message_passing".to_string(),
                    description: "Message Passing".to_string(),
                },
                Submodule {
                    name: "scoped_threads".to_string(),
                    description: "Scoped Threads".to_string(),
                },
                Submodule {
                    name: "sharing_state_concurrently".to_string(),
                    description: "Sharing State Concurrently".to_string(),
                },
            ],
        },
        Module {
            name: "cow_type".to_string(),
            description: "Clone-on-Write smart pointer for efficient borrowing".to_string(),
            category: "Advanced".to_string(),
            submodules: vec![
                Submodule {
                    name: "add_remove".to_string(),
                    description: "Add Remove".to_string(),
                },
                Submodule {
                    name: "create_vector".to_string(),
                    description: "Create Vector".to_string(),
                },
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "get_method".to_string(),
                    description: "Get Method".to_string(),
                },
                Submodule {
                    name: "reading_vector".to_string(),
                    description: "Reading Vector".to_string(),
                },
                Submodule {
                    name: "sort_vector".to_string(),
                    description: "Sort Vector".to_string(),
                },
                Submodule {
                    name: "vector_capacity".to_string(),
                    description: "Vector Capacity".to_string(),
                },
            ],
        },
        Module {
            name: "cubesats".to_string(),
            description: "CubeSat simulation example".to_string(),
            category: "Intermediate".to_string(),
            submodules: vec![],
        },
        Module {
            name: "custom_smart_pointer".to_string(),
            description: "Building custom smart pointers with Deref and Drop".to_string(),
            category: "Advanced".to_string(),
            submodules: vec![],
        },
        Module {
            name: "data_collections".to_string(),
            description: "Working with various data collection types".to_string(),
            category: "Data Structures".to_string(),
            submodules: vec![
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "shipping_box".to_string(),
                    description: "Shipping Box".to_string(),
                },
            ],
        },
        Module {
            name: "decisions".to_string(),
            description: "Decision making with if/else and pattern matching".to_string(),
            category: "Basics".to_string(),
            submodules: vec![],
        },
        Module {
            name: "deconstructing_float".to_string(),
            description: "IEEE 754 floating point representation analysis".to_string(),
            category: "Advanced".to_string(),
            submodules: vec![],
        },
        Module {
            name: "destructuring_structs".to_string(),
            description: "Pattern matching on struct fields".to_string(),
            category: "Intermediate".to_string(),
            submodules: vec![],
        },
        Module {
            name: "doc".to_string(),
            description: "Documentation comments and rustdoc".to_string(),
            category: "Basics".to_string(),
            submodules: vec![],
        },
        Module {
            name: "draw".to_string(),
            description: "Drawing and graphics examples".to_string(),
            category: "Intermediate".to_string(),
            submodules: vec![],
        },
        Module {
            name: "enums".to_string(),
            description: "Enumeration types and pattern matching".to_string(),
            category: "Basics".to_string(),
            submodules: vec![
                Submodule {
                    name: "cards".to_string(),
                    description: "Cards".to_string(),
                },
                Submodule {
                    name: "catching_multiple_values".to_string(),
                    description: "Catching Multiple Values".to_string(),
                },
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "demo".to_string(),
                    description: "Demo".to_string(),
                },
                Submodule {
                    name: "emulator".to_string(),
                    description: "Emulator".to_string(),
                },
                Submodule {
                    name: "enum_multi_array".to_string(),
                    description: "Enum Multi Array".to_string(),
                },
                Submodule {
                    name: "enum_with_data".to_string(),
                    description: "Enum With Data".to_string(),
                },
                Submodule {
                    name: "enum_with_match".to_string(),
                    description: "Enum With Match".to_string(),
                },
                Submodule {
                    name: "enums_and_functions".to_string(),
                    description: "Enums And Functions".to_string(),
                },
                Submodule {
                    name: "enums_with_enums".to_string(),
                    description: "Enums With Enums".to_string(),
                },
                Submodule {
                    name: "event".to_string(),
                    description: "Event".to_string(),
                },
                Submodule {
                    name: "german_months".to_string(),
                    description: "German Months".to_string(),
                },
                Submodule {
                    name: "hold_data".to_string(),
                    description: "Hold Data".to_string(),
                },
                Submodule {
                    name: "match_exact_value".to_string(),
                    description: "Match Exact Value".to_string(),
                },
                Submodule {
                    name: "matches".to_string(),
                    description: "Matches".to_string(),
                },
                Submodule {
                    name: "options".to_string(),
                    description: "Options".to_string(),
                },
                Submodule {
                    name: "payment_method_type".to_string(),
                    description: "Payment Method Type".to_string(),
                },
                Submodule {
                    name: "result_enum".to_string(),
                    description: "Result Enum".to_string(),
                },
            ],
        },
        Module {
            name: "errors".to_string(),
            description: "Error handling with Result and custom error types".to_string(),
            category: "Intermediate".to_string(),
            submodules: vec![
                Submodule {
                    name: "catch_all".to_string(),
                    description: "Catch All".to_string(),
                },
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "if_let".to_string(),
                    description: "If Let".to_string(),
                },
                Submodule {
                    name: "match_control_flow".to_string(),
                    description: "Match Control Flow".to_string(),
                },
                Submodule {
                    name: "match_guard".to_string(),
                    description: "Match Guard".to_string(),
                },
                Submodule {
                    name: "match_vector".to_string(),
                    description: "Match Vector".to_string(),
                },
                Submodule {
                    name: "multiple_placeholders".to_string(),
                    description: "Multiple Placeholders".to_string(),
                },
                Submodule {
                    name: "pattern_match".to_string(),
                    description: "Pattern Match".to_string(),
                },
                Submodule {
                    name: "placeholder".to_string(),
                    description: "Placeholder".to_string(),
                },
                Submodule {
                    name: "while_let".to_string(),
                    description: "While Let".to_string(),
                },
            ],
        },
        Module {
            name: "exercises".to_string(),
            description: "Practice exercises and challenges".to_string(),
            category: "Basics".to_string(),
            submodules: vec![
                Submodule {
                    name: "ex03".to_string(),
                    description: "Exercise 03".to_string(),
                },
                Submodule {
                    name: "ex08".to_string(),
                    description: "Exercise 08".to_string(),
                },
            ],
        },
        Module {
            name: "expressions".to_string(),
            description: "Expressions vs statements in Rust".to_string(),
            category: "Basics".to_string(),
            submodules: vec![
                Submodule {
                    name: "ex03".to_string(),
                    description: "Ex03".to_string(),
                },
                Submodule {
                    name: "ex08".to_string(),
                    description: "Ex08".to_string(),
                },
            ],
        },
        Module {
            name: "extern_functions".to_string(),
            description: "Foreign Function Interface (FFI) examples".to_string(),
            category: "System".to_string(),
            submodules: vec![
                Submodule {
                    name: "activity_custom_errors".to_string(),
                    description: "Activity Custom Errors".to_string(),
                },
                Submodule {
                    name: "closures".to_string(),
                    description: "Closures".to_string(),
                },
                Submodule {
                    name: "custom_errors".to_string(),
                    description: "Custom Errors".to_string(),
                },
                Submodule {
                    name: "demo_custom_error".to_string(),
                    description: "Demo Custom Error".to_string(),
                },
                Submodule {
                    name: "error_handling".to_string(),
                    description: "Error Handling".to_string(),
                },
                Submodule {
                    name: "error_propagation".to_string(),
                    description: "Error Propagation".to_string(),
                },
            ],
        },
        Module {
            name: "external_crates".to_string(),
            description: "Using external dependencies from crates.io".to_string(),
            category: "Intermediate".to_string(),
            submodules: vec![Submodule {
                name: "demo".to_string(),
                description: "Demo".to_string(),
            }],
        },
        Module {
            name: "files".to_string(),
            description: "File I/O operations".to_string(),
            category: "Intermediate".to_string(),
            submodules: vec![],
        },
        Module {
            name: "for_loops".to_string(),
            description: "For loop iteration patterns".to_string(),
            category: "Basics".to_string(),
            submodules: vec![],
        },
        Module {
            name: "formatted_print".to_string(),
            description: "Formatted output with println! and format!".to_string(),
            category: "Basics".to_string(),
            submodules: vec![],
        },
        Module {
            name: "functions".to_string(),
            description: "Function definition and usage".to_string(),
            category: "Basics".to_string(),
            submodules: vec![
                Submodule {
                    name: "blocks_in_functions".to_string(),
                    description: "Blocks In Functions".to_string(),
                },
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "func_return_option".to_string(),
                    description: "Func Return Option".to_string(),
                },
            ],
        },
        Module {
            name: "futures".to_string(),
            description: "Asynchronous programming with futures".to_string(),
            category: "Advanced".to_string(),
            submodules: vec![],
        },
        Module {
            name: "generics".to_string(),
            description: "Generic types and functions".to_string(),
            category: "Intermediate".to_string(),
            submodules: vec![
                Submodule {
                    name: "conveyor_belt".to_string(),
                    description: "Conveyor Belt".to_string(),
                },
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "demo".to_string(),
                    description: "Demo".to_string(),
                },
                Submodule {
                    name: "generic_add".to_string(),
                    description: "Generic Add".to_string(),
                },
                Submodule {
                    name: "generic_enums".to_string(),
                    description: "Generic Enums".to_string(),
                },
                Submodule {
                    name: "generic_functions".to_string(),
                    description: "Generic Functions".to_string(),
                },
                Submodule {
                    name: "generic_implementation".to_string(),
                    description: "Generic Implementation".to_string(),
                },
                Submodule {
                    name: "generic_structs".to_string(),
                    description: "Generic Structs".to_string(),
                },
                Submodule {
                    name: "generic_structures".to_string(),
                    description: "Generic Structures".to_string(),
                },
                Submodule {
                    name: "generic_with_where".to_string(),
                    description: "Generic With Where".to_string(),
                },
                Submodule {
                    name: "introduction".to_string(),
                    description: "Introduction".to_string(),
                },
                Submodule {
                    name: "multiple_generics".to_string(),
                    description: "Multiple Generics".to_string(),
                },
                Submodule {
                    name: "question_operator".to_string(),
                    description: "Question Operator".to_string(),
                },
                Submodule {
                    name: "vehicle".to_string(),
                    description: "Vehicle".to_string(),
                },
            ],
        },
        Module {
            name: "get_type".to_string(),
            description: "Runtime type introspection".to_string(),
            category: "Intermediate".to_string(),
            submodules: vec![],
        },
        Module {
            name: "guessing_game".to_string(),
            description: "Classic guessing game tutorial".to_string(),
            category: "Basics".to_string(),
            submodules: vec![],
        },
        Module {
            name: "hashmaps".to_string(),
            description: "HashMap data structure and operations".to_string(),
            category: "Data Structures".to_string(),
            submodules: vec![
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "furniture".to_string(),
                    description: "Furniture".to_string(),
                },
                Submodule {
                    name: "hashmap_entry_method".to_string(),
                    description: "Hashmap Entry Method".to_string(),
                },
                Submodule {
                    name: "hashmap_get".to_string(),
                    description: "Hashmap Get".to_string(),
                },
                Submodule {
                    name: "hashmap_loop".to_string(),
                    description: "Hashmap Loop".to_string(),
                },
                Submodule {
                    name: "hashmap_old_values".to_string(),
                    description: "Hashmap Old Values".to_string(),
                },
            ],
        },
        Module {
            name: "hello_world".to_string(),
            description: "The classic first program".to_string(),
            category: "Basics".to_string(),
            submodules: vec![],
        },
        Module {
            name: "inspecting_endianness".to_string(),
            description: "Byte order and endianness detection".to_string(),
            category: "System".to_string(),
            submodules: vec![],
        },
        Module {
            name: "interior_mutability".to_string(),
            description: "Interior mutability patterns with Cell and RefCell".to_string(),
            category: "Advanced".to_string(),
            submodules: vec![],
        },
        Module {
            name: "interpreting_float".to_string(),
            description: "Floating point number interpretation".to_string(),
            category: "Advanced".to_string(),
            submodules: vec![],
        },
        Module {
            name: "iterators".to_string(),
            description: "Iterator trait and lazy evaluation".to_string(),
            category: "Intermediate".to_string(),
            submodules: vec![
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "demo".to_string(),
                    description: "Demo".to_string(),
                },
                Submodule {
                    name: "extend".to_string(),
                    description: "Extend".to_string(),
                },
                Submodule {
                    name: "filter_map".to_string(),
                    description: "Filter Map".to_string(),
                },
                Submodule {
                    name: "find_position".to_string(),
                    description: "Find Position".to_string(),
                },
                Submodule {
                    name: "fold".to_string(),
                    description: "Fold".to_string(),
                },
                Submodule {
                    name: "functional_style".to_string(),
                    description: "Functional Style".to_string(),
                },
                Submodule {
                    name: "impl_iterator".to_string(),
                    description: "Impl Iterator".to_string(),
                },
                Submodule {
                    name: "iter_in_collections".to_string(),
                    description: "Iter In Collections".to_string(),
                },
                Submodule {
                    name: "iterator_next".to_string(),
                    description: "Iterator Next".to_string(),
                },
                Submodule {
                    name: "mapping_and_filtering".to_string(),
                    description: "Mapping And Filtering".to_string(),
                },
                Submodule {
                    name: "nonfunctional_style".to_string(),
                    description: "Nonfunctional Style".to_string(),
                },
                Submodule {
                    name: "partition".to_string(),
                    description: "Partition".to_string(),
                },
                Submodule {
                    name: "result_option_ok".to_string(),
                    description: "Result Option Ok".to_string(),
                },
                Submodule {
                    name: "rev".to_string(),
                    description: "Rev".to_string(),
                },
                Submodule {
                    name: "scan_hash_table".to_string(),
                    description: "Scan Hash Table".to_string(),
                },
                Submodule {
                    name: "skip".to_string(),
                    description: "Skip".to_string(),
                },
                Submodule {
                    name: "without_for".to_string(),
                    description: "Without For".to_string(),
                },
            ],
        },
        Module {
            name: "joinhandle".to_string(),
            description: "Thread join handles for synchronization".to_string(),
            category: "Concurrency".to_string(),
            submodules: vec![],
        },
        Module {
            name: "lifetimes".to_string(),
            description: "Lifetime annotations and borrow checker".to_string(),
            category: "Advanced".to_string(),
            submodules: vec![
                Submodule {
                    name: "anonymous_lifetime".to_string(),
                    description: "Anonymous Lifetime".to_string(),
                },
                Submodule {
                    name: "compare_activity".to_string(),
                    description: "Compare Activity".to_string(),
                },
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "demo".to_string(),
                    description: "Demo".to_string(),
                },
                Submodule {
                    name: "example".to_string(),
                    description: "Example".to_string(),
                },
                Submodule {
                    name: "multiple_lifetimes".to_string(),
                    description: "Multiple Lifetimes".to_string(),
                },
                Submodule {
                    name: "named_lifetime".to_string(),
                    description: "Named Lifetime".to_string(),
                },
            ],
        },
        Module {
            name: "loops".to_string(),
            description: "Loop constructs: loop, while, for".to_string(),
            category: "Basics".to_string(),
            submodules: vec![
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "returning_values_from_loops".to_string(),
                    description: "Returning Values From Loops".to_string(),
                },
            ],
        },
        Module {
            name: "machine_learning".to_string(),
            description: "Machine learning examples with Polars".to_string(),
            category: "Advanced".to_string(),
            submodules: vec![Submodule {
                name: "reading_csv_file".to_string(),
                description: "Reading and processing CSV files with Polars".to_string(),
            }],
        },
        Module {
            name: "macros".to_string(),
            description: "Macro rules and procedural macros".to_string(),
            category: "Advanced".to_string(),
            submodules: vec![
                Submodule {
                    name: "create_vector".to_string(),
                    description: "Create Vector".to_string(),
                },
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
            ],
        },
        Module {
            name: "mandelbrot".to_string(),
            description: "Mandelbrot set fractal generation".to_string(),
            category: "Intermediate".to_string(),
            submodules: vec![],
        },
        Module {
            name: "matches".to_string(),
            description: "Pattern matching fundamentals".to_string(),
            category: "Basics".to_string(),
            submodules: vec![
                Submodule {
                    name: "catch_all".to_string(),
                    description: "Catch All".to_string(),
                },
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "if_let".to_string(),
                    description: "If Let".to_string(),
                },
                Submodule {
                    name: "match_control_flow".to_string(),
                    description: "Match Control Flow".to_string(),
                },
                Submodule {
                    name: "match_guard".to_string(),
                    description: "Match Guard".to_string(),
                },
                Submodule {
                    name: "match_vector".to_string(),
                    description: "Match Vector".to_string(),
                },
                Submodule {
                    name: "multiple_placeholders".to_string(),
                    description: "Multiple Placeholders".to_string(),
                },
                Submodule {
                    name: "pattern_match".to_string(),
                    description: "Pattern Match".to_string(),
                },
                Submodule {
                    name: "placeholder".to_string(),
                    description: "Placeholder".to_string(),
                },
                Submodule {
                    name: "while_let".to_string(),
                    description: "While Let".to_string(),
                },
            ],
        },
        Module {
            name: "message_passing".to_string(),
            description: "Inter-thread communication with message passing".to_string(),
            category: "Concurrency".to_string(),
            submodules: vec![],
        },
        Module {
            name: "messenger".to_string(),
            description: "Messaging patterns and implementations".to_string(),
            category: "Intermediate".to_string(),
            submodules: vec![Submodule {
                name: "definition".to_string(),
                description: "Definition".to_string(),
            }],
        },
        Module {
            name: "methods".to_string(),
            description: "Struct methods and associated functions".to_string(),
            category: "Basics".to_string(),
            submodules: vec![
                Submodule {
                    name: "associated_functions".to_string(),
                    description: "Associated Functions".to_string(),
                },
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "multiple_impl_blocks".to_string(),
                    description: "Multiple Impl Blocks".to_string(),
                },
                Submodule {
                    name: "parameters".to_string(),
                    description: "Parameters".to_string(),
                },
            ],
        },
        Module {
            name: "modules".to_string(),
            description: "Module system and code organization".to_string(),
            category: "Intermediate".to_string(),
            submodules: vec![
                Submodule {
                    name: "inline_modules_activity".to_string(),
                    description: "Inline Modules Activity".to_string(),
                },
                Submodule {
                    name: "inline_modules_demo".to_string(),
                    description: "Inline Modules Demo".to_string(),
                },
                Submodule {
                    name: "mod_basics".to_string(),
                    description: "Mod Basics".to_string(),
                },
            ],
        },
        Module {
            name: "move_vars".to_string(),
            description: "Move semantics and ownership transfer".to_string(),
            category: "Intermediate".to_string(),
            submodules: vec![],
        },
        Module {
            name: "mutable_reference".to_string(),
            description: "Mutable references and borrowing rules".to_string(),
            category: "Intermediate".to_string(),
            submodules: vec![],
        },
        Module {
            name: "network".to_string(),
            description: "Network programming examples".to_string(),
            category: "Advanced".to_string(),
            submodules: vec![],
        },
        Module {
            name: "new_type".to_string(),
            description: "Newtype pattern for type safety".to_string(),
            category: "Intermediate".to_string(),
            submodules: vec![
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "demo".to_string(),
                    description: "Demo".to_string(),
                },
            ],
        },
        Module {
            name: "numbers".to_string(),
            description: "Numeric types and operations".to_string(),
            category: "Basics".to_string(),
            submodules: vec![],
        },
        Module {
            name: "object_oriented".to_string(),
            description: "Object-oriented programming patterns in Rust".to_string(),
            category: "Intermediate".to_string(),
            submodules: vec![],
        },
        Module {
            name: "open_file".to_string(),
            description: "File opening and handling".to_string(),
            category: "Intermediate".to_string(),
            submodules: vec![],
        },
        Module {
            name: "operator_overloading".to_string(),
            description: "Implementing operator traits".to_string(),
            category: "Advanced".to_string(),
            submodules: vec![],
        },
        Module {
            name: "optionals".to_string(),
            description: "Option type for null safety".to_string(),
            category: "Basics".to_string(),
            submodules: vec![
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "unwrap_or_else".to_string(),
                    description: "Unwrap Or Else".to_string(),
                },
            ],
        },
        Module {
            name: "ownership".to_string(),
            description: "Ownership system fundamentals".to_string(),
            category: "Basics".to_string(),
            submodules: vec![
                Submodule {
                    name: "borrowing".to_string(),
                    description: "Borrowing".to_string(),
                },
                Submodule {
                    name: "dereference_operator".to_string(),
                    description: "Dereference Operator".to_string(),
                },
                Submodule {
                    name: "functions".to_string(),
                    description: "Functions".to_string(),
                },
                Submodule {
                    name: "mutable_borrowing".to_string(),
                    description: "Mutable Borrowing".to_string(),
                },
                Submodule {
                    name: "mutable_references".to_string(),
                    description: "Mutable References".to_string(),
                },
                Submodule {
                    name: "ownership_arrays".to_string(),
                    description: "Ownership Arrays".to_string(),
                },
                Submodule {
                    name: "references_borrowing".to_string(),
                    description: "References Borrowing".to_string(),
                },
                Submodule {
                    name: "return_multiple_values".to_string(),
                    description: "Return Multiple Values".to_string(),
                },
                Submodule {
                    name: "return_values_and_scope".to_string(),
                    description: "Return Values And Scope".to_string(),
                },
                Submodule {
                    name: "returning_ownership".to_string(),
                    description: "Returning Ownership".to_string(),
                },
                Submodule {
                    name: "string_clone_heap".to_string(),
                    description: "String Clone Heap".to_string(),
                },
                Submodule {
                    name: "string_type".to_string(),
                    description: "String Type".to_string(),
                },
            ],
        },
        Module {
            name: "patterns".to_string(),
            description: "Pattern matching and destructuring".to_string(),
            category: "Intermediate".to_string(),
            submodules: vec![],
        },
        Module {
            name: "post".to_string(),
            description: "Blog post state machine example".to_string(),
            category: "Intermediate".to_string(),
            submodules: vec![],
        },
        Module {
            name: "print_var_type".to_string(),
            description: "Printing variable types".to_string(),
            category: "Basics".to_string(),
            submodules: vec![],
        },
        Module {
            name: "random_numbers".to_string(),
            description: "Random number generation with rand crate".to_string(),
            category: "Intermediate".to_string(),
            submodules: vec![],
        },
        Module {
            name: "ranges".to_string(),
            description: "Range expressions and iteration".to_string(),
            category: "Basics".to_string(),
            submodules: vec![],
        },
        Module {
            name: "rc_t".to_string(),
            description: "Reference counting with Rc and Arc".to_string(),
            category: "Advanced".to_string(),
            submodules: vec![],
        },
        Module {
            name: "references".to_string(),
            description: "References and borrowing basics".to_string(),
            category: "Basics".to_string(),
            submodules: vec![
                Submodule {
                    name: "basic_reference".to_string(),
                    description: "Basic Reference".to_string(),
                },
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "mut_reference".to_string(),
                    description: "Mut Reference".to_string(),
                },
                Submodule {
                    name: "ref_of_references".to_string(),
                    description: "Ref Of References".to_string(),
                },
            ],
        },
        Module {
            name: "results".to_string(),
            description: "Result type for error handling".to_string(),
            category: "Intermediate".to_string(),
            submodules: vec![
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "question_mark_operator".to_string(),
                    description: "Question Mark Operator".to_string(),
                },
            ],
        },
        Module {
            name: "rust_atomics_and_locks".to_string(),
            description: "Low-level concurrency with atomics and locks".to_string(),
            category: "Advanced".to_string(),
            submodules: vec![],
        },
        Module {
            name: "shadowing".to_string(),
            description: "Variable shadowing and rebinding".to_string(),
            category: "Basics".to_string(),
            submodules: vec![],
        },
        Module {
            name: "slices".to_string(),
            description: "Slice types and references".to_string(),
            category: "Basics".to_string(),
            submodules: vec![
                Submodule {
                    name: "array_slices".to_string(),
                    description: "Array Slices".to_string(),
                },
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "mutable_slice".to_string(),
                    description: "Mutable Slice".to_string(),
                },
                Submodule {
                    name: "reference_array".to_string(),
                    description: "Reference Array".to_string(),
                },
            ],
        },
        Module {
            name: "static_variable".to_string(),
            description: "Static variables and lifetimes".to_string(),
            category: "Intermediate".to_string(),
            submodules: vec![],
        },
        Module {
            name: "strings".to_string(),
            description: "String and &str types".to_string(),
            category: "Basics".to_string(),
            submodules: vec![
                Submodule {
                    name: "borrowed_string".to_string(),
                    description: "Borrowed String".to_string(),
                },
                Submodule {
                    name: "concatenation".to_string(),
                    description: "Concatenation".to_string(),
                },
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "iterate".to_string(),
                    description: "Iterate".to_string(),
                },
                Submodule {
                    name: "matches".to_string(),
                    description: "Matches".to_string(),
                },
                Submodule {
                    name: "string_functions".to_string(),
                    description: "String Functions".to_string(),
                },
                Submodule {
                    name: "string_is_encoded".to_string(),
                    description: "String Is Encoded".to_string(),
                },
                Submodule {
                    name: "string_literals".to_string(),
                    description: "String Literals".to_string(),
                },
                Submodule {
                    name: "substrings".to_string(),
                    description: "Substrings".to_string(),
                },
            ],
        },
        Module {
            name: "structs".to_string(),
            description: "Struct definitions and usage".to_string(),
            category: "Basics".to_string(),
            submodules: vec![
                Submodule {
                    name: "builder_pattern".to_string(),
                    description: "Builder Pattern".to_string(),
                },
                Submodule {
                    name: "coffee".to_string(),
                    description: "Coffee".to_string(),
                },
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "file".to_string(),
                    description: "File".to_string(),
                },
                Submodule {
                    name: "human".to_string(),
                    description: "Human".to_string(),
                },
                Submodule {
                    name: "integrate_struct_enum_match".to_string(),
                    description: "Integrate Struct Enum Match".to_string(),
                },
                Submodule {
                    name: "methods".to_string(),
                    description: "Methods".to_string(),
                },
                Submodule {
                    name: "person".to_string(),
                    description: "Person".to_string(),
                },
                Submodule {
                    name: "queue".to_string(),
                    description: "Queue".to_string(),
                },
                Submodule {
                    name: "struct_update_syntax".to_string(),
                    description: "Struct Update Syntax".to_string(),
                },
                Submodule {
                    name: "structs_owned".to_string(),
                    description: "Structs Owned".to_string(),
                },
                Submodule {
                    name: "tuple_structs".to_string(),
                    description: "Tuple Structs".to_string(),
                },
                Submodule {
                    name: "unit_struct".to_string(),
                    description: "Unit Struct".to_string(),
                },
            ],
        },
        Module {
            name: "system_calls".to_string(),
            description: "System call interface with libc".to_string(),
            category: "System".to_string(),
            submodules: vec![
                Submodule {
                    name: "c_call".to_string(),
                    description: "C Call".to_string(),
                },
                Submodule {
                    name: "raw_syscall".to_string(),
                    description: "Raw Syscall".to_string(),
                },
            ],
        },
        Module {
            name: "testing".to_string(),
            description: "Unit testing and integration tests".to_string(),
            category: "Intermediate".to_string(),
            submodules: vec![
                Submodule {
                    name: "definition".to_string(),
                    description: "Test definitions and structure".to_string(),
                },
                Submodule {
                    name: "demo".to_string(),
                    description: "Testing demonstration".to_string(),
                },
            ],
        },
        Module {
            name: "threads".to_string(),
            description: "Thread creation and management".to_string(),
            category: "Concurrency".to_string(),
            submodules: vec![
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "message_passing_between_threads".to_string(),
                    description: "Message Passing Between Threads".to_string(),
                },
                Submodule {
                    name: "move_data_between_threads".to_string(),
                    description: "Move Data Between Threads".to_string(),
                },
                Submodule {
                    name: "multithreading".to_string(),
                    description: "Multithreading".to_string(),
                },
                Submodule {
                    name: "mutexes".to_string(),
                    description: "Mutexes".to_string(),
                },
                Submodule {
                    name: "preventing_race_condition".to_string(),
                    description: "Preventing Race Condition".to_string(),
                },
                Submodule {
                    name: "scoped_threads".to_string(),
                    description: "Scoped Threads".to_string(),
                },
                Submodule {
                    name: "sending_multiple_values".to_string(),
                    description: "Sending Multiple Values".to_string(),
                },
                Submodule {
                    name: "sharing_data_between_threads".to_string(),
                    description: "Sharing Data Between Threads".to_string(),
                },
                Submodule {
                    name: "threads_in_array".to_string(),
                    description: "Threads In Array".to_string(),
                },
            ],
        },
        Module {
            name: "traits".to_string(),
            description: "Trait definitions and implementations".to_string(),
            category: "Intermediate".to_string(),
            submodules: vec![
                Submodule {
                    name: "activity_trait_objects".to_string(),
                    description: "Activity Trait Objects".to_string(),
                },
                Submodule {
                    name: "complex_trait".to_string(),
                    description: "Complex Trait".to_string(),
                },
                Submodule {
                    name: "copy_trait".to_string(),
                    description: "Copy Trait".to_string(),
                },
                Submodule {
                    name: "default_trait".to_string(),
                    description: "Default Trait".to_string(),
                },
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "demo".to_string(),
                    description: "Demo".to_string(),
                },
                Submodule {
                    name: "demo_trait_objets".to_string(),
                    description: "Demo Trait Objets".to_string(),
                },
                Submodule {
                    name: "derives".to_string(),
                    description: "Derives".to_string(),
                },
                Submodule {
                    name: "from_trait".to_string(),
                    description: "From Trait".to_string(),
                },
                Submodule {
                    name: "impl_display".to_string(),
                    description: "Impl Display".to_string(),
                },
                Submodule {
                    name: "media_aggregator".to_string(),
                    description: "Media Aggregator".to_string(),
                },
                Submodule {
                    name: "returning_traits_with_dyn".to_string(),
                    description: "Returning Traits With Dyn".to_string(),
                },
                Submodule {
                    name: "simple_trait".to_string(),
                    description: "Simple Trait".to_string(),
                },
                Submodule {
                    name: "with_generics".to_string(),
                    description: "With Generics".to_string(),
                },
            ],
        },
        Module {
            name: "tree_data_structure".to_string(),
            description: "Tree data structure implementations".to_string(),
            category: "Data Structures".to_string(),
            submodules: vec![
                Submodule {
                    name: "catch_all".to_string(),
                    description: "Catch All".to_string(),
                },
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "if_let".to_string(),
                    description: "If Let".to_string(),
                },
                Submodule {
                    name: "match_control_flow".to_string(),
                    description: "Match Control Flow".to_string(),
                },
                Submodule {
                    name: "match_guard".to_string(),
                    description: "Match Guard".to_string(),
                },
                Submodule {
                    name: "match_vector".to_string(),
                    description: "Match Vector".to_string(),
                },
                Submodule {
                    name: "multiple_placeholders".to_string(),
                    description: "Multiple Placeholders".to_string(),
                },
                Submodule {
                    name: "pattern_match".to_string(),
                    description: "Pattern Match".to_string(),
                },
                Submodule {
                    name: "placeholder".to_string(),
                    description: "Placeholder".to_string(),
                },
                Submodule {
                    name: "while_let".to_string(),
                    description: "While Let".to_string(),
                },
            ],
        },
        Module {
            name: "tuples".to_string(),
            description: "Tuple types and destructuring".to_string(),
            category: "Basics".to_string(),
            submodules: vec![
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "single_tuple".to_string(),
                    description: "Single Tuple".to_string(),
                },
            ],
        },
        Module {
            name: "type_aliases".to_string(),
            description: "Type aliases for code clarity".to_string(),
            category: "Intermediate".to_string(),
            submodules: vec![
                Submodule {
                    name: "demo".to_string(),
                    description: "Demo".to_string(),
                },
                Submodule {
                    name: "long_types".to_string(),
                    description: "Long Types".to_string(),
                },
            ],
        },
        Module {
            name: "typestate_pattern".to_string(),
            description: "Compile-time state machine pattern".to_string(),
            category: "Advanced".to_string(),
            submodules: vec![Submodule {
                name: "demo".to_string(),
                description: "Demo".to_string(),
            }],
        },
        Module {
            name: "unsafe_code".to_string(),
            description: "Unsafe Rust and raw pointers".to_string(),
            category: "Advanced".to_string(),
            submodules: vec![
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "demo".to_string(),
                    description: "Demo".to_string(),
                },
            ],
        },
        Module {
            name: "unsafe_trait".to_string(),
            description: "Unsafe trait implementations".to_string(),
            category: "Advanced".to_string(),
            submodules: vec![],
        },
        Module {
            name: "user_input".to_string(),
            description: "Reading and processing user input".to_string(),
            category: "Basics".to_string(),
            submodules: vec![Submodule {
                name: "demo".to_string(),
                description: "User input demonstration".to_string(),
            }],
        },
        Module {
            name: "variables".to_string(),
            description: "Variable declaration and mutability".to_string(),
            category: "Basics".to_string(),
            submodules: vec![
                Submodule {
                    name: "constants".to_string(),
                    description: "Constants".to_string(),
                },
                Submodule {
                    name: "vars".to_string(),
                    description: "Vars".to_string(),
                },
            ],
        },
        Module {
            name: "vectors".to_string(),
            description: "Dynamic array Vec type".to_string(),
            category: "Data Structures".to_string(),
            submodules: vec![
                Submodule {
                    name: "add_remove".to_string(),
                    description: "Add Remove".to_string(),
                },
                Submodule {
                    name: "create_vector".to_string(),
                    description: "Create Vector".to_string(),
                },
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "get_method".to_string(),
                    description: "Get Method".to_string(),
                },
                Submodule {
                    name: "reading_vector".to_string(),
                    description: "Reading Vector".to_string(),
                },
                Submodule {
                    name: "sort_vector".to_string(),
                    description: "Sort Vector".to_string(),
                },
                Submodule {
                    name: "vector_capacity".to_string(),
                    description: "Vector Capacity".to_string(),
                },
            ],
        },
        Module {
            name: "webserver".to_string(),
            description: "HTTP web server implementation".to_string(),
            category: "Advanced".to_string(),
            submodules: vec![Submodule {
                name: "demo".to_string(),
                description: "Demo".to_string(),
            }],
        },
        Module {
            name: "while_loop".to_string(),
            description: "While loop patterns".to_string(),
            category: "Basics".to_string(),
            submodules: vec![
                Submodule {
                    name: "definition".to_string(),
                    description: "Definition".to_string(),
                },
                Submodule {
                    name: "while_let".to_string(),
                    description: "While Let".to_string(),
                },
            ],
        },
        Module {
            name: "zero_cost_abstraction".to_string(),
            description: "Zero-cost abstractions in Rust".to_string(),
            category: "Advanced".to_string(),
            submodules: vec![],
        },
    ]
}
