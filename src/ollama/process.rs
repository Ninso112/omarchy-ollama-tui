use anyhow::Result;
use std::process::{Child, Command, Stdio};
use tokio::sync::Mutex;
use tracing::{debug, error, info};

/// Manages the Ollama server process
pub struct OllamaProcess {
    child: Mutex<Option<Child>>,
}

impl OllamaProcess {
    /// Create a new OllamaProcess manager
    pub fn new() -> Self {
        Self {
            child: Mutex::new(None),
        }
    }

    /// Start the Ollama server process
    pub async fn start(&self) -> Result<()> {
        let mut child_lock = self.child.lock().await;

        // Check if already running
        if let Some(ref mut child) = *child_lock {
            match child.try_wait() {
                Ok(None) => {
                    // Process is still running
                    info!("Ollama process is already running");
                    return Ok(());
                }
                Ok(Some(status)) => {
                    info!("Previous Ollama process exited with status: {}", status);
                }
                Err(e) => {
                    error!("Error checking Ollama process status: {}", e);
                }
            }
        }

        // Try to start Ollama
        debug!("Starting Ollama server process");

        let child = Command::new("ollama")
            .arg("serve")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();

        match child {
            Ok(process) => {
                info!("Ollama server started with PID: {}", process.id());
                *child_lock = Some(process);
                Ok(())
            }
            Err(e) => {
                error!("Failed to start Ollama server: {}", e);
                anyhow::bail!("Failed to start Ollama: {}. Is Ollama installed?", e)
            }
        }
    }

    /// Stop the Ollama server process
    pub async fn stop(&self) -> Result<()> {
        let mut child_lock = self.child.lock().await;

        if let Some(mut child) = child_lock.take() {
            debug!("Stopping Ollama server process (PID: {})", child.id());

            // Try graceful shutdown first
            #[cfg(unix)]
            {
                use nix::sys::signal::{kill, Signal};
                use nix::unistd::Pid;

                let pid = Pid::from_raw(child.id() as i32);
                match kill(pid, Signal::SIGTERM) {
                    Ok(_) => {
                        info!("Sent SIGTERM to Ollama process");

                        // Wait up to 5 seconds for graceful shutdown
                        for _ in 0..50 {
                            match child.try_wait() {
                                Ok(Some(status)) => {
                                    info!("Ollama process exited with status: {}", status);
                                    return Ok(());
                                }
                                Ok(None) => {
                                    tokio::time::sleep(tokio::time::Duration::from_millis(100))
                                        .await;
                                }
                                Err(e) => {
                                    error!("Error waiting for Ollama process: {}", e);
                                    break;
                                }
                            }
                        }

                        // Force kill if still running
                        info!("Force killing Ollama process");
                        match kill(pid, Signal::SIGKILL) {
                            Ok(_) => {
                                let _ = child.wait();
                                info!("Ollama process killed");
                            }
                            Err(e) => {
                                error!("Failed to kill Ollama process: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to send SIGTERM to Ollama process: {}", e);
                        // Try force kill as fallback
                        let _ = child.kill();
                        let _ = child.wait();
                    }
                }
            }

            #[cfg(not(unix))]
            {
                // On non-Unix systems, just kill the process
                match child.kill() {
                    Ok(_) => {
                        let _ = child.wait();
                        info!("Ollama process killed");
                    }
                    Err(e) => {
                        error!("Failed to kill Ollama process: {}", e);
                        anyhow::bail!("Failed to kill Ollama process: {}", e);
                    }
                }
            }

            Ok(())
        } else {
            info!("No Ollama process to stop");
            Ok(())
        }
    }

    /// Check if the managed process is running
    #[allow(dead_code)]
    pub async fn is_running(&self) -> bool {
        let mut child_lock = self.child.lock().await;

        if let Some(ref mut child) = *child_lock {
            match child.try_wait() {
                Ok(None) => true, // Process is still running
                Ok(Some(_)) => {
                    // Process has exited
                    *child_lock = None;
                    false
                }
                Err(_) => false,
            }
        } else {
            false
        }
    }

    /// Get the PID of the managed process
    #[allow(dead_code)]
    pub async fn pid(&self) -> Option<u32> {
        let child_lock = self.child.lock().await;
        child_lock.as_ref().map(|c| c.id())
    }
}

impl Default for OllamaProcess {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for OllamaProcess {
    fn drop(&mut self) {
        // Note: This runs in sync context, so we can't use async stop()
        // We'll just kill the process if it exists
        if let Ok(mut child_lock) = self.child.try_lock() {
            if let Some(mut child) = child_lock.take() {
                let _ = child.kill();
                let _ = child.wait();
            }
        }
    }
}
