//! Ratatui TUI Application - Module Browser
//!
//! A professional terminal user interface for browsing all available learning modules
//! in this repository. Navigate with arrow keys, press Enter to view details, and 'q' to quit.

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, List, ListItem, ListState, Padding, Paragraph, Scrollbar,
        ScrollbarOrientation, ScrollbarState, Wrap,
    },
    Frame, Terminal,
};
use std::io;

/// Main entry point following the repository's index pattern
pub fn index(show: bool) {
    if show {
        if let Err(e) = run_app() {
            eprintln!("Error running ratatui app: {}", e);
        }
    }
}

/// Module information structure
#[derive(Clone)]
struct Module {
    name: String,
    description: String,
    category: String,
}

/// Application state
struct App {
    modules: Vec<Module>,
    list_state: ListState,
    scroll_state: ScrollbarState,
    selected_module: Option<Module>,
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
    }
}

/// Run the TUI application
fn run_app() -> io::Result<()> {
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

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break Ok(()),
                    KeyCode::Down | KeyCode::Char('j') => app.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous(),
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
            Constraint::Length(3),  // Header
            Constraint::Min(10),    // Main content
            Constraint::Length(3),  // Footer
        ])
        .split(size);

    // Header
    render_header(f, chunks[0]);

    // Main content area - split into module list and details
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),  // Module list
            Constraint::Percentage(60),  // Details panel
        ])
        .split(chunks[1]);

    // Render module list
    render_module_list(f, app, main_chunks[0]);

    // Render details panel
    render_details_panel(f, app, main_chunks[1]);

    // Footer
    render_footer(f, chunks[2]);
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
                .title(" Modules ")
                .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
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

/// Render the details panel
fn render_details_panel(f: &mut Frame, app: &App, area: Rect) {
    let content = if let Some(module) = &app.selected_module {
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
                Span::styled(&module.name, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Category: ", Style::default().fg(Color::DarkGray)),
                Span::styled(&module.category, category_style.add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(Span::styled("Description:", Style::default().fg(Color::DarkGray))),
            Line::from(module.description.as_str()),
            Line::from(""),
            Line::from(vec![
                Span::styled("Location: ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format!("src/{}/", &module.name),
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::ITALIC),
                ),
            ]),
        ]);

        Paragraph::new(text)
    } else {
        Paragraph::new("No module selected")
    };

    let block = Block::default()
        .title(" Details ")
        .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .padding(Padding::uniform(1));

    f.render_widget(content.block(block).wrap(Wrap { trim: true }), area);
}

/// Render the footer section
fn render_footer(f: &mut Frame, area: Rect) {
    let help_text = Line::from(vec![
        Span::styled(" ↑/k ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw("Up  "),
        Span::styled("↓/j ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw("Down  "),
        Span::styled("Home ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw("First  "),
        Span::styled("End ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw("Last  "),
        Span::styled("q/ESC ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        Span::raw("Quit"),
    ]);

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

/// Get all modules with their metadata
fn get_all_modules() -> Vec<Module> {
    vec![
        Module { name: "add_millimeters_to_meters".to_string(), description: "Unit conversion example demonstrating basic arithmetic operations".to_string(), category: "Basics".to_string() },
        Module { name: "advanced_match".to_string(), description: "Advanced pattern matching techniques and match guards".to_string(), category: "Intermediate".to_string() },
        Module { name: "algorithms".to_string(), description: "Common algorithms implementation in Rust".to_string(), category: "Intermediate".to_string() },
        Module { name: "api_mutex".to_string(), description: "API examples using Mutex for safe concurrent access".to_string(), category: "Concurrency".to_string() },
        Module { name: "arrays".to_string(), description: "Fixed-size array operations and manipulations".to_string(), category: "Basics".to_string() },
        Module { name: "automatic_clean".to_string(), description: "Automatic resource cleanup using RAII pattern".to_string(), category: "Intermediate".to_string() },
        Module { name: "basics".to_string(), description: "Fundamental Rust concepts and syntax".to_string(), category: "Basics".to_string() },
        Module { name: "bit_patterns".to_string(), description: "Bit manipulation and binary patterns".to_string(), category: "Advanced".to_string() },
        Module { name: "boxes".to_string(), description: "Heap allocation with Box smart pointer".to_string(), category: "Intermediate".to_string() },
        Module { name: "channels".to_string(), description: "Message passing between threads using channels".to_string(), category: "Concurrency".to_string() },
        Module { name: "closures".to_string(), description: "Anonymous functions and closure captures".to_string(), category: "Intermediate".to_string() },
        Module { name: "collections".to_string(), description: "Standard library collection types".to_string(), category: "Data Structures".to_string() },
        Module { name: "compiler_directives".to_string(), description: "Conditional compilation and attributes".to_string(), category: "Advanced".to_string() },
        Module { name: "concurrency".to_string(), description: "Concurrent programming patterns".to_string(), category: "Concurrency".to_string() },
        Module { name: "cow_type".to_string(), description: "Clone-on-Write smart pointer for efficient borrowing".to_string(), category: "Advanced".to_string() },
        Module { name: "cubesats".to_string(), description: "CubeSat simulation example".to_string(), category: "Intermediate".to_string() },
        Module { name: "custom_smart_pointer".to_string(), description: "Building custom smart pointers with Deref and Drop".to_string(), category: "Advanced".to_string() },
        Module { name: "data_collections".to_string(), description: "Working with various data collection types".to_string(), category: "Data Structures".to_string() },
        Module { name: "decisions".to_string(), description: "Decision making with if/else and pattern matching".to_string(), category: "Basics".to_string() },
        Module { name: "deconstructing_float".to_string(), description: "IEEE 754 floating point representation analysis".to_string(), category: "Advanced".to_string() },
        Module { name: "destructuring_structs".to_string(), description: "Pattern matching on struct fields".to_string(), category: "Intermediate".to_string() },
        Module { name: "doc".to_string(), description: "Documentation comments and rustdoc".to_string(), category: "Basics".to_string() },
        Module { name: "draw".to_string(), description: "Drawing and graphics examples".to_string(), category: "Intermediate".to_string() },
        Module { name: "enums".to_string(), description: "Enumeration types and pattern matching".to_string(), category: "Basics".to_string() },
        Module { name: "errors".to_string(), description: "Error handling with Result and custom error types".to_string(), category: "Intermediate".to_string() },
        Module { name: "exercises".to_string(), description: "Practice exercises and challenges".to_string(), category: "Basics".to_string() },
        Module { name: "expressions".to_string(), description: "Expressions vs statements in Rust".to_string(), category: "Basics".to_string() },
        Module { name: "extern_functions".to_string(), description: "Foreign Function Interface (FFI) examples".to_string(), category: "System".to_string() },
        Module { name: "external_crates".to_string(), description: "Using external dependencies from crates.io".to_string(), category: "Intermediate".to_string() },
        Module { name: "files".to_string(), description: "File I/O operations".to_string(), category: "Intermediate".to_string() },
        Module { name: "for_loops".to_string(), description: "For loop iteration patterns".to_string(), category: "Basics".to_string() },
        Module { name: "formatted_print".to_string(), description: "Formatted output with println! and format!".to_string(), category: "Basics".to_string() },
        Module { name: "functions".to_string(), description: "Function definition and usage".to_string(), category: "Basics".to_string() },
        Module { name: "futures".to_string(), description: "Asynchronous programming with futures".to_string(), category: "Advanced".to_string() },
        Module { name: "generics".to_string(), description: "Generic types and functions".to_string(), category: "Intermediate".to_string() },
        Module { name: "get_type".to_string(), description: "Runtime type introspection".to_string(), category: "Intermediate".to_string() },
        Module { name: "guessing_game".to_string(), description: "Classic guessing game tutorial".to_string(), category: "Basics".to_string() },
        Module { name: "hashmaps".to_string(), description: "HashMap data structure and operations".to_string(), category: "Data Structures".to_string() },
        Module { name: "hello_world".to_string(), description: "The classic first program".to_string(), category: "Basics".to_string() },
        Module { name: "inspecting_endianness".to_string(), description: "Byte order and endianness detection".to_string(), category: "System".to_string() },
        Module { name: "interior_mutability".to_string(), description: "Interior mutability patterns with Cell and RefCell".to_string(), category: "Advanced".to_string() },
        Module { name: "interpreting_float".to_string(), description: "Floating point number interpretation".to_string(), category: "Advanced".to_string() },
        Module { name: "iterators".to_string(), description: "Iterator trait and lazy evaluation".to_string(), category: "Intermediate".to_string() },
        Module { name: "joinhandle".to_string(), description: "Thread join handles for synchronization".to_string(), category: "Concurrency".to_string() },
        Module { name: "lifetimes".to_string(), description: "Lifetime annotations and borrow checker".to_string(), category: "Advanced".to_string() },
        Module { name: "loops".to_string(), description: "Loop constructs: loop, while, for".to_string(), category: "Basics".to_string() },
        Module { name: "machine_learning".to_string(), description: "Machine learning examples with Polars".to_string(), category: "Advanced".to_string() },
        Module { name: "macros".to_string(), description: "Macro rules and procedural macros".to_string(), category: "Advanced".to_string() },
        Module { name: "mandelbrot".to_string(), description: "Mandelbrot set fractal generation".to_string(), category: "Intermediate".to_string() },
        Module { name: "matches".to_string(), description: "Pattern matching fundamentals".to_string(), category: "Basics".to_string() },
        Module { name: "message_passing".to_string(), description: "Inter-thread communication with message passing".to_string(), category: "Concurrency".to_string() },
        Module { name: "messenger".to_string(), description: "Messaging patterns and implementations".to_string(), category: "Intermediate".to_string() },
        Module { name: "methods".to_string(), description: "Struct methods and associated functions".to_string(), category: "Basics".to_string() },
        Module { name: "modules".to_string(), description: "Module system and code organization".to_string(), category: "Intermediate".to_string() },
        Module { name: "move_vars".to_string(), description: "Move semantics and ownership transfer".to_string(), category: "Intermediate".to_string() },
        Module { name: "mutable_reference".to_string(), description: "Mutable references and borrowing rules".to_string(), category: "Intermediate".to_string() },
        Module { name: "network".to_string(), description: "Network programming examples".to_string(), category: "Advanced".to_string() },
        Module { name: "new_type".to_string(), description: "Newtype pattern for type safety".to_string(), category: "Intermediate".to_string() },
        Module { name: "numbers".to_string(), description: "Numeric types and operations".to_string(), category: "Basics".to_string() },
        Module { name: "object_oriented".to_string(), description: "Object-oriented programming patterns in Rust".to_string(), category: "Intermediate".to_string() },
        Module { name: "open_file".to_string(), description: "File opening and handling".to_string(), category: "Intermediate".to_string() },
        Module { name: "operator_overloading".to_string(), description: "Implementing operator traits".to_string(), category: "Advanced".to_string() },
        Module { name: "optionals".to_string(), description: "Option type for null safety".to_string(), category: "Basics".to_string() },
        Module { name: "ownership".to_string(), description: "Ownership system fundamentals".to_string(), category: "Basics".to_string() },
        Module { name: "patterns".to_string(), description: "Pattern matching and destructuring".to_string(), category: "Intermediate".to_string() },
        Module { name: "post".to_string(), description: "Blog post state machine example".to_string(), category: "Intermediate".to_string() },
        Module { name: "print_var_type".to_string(), description: "Printing variable types".to_string(), category: "Basics".to_string() },
        Module { name: "random_numbers".to_string(), description: "Random number generation with rand crate".to_string(), category: "Intermediate".to_string() },
        Module { name: "ranges".to_string(), description: "Range expressions and iteration".to_string(), category: "Basics".to_string() },
        Module { name: "rc_t".to_string(), description: "Reference counting with Rc and Arc".to_string(), category: "Advanced".to_string() },
        Module { name: "references".to_string(), description: "References and borrowing basics".to_string(), category: "Basics".to_string() },
        Module { name: "results".to_string(), description: "Result type for error handling".to_string(), category: "Intermediate".to_string() },
        Module { name: "rust_atomics_and_locks".to_string(), description: "Low-level concurrency with atomics and locks".to_string(), category: "Advanced".to_string() },
        Module { name: "shadowing".to_string(), description: "Variable shadowing and rebinding".to_string(), category: "Basics".to_string() },
        Module { name: "slices".to_string(), description: "Slice types and references".to_string(), category: "Basics".to_string() },
        Module { name: "static_variable".to_string(), description: "Static variables and lifetimes".to_string(), category: "Intermediate".to_string() },
        Module { name: "strings".to_string(), description: "String and &str types".to_string(), category: "Basics".to_string() },
        Module { name: "structs".to_string(), description: "Struct definitions and usage".to_string(), category: "Basics".to_string() },
        Module { name: "system_calls".to_string(), description: "System call interface with libc".to_string(), category: "System".to_string() },
        Module { name: "testing".to_string(), description: "Unit testing and integration tests".to_string(), category: "Intermediate".to_string() },
        Module { name: "threads".to_string(), description: "Thread creation and management".to_string(), category: "Concurrency".to_string() },
        Module { name: "traits".to_string(), description: "Trait definitions and implementations".to_string(), category: "Intermediate".to_string() },
        Module { name: "tree_data_structure".to_string(), description: "Tree data structure implementations".to_string(), category: "Data Structures".to_string() },
        Module { name: "tuples".to_string(), description: "Tuple types and destructuring".to_string(), category: "Basics".to_string() },
        Module { name: "type_aliases".to_string(), description: "Type aliases for code clarity".to_string(), category: "Intermediate".to_string() },
        Module { name: "typestate_pattern".to_string(), description: "Compile-time state machine pattern".to_string(), category: "Advanced".to_string() },
        Module { name: "unsafe_code".to_string(), description: "Unsafe Rust and raw pointers".to_string(), category: "Advanced".to_string() },
        Module { name: "unsafe_trait".to_string(), description: "Unsafe trait implementations".to_string(), category: "Advanced".to_string() },
        Module { name: "user_input".to_string(), description: "Reading and processing user input".to_string(), category: "Basics".to_string() },
        Module { name: "variables".to_string(), description: "Variable declaration and mutability".to_string(), category: "Basics".to_string() },
        Module { name: "vectors".to_string(), description: "Dynamic array Vec type".to_string(), category: "Data Structures".to_string() },
        Module { name: "webserver".to_string(), description: "HTTP web server implementation".to_string(), category: "Advanced".to_string() },
        Module { name: "while_loop".to_string(), description: "While loop patterns".to_string(), category: "Basics".to_string() },
        Module { name: "zero_cost_abstraction".to_string(), description: "Zero-cost abstractions in Rust".to_string(), category: "Advanced".to_string() },
    ]
}
