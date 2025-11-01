# Integration Test Suite

This directory contains integration tests for the finance-news-aggregator-rs library. The tests validate all news source modules with real network requests to ensure RSS feeds are accessible.

## Quick Start

### Run All Integration Tests
```bash
cargo test --tests
```

### Test Specific Sources
```bash
cargo test --test test_nasdaq_integration
cargo test --test test_cnbc_integration
cargo test --test test_wsj_integration
cargo test --test test_yahoo_finance_integration
cargo test --test test_seeking_alpha_integration
cargo test --test test_market_watch_integration
```

## Test Philosophy

The integration tests focus on **feed accessibility** rather than content validation:

✅ **What we test:**
- Can we connect to the RSS feed?
- Does the feed return articles?
- Are articles properly tagged with the source name?
- Do all topic methods work without errors?

❌ **What we don't test:**
- Article content quality or completeness
- Specific field validation (titles, descriptions, etc.)
- Date format validation
- URL structure validation

This approach keeps tests fast, reliable, and focused on the main concern: **are the feeds accessible?**

## Test Structure

```
tests/
├── README.md                           # This file
├── test_nasdaq_integration.rs          # NASDAQ tests
├── test_cnbc_integration.rs            # CNBC tests
├── test_wsj_integration.rs             # Wall Street Journal tests
├── test_yahoo_finance_integration.rs   # Yahoo Finance tests
├── test_seeking_alpha_integration.rs   # Seeking Alpha tests
├── test_market_watch_integration.rs    # MarketWatch tests
└── integration/                        # Test utilities
    ├── mod.rs
    ├── test_runner.rs                  # Comprehensive test runner
    └── utils/
        ├── mod.rs
        ├── client_factory.rs           # HTTP client setup
        ├── environment.rs              # Environment configuration
        ├── assertions.rs               # Custom assertions
        └── deprecation_tracker.rs      # Endpoint monitoring
```

## Individual Source Tests

Each source has its own test file with a consistent structure:

### Basic Functionality Test
Validates that the source is properly configured:
```rust
#[tokio::test]
async fn test_source_basic_functionality() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let source = Source::new(client);

    assert_eq!(source.name(), "Source Name");
    
    let topics = source.available_topics();
    assert!(!topics.is_empty());
}
```

### Individual Topic Tests
Tests each major topic/method:
```rust
#[tokio::test]
async fn test_source_topic() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let source = Source::new(client);

    match source.topic().await {
        Ok(articles) => {
            println!("✓ topic returned {} articles", articles.len());
            for article in &articles {
                assert_eq!(article.source, Some("Source Name".to_string()));
            }
        }
        Err(e) => println!("✗ topic failed: {}", e),
    }
}
```

### Comprehensive Topic Test
Tests all available topics:
```rust
#[tokio::test]
async fn test_source_all_topics() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let source = Source::new(client);

    let topics = source.available_topics();
    let mut successful = 0;

    for &topic in &topics {
        match source.fetch_topic(topic).await {
            Ok(articles) => {
                successful += 1;
                println!("✓ {} returned {} articles", topic, articles.len());
            }
            Err(e) => {
                println!("✗ {} failed: {}", topic, e);
            }
        }
    }

    println!("\nSource Summary: {}/{} topics accessible", successful, topics.len());
    assert!(successful > 0, "At least one feed should be accessible");
}
```

## Test Results

All news sources are tested equally. Feed accessibility varies by source and can change over time due to:
- RSS feed URL changes
- Temporary network issues
- XML formatting issues in the feeds
- Rate limiting

Run the tests to see current accessibility status:
```bash
cargo test --tests -- --nocapture
```

## Running Tests

### Run All Tests
```bash
cargo test --tests
```

### Run Specific Source
```bash
cargo test --test test_nasdaq_integration
```

### Run with Output
```bash
cargo test --test test_nasdaq_integration -- --nocapture
```

### Run Specific Test
```bash
cargo test --test test_nasdaq_integration test_nasdaq_technology
```

## Test Utilities

### Client Factory
Creates HTTP clients with appropriate timeouts and retry logic:
```rust
use integration::utils::client_factory::ClientFactory;

let client = ClientFactory::create_test_client()
    .expect("Failed to create test client");
```

### Environment Configuration
Adapts test behavior based on environment:
```rust
use integration::utils::environment::EnvironmentConfig;

let config = EnvironmentConfig::from_env();
// Automatically detects CI, local, or nightly mode
```

### Deprecation Tracker
Monitors endpoint failures for deprecation analysis:
```rust
use integration::utils::deprecation_tracker::DeprecationTracker;

let mut tracker = DeprecationTracker::new();
tracker.record_failure("Source", "function", &error);
let report = tracker.generate_report();
```

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `INTEGRATION_TIMEOUT` | Network timeout in seconds | `30` |
| `INTEGRATION_RETRIES` | Max retry attempts | `3` |
| `SKIP_NETWORK_TESTS` | Skip network connectivity tests | `false` |

### Examples

```bash
# Run with longer timeout
INTEGRATION_TIMEOUT=60 cargo test --tests

# Run with more retries
INTEGRATION_RETRIES=5 cargo test --tests

# Skip network tests
SKIP_NETWORK_TESTS=1 cargo test --tests
```

## Adding New Tests

When adding tests for a new source:

1. **Create a new test file** following the naming convention:
   ```
   tests/test_newsource_integration.rs
   ```

2. **Import required modules:**
   ```rust
   use finance_news_aggregator_rs::news_source::NewsSource;
   use finance_news_aggregator_rs::news_source::newsource::NewSource;
   use tokio;

   mod integration;
   use integration::utils::client_factory::ClientFactory;
   ```

3. **Add basic functionality test:**
   ```rust
   #[tokio::test]
   async fn test_newsource_basic_functionality() {
       let client = ClientFactory::create_test_client()
           .expect("Failed to create test client");
       let source = NewSource::new(client);

       assert_eq!(source.name(), "New Source");
       
       let topics = source.available_topics();
       assert!(!topics.is_empty());
   }
   ```

4. **Add individual topic tests** for each major method

5. **Add comprehensive test** that validates all topics

6. **Update this README** with the new source information

## Troubleshooting

### Network Timeouts
If tests are timing out:
```bash
INTEGRATION_TIMEOUT=60 cargo test --tests
```

### Rate Limiting or Timeouts
If you're experiencing issues, try:
```bash
# Run tests sequentially
cargo test --tests -- --test-threads=1

# Increase timeout
INTEGRATION_TIMEOUT=60 cargo test --tests
```

### Debugging Failures
Run with output to see detailed error messages:
```bash
cargo test --test test_source_integration -- --nocapture
```

### Test Failures
Tests may fail due to:
- Network connectivity issues
- Temporary RSS feed unavailability
- XML parsing errors in the feed content
- Rate limiting from the news source

This is normal for integration tests that depend on external services. Retry the tests if failures occur.

## Contributing

When contributing tests:

1. **Keep tests simple** - Focus on feed accessibility
2. **Don't validate content** - We only check if feeds work
3. **Handle failures gracefully** - Print errors but don't panic
4. **Use consistent patterns** - Follow existing test structure
5. **Update documentation** - Keep this README current

## Test Maintenance

### Regular Checks
- Monitor feed accessibility rates
- Update tests when APIs change
- Remove tests for deprecated endpoints
- Add tests for new features

### Deprecation Management
When feeds consistently fail:
1. Check if the RSS feed URL has changed
2. Verify the feed still exists
3. Update the source implementation if needed
4. Remove the test if the feed is permanently gone

### Performance
Tests should complete in under 30 seconds total. If tests are slow:
- Check network connectivity
- Verify timeout settings
- Consider reducing the number of topics tested
