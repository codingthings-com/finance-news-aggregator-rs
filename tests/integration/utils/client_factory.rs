use fake_user_agent::get_safari_rua;
use reqwest::{Client, ClientBuilder};
use std::time::Duration;

/// Factory for creating standardized HTTP clients for integration testing
pub struct ClientFactory;

impl ClientFactory {
    /// Create a new HTTP client with standard configuration for integration testing
    pub fn create_test_client() -> Result<Client, reqwest::Error> {
        Self::create_client_with_timeout(Duration::from_secs(30))
    }

    /// Create an HTTP client with custom timeout
    pub fn create_client_with_timeout(timeout: Duration) -> Result<Client, reqwest::Error> {
        let user_agent = get_safari_rua();

        ClientBuilder::new()
            .timeout(timeout)
            .connect_timeout(Duration::from_secs(10))
            .pool_idle_timeout(Duration::from_secs(90))
            .pool_max_idle_per_host(10)
            .user_agent(user_agent)
            .build()
    }

    /// Create a client with retry-friendly configuration
    pub fn create_retry_client() -> Result<Client, reqwest::Error> {
        let user_agent = get_safari_rua();

        ClientBuilder::new()
            .timeout(Duration::from_secs(45))
            .connect_timeout(Duration::from_secs(15))
            .pool_idle_timeout(Duration::from_secs(120))
            .pool_max_idle_per_host(5)
            .user_agent(user_agent)
            .tcp_keepalive(Duration::from_secs(60))
            .build()
    }

    /// Get a rotated user agent string for avoiding rate limits
    pub fn get_rotated_user_agent() -> String {
        get_safari_rua().to_string()
    }
}

/// Retry configuration for network operations
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub base_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay_ms: 1000,
            max_delay_ms: 10000,
            backoff_multiplier: 2.0,
        }
    }
}

impl RetryConfig {
    /// Calculate delay for a given attempt number (0-based)
    pub fn calculate_delay(&self, attempt: u32) -> Duration {
        let delay_ms =
            (self.base_delay_ms as f64 * self.backoff_multiplier.powi(attempt as i32)) as u64;
        let capped_delay = delay_ms.min(self.max_delay_ms);
        Duration::from_millis(capped_delay)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_config_delay_calculation() {
        let config = RetryConfig::default();

        assert_eq!(config.calculate_delay(0), Duration::from_millis(1000));
        assert_eq!(config.calculate_delay(1), Duration::from_millis(2000));
        assert_eq!(config.calculate_delay(2), Duration::from_millis(4000));
        assert_eq!(config.calculate_delay(3), Duration::from_millis(8000));
        assert_eq!(config.calculate_delay(4), Duration::from_millis(10000)); // Capped at max
    }

    #[tokio::test]
    async fn test_client_creation() {
        let client = ClientFactory::create_test_client();
        assert!(client.is_ok());
    }
}
