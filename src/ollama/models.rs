use serde::{Deserialize, Serialize};

/// Represents a model in Ollama
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaModel {
    pub name: String,
    pub modified_at: String,
    pub size: u64,
    pub digest: String,
    #[serde(default)]
    pub details: ModelDetails,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModelDetails {
    #[serde(default)]
    pub format: String,
    #[serde(default)]
    pub family: String,
    #[serde(default)]
    pub families: Vec<String>,
    #[serde(default)]
    pub parameter_size: String,
    #[serde(default)]
    pub quantization_level: String,
}

/// Response from the /api/tags endpoint (list models)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelListResponse {
    pub models: Vec<OllamaModel>,
}

/// Request for the /api/generate endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateRequest {
    pub model: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<GenerateOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_predict: Option<i32>,
}

/// Response from the /api/generate endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateResponse {
    pub model: String,
    pub created_at: String,
    pub response: String,
    pub done: bool,
    #[serde(default)]
    pub context: Vec<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_eval_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_eval_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eval_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eval_duration: Option<u64>,
}

/// Request for the /api/pull endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct PullRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

/// Response from the /api/pull endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct PullResponse {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<u64>,
}

/// Request for the /api/delete endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct DeleteRequest {
    pub name: String,
}

/// Response from the /api/ps endpoint (running models)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ProcessListResponse {
    pub models: Vec<RunningModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct RunningModel {
    pub name: String,
    pub model: String,
    pub size: u64,
    pub digest: String,
    #[serde(default)]
    pub details: ModelDetails,
    pub expires_at: String,
    pub size_vram: u64,
}

/// Request for the /api/show endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ShowRequest {
    pub name: String,
}

/// Response from the /api/show endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ShowResponse {
    pub license: String,
    pub modelfile: String,
    pub parameters: String,
    pub template: String,
    #[serde(default)]
    pub details: ModelDetails,
}

impl OllamaModel {
    /// Format size in human-readable format
    pub fn size_human_readable(&self) -> String {
        let size = self.size as f64;
        if size < 1024.0 {
            format!("{} B", size)
        } else if size < 1024.0 * 1024.0 {
            format!("{:.2} KB", size / 1024.0)
        } else if size < 1024.0 * 1024.0 * 1024.0 {
            format!("{:.2} MB", size / (1024.0 * 1024.0))
        } else {
            format!("{:.2} GB", size / (1024.0 * 1024.0 * 1024.0))
        }
    }

    /// Get short name (without tag if present)
    #[allow(dead_code)]
    pub fn short_name(&self) -> &str {
        self.name.split(':').next().unwrap_or(&self.name)
    }

    /// Get tag (part after colon)
    #[allow(dead_code)]
    pub fn tag(&self) -> &str {
        self.name.split(':').nth(1).unwrap_or("latest")
    }
}
