use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

use crate::app::App;

/// Draw the GPU statistics panel
pub fn draw_gpu_stats(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .title(" GPU Statistics ");

    // If no GPU data available
    if app.gpu_memory_total == 0 && app.gpu_name == "Unknown" {
        let paragraph = ratatui::widgets::Paragraph::new(
            "No GPU detected or NVML not available.\n\nTo enable GPU monitoring:\n- Install NVIDIA drivers\n- Compile with --features nvidia",
        )
        .block(block)
        .style(Style::default().fg(Color::Gray));

        f.render_widget(paragraph, area);
        return;
    }

    // Create rows for the table
    let mut rows = Vec::new();

    // GPU Name
    rows.push(Row::new(vec![
        Cell::from(Span::styled(
            "GPU:",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::raw(&app.gpu_name)),
    ]));

    // GPU Utilization
    let util_color = if app.gpu_utilization > 80 {
        Color::Red
    } else if app.gpu_utilization > 50 {
        Color::Yellow
    } else {
        Color::Green
    };

    rows.push(Row::new(vec![
        Cell::from(Span::styled(
            "Utilization:",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            format!("{}%", app.gpu_utilization),
            Style::default().fg(util_color),
        )),
    ]));

    // Create a simple progress bar for utilization
    let bar_width = 30;
    let filled = (app.gpu_utilization as usize * bar_width) / 100;
    let empty = bar_width - filled;
    let progress_bar = format!("[{}{}]", "█".repeat(filled), "░".repeat(empty));

    rows.push(Row::new(vec![
        Cell::from(Span::raw("")),
        Cell::from(Span::styled(progress_bar, Style::default().fg(util_color))),
    ]));

    // VRAM Usage
    if app.gpu_memory_total > 0 {
        let memory_used_gb = app.gpu_memory_used as f64 / (1024.0 * 1024.0 * 1024.0);
        let memory_total_gb = app.gpu_memory_total as f64 / (1024.0 * 1024.0 * 1024.0);
        let memory_percent =
            (app.gpu_memory_used as f64 / app.gpu_memory_total as f64 * 100.0) as u32;

        let mem_color = if memory_percent > 80 {
            Color::Red
        } else if memory_percent > 50 {
            Color::Yellow
        } else {
            Color::Green
        };

        rows.push(Row::new(vec![
            Cell::from(Span::styled(
                "VRAM:",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                format!("{:.2} / {:.2} GB", memory_used_gb, memory_total_gb),
                Style::default().fg(mem_color),
            )),
        ]));

        // VRAM progress bar
        let mem_filled = (memory_percent as usize * bar_width) / 100;
        let mem_empty = bar_width - mem_filled;
        let mem_progress_bar = format!("[{}{}]", "█".repeat(mem_filled), "░".repeat(mem_empty));

        rows.push(Row::new(vec![
            Cell::from(Span::raw("")),
            Cell::from(Span::styled(
                mem_progress_bar,
                Style::default().fg(mem_color),
            )),
        ]));
    }

    // Temperature
    if app.gpu_temperature > 0 {
        let temp_color = if app.gpu_temperature > 80 {
            Color::Red
        } else if app.gpu_temperature > 70 {
            Color::Yellow
        } else {
            Color::Green
        };

        rows.push(Row::new(vec![
            Cell::from(Span::styled(
                "Temperature:",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                format!("{}°C", app.gpu_temperature),
                Style::default().fg(temp_color),
            )),
        ]));
    }

    // Ollama Status
    let ollama_status = if app.ollama_running {
        ("Running", Color::Green)
    } else {
        ("Stopped", Color::Red)
    };

    rows.push(Row::new(vec![
        Cell::from(Span::styled(
            "Ollama:",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            ollama_status.0,
            Style::default()
                .fg(ollama_status.1)
                .add_modifier(Modifier::BOLD),
        )),
    ]));

    let table = Table::new(rows, [Constraint::Length(15), Constraint::Min(20)])
        .block(block)
        .column_spacing(2);

    f.render_widget(table, area);
}
