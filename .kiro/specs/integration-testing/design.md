# Integration Testing Design Document

## Overview

This design implements a comprehensive integration testing suite for all news source modules in the finance-news-aggregator-rs library. The testing framework will validate real-world functionality of each news source, identify deprecated endpoints, and ensure data integrity across all public APIs.

The integration tests will be organized as separate test modules that mirror the source structure, with each test module focusing on a specific news source (CNBC, CNN Finance, Market Watch, NASDAQ, Seeking Alpha, WSJ, Yahoo Finance). Tests will make real network requests to validate actual functionality and data quality.

## Architecture

### Test Organization Structure

```
tests/
├── integration/
│   ├── mod.rs                    # Common test utilities and setup
│   ├── test_cnbc.rs             # CNBC integration tests
│   ├── test_cnn_finance.rs      # CNN Finance integration tests  
│   ├── test_market_watch.rs     # Market Watch integration tests
│   ├── test_nasdaq.rs           # NASDAQ integration tests
│   ├── test_seeking_alpha.rs    # Seeking Alpha integration tests
│   ├── test_wsj.rs              # Wall Street Journal integration tests
│   ├── test_yahoo_finance.rs    # Yahoo Finance integration tests
│   └── utils/
│       ├── mod.rs               # Test utility functions
│       ├── assertions.rs        # Custom assertion helpers
│       ├── client_factory.rs    # HTTP client creation
│       └── deprecation_tracker.rs # Endpoint deprecation tracking
```

### Test Execution Flow

1. **Setup Phase**: Initialize HTTP client with appropriate timeouts and retry logic
2. **Source Testing Phase**: Execute all public function tests for each news source
3. **Validation Phase**: Verify data structure integrity and content quality
4. **Deprecation Detection Phase**: Track and report failed endpoints
5. **Reporting Phase**: Generate summary of test results and deprecation candidates

## Components and Interfaces

### Core Test Utilities (`tests/integration/utils/mod.rs`)

```rust
pub struct TestConfig {
    pub timeout_seconds: u64,
    pub max_retries: u32,
    pub retry_delay_ms: u64,
    pub enable_deprecation_tracking: bool,
}

pub struct TestResult {
    pub source_name: String,
    pub function_name: String,
    pub success: bool,
    pub error_message: Option<String>,
    pub article_count: usize,
    pub execution_time_ms: u128,
}

pub struct DeprecationReport {
    pub deprecated_endpoints: Vec<DeprecatedEndpoint>,
    pub removal_candidates: Vec<String>,
}

pub struct DeprecatedEndpoint {
    pub source: String,
    pub function: String,
    pub url: String,
    pub error_type: String,
    pub last_working: Option<String>,
}
```

### HTTP Client Factory (`tests/integration/utils/client_factory.rs`)

Provides standardized HTTP client creation with:
- Consistent timeout configuration (30 seconds default)
- Retry logic for transient failures
- User agent rotation to avoid rate limiting
- Connection pooling for efficient testing

### Custom Assertions (`tests/integration/utils/assertions.rs`)

Specialized assertion functions for:
- `assert_valid_news_article()` - Validates NewsArticle structure
- `assert_valid_url()` - Validates URL format and accessibility
- `assert_non_empty_collection()` - Ensures collections contain data
- `assert_execution_time()` - Validates reasonable response times

### Deprecation Tracker (`tests/integration/utils/deprecation_tracker.rs`)

Tracks and categorizes endpoint failures:
- HTTP 404/403 errors → Likely deprecated
- Timeout errors → Possible network issues
- Parse errors → API format changes
- Consistent failures → Removal candidates

## Data Models

### Test Configuration

```rust
#[derive(Debug, Clone)]
pub struct IntegrationTestConfig {
    pub sources_to_test: Vec<String>,
    pub functions_to_skip: HashMap<String, Vec<String>>,
    pub test_timeout_seconds: u64,
    pub network_retry_attempts: u32,
    pub deprecation_tracking_enabled: bool,
    pub ci_mode: bool,
}
```

### Test Execution Context

```rust
pub struct TestContext {
    pub client: Client,
    pub config: IntegrationTestConfig,
    pub deprecation_tracker: DeprecationTracker,
    pub start_time: Instant,
}
```

### Article Validation Rules

```rust
pub struct ArticleValidationRules {
    pub require_title: bool,
    pub require_link: bool,
    pub require_description: bool,
    pub validate_url_format: bool,
    pub validate_date_format: bool,
    pub minimum_article_count: usize,
}
```

## Error Handling

### Error Classification System

1. **Network Errors**
   - Timeout errors → Retry with exponential backoff
   - Connection errors → Mark as temporary failure
   - DNS errors → Mark as potential deprecation

2. **HTTP Errors**
   - 404 Not Found → Mark as deprecated endpoint
   - 403 Forbidden → Mark as access restriction
   - 429 Rate Limited → Implement retry with delay
   - 5xx Server Errors → Mark as temporary failure

3. **Data Parsing Errors**
   - XML/RSS parsing failures → Mark as format change
   - Empty response → Mark as potential deprecation
   - Invalid data structure → Mark as API change

### Error Recovery Strategies

- **Transient Failures**: Retry up to 3 times with exponential backoff
- **Rate Limiting**: Implement progressive delay (1s, 2s, 4s)
- **Permanent Failures**: Log for deprecation tracking, continue testing
- **Critical Failures**: Fail fast for infrastructure issues

## Testing Strategy

### Test Categories

#### 1. Smoke Tests
- Verify each news source can be instantiated
- Test basic connectivity to each source
- Validate that `name()` and `base_url()` return expected values

#### 2. Functional Tests
- Test all public methods for each news source
- Validate return types and data structures
- Test with various topic parameters where applicable

#### 3. Data Quality Tests
- Verify NewsArticle objects contain valid data
- Check URL accessibility and format
- Validate date formats and content structure

#### 4. Performance Tests
- Measure response times for each endpoint
- Identify slow or unresponsive endpoints
- Track performance degradation over time

#### 5. Deprecation Detection Tests
- Systematically test all endpoints
- Categorize failures by error type
- Generate removal recommendations

### Test Execution Patterns

#### Per-Source Test Structure
```rust
#[tokio::test]
async fn test_cnbc_all_functions() {
    let context = setup_test_context().await;
    let cnbc = CNBC::new(context.client.clone());
    
    // Test basic functionality
    test_source_basics(&cnbc, &context).await;
    
    // Test all topic-based functions
    test_topic_functions(&cnbc, &context).await;
    
    // Test specialized functions
    test_specialized_functions(&cnbc, &context).await;
    
    // Generate deprecation report
    generate_source_report("CNBC", &context).await;
}
```

#### Function Testing Template
```rust
async fn test_function_with_validation<F, Fut>(
    function_name: &str,
    test_fn: F,
    context: &TestContext,
) -> TestResult
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<Vec<NewsArticle>>>,
{
    let start_time = Instant::now();
    
    match test_fn().await {
        Ok(articles) => {
            validate_articles(&articles, context);
            TestResult::success(function_name, articles.len(), start_time.elapsed())
        }
        Err(e) => {
            context.deprecation_tracker.record_failure(function_name, &e);
            TestResult::failure(function_name, e.to_string(), start_time.elapsed())
        }
    }
}
```

### CI/CD Integration

#### Environment-Based Configuration
- **Local Development**: Full testing with detailed reporting
- **CI Environment**: Faster execution with essential validations
- **Nightly Builds**: Comprehensive deprecation scanning

#### Test Execution Controls
```rust
// Skip tests in CI if network is unreliable
#[cfg_attr(feature = "ci-skip-network", ignore)]
#[tokio::test]
async fn test_network_dependent_function() { ... }

// Run only in nightly builds for deprecation detection
#[cfg(feature = "nightly-deprecation-scan")]
#[tokio::test]
async fn comprehensive_deprecation_scan() { ... }
```

## Implementation Phases

### Phase 1: Core Infrastructure
- Set up test directory structure
- Implement test utilities and client factory
- Create basic test execution framework
- Implement custom assertions

### Phase 2: Basic Source Testing
- Implement smoke tests for all sources
- Test basic functionality (name, base_url, available_topics)
- Validate client instantiation and configuration

### Phase 3: Comprehensive Function Testing
- Test all public methods for each source
- Implement data validation and quality checks
- Add performance monitoring and timing

### Phase 4: Deprecation Detection
- Implement deprecation tracking system
- Add error classification and reporting
- Create removal recommendation engine

### Phase 5: CI/CD Integration
- Add environment-based test configuration
- Implement test result reporting
- Add performance regression detection