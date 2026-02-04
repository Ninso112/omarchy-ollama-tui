// Library exports for Ollama TUI
// This allows the modules to be tested and potentially used as a library

pub mod app;
pub mod config;
pub mod events;
pub mod gpu;
pub mod ollama;
pub mod ui;

// Re-export commonly used types
pub use app::App;
pub use config::Config;
pub use gpu::GpuMonitor;
pub use ollama::{OllamaClient, OllamaModel, OllamaProcess};
