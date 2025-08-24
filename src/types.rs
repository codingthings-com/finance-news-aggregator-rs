use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use fake_user_agent::get_safari_rua;

/// Represents a news article from any source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsArticle {
    pub title: Option<String>,
    pub link: Option<String>,
    pub description: Option<String>,
    pub pub_date: Option<String>,
    pub guid: Option<String>,
    pub category: Option<String>,
    pub author: Option<String>,
    pub source: Option<String>,
    /// Additional fields that might be source-specific
    pub extra_fields: HashMap<String, String>,
}

impl NewsArticle {
    pub fn new() -> Self {
        Self {
            title: None,
            link: None,
            description: None,
            pub_date: None,
            guid: None,
            category: None,
            author: None,
            source: None,
            extra_fields: HashMap::new(),
        }
    }
}

impl Default for NewsArticle {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for news sources
#[derive(Debug, Clone)]
pub struct SourceConfig {
    pub base_url: String,
    pub user_agent: String,
    pub timeout_seconds: u64,
    pub max_retries: u32,
    pub retry_delay_ms: u64,
}

impl SourceConfig {
    /// Create a new SourceConfig with the given base URL
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            user_agent: get_safari_rua().to_string(),
            timeout_seconds: 30,
            max_retries: 3,
            retry_delay_ms: 1000,
        }
    }

    /// Set a custom user agent
    pub fn with_user_agent(mut self, user_agent: &str) -> Self {
        self.user_agent = user_agent.to_string();
        self
    }

    /// Set timeout in seconds
    pub fn with_timeout(mut self, timeout_seconds: u64) -> Self {
        self.timeout_seconds = timeout_seconds;
        self
    }

    /// Set retry configuration
    pub fn with_retries(mut self, max_retries: u32, retry_delay_ms: u64) -> Self {
        self.max_retries = max_retries;
        self.retry_delay_ms = retry_delay_ms;
        self
    }

    /// Get timeout as Duration
    pub fn timeout_duration(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.timeout_seconds)
    }

    /// Get retry delay as Duration
    pub fn retry_delay_duration(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.retry_delay_ms)
    }
}

impl Default for SourceConfig {
    fn default() -> Self {
        Self {
            base_url: String::new(),
            user_agent: get_safari_rua().to_string(),
            timeout_seconds: 30,
            max_retries: 3,
            retry_delay_ms: 1000,
        }
    }
}