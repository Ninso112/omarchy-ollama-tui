use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

/// Draw the status bar at the bottom of the screen
pub fn draw_status_bar(f: &mut Frame, area: Rect, _app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));

    // Create keybinding hints
    let keybindings = [
        ("q", "Quit"),
        ("↑/k", "Up"),
        ("↓/j", "Down"),
        ("Enter", "Load"),
        ("r", "Refresh"),
        ("s", "Start/Stop"),
        ("u", "Unload"),
        ("p", "Pull"),
    ];

    let mut spans = Vec::new();

    for (i, (key, action)) in keybindings.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled(" │ ", Style::default().fg(Color::DarkGray)));
        }

        spans.push(Span::styled(
            *key,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::raw(":"));
        spans.push(Span::styled(*action, Style::default().fg(Color::White)));
    }

    let keybinding_line = Line::from(spans);

    let paragraph = Paragraph::new(keybinding_line)
        .block(block)
        .style(Style::default());

    f.render_widget(paragraph, area);
}
