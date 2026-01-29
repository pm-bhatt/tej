use std::time::Duration;

#[derive(Debug, Clone)]
pub struct TestConfig {
    pub download_url: String,
    pub upload_url: String,
    pub latency_url: String,
    pub parallel_connections: usize,
    pub download_sizes: Vec<usize>,
    pub upload_size: usize,
    pub latency_samples: usize,
    pub latency_warmup: usize,
    pub timeout: Duration,
    pub packet_loss_count: usize,
    pub packet_loss_timeout: Duration,
    pub skip_download: bool,
    pub skip_upload: bool,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            download_url: "https://speed.cloudflare.com/__down".to_string(),
            upload_url: "https://speed.cloudflare.com/__up".to_string(),
            latency_url: "https://speed.cloudflare.com/__down".to_string(),
            parallel_connections: 6,
            download_sizes: vec![
                100_000,    // 100KB warmup
                1_000_000,  // 1MB
                10_000_000, // 10MB
                25_000_000, // 25MB
            ],
            upload_size: 10_000_000, // 10MB
            latency_samples: 20,
            latency_warmup: 3,
            timeout: Duration::from_secs(30),
            packet_loss_count: 20,
            packet_loss_timeout: Duration::from_secs(2),
            skip_download: false,
            skip_upload: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_has_valid_download_sizes() {
        let config = TestConfig::default();
        assert!(!config.download_sizes.is_empty());
        assert!(config.download_sizes[0] > 0);
    }

    #[test]
    fn test_default_config_warmup_less_than_samples() {
        let config = TestConfig::default();
        assert!(config.latency_warmup < config.latency_samples);
    }

    #[test]
    fn test_default_config_connections_in_range() {
        let config = TestConfig::default();
        assert!(config.parallel_connections >= 1 && config.parallel_connections <= 32);
    }
}
