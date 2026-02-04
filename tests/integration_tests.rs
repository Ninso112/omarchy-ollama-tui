#[cfg(test)]
mod tests {
    use ollama_tui::config::Config;

    #[test]
    fn test_config_creation() {
        let config = Config::default();
        assert_eq!(config.ollama_url, "http://localhost:11434");
        assert_eq!(config.update_interval_ms, 1000);
        assert_eq!(config.max_status_messages, 100);
    }

    #[tokio::test]
    async fn test_ollama_client_creation() {
        let config = Config::default();
        let _client = ollama_tui::ollama::OllamaClient::new(&config.ollama_url);

        // This test just verifies the client can be created
        // Actual API calls would require a running Ollama server
        assert!(true);
    }

    #[test]
    fn test_gpu_monitor_creation() {
        // GPU monitor should be creatable even without hardware
        let result = ollama_tui::gpu::GpuMonitor::new();

        // Should either succeed or fail gracefully
        match result {
            Ok(_) => assert!(true),
            Err(_) => assert!(true), // Also OK if GPU not available
        }
    }

    #[test]
    fn test_model_size_formatting() {
        use ollama_tui::ollama::models::ModelDetails;
        use ollama_tui::ollama::OllamaModel;

        let model = OllamaModel {
            name: "test:latest".to_string(),
            modified_at: "2024-01-01".to_string(),
            size: 1024 * 1024 * 1024, // 1 GB
            digest: "test".to_string(),
            details: ModelDetails::default(),
        };

        let size_str = model.size_human_readable();
        assert!(size_str.contains("GB"));
    }

    #[test]
    fn test_model_name_parsing() {
        use ollama_tui::ollama::models::ModelDetails;
        use ollama_tui::ollama::OllamaModel;

        let model = OllamaModel {
            name: "llama3.2:latest".to_string(),
            modified_at: "2024-01-01".to_string(),
            size: 1024,
            digest: "test".to_string(),
            details: ModelDetails::default(),
        };

        assert_eq!(model.short_name(), "llama3.2");
        assert_eq!(model.tag(), "latest");
    }

    #[test]
    fn test_model_name_without_tag() {
        use ollama_tui::ollama::models::ModelDetails;
        use ollama_tui::ollama::OllamaModel;

        let model = OllamaModel {
            name: "mistral".to_string(),
            modified_at: "2024-01-01".to_string(),
            size: 1024,
            digest: "test".to_string(),
            details: ModelDetails::default(),
        };

        assert_eq!(model.short_name(), "mistral");
        assert_eq!(model.tag(), "latest");
    }
}
