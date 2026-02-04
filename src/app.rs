use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::time::{Duration, Instant};
use tracing::{debug, error, info};

use crate::config::Config;
use crate::gpu::GpuMonitor;
use crate::ollama::{OllamaClient, OllamaModel, OllamaProcess};

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    Normal,
    #[allow(dead_code)]
    Loading,
    #[allow(dead_code)]
    Error(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    PullDialog,
    Pulling(String), // Model name being pulled
}

pub struct App {
    #[allow(dead_code)]
    pub config: Config,
    #[allow(dead_code)]
    pub state: AppState,
    pub input_mode: InputMode,

    // Ollama
    pub ollama_client: OllamaClient,
    pub ollama_process: OllamaProcess,
    pub models: Vec<OllamaModel>,
    pub selected_model_index: usize,
    pub ollama_running: bool,

    // GPU
    pub gpu_monitor: GpuMonitor,
    pub gpu_utilization: u32,
    pub gpu_memory_used: u64,
    pub gpu_memory_total: u64,
    pub gpu_temperature: u32,
    pub gpu_name: String,

    // Status log
    pub status_messages: Vec<StatusMessage>,
    pub max_status_messages: usize,

    // Pull dialog
    pub pull_input: String,
    pub pull_error: Option<String>,

    // Update timing
    last_update: Instant,
    update_interval: Duration,
}

#[derive(Debug, Clone)]
pub struct StatusMessage {
    #[allow(dead_code)]
    pub timestamp: Instant,
    pub level: LogLevel,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    Info,
    #[allow(dead_code)]
    Warning,
    Error,
}

impl App {
    pub async fn new() -> Result<Self> {
        let config = Config::load()?;
        let ollama_client = OllamaClient::new(&config.ollama_url);
        let ollama_process = OllamaProcess::new();
        let gpu_monitor = GpuMonitor::new()?;

        let mut app = Self {
            config,
            state: AppState::Normal,
            input_mode: InputMode::Normal,
            ollama_client,
            ollama_process,
            models: Vec::new(),
            selected_model_index: 0,
            ollama_running: false,
            gpu_monitor,
            gpu_utilization: 0,
            gpu_memory_used: 0,
            gpu_memory_total: 0,
            gpu_temperature: 0,
            gpu_name: String::from("Unknown"),
            status_messages: Vec::new(),
            max_status_messages: 100,
            pull_input: String::new(),
            pull_error: None,
            last_update: Instant::now(),
            update_interval: Duration::from_millis(1000),
        };

        app.add_status_message(LogLevel::Info, "Application started".to_string());

        // Initial checks
        app.check_ollama_status().await;
        app.refresh_models().await?;
        app.update_gpu_stats().await?;

        Ok(app)
    }

    pub async fn on_tick(&mut self) -> Result<()> {
        let now = Instant::now();
        if now.duration_since(self.last_update) >= self.update_interval {
            self.update_gpu_stats().await?;
            self.check_ollama_status().await;
            self.last_update = now;
        }
        Ok(())
    }

    pub async fn handle_key_event(&mut self, key: KeyEvent) -> Result<bool> {
        match self.input_mode {
            InputMode::Normal => self.handle_normal_mode_key(key).await,
            InputMode::PullDialog => self.handle_pull_dialog_key(key).await,
            InputMode::Pulling(_) => {
                // In pulling mode, just wait
                Ok(false)
            }
        }
    }

    async fn handle_normal_mode_key(&mut self, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                info!("Quit requested");
                return Ok(true);
            }
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                info!("Ctrl+C pressed, quitting");
                return Ok(true);
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.select_previous_model();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.select_next_model();
            }
            KeyCode::Enter => {
                self.load_selected_model().await?;
            }
            KeyCode::Char('r') => {
                self.refresh_models().await?;
            }
            KeyCode::Char('s') => {
                self.toggle_ollama_server().await?;
            }
            KeyCode::Char('p') => {
                self.open_pull_dialog();
            }
            KeyCode::Char('u') => {
                self.unload_all_models().await?;
            }
            _ => {}
        }
        Ok(false)
    }

    fn select_previous_model(&mut self) {
        if !self.models.is_empty() {
            self.selected_model_index = if self.selected_model_index == 0 {
                self.models.len() - 1
            } else {
                self.selected_model_index - 1
            };
        }
    }

    fn select_next_model(&mut self) {
        if !self.models.is_empty() {
            self.selected_model_index = (self.selected_model_index + 1) % self.models.len();
        }
    }

    pub async fn refresh_models(&mut self) -> Result<()> {
        self.add_status_message(LogLevel::Info, "Refreshing model list...".to_string());

        match self.ollama_client.list_models().await {
            Ok(models) => {
                self.models = models;
                if self.selected_model_index >= self.models.len() && !self.models.is_empty() {
                    self.selected_model_index = self.models.len() - 1;
                }
                self.add_status_message(
                    LogLevel::Info,
                    format!("Found {} models", self.models.len()),
                );
            }
            Err(e) => {
                error!("Failed to list models: {}", e);
                self.add_status_message(LogLevel::Error, format!("Failed to list models: {}", e));
            }
        }

        Ok(())
    }

    async fn load_selected_model(&mut self) -> Result<()> {
        if let Some(model) = self.models.get(self.selected_model_index) {
            let model_name = model.name.clone();
            self.add_status_message(LogLevel::Info, format!("Loading model: {}", model_name));

            match self.ollama_client.generate(&model_name, "").await {
                Ok(_) => {
                    self.add_status_message(
                        LogLevel::Info,
                        format!("Model {} loaded successfully", model_name),
                    );
                }
                Err(e) => {
                    error!("Failed to load model: {}", e);
                    self.add_status_message(
                        LogLevel::Error,
                        format!("Failed to load model: {}", e),
                    );
                }
            }
        }
        Ok(())
    }

    async fn unload_all_models(&mut self) -> Result<()> {
        self.add_status_message(LogLevel::Info, "Unloading all models...".to_string());

        match self.ollama_client.unload_all_models().await {
            Ok(_) => {
                self.add_status_message(
                    LogLevel::Info,
                    "All models unloaded successfully".to_string(),
                );
            }
            Err(e) => {
                error!("Failed to unload models: {}", e);
                self.add_status_message(LogLevel::Error, format!("Failed to unload models: {}", e));
            }
        }

        Ok(())
    }

    async fn toggle_ollama_server(&mut self) -> Result<()> {
        if self.ollama_running {
            self.add_status_message(LogLevel::Info, "Stopping Ollama server...".to_string());
            match self.ollama_process.stop().await {
                Ok(_) => {
                    self.ollama_running = false;
                    self.add_status_message(LogLevel::Info, "Ollama server stopped".to_string());
                }
                Err(e) => {
                    error!("Failed to stop Ollama: {}", e);
                    self.add_status_message(
                        LogLevel::Error,
                        format!("Failed to stop Ollama: {}", e),
                    );
                }
            }
        } else {
            self.add_status_message(LogLevel::Info, "Starting Ollama server...".to_string());
            match self.ollama_process.start().await {
                Ok(_) => {
                    self.ollama_running = true;
                    self.add_status_message(LogLevel::Info, "Ollama server started".to_string());
                    // Refresh models after starting
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    self.refresh_models().await?;
                }
                Err(e) => {
                    error!("Failed to start Ollama: {}", e);
                    self.add_status_message(
                        LogLevel::Error,
                        format!("Failed to start Ollama: {}", e),
                    );
                }
            }
        }
        Ok(())
    }

    async fn check_ollama_status(&mut self) {
        match self.ollama_client.check_health().await {
            Ok(running) => {
                if running != self.ollama_running {
                    self.ollama_running = running;
                    let msg = if running {
                        "Ollama server is running"
                    } else {
                        "Ollama server is not running"
                    };
                    self.add_status_message(LogLevel::Info, msg.to_string());
                }
            }
            Err(e) => {
                debug!("Failed to check Ollama status: {}", e);
                self.ollama_running = false;
            }
        }
    }

    async fn update_gpu_stats(&mut self) -> Result<()> {
        match self.gpu_monitor.get_stats().await {
            Ok(stats) => {
                self.gpu_name = stats.name;
                self.gpu_utilization = stats.utilization;
                self.gpu_memory_used = stats.memory_used;
                self.gpu_memory_total = stats.memory_total;
                self.gpu_temperature = stats.temperature;
            }
            Err(e) => {
                debug!("Failed to get GPU stats: {}", e);
            }
        }
        Ok(())
    }

    pub fn add_status_message(&mut self, level: LogLevel, message: String) {
        let status = StatusMessage {
            timestamp: Instant::now(),
            level,
            message,
        };

        self.status_messages.push(status);

        // Keep only the last N messages
        if self.status_messages.len() > self.max_status_messages {
            self.status_messages.remove(0);
        }
    }

    fn open_pull_dialog(&mut self) {
        self.input_mode = InputMode::PullDialog;
        self.pull_input.clear();
        self.pull_error = None;
        self.add_status_message(LogLevel::Info, "Opening pull model dialog...".to_string());
    }

    async fn handle_pull_dialog_key(&mut self, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
                self.pull_input.clear();
                self.pull_error = None;
                Ok(false)
            }
            KeyCode::Enter => {
                if !self.pull_input.is_empty() {
                    self.pull_model().await?;
                }
                Ok(false)
            }
            KeyCode::Char(c) => {
                self.pull_input.push(c);
                self.pull_error = None;
                Ok(false)
            }
            KeyCode::Backspace => {
                self.pull_input.pop();
                self.pull_error = None;
                Ok(false)
            }
            _ => Ok(false),
        }
    }

    async fn pull_model(&mut self) -> Result<()> {
        let model_name = self.pull_input.clone();
        self.input_mode = InputMode::Pulling(model_name.clone());
        self.add_status_message(LogLevel::Info, format!("Pulling model: {}", model_name));

        match self.ollama_client.pull_model(&model_name).await {
            Ok(_) => {
                self.add_status_message(
                    LogLevel::Info,
                    format!("Successfully pulled model: {}", model_name),
                );
                self.input_mode = InputMode::Normal;
                self.pull_input.clear();
                self.pull_error = None;
                // Refresh model list after successful pull
                tokio::time::sleep(Duration::from_secs(1)).await;
                self.refresh_models().await?;
            }
            Err(e) => {
                error!("Failed to pull model: {}", e);
                self.pull_error = Some(format!("Error: {}", e));
                self.add_status_message(
                    LogLevel::Error,
                    format!("Failed to pull model {}: {}", model_name, e),
                );
                self.input_mode = InputMode::PullDialog;
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_selected_model(&self) -> Option<&OllamaModel> {
        self.models.get(self.selected_model_index)
    }
}
