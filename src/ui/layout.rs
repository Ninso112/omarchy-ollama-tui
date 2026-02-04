use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, LogLevel};

/// Draw the title bar at the top of the screen
pub fn draw_title_bar(f: &mut Frame, area: Rect, app: &App) {
    let title = "Ollama TUI";

    // Format GPU info for title bar
    let gpu_info = if app.gpu_memory_total > 0 {
        let memory_gb = app.gpu_memory_used as f64 / (1024.0 * 1024.0 * 1024.0);
        format!("GPU: {}% {:.1}GB", app.gpu_utilization, memory_gb)
    } else {
        "GPU: N/A".to_string()
    };

    // Create title line with app name on left and GPU info on right
    let title_line = Line::from(vec![
        Span::styled(
            format!("  {}  ", title),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(
            " ".repeat(
                area.width
                    .saturating_sub(title.len() as u16 + gpu_info.len() as u16 + 6)
                    as usize,
            ),
        ),
        Span::styled(
            gpu_info,
            Style::default()
                .fg(if app.gpu_utilization > 80 {
                    Color::Red
                } else if app.gpu_utilization > 50 {
                    Color::Yellow
                } else {
                    Color::Green
                })
                .add_modifier(Modifier::BOLD),
        ),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(title_line).block(block);

    f.render_widget(paragraph, area);
}

/// Draw the status log panel
pub fn draw_status_log(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .title(" Status Log ");

    // Get the last messages that fit in the area
    let available_height = area.height.saturating_sub(2) as usize; // Account for borders
    let messages_to_show = app
        .status_messages
        .iter()
        .rev()
        .take(available_height)
        .rev()
        .collect::<Vec<_>>();

    let mut lines: Vec<Line> = Vec::new();

    for msg in messages_to_show {
        let (level_str, level_color) = match msg.level {
            LogLevel::Info => ("[INFO]", Color::Green),
            LogLevel::Warning => ("[WARN]", Color::Yellow),
            LogLevel::Error => ("[ERROR]", Color::Red),
        };

        let line = Line::from(vec![
            Span::styled(
                format!("{} ", level_str),
                Style::default()
                    .fg(level_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(&msg.message),
        ]);

        lines.push(line);
    }

    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}
