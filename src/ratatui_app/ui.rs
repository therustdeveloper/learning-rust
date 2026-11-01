//! src/ratatui_app/ui.rs
//!
//! Rendering only: reads `App` state and draws it, never mutates navigation.

use super::app::App;
use super::registry::{Category, TuiModule};
use ratatui::{
    layout::{Alignment, Constraint, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, List, ListItem, Padding, Paragraph, Scrollbar, ScrollbarOrientation, Wrap,
    },
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App) {
    let [header, main, footer] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(10),
        Constraint::Length(3),
    ])
    .areas(f.area());
    let [list, details] =
        Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)]).areas(main);

    render_header(f, header);
    render_module_list(f, app, list);
    match app.selected_module() {
        Some(module) if !module.sections().is_empty() => render_sections(f, app, details, module),
        Some(module) => render_module_details(f, app, details, module),
        None => f.render_widget(
            Paragraph::new("No module selected").block(panel(" Details ", Color::Yellow)),
            details,
        ),
    }
    render_footer(f, app, footer);
}

fn render_header(f: &mut Frame, area: Rect) {
    let title = Paragraph::new("Learning Rust - Module Browser")
        .style(bold(Color::Cyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .padding(Padding::horizontal(1)),
        );
    f.render_widget(title, area);
}

fn render_module_list(f: &mut Frame, app: &mut App, area: Rect) {
    let items = numbered(app.modules.iter().map(|module| module.name()), 3);
    let list = List::new(items)
        .block(
            panel(format!(" Modules ({}) ", app.modules.len()), Color::Yellow)
                .padding(Padding::horizontal(1)),
        )
        .highlight_style(bold(Color::White).bg(Color::DarkGray))
        .highlight_symbol("▶ ");
    f.render_stateful_widget(list, area, &mut app.module_list);

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));
    let track = area.inner(Margin {
        vertical: 1,
        horizontal: 0,
    });
    f.render_stateful_widget(scrollbar, track, &mut app.module_scroll);
}

/// Section list on top, selected section details or its output below.
fn render_sections(f: &mut Frame, app: &mut App, area: Rect, module: &TuiModule) {
    let [list, output] =
        Layout::vertical([Constraint::Percentage(40), Constraint::Percentage(60)]).areas(area);
    let sections = module.sections();

    let items = numbered(sections.iter().map(|section| section.name), 2);
    let title = format!(" {} Submodules ({}) ", module.name(), sections.len());
    let section_list = List::new(items)
        .block(panel(title, Color::Cyan).padding(Padding::horizontal(1)))
        .highlight_style(bold(Color::Black).bg(Color::Yellow))
        .highlight_symbol(">> ");
    f.render_stateful_widget(section_list, list, &mut app.section_list);

    let (title, body) = match (&app.output, app.selected_section()) {
        (Some(out), _) => (format!(" Output: {} ", out.source), out.text.clone()),
        (None, Some(section)) => (
            " Submodule Details ".to_string(),
            format!(
                "Submodule: {}\n\nDescription: {}\n\nPress Enter to run this submodule",
                section.name, section.description
            ),
        ),
        (None, None) => (
            " Submodule Details ".to_string(),
            "Use ↑↓ arrows to navigate submodules\nPress Enter to run selected submodule"
                .to_string(),
        ),
    };
    let paragraph = Paragraph::new(body)
        .block(panel(title, Color::Green).padding(Padding::uniform(1)))
        .wrap(Wrap { trim: false });
    f.render_widget(paragraph, output);
}

/// Module metadata, replaced by its output once it has been run.
fn render_module_details(f: &mut Frame, app: &App, area: Rect, module: &TuiModule) {
    let (title, content) = match &app.output {
        Some(out) => (
            format!(" Output: {} ", out.source),
            Text::from(out.text.as_str()),
        ),
        None => (" Details ".to_string(), module_summary(module)),
    };
    let paragraph = Paragraph::new(content)
        .block(panel(title, Color::Yellow).padding(Padding::uniform(1)))
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn module_summary(module: &TuiModule) -> Text<'static> {
    let label = |text| Span::styled(text, Style::default().fg(Color::DarkGray));
    Text::from(vec![
        Line::from(vec![
            label("Module: "),
            Span::styled(module.name(), bold(Color::White)),
        ]),
        Line::from(""),
        Line::from(vec![
            label("Category: "),
            Span::styled(
                module.category.label(),
                bold(category_color(module.category)),
            ),
        ]),
        Line::from(""),
        Line::from(label("Description:")),
        Line::from(module.description),
        Line::from(""),
        Line::from(vec![
            label("Location: "),
            Span::styled(
                format!("src/{}/", module.name()),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::ITALIC),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            label("Press "),
            Span::styled("Enter", bold(Color::Green)),
            label(" to run this module"),
        ]),
    ])
}

fn render_footer(f: &mut Frame, app: &App, area: Rect) {
    let mut keys = if app
        .selected_module()
        .is_some_and(|m| !m.sections().is_empty())
    {
        vec![
            (" ←/h ", Color::Cyan, "Prev Module  "),
            ("→/l ", Color::Cyan, "Next Module  "),
            ("↑/k ", Color::Yellow, "Prev Sub  "),
            ("↓/j ", Color::Yellow, "Next Sub  "),
            ("Enter ", Color::Green, "Run  "),
        ]
    } else {
        vec![
            (" ↑/k ", Color::Cyan, "Up  "),
            ("↓/j ", Color::Cyan, "Down  "),
            ("Enter ", Color::Green, "Run  "),
        ]
    };
    if app.output.is_some() {
        keys.push(("c/⌫ ", Color::Magenta, "Clear  "));
    }
    keys.extend([
        ("Home ", Color::Cyan, "First  "),
        ("End ", Color::Cyan, "Last  "),
        ("q/ESC ", Color::Red, "Quit"),
    ]);

    let spans: Vec<Span> = keys
        .into_iter()
        .flat_map(|(key, color, action)| [Span::styled(key, bold(color)), Span::raw(action)])
        .collect();
    let footer = Paragraph::new(Line::from(spans))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .padding(Padding::horizontal(1)),
        );
    f.render_widget(footer, area);
}

/// Bordered panel with a bold, colored title.
fn panel<'a>(title: impl Into<Line<'a>>, title_color: Color) -> Block<'a> {
    Block::default()
        .title(title)
        .title_style(bold(title_color))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
}

/// List items rendered as `  1. name`, with the index right-aligned to `width`.
fn numbered<'a>(names: impl Iterator<Item = &'a str>, width: usize) -> Vec<ListItem<'a>> {
    names
        .enumerate()
        .map(|(i, name)| {
            ListItem::new(Line::from(vec![
                Span::styled(
                    format!("{:>width$}. ", i + 1),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(name, Style::default().fg(Color::White)),
            ]))
        })
        .collect()
}

fn bold(color: Color) -> Style {
    Style::default().fg(color).add_modifier(Modifier::BOLD)
}

fn category_color(category: Category) -> Color {
    match category {
        Category::Basics => Color::Green,
        Category::Intermediate => Color::Yellow,
        Category::Advanced => Color::Red,
        Category::Concurrency => Color::Magenta,
        Category::DataStructures => Color::Blue,
        Category::System => Color::Cyan,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyCode;
    use ratatui::{backend::TestBackend, Terminal};

    fn screen(app: &mut App) -> String {
        let mut terminal = Terminal::new(TestBackend::new(160, 50)).unwrap();
        terminal.draw(|f| draw(f, app)).unwrap();
        let buffer = terminal.backend().buffer();
        (0..buffer.area.height)
            .map(|y| {
                (0..buffer.area.width)
                    .map(|x| buffer[(x, y)].symbol())
                    .collect::<String>()
                    + "\n"
            })
            .collect()
    }

    #[test]
    fn basics_lists_its_sections() {
        let mut app = App::new();
        while app.selected_module().map(|m| m.name()) != Some("basics") {
            app.on_key(KeyCode::Right);
        }
        app.on_key(KeyCode::Up);
        let screen = screen(&mut app);

        assert!(screen.contains("basics Submodules (26)"));
        assert!(screen.contains("Submodule: variables"));
        assert!(screen.contains("Description: Variables"));
    }
}
