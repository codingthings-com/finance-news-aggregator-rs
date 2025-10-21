use std::env;
use std::collections::HashMap;

/// Environment configuration for integration tests
#[derive(Debug, Clone)]
pub struct EnvironmentConfig {
    pub test_mode: TestMode,
    pub timeout_seconds: u64,
    pub max_retries: u32,
    pub sources_filter: Option<Vec<String>>,
    pub enable_deprecation_tracking: bool,
    pub enable_performance_tracking: bool,
    pub parallel_execution: bool,
    pub verbose_output: bool,
}

/// Test execution mode based on environment
#[derive(Debug, Clone, PartialEq)]
pub enum TestMode {
    /// Local development - full testing with detailed reporting
    Local,
    /// CI environment - faster execution with essential validations
    CI,
    /// Nightly builds - comprehensive deprecation scanning
    Nightly,
}

impl EnvironmentConfig {
    /// Create environment configuration based on environment variables
    pub fn from_env() -> Self {
        let test_mode = Self::detect_test_mode();
        
        match test_mode {
            TestMode::Local => Self::local_config(),
            TestMode::CI => Self::ci_config(),
            TestMode::Nightly => Self::nightly_config(),
        }
    }

    /// Detect test mode from environment variables
    fn detect_test_mode() -> TestMode {
        if env::var("NIGHTLY_BUILD").is_ok() || env::var("INTEGRATION_NIGHTLY").is_ok() {
            TestMode::Nightly
        } else if env::var("CI").is_ok() || env::var("GITHUB_ACTIONS").is_ok() || env::var("CONTINUOUS_INTEGRATION").is_ok() {
            TestMode::CI
        } else {
            TestMode::Local
        }
    }

    /// Configuration for local development
    fn local_config() -> Self {
        Self {
            test_mode: TestMode::Local,
            timeout_seconds: Self::env_var_or_default("INTEGRATION_TIMEOUT", 45),
            max_retries: Self::env_var_or_default("INTEGRATION_RETRIES", 3),
            sources_filter: Self::parse_sources_filter(),
            enable_deprecation_tracking: Self::env_var_or_default("ENABLE_DEPRECATION_TRACKING", true),
            enable_performance_tracking: Self::env_var_or_default("ENABLE_PERFORMANCE_TRACKING", true),
            parallel_execution: Self::env_var_or_default("PARALLEL_EXECUTION", true),
            verbose_output: Self::env_var_or_default("VERBOSE_OUTPUT", true),
        }
    }

    /// Configuration for CI environment
    fn ci_config() -> Self {
        Self {
            test_mode: TestMode::CI,
            timeout_seconds: Self::env_var_or_default("INTEGRATION_TIMEOUT", 30),
            max_retries: Self::env_var_or_default("INTEGRATION_RETRIES", 2),
            sources_filter: Self::parse_sources_filter(),
            enable_deprecation_tracking: Self::env_var_or_default("ENABLE_DEPRECATION_TRACKING", false),
            enable_performance_tracking: Self::env_var_or_default("ENABLE_PERFORMANCE_TRACKING", false),
            parallel_execution: Self::env_var_or_default("PARALLEL_EXECUTION", false),
            verbose_output: Self::env_var_or_default("VERBOSE_OUTPUT", false),
        }
    }

    /// Configuration for nightly builds
    fn nightly_config() -> Self {
        Self {
            test_mode: TestMode::Nightly,
            timeout_seconds: Self::env_var_or_default("INTEGRATION_TIMEOUT", 60),
            max_retries: Self::env_var_or_default("INTEGRATION_RETRIES", 5),
            sources_filter: None, // Test all sources in nightly
            enable_deprecation_tracking: true,
            enable_performance_tracking: true,
            parallel_execution: Self::env_var_or_default("PARALLEL_EXECUTION", true),
            verbose_output: Self::env_var_or_default("VERBOSE_OUTPUT", true),
        }
    }

    /// Parse environment variable or return default value
    fn env_var_or_default<T>(var_name: &str, default: T) -> T
    where
        T: std::str::FromStr + Clone,
    {
        env::var(var_name)
            .ok()
            .and_then(|val| val.parse().ok())
            .unwrap_or(default)
    }

    /// Parse sources filter from environment variable
    fn parse_sources_filter() -> Option<Vec<String>> {
        env::var("INTEGRATION_SOURCES")
            .ok()
            .map(|sources| {
                sources
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            })
    }

    /// Check if a specific source should be tested
    pub fn should_test_source(&self, source_name: &str) -> bool {
        match &self.sources_filter {
            Some(filter) => filter.iter().any(|s| s.eq_ignore_ascii_case(source_name)),
            None => true,
        }
    }

    /// Get feature flags for conditional test execution
    pub fn get_feature_flags() -> HashMap<String, bool> {
        let mut flags = HashMap::new();
        
        // Network-dependent tests
        flags.insert(
            "network_tests".to_string(),
            !Self::env_var_or_default("SKIP_NETWORK_TESTS", false)
        );
        
        // Performance regression tests
        flags.insert(
            "performance_tests".to_string(),
            Self::env_var_or_default("ENABLE_PERFORMANCE_TESTS", false)
        );
        
        // Deprecation scanning
        flags.insert(
            "deprecation_scan".to_string(),
            Self::env_var_or_default("ENABLE_DEPRECATION_SCAN", false)
        );
        
        // Comprehensive validation
        flags.insert(
            "comprehensive_validation".to_string(),
            Self::env_var_or_default("ENABLE_COMPREHENSIVE_VALIDATION", true)
        );

        flags
    }

    /// Check if a specific feature is enabled
    pub fn is_feature_enabled(feature: &str) -> bool {
        let flags = Self::get_feature_flags();
        flags.get(feature).copied().unwrap_or(false)
    }
}

/// Macro for conditional test execution based on features
#[macro_export]
macro_rules! feature_test {
    ($feature:expr, $test_fn:expr) => {
        if crate::integration::utils::environment::EnvironmentConfig::is_feature_enabled($feature) {
            $test_fn
        } else {
            println!("Skipping test - feature '{}' disabled", $feature);
        }
    };
}

/// Macro for CI-specific test skipping
#[macro_export]
macro_rules! skip_in_ci {
    ($test_fn:expr) => {
        if std::env::var("CI").is_ok() {
            println!("Skipping test in CI environment");
        } else {
            $test_fn
        }
    };
}

/// Macro for nightly-only tests
#[macro_export]
macro_rules! nightly_only {
    ($test_fn:expr) => {
        if std::env::var("NIGHTLY_BUILD").is_ok() || std::env::var("INTEGRATION_NIGHTLY").is_ok() {
            $test_fn
        } else {
            println!("Skipping nightly-only test");
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_environment_detection() {
        // Test CI detection
        unsafe {
            env::set_var("CI", "true");
        }
        assert_eq!(EnvironmentConfig::detect_test_mode(), TestMode::CI);
        unsafe {
            env::remove_var("CI");
        }

        // Test nightly detection
        unsafe {
            env::set_var("NIGHTLY_BUILD", "true");
        }
        assert_eq!(EnvironmentConfig::detect_test_mode(), TestMode::Nightly);
        unsafe {
            env::remove_var("NIGHTLY_BUILD");
        }

        // Test local (default)
        assert_eq!(EnvironmentConfig::detect_test_mode(), TestMode::Local);
    }

    #[test]
    fn test_sources_filter() {
        unsafe {
            env::set_var("INTEGRATION_SOURCES", "CNBC,WSJ,YahooFinance");
        }
        let config = EnvironmentConfig::from_env();
        
        assert!(config.should_test_source("CNBC"));
        assert!(config.should_test_source("WSJ"));
        assert!(config.should_test_source("YahooFinance"));
        assert!(!config.should_test_source("NASDAQ"));
        
        unsafe {
            env::remove_var("INTEGRATION_SOURCES");
        }
    }

    #[test]
    fn test_feature_flags() {
        unsafe {
            env::set_var("SKIP_NETWORK_TESTS", "true");
            env::set_var("ENABLE_PERFORMANCE_TESTS", "true");
        }
        
        let flags = EnvironmentConfig::get_feature_flags();
        assert_eq!(flags.get("network_tests"), Some(&false));
        assert_eq!(flags.get("performance_tests"), Some(&true));
        
        unsafe {
            env::remove_var("SKIP_NETWORK_TESTS");
            env::remove_var("ENABLE_PERFORMANCE_TESTS");
        }
    }
}