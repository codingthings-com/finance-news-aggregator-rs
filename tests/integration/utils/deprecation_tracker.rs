use std::collections::HashMap;
use std::fmt;
use chrono::{DateTime, Utc};

/// Tracks deprecated endpoints and categorizes failures for reporting
#[derive(Debug, Clone)]
pub struct DeprecationTracker {
    failures: Vec<FailureRecord>,
    error_counts: HashMap<String, u32>,
    source_failures: HashMap<String, Vec<FailureRecord>>,
}

impl DeprecationTracker {
    pub fn new() -> Self {
        Self {
            failures: Vec::new(),
            error_counts: HashMap::new(),
            source_failures: HashMap::new(),
        }
    }

    /// Record a failure for deprecation tracking
    pub fn record_failure(&mut self, source: &str, function: &str, error: &dyn std::error::Error) {
        let error_type = Self::classify_error(error);
        let failure = FailureRecord {
            source: source.to_string(),
            function: function.to_string(),
            error_type: error_type.clone(),
            error_message: error.to_string(),
            timestamp: Utc::now(),
            url: None, // Will be set if available
        };

        // Update counts
        *self.error_counts.entry(error_type).or_insert(0) += 1;
        
        // Store by source
        self.source_failures
            .entry(source.to_string())
            .or_insert_with(Vec::new)
            .push(failure.clone());
        
        self.failures.push(failure);
    }

    /// Record a failure with URL information
    pub fn record_failure_with_url(
        &mut self,
        source: &str,
        function: &str,
        url: &str,
        error: &dyn std::error::Error,
    ) {
        let error_type = Self::classify_error(error);
        let failure = FailureRecord {
            source: source.to_string(),
            function: function.to_string(),
            error_type: error_type.clone(),
            error_message: error.to_string(),
            timestamp: Utc::now(),
            url: Some(url.to_string()),
        };

        *self.error_counts.entry(error_type).or_insert(0) += 1;
        
        self.source_failures
            .entry(source.to_string())
            .or_insert_with(Vec::new)
            .push(failure.clone());
        
        self.failures.push(failure);
    }

    /// Classify error types for deprecation analysis
    fn classify_error(error: &dyn std::error::Error) -> String {
        let error_msg = error.to_string().to_lowercase();
        
        if error_msg.contains("404") || error_msg.contains("not found") {
            "HTTP_404_NOT_FOUND".to_string()
        } else if error_msg.contains("403") || error_msg.contains("forbidden") {
            "HTTP_403_FORBIDDEN".to_string()
        } else if error_msg.contains("timeout") || error_msg.contains("timed out") {
            "NETWORK_TIMEOUT".to_string()
        } else if error_msg.contains("connection") || error_msg.contains("connect") {
            "CONNECTION_ERROR".to_string()
        } else if error_msg.contains("dns") || error_msg.contains("resolve") {
            "DNS_ERROR".to_string()
        } else if error_msg.contains("parse") || error_msg.contains("xml") || error_msg.contains("json") {
            "PARSE_ERROR".to_string()
        } else if error_msg.contains("500") || error_msg.contains("502") || error_msg.contains("503") {
            "SERVER_ERROR".to_string()
        } else if error_msg.contains("429") || error_msg.contains("rate limit") {
            "RATE_LIMITED".to_string()
        } else {
            "UNKNOWN_ERROR".to_string()
        }
    }

    /// Generate a deprecation report
    pub fn generate_report(&self) -> DeprecationReport {
        let mut deprecated_endpoints = Vec::new();
        let mut removal_candidates = Vec::new();

        // Identify deprecated endpoints (404, 403, DNS errors)
        for failure in &self.failures {
            if matches!(
                failure.error_type.as_str(),
                "HTTP_404_NOT_FOUND" | "HTTP_403_FORBIDDEN" | "DNS_ERROR"
            ) {
                deprecated_endpoints.push(DeprecatedEndpoint {
                    source: failure.source.clone(),
                    function: failure.function.clone(),
                    url: failure.url.clone().unwrap_or_default(),
                    error_type: failure.error_type.clone(),
                    last_working: None, // Would need historical data
                });
            }
        }

        // Identify removal candidates (functions with consistent failures)
        let mut function_failure_counts: HashMap<String, u32> = HashMap::new();
        for failure in &self.failures {
            let key = format!("{}::{}", failure.source, failure.function);
            *function_failure_counts.entry(key).or_insert(0) += 1;
        }

        for (function_key, count) in function_failure_counts {
            if count >= 3 {
                // Functions that fail 3+ times are removal candidates
                removal_candidates.push(function_key);
            }
        }

        DeprecationReport {
            deprecated_endpoints,
            removal_candidates,
            error_summary: self.error_counts.clone(),
            total_failures: self.failures.len(),
            sources_affected: self.source_failures.keys().cloned().collect(),
        }
    }

    /// Get failures for a specific source
    pub fn get_source_failures(&self, source: &str) -> Vec<&FailureRecord> {
        self.source_failures
            .get(source)
            .map(|failures| failures.iter().collect())
            .unwrap_or_default()
    }

    /// Get error count summary
    pub fn get_error_summary(&self) -> &HashMap<String, u32> {
        &self.error_counts
    }

    /// Check if a source has critical failures (likely deprecated)
    pub fn has_critical_failures(&self, source: &str) -> bool {
        if let Some(failures) = self.source_failures.get(source) {
            failures.iter().any(|f| {
                matches!(
                    f.error_type.as_str(),
                    "HTTP_404_NOT_FOUND" | "HTTP_403_FORBIDDEN" | "DNS_ERROR"
                )
            })
        } else {
            false
        }
    }
}

impl Default for DeprecationTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Record of a single failure for deprecation tracking
#[derive(Debug, Clone)]
pub struct FailureRecord {
    pub source: String,
    pub function: String,
    pub error_type: String,
    pub error_message: String,
    pub timestamp: DateTime<Utc>,
    pub url: Option<String>,
}

/// Deprecated endpoint information
#[derive(Debug, Clone)]
pub struct DeprecatedEndpoint {
    pub source: String,
    pub function: String,
    pub url: String,
    pub error_type: String,
    pub last_working: Option<String>,
}

/// Complete deprecation report
#[derive(Debug, Clone)]
pub struct DeprecationReport {
    pub deprecated_endpoints: Vec<DeprecatedEndpoint>,
    pub removal_candidates: Vec<String>,
    pub error_summary: HashMap<String, u32>,
    pub total_failures: usize,
    pub sources_affected: Vec<String>,
}

impl fmt::Display for DeprecationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== DEPRECATION REPORT ===")?;
        writeln!(f, "Total Failures: {}", self.total_failures)?;
        writeln!(f, "Sources Affected: {}", self.sources_affected.len())?;
        writeln!(f)?;

        writeln!(f, "Error Summary:")?;
        for (error_type, count) in &self.error_summary {
            writeln!(f, "  {}: {}", error_type, count)?;
        }
        writeln!(f)?;

        if !self.deprecated_endpoints.is_empty() {
            writeln!(f, "Deprecated Endpoints ({}):", self.deprecated_endpoints.len())?;
            for endpoint in &self.deprecated_endpoints {
                writeln!(
                    f,
                    "  {}::{} - {} ({})",
                    endpoint.source, endpoint.function, endpoint.error_type, endpoint.url
                )?;
            }
            writeln!(f)?;
        }

        if !self.removal_candidates.is_empty() {
            writeln!(f, "Removal Candidates ({}):", self.removal_candidates.len())?;
            for candidate in &self.removal_candidates {
                writeln!(f, "  {}", candidate)?;
            }
            writeln!(f)?;
        }

        writeln!(f, "=== END REPORT ===")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::fmt;

    #[derive(Debug)]
    struct TestError {
        message: String,
    }

    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.message)
        }
    }

    impl Error for TestError {}

    #[test]
    fn test_error_classification() {
        let error_404 = TestError {
            message: "HTTP 404 Not Found".to_string(),
        };
        assert_eq!(
            DeprecationTracker::classify_error(&error_404),
            "HTTP_404_NOT_FOUND"
        );

        let timeout_error = TestError {
            message: "Request timed out".to_string(),
        };
        assert_eq!(
            DeprecationTracker::classify_error(&timeout_error),
            "NETWORK_TIMEOUT"
        );
    }

    #[test]
    fn test_failure_recording() {
        let mut tracker = DeprecationTracker::new();
        let error = TestError {
            message: "HTTP 404 Not Found".to_string(),
        };

        tracker.record_failure("TestSource", "test_function", &error);

        assert_eq!(tracker.failures.len(), 1);
        assert_eq!(tracker.error_counts.get("HTTP_404_NOT_FOUND"), Some(&1));
    }

    #[test]
    fn test_deprecation_report_generation() {
        let mut tracker = DeprecationTracker::new();
        
        // Add multiple failures for the same function
        for _ in 0..3 {
            let error = TestError {
                message: "HTTP 404 Not Found".to_string(),
            };
            tracker.record_failure("TestSource", "deprecated_function", &error);
        }

        let report = tracker.generate_report();
        assert_eq!(report.deprecated_endpoints.len(), 3);
        assert!(report.removal_candidates.contains(&"TestSource::deprecated_function".to_string()));
    }
}