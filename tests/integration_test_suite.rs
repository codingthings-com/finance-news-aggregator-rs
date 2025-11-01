use std::env;

// Import integration test utilities
mod integration;
use integration::test_runner::{IntegrationTestRunner, TestSummary};

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

/// Validate test results
fn validate_test_results(summary: &TestSummary) {
    let overall_success_rate = summary.successful_tests as f64 / summary.total_tests as f64;

    // We expect at least 50% success rate
    assert!(
        overall_success_rate >= 0.5,
        "Test success rate too low: {:.1}% (expected >= 50%)",
        overall_success_rate * 100.0
    );

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

/// Deprecation detection test
#[tokio::test]
#[ignore] // Ignored by default, run with --ignored flag
async fn run_deprecation_detection_tests() {
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
    if !summary.deprecation_report.is_empty()
        && summary.deprecation_report != "Deprecation tracking disabled"
    {
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
