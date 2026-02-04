use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, InputMode};

/// Draw the pull model dialog
pub fn draw_pull_dialog(f: &mut Frame, app: &App) {
    // Only show dialog if in pull mode
    if !matches!(
        app.input_mode,
        InputMode::PullDialog | InputMode::Pulling(_)
    ) {
        return;
    }

    // Calculate dialog size (centered popup)
    let area = centered_rect(60, 30, f.size());

    // Clear the area behind the dialog
    f.render_widget(Clear, area);

    // Create the dialog block
    let title = match &app.input_mode {
        InputMode::PullDialog => " Pull Model ",
        InputMode::Pulling(name) => &format!(" Pulling: {} ", name),
        _ => " Pull Model ",
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(title)
        .title_alignment(Alignment::Center);

    // Split dialog into sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Instructions
            Constraint::Length(3), // Input field
            Constraint::Length(2), // Error message
            Constraint::Min(3),    // Examples
            Constraint::Length(2), // Bottom help
        ])
        .split(area);

    // Render dialog background
    f.render_widget(block, area);

    // Instructions
    let instructions = if matches!(app.input_mode, InputMode::Pulling(_)) {
        Paragraph::new("Pulling model... Please wait.")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
    } else {
        Paragraph::new("Enter the model name to pull from Ollama library:")
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center)
    };
    f.render_widget(instructions, chunks[0]);

    // Input field
    if !matches!(app.input_mode, InputMode::Pulling(_)) {
        let input_text = if app.pull_input.is_empty() {
            Line::from(vec![
                Span::styled("Model: ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    "_",
                    Style::default()
                        .fg(Color::Gray)
                        .add_modifier(Modifier::SLOW_BLINK),
                ),
            ])
        } else {
            Line::from(vec![
                Span::styled("Model: ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    &app.pull_input,
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "_",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::SLOW_BLINK),
                ),
            ])
        };

        let input = Paragraph::new(input_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Green)),
            )
            .alignment(Alignment::Left);
        f.render_widget(input, chunks[1]);
    } else {
        // Show pulling progress
        let pulling_text = Line::from(vec![
            Span::styled("● ", Style::default().fg(Color::Yellow)),
            Span::styled(
                "Downloading and processing model...",
                Style::default().fg(Color::Yellow),
            ),
        ]);
        let progress = Paragraph::new(pulling_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow)),
            )
            .alignment(Alignment::Center);
        f.render_widget(progress, chunks[1]);
    }

    // Error message (if any)
    if let Some(error) = &app.pull_error {
        let error_text = Paragraph::new(error.as_str())
            .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        f.render_widget(error_text, chunks[2]);
    }

    // Examples
    if !matches!(app.input_mode, InputMode::Pulling(_)) {
        let examples = vec![
            Line::from(vec![Span::styled(
                "Examples:",
                Style::default().fg(Color::Cyan),
            )]),
            Line::from(vec![
                Span::raw("  • "),
                Span::styled("llama3.2", Style::default().fg(Color::White)),
                Span::styled(" or ", Style::default().fg(Color::Gray)),
                Span::styled("llama3.2:latest", Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::raw("  • "),
                Span::styled("mistral:7b", Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::raw("  • "),
                Span::styled("codellama:13b", Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::raw("  • "),
                Span::styled("qwen2.5-coder:7b", Style::default().fg(Color::White)),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Visit https://ollama.com/library for more models",
                Style::default()
                    .fg(Color::Gray)
                    .add_modifier(Modifier::ITALIC),
            )]),
        ];

        let examples_widget = Paragraph::new(examples)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });
        f.render_widget(examples_widget, chunks[3]);
    }

    // Bottom help
    if !matches!(app.input_mode, InputMode::Pulling(_)) {
        let help = Paragraph::new(Line::from(vec![
            Span::styled("Enter", Style::default().fg(Color::Green)),
            Span::raw(": Pull │ "),
            Span::styled("Esc", Style::default().fg(Color::Red)),
            Span::raw(": Cancel"),
        ]))
        .alignment(Alignment::Center);
        f.render_widget(help, chunks[4]);
    } else {
        let help = Paragraph::new(Line::from(vec![Span::styled(
            "Please wait... This may take a few minutes.",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::ITALIC),
        )]))
        .alignment(Alignment::Center);
        f.render_widget(help, chunks[4]);
    }
}

/// Helper function to create a centered rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
