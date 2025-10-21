# Integration Test Suite

This directory contains comprehensive integration tests for the finance-news-aggregator-rs library. The test suite validates all news source modules with real network requests and provides detailed reporting on functionality, performance, and deprecation status.

## Quick Start

### Run All Tests
```bash
cargo test --test integration_test_suite
```

### Test Specific Sources
```bash
INTEGRATION_SOURCES=CNBC,WSJ cargo test --test integration_test_suite
```

### Run in CI Mode (Faster)
```bash
CI=1 cargo test --test integration_test_suite
```

## Test Categories

### 1. Comprehensive Integration Tests
The main test suite that validates all news sources:
```bash
cargo test --test integration_test_suite run_comprehensive_integration_tests
```

### 2. Source-Specific Tests
Test individual news sources:
```bash
# Test only CNBC
cargo test --test integration_test_suite run_cnbc_only_tests -- --ignored

# Test premium sources (WSJ, Yahoo Finance)
cargo test --test integration_test_suite run_premium_sources_tests -- --ignored
```

### 3. Performance Tests
Monitor response times and detect regressions:
```bash
cargo test --test integration_test_suite run_performance_regression_tests -- --ignored
```

### 4. Deprecation Detection
Identify outdated endpoints and broken feeds:
```bash
cargo test --test integration_test_suite run_deprecation_detection_tests -- --ignored
```

### 5. Network Connectivity
Basic connectivity validation:
```bash
cargo test --test integration_test_suite test_network_connectivity
```

## Environment Configuration

The test suite automatically adapts based on environment variables:

### Test Modes

| Environment | Description | Timeout | Retries | Features |
|-------------|-------------|---------|---------|----------|
| **Local** | Development mode | 45s | 3 | Full reporting, deprecation tracking |
| **CI** | Continuous integration | 30s | 2 | Essential validation only |
| **Nightly** | Comprehensive scanning | 60s | 5 | Full deprecation scan, performance tracking |

### Environment Variables

| Variable | Description | Default | Example |
|----------|-------------|---------|---------|
| `CI` | Enable CI mode | `false` | `CI=1` |
| `NIGHTLY_BUILD` | Enable nightly mode | `false` | `NIGHTLY_BUILD=1` |
| `INTEGRATION_SOURCES` | Sources to test | All | `CNBC,WSJ,YahooFinance` |
| `INTEGRATION_TIMEOUT` | Network timeout (seconds) | 30-60 | `INTEGRATION_TIMEOUT=45` |
| `INTEGRATION_RETRIES` | Max retry attempts | 2-5 | `INTEGRATION_RETRIES=3` |
| `SKIP_NETWORK_TESTS` | Skip connectivity tests | `false` | `SKIP_NETWORK_TESTS=1` |
| `ENABLE_DEPRECATION_TRACKING` | Track deprecated endpoints | `true` | `ENABLE_DEPRECATION_TRACKING=1` |
| `ENABLE_PERFORMANCE_TRACKING` | Monitor performance | `true` | `ENABLE_PERFORMANCE_TRACKING=1` |
| `VERBOSE_OUTPUT` | Detailed logging | `true` | `VERBOSE_OUTPUT=1` |
| `PARALLEL_EXECUTION` | Run tests in parallel | `true` | `PARALLEL_EXECUTION=0` |

## Test Structure

```
tests/
├── integration/
│   ├── mod.rs                    # Module definitions
│   ├── test_runner.rs           # Main test execution engine
│   ├── cli_runner.rs            # Command-line interface
│   └── utils/
│       ├── mod.rs               # Common utilities
│       ├── environment.rs       # Environment configuration
│       ├── assertions.rs        # Custom assertions
│       ├── client_factory.rs    # HTTP client setup
│       └── deprecation_tracker.rs # Endpoint monitoring
├── integration_test_suite.rs    # Main test entry point
└── test_*_integration.rs        # Individual source tests
```

## Usage Examples

### Development Workflow

```bash
# Quick validation during development
cargo test --test integration_test_suite

# Test changes to specific source
INTEGRATION_SOURCES=CNBC cargo test --test integration_test_suite

# Debug network issues
VERBOSE_OUTPUT=1 cargo test --test integration_test_suite test_network_connectivity
```

### CI/CD Pipeline

```bash
# Fast CI validation
CI=1 INTEGRATION_TIMEOUT=20 cargo test --test integration_test_suite

# Nightly comprehensive scan
NIGHTLY_BUILD=1 cargo test --test integration_test_suite
```

### Performance Monitoring

```bash
# Check for performance regressions
ENABLE_PERFORMANCE_TRACKING=1 cargo test --test integration_test_suite

# Run performance-specific tests
cargo test --test integration_test_suite run_performance_regression_tests -- --ignored
```

### Deprecation Management

```bash
# Scan for deprecated endpoints
ENABLE_DEPRECATION_TRACKING=1 cargo test --test integration_test_suite

# Comprehensive deprecation analysis
cargo test --test integration_test_suite run_deprecation_detection_tests -- --ignored
```

## Test Output

### Success Example
```
🚀 Starting comprehensive integration test suite
Environment: Local
Configuration: EnvironmentConfig { test_mode: Local, timeout_seconds: 45, ... }

🔄 Running tests in parallel mode
✅ Completed tests for CNBC
✅ Completed tests for CNNFinance
...

🎯 ===== INTEGRATION TEST SUMMARY =====
⏱️  Total execution time: 23.45s
📊 Total tests: 87
✅ Successful: 82 (94.3%)
❌ Failed: 5 (5.7%)
📰 Total articles fetched: 1,247

📈 === SOURCE BREAKDOWN ===
🔸 CNBC: 12/13 passed (92.3%) - 156 articles - avg 1.2s
🔸 CNNFinance: 9/9 passed (100.0%) - 203 articles - avg 0.8s
...

🎉 Overall Status: EXCELLENT (94.3% success rate)
```

### Failure Analysis
```
❌ Failed: 5 (5.7%)

📈 === SOURCE BREAKDOWN ===
🔸 MarketWatch: 8/12 passed (66.7%) - 89 articles - avg 2.1s
   Failed functions: ["newsletter_and_research", "stocks_to_watch"]

🔍 === DEPRECATION REPORT ===
Deprecated endpoints detected:
  - MarketWatch::newsletter_and_research (404 Not Found)
  - NASDAQ::original_content (403 Forbidden)

⚠️  Overall Status: NEEDS ATTENTION (66.7% success rate)
```

## Troubleshooting

### Common Issues

1. **Network Timeouts**
   ```bash
   # Increase timeout for slow connections
   INTEGRATION_TIMEOUT=60 cargo test --test integration_test_suite
   ```

2. **Rate Limiting**
   ```bash
   # Disable parallel execution
   PARALLEL_EXECUTION=0 cargo test --test integration_test_suite
   ```

3. **CI Failures**
   ```bash
   # Skip network-dependent tests
   SKIP_NETWORK_TESTS=1 cargo test --test integration_test_suite
   ```

4. **Debugging Specific Source**
   ```bash
   # Test single source with verbose output
   INTEGRATION_SOURCES=CNBC VERBOSE_OUTPUT=1 cargo test --test integration_test_suite
   ```

### Expected Failure Rates

| Environment | Expected Success Rate | Action Threshold |
|-------------|----------------------|------------------|
| Local | ≥ 60% | Investigate below 60% |
| CI | ≥ 70% | Fail build below 70% |
| Nightly | ≥ 50% | Generate deprecation report |

### Performance Benchmarks

| Source | Expected Avg Response | Warning Threshold | Critical Threshold |
|--------|----------------------|-------------------|-------------------|
| CNBC | < 2s | > 5s | > 10s |
| CNNFinance | < 1.5s | > 4s | > 8s |
| MarketWatch | < 2.5s | > 6s | > 12s |
| NASDAQ | < 2s | > 5s | > 10s |
| SeekingAlpha | < 3s | > 7s | > 15s |
| WSJ | < 2s | > 5s | > 10s |
| YahooFinance | < 1.5s | > 4s | > 8s |

## Contributing

When adding new tests:

1. Follow the existing test structure
2. Use the provided utilities and assertions
3. Add appropriate error handling and timeouts
4. Update this documentation for new features
5. Test in all three environments (Local, CI, Nightly)

### Adding a New Source Test

```rust
async fn test_new_source(client: reqwest::Client) -> Vec<TestResult> {
    let source = NewSource::new(client);
    let mut results = Vec::new();

    // Test basic functionality
    results.push(Self::test_basic_functionality(&source, "NewSource").await);

    // Test specific functions
    results.extend(vec![
        Self::test_function("function1", || source.function1()).await,
        Self::test_function("function2", || source.function2()).await,
    ]);

    results
}
```

Then add the source to the test runner's `test_source_async` method.