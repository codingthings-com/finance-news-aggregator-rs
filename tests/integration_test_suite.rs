use std::env;

// Import integration test utilities
mod integration;
use integration::{
    test_runner::{IntegrationTestRunner, TestSummary},
    utils::environment::{EnvironmentConfig, TestMode},
};

/// Main integration test that runs the comprehensive test suite
#[tokio::test]
async fn run_comprehensive_integration_tests() {
    // Initialize logging for better debugging
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .try_init();

    println!("🚀 Starting comprehensive integration test suite");
    
    // Create and run the test runner
    let mut runner = IntegrationTestRunner::new()
        .await
        .expect("Failed to create integration test runner");

    let summary = runner
        .run_all_tests()
        .await
        .expect("Failed to run integration tests");

    // Validate results based on environment
    validate_test_results(&summary);
}

/// Validate test results based on environment expectations
fn validate_test_results(summary: &TestSummary) {
    let env_config = EnvironmentConfig::from_env();
    let overall_success_rate = summary.successful_tests as f64 / summary.total_tests as f64;

    match env_config.test_mode {
        TestMode::CI => {
            // In CI, we expect at least 70% success rate
            assert!(
                overall_success_rate >= 0.7,
                "CI test success rate too low: {:.1}% (expected >= 70%)",
                overall_success_rate * 100.0
            );
            
            // Ensure we tested at least some sources
            assert!(
                summary.total_tests >= 10,
                "Too few tests run in CI: {} (expected >= 10)",
                summary.total_tests
            );
        }
        TestMode::Local => {
            // In local development, we expect at least 60% success rate
            assert!(
                overall_success_rate >= 0.6,
                "Local test success rate too low: {:.1}% (expected >= 60%)",
                overall_success_rate * 100.0
            );
        }
        TestMode::Nightly => {
            // In nightly builds, we're more lenient but want comprehensive coverage
            assert!(
                overall_success_rate >= 0.5,
                "Nightly test success rate too low: {:.1}% (expected >= 50%)",
                overall_success_rate * 100.0
            );
            
            // Ensure comprehensive testing in nightly builds
            assert!(
                summary.total_tests >= 50,
                "Insufficient test coverage in nightly build: {} (expected >= 50)",
                summary.total_tests
            );
        }
    }

    // Validate that we fetched some articles (indicates sources are working)
    if summary.successful_tests > 0 {
        assert!(
            summary.total_articles > 0,
            "No articles fetched despite successful tests"
        );
    }

    // Check individual source health
    for (source_name, source_summary) in &summary.source_summaries {
        if source_summary.tests_run > 0 {
            // Each source should have at least some success
            assert!(
                source_summary.success_rate > 0.0,
                "Source {} has 0% success rate",
                source_name
            );
            
            // Warn about sources with very low success rates
            if source_summary.success_rate < 0.3 {
                println!(
                    "⚠️  WARNING: Source {} has low success rate: {:.1}%",
                    source_name,
                    source_summary.success_rate * 100.0
                );
            }
        }
    }

    println!("✅ Test validation completed successfully");
}

/// Test runner for specific sources (can be used for targeted testing)
#[tokio::test]
#[ignore] // Ignored by default, run with --ignored flag
async fn run_cnbc_only_tests() {
    unsafe {
        env::set_var("INTEGRATION_SOURCES", "CNBC");
    }
    
    let mut runner = IntegrationTestRunner::new()
        .await
        .expect("Failed to create test runner");

    let summary = runner
        .run_all_tests()
        .await
        .expect("Failed to run CNBC tests");

    // CNBC should have reasonable success rate
    let cnbc_summary = summary.source_summaries.get("CNBC")
        .expect("CNBC summary not found");
    
    assert!(
        cnbc_summary.success_rate >= 0.5,
        "CNBC success rate too low: {:.1}%",
        cnbc_summary.success_rate * 100.0
    );
    
    unsafe {
        env::remove_var("INTEGRATION_SOURCES");
    }
}

/// Test runner for WSJ and Yahoo Finance (premium sources)
#[tokio::test]
#[ignore] // Ignored by default, run with --ignored flag
async fn run_premium_sources_tests() {
    unsafe {
        env::set_var("INTEGRATION_SOURCES", "WallStreetJournal,YahooFinance");
    }
    
    let mut runner = IntegrationTestRunner::new()
        .await
        .expect("Failed to create test runner");

    let summary = runner
        .run_all_tests()
        .await
        .expect("Failed to run premium source tests");

    // Premium sources might have different success expectations
    for source_name in ["WallStreetJournal", "YahooFinance"] {
        if let Some(source_summary) = summary.source_summaries.get(source_name) {
            println!(
                "Premium source {} success rate: {:.1}%",
                source_name,
                source_summary.success_rate * 100.0
            );
        }
    }
    
    unsafe {
        env::remove_var("INTEGRATION_SOURCES");
    }
}

/// Performance regression test (nightly only)
#[tokio::test]
#[ignore] // Ignored by default, run with --ignored flag
async fn run_performance_regression_tests() {
    // Only run in nightly or when explicitly enabled
    if !env::var("NIGHTLY_BUILD").is_ok() && !env::var("ENABLE_PERFORMANCE_TESTS").is_ok() {
        println!("Skipping performance tests - not in nightly mode");
        return;
    }

    unsafe {
        env::set_var("ENABLE_PERFORMANCE_TRACKING", "true");
    }
    
    let mut runner = IntegrationTestRunner::new()
        .await
        .expect("Failed to create test runner");

    let summary = runner
        .run_all_tests()
        .await
        .expect("Failed to run performance tests");

    // Check for performance regressions
    for (source_name, source_summary) in &summary.source_summaries {
        let avg_time_ms = source_summary.average_response_time.as_millis();
        
        // Warn about very slow sources (> 10 seconds average)
        if avg_time_ms > 10000 {
            println!(
                "⚠️  PERFORMANCE WARNING: {} average response time is {}ms",
                source_name, avg_time_ms
            );
        }
        
        // Fail if any source is extremely slow (> 30 seconds average)
        assert!(
            avg_time_ms <= 30000,
            "PERFORMANCE REGRESSION: {} average response time {}ms exceeds 30s limit",
            source_name, avg_time_ms
        );
    }
    
    unsafe {
        env::remove_var("ENABLE_PERFORMANCE_TRACKING");
    }
}

/// Deprecation detection test (nightly only)
#[tokio::test]
#[ignore] // Ignored by default, run with --ignored flag
async fn run_deprecation_detection_tests() {
    // Only run in nightly or when explicitly enabled
    if !env::var("NIGHTLY_BUILD").is_ok() && !env::var("ENABLE_DEPRECATION_SCAN").is_ok() {
        println!("Skipping deprecation tests - not in nightly mode");
        return;
    }

    unsafe {
        env::set_var("ENABLE_DEPRECATION_TRACKING", "true");
    }
    
    let mut runner = IntegrationTestRunner::new()
        .await
        .expect("Failed to create test runner");

    let summary = runner
        .run_all_tests()
        .await
        .expect("Failed to run deprecation tests");

    // Analyze deprecation report
    if !summary.deprecation_report.is_empty() && summary.deprecation_report != "Deprecation tracking disabled" {
        println!("📋 Deprecation Analysis:");
        println!("{}", summary.deprecation_report);
        
        // Count sources with high failure rates (potential deprecation)
        let mut sources_needing_attention = Vec::new();
        
        for (source_name, source_summary) in &summary.source_summaries {
            if source_summary.success_rate < 0.3 && source_summary.tests_run > 5 {
                sources_needing_attention.push(source_name);
            }
        }
        
        if !sources_needing_attention.is_empty() {
            println!(
                "🚨 DEPRECATION ALERT: Sources needing attention: {:?}",
                sources_needing_attention
            );
        }
    }
    
    unsafe {
        env::remove_var("ENABLE_DEPRECATION_TRACKING");
    }
}

/// Network connectivity test
#[tokio::test]
async fn test_network_connectivity() {
    // Skip if network tests are disabled
    if env::var("SKIP_NETWORK_TESTS").is_ok() {
        println!("Skipping network connectivity test");
        return;
    }

    let client = integration::utils::client_factory::ClientFactory::create_test_client()
        .expect("Failed to create HTTP client");

    // Test basic internet connectivity
    let test_urls = vec![
        "https://www.cnbc.com",
        "https://money.cnn.com", 
        "https://www.marketwatch.com",
        "https://www.nasdaq.com",
        "https://seekingalpha.com",
        "https://www.wsj.com",
        "https://finance.yahoo.com",
    ];

    let mut connectivity_issues = Vec::new();

    let test_urls_len = test_urls.len();
    for url in test_urls {
        match client.head(url).send().await {
            Ok(response) => {
                if !response.status().is_success() && !response.status().is_redirection() {
                    connectivity_issues.push(format!("{}: {}", url, response.status()));
                }
            }
            Err(e) => {
                connectivity_issues.push(format!("{}: {}", url, e));
            }
        }
    }

    if !connectivity_issues.is_empty() {
        println!("⚠️  Network connectivity issues detected:");
        for issue in &connectivity_issues {
            println!("  - {}", issue);
        }
        
        // Don't fail the test for connectivity issues, just warn
        if connectivity_issues.len() > test_urls_len / 2 {
            println!("🚨 WARNING: More than half of news sources are unreachable");
        }
    } else {
        println!("✅ Network connectivity test passed");
    }
}