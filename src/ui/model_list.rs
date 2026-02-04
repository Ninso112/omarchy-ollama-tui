use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::app::App;

/// Draw the model list panel
pub fn draw_model_list(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .title(" Models ");

    if app.models.is_empty() {
        let empty_message = if app.ollama_running {
            "No models found. Press 'p' to pull a model."
        } else {
            "Ollama not running. Press 's' to start."
        };

        let paragraph = ratatui::widgets::Paragraph::new(empty_message)
            .block(block)
            .style(Style::default().fg(Color::Gray));

        f.render_widget(paragraph, area);
        return;
    }

    let items: Vec<ListItem> = app
        .models
        .iter()
        .enumerate()
        .map(|(i, model)| {
            let is_selected = i == app.selected_model_index;

            // Format model name with tag
            let model_name = model.name.to_string();

            // Format size
            let size_str = model.size_human_readable();

            // Create the display line
            let content = if is_selected {
                Line::from(vec![
                    Span::styled(
                        "> ",
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("{:<30}", model_name),
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        size_str,
                        Style::default()
                            .fg(Color::Gray)
                            .add_modifier(Modifier::BOLD),
                    ),
                ])
            } else {
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled(
                        format!("{:<30}", model_name),
                        Style::default().fg(Color::White),
                    ),
                    Span::styled(size_str, Style::default().fg(Color::Gray)),
                ])
            };

            ListItem::new(content)
        })
        .collect();

    let list = List::new(items).block(block).highlight_style(
        Style::default()
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD),
    );

    f.render_widget(list, area);
}
