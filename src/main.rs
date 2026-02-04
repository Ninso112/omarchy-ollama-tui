use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use tracing::info;

mod app;
mod config;
mod events;
mod gpu;
mod ollama;
mod ui;

use app::App;
use events::EventHandler;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("ollama_tui=debug,info")
        .init();

    info!("Starting Ollama TUI");

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and event handler
    let mut app = App::new().await?;
    let mut event_handler = EventHandler::new(250); // 250ms tick rate

    // Run the application
    let res = run_app(&mut terminal, &mut app, &mut event_handler).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    info!("Ollama TUI stopped");
    Ok(())
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
    event_handler: &mut EventHandler,
) -> Result<()> {
    loop {
        // Draw UI
        terminal.draw(|f| ui::draw(f, app))?;

        // Handle events
        if let Some(event) = event_handler.next().await {
            match event {
                events::Event::Tick => {
                    app.on_tick().await?;
                }
                events::Event::Key(key_event) => {
                    if app.handle_key_event(key_event).await? {
                        return Ok(());
                    }
                }
                events::Event::Mouse(_) => {}
                events::Event::Resize(_, _) => {}
            }
        }
    }
}
