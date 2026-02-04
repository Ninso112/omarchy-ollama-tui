use anyhow::Result;
use reqwest::Client;
use std::time::Duration;
use tracing::{debug, error};

use super::models::{
    DeleteRequest, GenerateRequest, GenerateResponse, ModelListResponse, OllamaModel,
    ProcessListResponse, PullRequest, ShowRequest, ShowResponse,
};

/// Client for communicating with the Ollama API
pub struct OllamaClient {
    base_url: String,
    client: Client,
}

impl OllamaClient {
    /// Create a new Ollama client
    pub fn new(base_url: &str) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            base_url: base_url.to_string(),
            client,
        }
    }

    /// Create a client with custom timeout (for long operations like pulling)
    fn client_with_timeout(&self, timeout_secs: u64) -> Client {
        Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .expect("Failed to create HTTP client")
    }

    /// Check if Ollama server is running
    pub async fn check_health(&self) -> Result<bool> {
        let url = format!("{}/api/tags", self.base_url);

        match self.client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    /// List all available models
    pub async fn list_models(&self) -> Result<Vec<OllamaModel>> {
        let url = format!("{}/api/tags", self.base_url);
        debug!("Fetching models from: {}", url);

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Failed to list models: {} - {}", status, error_text);
            anyhow::bail!("Failed to list models: {}", status);
        }

        let model_list: ModelListResponse = response.json().await?;
        debug!("Found {} models", model_list.models.len());

        Ok(model_list.models)
    }

    /// List currently running models
    #[allow(dead_code)]
    pub async fn list_running_models(&self) -> Result<ProcessListResponse> {
        let url = format!("{}/api/ps", self.base_url);
        debug!("Fetching running models from: {}", url);

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Failed to list running models: {} - {}", status, error_text);
            anyhow::bail!("Failed to list running models: {}", status);
        }

        let process_list: ProcessListResponse = response.json().await?;
        Ok(process_list)
    }

    /// Generate text with a model (this also loads the model if not already loaded)
    pub async fn generate(&self, model: &str, prompt: &str) -> Result<GenerateResponse> {
        let url = format!("{}/api/generate", self.base_url);
        debug!("Generating with model: {}", model);

        let request = GenerateRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
            stream: Some(false),
            options: None,
        };

        let response = self.client.post(&url).json(&request).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Failed to generate: {} - {}", status, error_text);
            anyhow::bail!("Failed to generate: {}", status);
        }

        let generate_response: GenerateResponse = response.json().await?;
        Ok(generate_response)
    }

    /// Pull a model from the Ollama library
    pub async fn pull_model(&self, model_name: &str) -> Result<()> {
        let url = format!("{}/api/pull", self.base_url);
        debug!("Pulling model: {}", model_name);

        let request = PullRequest {
            name: model_name.to_string(),
            stream: Some(false),
        };

        // Use a longer timeout for pulling (30 minutes)
        let client = self.client_with_timeout(1800);
        let response = client.post(&url).json(&request).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Failed to pull model: {} - {}", status, error_text);
            anyhow::bail!("Failed to pull model: {}", status);
        }

        debug!("Successfully pulled model: {}", model_name);
        Ok(())
    }

    /// Delete a model
    #[allow(dead_code)]
    pub async fn delete_model(&self, model_name: &str) -> Result<()> {
        let url = format!("{}/api/delete", self.base_url);
        debug!("Deleting model: {}", model_name);

        let request = DeleteRequest {
            name: model_name.to_string(),
        };

        let response = self.client.delete(&url).json(&request).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Failed to delete model: {} - {}", status, error_text);
            anyhow::bail!("Failed to delete model: {}", status);
        }

        Ok(())
    }

    /// Show model information
    #[allow(dead_code)]
    pub async fn show_model(&self, model_name: &str) -> Result<ShowResponse> {
        let url = format!("{}/api/show", self.base_url);
        debug!("Showing model info: {}", model_name);

        let request = ShowRequest {
            name: model_name.to_string(),
        };

        let response = self.client.post(&url).json(&request).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Failed to show model: {} - {}", status, error_text);
            anyhow::bail!("Failed to show model: {}", status);
        }

        let show_response: ShowResponse = response.json().await?;
        Ok(show_response)
    }

    /// Unload all models from memory
    pub async fn unload_all_models(&self) -> Result<()> {
        let url = format!("{}/api/generate", self.base_url);
        debug!("Unloading all models");

        // Send an empty generate request with keep_alive = 0 to unload all models
        let request = serde_json::json!({
            "model": "",
            "keep_alive": 0
        });

        let response = self.client.post(&url).json(&request).send().await?;

        // This endpoint might return an error, but that's okay
        // The models will still be unloaded
        debug!("Unload response status: {}", response.status());

        Ok(())
    }
}
