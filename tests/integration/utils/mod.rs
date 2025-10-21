use std::collections::HashMap;
use std::time::{Duration, Instant};
use reqwest::Client;

pub mod assertions;
pub mod client_factory;
pub mod deprecation_tracker;
pub mod environment;

/// Configuration for integration tests
#[derive(Debug, Clone)]
pub struct TestConfig {
    pub timeout_seconds: u64,
    pub max_retries: u32,
    pub retry_delay_ms: u64,
    pub enable_deprecation_tracking: bool,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 30,
            max_retries: 3,
            retry_delay_ms: 1000,
            enable_deprecation_tracking: true,
        }
    }
}

/// Result of a single test execution
#[derive(Debug, Clone)]
pub struct TestResult {
    pub source_name: String,
    pub function_name: String,
    pub success: bool,
    pub error_message: Option<String>,
    pub article_count: usize,
    pub execution_time_ms: u128,
}

impl TestResult {
    pub fn success(function_name: &str, article_count: usize, execution_time: Duration) -> Self {
        Self {
            source_name: String::new(),
            function_name: function_name.to_string(),
            success: true,
            error_message: None,
            article_count,
            execution_time_ms: execution_time.as_millis(),
        }
    }

    pub fn failure(function_name: &str, error: String, execution_time: Duration) -> Self {
        Self {
            source_name: String::new(),
            function_name: function_name.to_string(),
            success: false,
            error_message: Some(error),
            article_count: 0,
            execution_time_ms: execution_time.as_millis(),
        }
    }
}

/// Context for test execution
pub struct TestContext {
    pub client: Client,
    pub config: IntegrationTestConfig,
    pub deprecation_tracker: deprecation_tracker::DeprecationTracker,
    pub start_time: Instant,
}

impl TestContext {
    pub fn new(client: Client, config: IntegrationTestConfig) -> Self {
        Self {
            client,
            config,
            deprecation_tracker: deprecation_tracker::DeprecationTracker::new(),
            start_time: Instant::now(),
        }
    }
}

/// Configuration for integration test execution
#[derive(Debug, Clone)]
pub struct IntegrationTestConfig {
    pub sources_to_test: Vec<String>,
    pub functions_to_skip: HashMap<String, Vec<String>>,
    pub test_timeout_seconds: u64,
    pub network_retry_attempts: u32,
    pub deprecation_tracking_enabled: bool,
    pub ci_mode: bool,
}

impl Default for IntegrationTestConfig {
    fn default() -> Self {
        Self {
            sources_to_test: vec![
                "CNBC".to_string(),
                "CNNFinance".to_string(),
                "MarketWatch".to_string(),
                "NASDAQ".to_string(),
                "SeekingAlpha".to_string(),
                "WallStreetJournal".to_string(),
                "YahooFinance".to_string(),
            ],
            functions_to_skip: HashMap::new(),
            test_timeout_seconds: 30,
            network_retry_attempts: 3,
            deprecation_tracking_enabled: true,
            ci_mode: std::env::var("CI").is_ok(),
        }
    }
}

impl IntegrationTestConfig {
    /// Create a new configuration with source filtering
    pub fn with_sources(sources: Vec<String>) -> Self {
        Self {
            sources_to_test: sources,
            ..Default::default()
        }
    }

    /// Set timeout for network operations
    pub fn with_timeout(mut self, timeout_seconds: u64) -> Self {
        self.test_timeout_seconds = timeout_seconds;
        self
    }

    /// Enable or disable deprecation tracking
    pub fn with_deprecation_tracking(mut self, enabled: bool) -> Self {
        self.deprecation_tracking_enabled = enabled;
        self
    }

    /// Skip specific functions for a source
    pub fn skip_functions(mut self, source: &str, functions: Vec<String>) -> Self {
        self.functions_to_skip.insert(source.to_string(), functions);
        self
    }
}