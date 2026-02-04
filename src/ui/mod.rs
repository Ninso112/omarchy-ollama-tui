pub mod gpu_stats;
pub mod layout;
pub mod model_list;
pub mod pull_dialog;
pub mod status_bar;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::app::App;

/// Main draw function for the entire UI
pub fn draw(f: &mut Frame, app: &App) {
    let size = f.size();

    // Create the main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title bar
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Status bar
        ])
        .split(size);

    // Draw title bar
    layout::draw_title_bar(f, chunks[0], app);

    // Split main content into left and right panels
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40), // Model list
            Constraint::Percentage(60), // GPU stats and logs
        ])
        .split(chunks[1]);

    // Draw model list
    model_list::draw_model_list(f, main_chunks[0], app);

    // Split right panel into GPU stats and status log
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40), // GPU stats
            Constraint::Percentage(60), // Status log
        ])
        .split(main_chunks[1]);

    // Draw GPU stats
    gpu_stats::draw_gpu_stats(f, right_chunks[0], app);

    // Draw status log
    layout::draw_status_log(f, right_chunks[1], app);

    // Draw status bar
    status_bar::draw_status_bar(f, chunks[2], app);

    // Draw pull dialog (if active)
    pull_dialog::draw_pull_dialog(f, app);
}
