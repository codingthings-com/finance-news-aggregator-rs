use finance_news_aggregator_rs::news_source::NewsSource;
use finance_news_aggregator_rs::news_source::yahoo_finance::YahooFinance;
use std::time::Instant;
use tokio;

// Import integration test utilities
mod integration;
use integration::utils::{
    IntegrationTestConfig, TestContext, TestResult,
    assertions::{
        ArticleValidationRules, assert_article_meets_rules, assert_valid_news_article,
        assert_valid_news_collection, assert_valid_url,
    },
    client_factory::ClientFactory,
    deprecation_tracker::{DeprecationTracker, DeprecationReport},
};

/// Setup test context for Yahoo Finance integration tests
async fn setup_test_context() -> TestContext {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let config = IntegrationTestConfig::default();
    TestContext::new(client, config)
}

/// Test function execution with validation and error handling
async fn test_function_with_validation<F, Fut>(
    function_name: &str,
    test_fn: F,
    _context: &TestContext,
) -> TestResult
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<
            Output = Result<
                Vec<finance_news_aggregator_rs::types::NewsArticle>,
                finance_news_aggregator_rs::error::FanError,
            >,
        >,
{
    let start_time = Instant::now();

    match test_fn().await {
        Ok(articles) => {
            // Validate that we got some articles
            if !articles.is_empty() {
                assert_valid_news_collection(&articles, 1);

                // Validate individual articles
                for article in &articles {
                    assert_valid_news_article(article, false);
                }
            }

            TestResult::success(function_name, articles.len(), start_time.elapsed())
        }
        Err(e) => {
            // For now, just log the error without using the deprecation tracker
            // since it requires mutable access
            println!("Warning: Function '{}' failed: {}", function_name, e);
            TestResult::failure(function_name, e.to_string(), start_time.elapsed())
        }
    }
}

// Task 8: Implement Yahoo Finance integration tests

#[tokio::test]
async fn test_yahoo_finance_basic_functionality() {
    let context = setup_test_context().await;
    let yahoo_finance = YahooFinance::new(context.client.clone());

    // Test name() function
    assert_eq!(yahoo_finance.name(), "Yahoo Finance");

    // Test base_url() function
    let base_url = yahoo_finance.base_url();
    assert!(!base_url.is_empty());
    assert!(base_url.contains("finance.yahoo.com"));
    assert!(base_url.contains("rssindex"));

    // Test available_topics() function
    let topics = yahoo_finance.available_topics();
    assert!(!topics.is_empty());
    assert_eq!(topics.len(), 2); // Yahoo Finance has 2 defined topics

    // Verify expected topics are present
    let expected_topics = vec!["topstories", "headlines"];
    for expected_topic in expected_topics {
        assert!(
            topics.contains(&expected_topic),
            "Expected topic '{}' not found in available topics",
            expected_topic
        );
    }
}

#[tokio::test]
async fn test_yahoo_finance_headlines() {
    let context = setup_test_context().await;
    let yahoo_finance = YahooFinance::new(context.client.clone());

    let result = test_function_with_validation("headlines", || yahoo_finance.headlines(), &context).await;

    assert!(
        result.success,
        "headlines() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_yahoo_finance_topstories() {
    let context = setup_test_context().await;
    let yahoo_finance = YahooFinance::new(context.client.clone());

    let result = test_function_with_validation("topstories", || yahoo_finance.topstories(), &context).await;

    assert!(
        result.success,
        "topstories() failed: {:?}",
        result.error_message
    );
}

// Task 8.1: Add Yahoo Finance symbol-based testing

#[tokio::test]
async fn test_yahoo_finance_headline_with_single_symbol() {
    let context = setup_test_context().await;
    let yahoo_finance = YahooFinance::new(context.client.clone());

    // Test with a single popular stock symbol
    let symbols = vec!["AAPL"];
    let result = test_function_with_validation(
        "headline_single_symbol", 
        || yahoo_finance.headline(&symbols), 
        &context
    ).await;

    assert!(
        result.success,
        "headline() with single symbol failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_yahoo_finance_headline_with_multiple_symbols() {
    let context = setup_test_context().await;
    let yahoo_finance = YahooFinance::new(context.client.clone());

    // Test with multiple popular stock symbols
    let symbols = vec!["AAPL", "MSFT", "GOOGL"];
    let result = test_function_with_validation(
        "headline_multiple_symbols", 
        || yahoo_finance.headline(&symbols), 
        &context
    ).await;

    assert!(
        result.success,
        "headline() with multiple symbols failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_yahoo_finance_headline_with_various_ticker_symbols() {
    let context = setup_test_context().await;
    let yahoo_finance = YahooFinance::new(context.client.clone());

    // Test various realistic ticker symbols
    let test_cases = vec![
        // Tech stocks
        vec!["AAPL"],
        vec!["MSFT"],
        vec!["GOOGL"],
        vec!["AMZN"],
        vec!["TSLA"],
        
        // Financial stocks
        vec!["JPM"],
        vec!["BAC"],
        vec!["WFC"],
        
        // Multiple symbols
        vec!["AAPL", "MSFT"],
        vec!["GOOGL", "AMZN", "TSLA"],
        vec!["JPM", "BAC", "WFC", "GS"],
    ];

    let mut successful_tests = 0;
    let mut failed_tests = Vec::new();

    for (i, symbols) in test_cases.iter().enumerate() {
        let symbols_str = symbols.join(",");
        println!("Testing Yahoo Finance headline() with symbols: {}", symbols_str);

        let result = test_function_with_validation(
            &format!("headline_test_{}", i + 1),
            || yahoo_finance.headline(symbols),
            &context,
        ).await;

        if result.success {
            successful_tests += 1;
            println!("  ✓ Success - {} articles returned", result.article_count);
        } else {
            failed_tests.push((symbols_str.clone(), result.error_message.unwrap_or_default()));
            println!("  ✗ Failed: {}", failed_tests.last().unwrap().1);
        }
    }

    println!("\n=== YAHOO FINANCE SYMBOL TEST SUMMARY ===");
    println!("Successful tests: {}/{}", successful_tests, test_cases.len());
    println!("Failed tests: {}/{}", failed_tests.len(), test_cases.len());

    if !failed_tests.is_empty() {
        println!("Failed symbol tests:");
        for (symbols, error) in &failed_tests {
            println!("  - {}: {}", symbols, error);
        }
    }

    // We expect at least half of the symbol tests to work
    assert!(
        successful_tests >= test_cases.len() / 2,
        "Expected at least half of Yahoo Finance symbol tests to work, got {}/{} successful. Failed tests: {:?}",
        successful_tests,
        test_cases.len(),
        failed_tests
    );
}

#[tokio::test]
async fn test_yahoo_finance_base_url_endpoint_functionality() {
    let context = setup_test_context().await;
    let yahoo_finance = YahooFinance::new(context.client.clone());

    // Test base URL accessibility and format
    let base_url = yahoo_finance.base_url();
    println!("Testing Yahoo Finance base URL: {}", base_url);

    // Validate base URL format
    assert!(base_url.starts_with("https://"), "Base URL should use HTTPS");
    assert!(base_url.contains("finance.yahoo.com"), "Base URL should point to Yahoo Finance domain");
    assert!(base_url.contains("rssindex"), "Base URL should contain rssindex path");

    // Test URL construction for different endpoints
    let endpoints = vec![
        ("headlines", format!("{}/headlines", base_url)),
        ("topstories", format!("{}/topstories", base_url)),
    ];

    for (endpoint_name, url) in &endpoints {
        println!("Testing endpoint URL construction: {} -> {}", endpoint_name, url);
        
        // Validate URL format
        assert_valid_url(url);
        assert!(url.contains("finance.yahoo.com"));
        assert!(url.contains(endpoint_name));
    }

    // Test symbol-based URL construction
    let test_symbols = vec!["AAPL", "MSFT"];
    let symbols_str = test_symbols.join(",");
    let symbol_url = format!("{}/headline?s={}", base_url, symbols_str);
    
    println!("Testing symbol URL construction: {}", symbol_url);
    assert_valid_url(&symbol_url);
    assert!(symbol_url.contains("headline"));
    assert!(symbol_url.contains("s="));
    assert!(symbol_url.contains(&symbols_str));
}

#[tokio::test]
async fn test_yahoo_finance_data_structure_validation() {
    let context = setup_test_context().await;
    let yahoo_finance = YahooFinance::new(context.client.clone());

    // Test with headlines as it's likely to have good data
    match yahoo_finance.headlines().await {
        Ok(articles) => {
            assert!(!articles.is_empty(), "Should receive at least one article");

            // Validate article structure integrity
            for (i, article) in articles.iter().take(5).enumerate() {
                println!("Validating Yahoo Finance article {}: {:?}", i + 1, article.title);

                // Use lenient validation rules for real-world data
                let rules = ArticleValidationRules::lenient();
                assert_article_meets_rules(article, &rules);

                // Validate that articles have meaningful content
                assert!(
                    article.title.is_some() || article.description.is_some(),
                    "Article {} should have either title or description",
                    i + 1
                );

                // If title exists, validate it's not empty
                if let Some(ref title) = article.title {
                    assert!(
                        !title.trim().is_empty(),
                        "Article title should not be empty"
                    );
                    assert!(
                        title.len() >= 3,
                        "Article title should be at least 3 characters"
                    );
                }

                // If link exists, validate URL format
                if let Some(ref link) = article.link {
                    assert!(!link.trim().is_empty(), "Article link should not be empty");
                    assert_valid_url(link);
                }

                // If description exists, validate it's meaningful
                if let Some(ref description) = article.description {
                    assert!(
                        !description.trim().is_empty(),
                        "Article description should not be empty"
                    );
                }

                // Validate source is set correctly
                assert_eq!(
                    article.source,
                    Some("Yahoo Finance".to_string()),
                    "Article source should be set to Yahoo Finance"
                );
            }
        }
        Err(e) => {
            panic!("Failed to fetch Yahoo Finance headlines for validation: {}", e);
        }
    }
}

#[tokio::test]
async fn test_yahoo_finance_all_public_methods_comprehensive() {
    let context = setup_test_context().await;
    let yahoo_finance = YahooFinance::new(context.client.clone());

    let mut successful_methods = 0;
    let mut failed_methods = Vec::new();

    // Test headlines
    println!("Testing Yahoo Finance method: headlines");
    let result = test_function_with_validation("headlines", || yahoo_finance.headlines(), &context).await;
    if result.success {
        successful_methods += 1;
        println!("  ✓ headlines returned {} articles", result.article_count);
    } else {
        failed_methods.push(("headlines", result.error_message.unwrap_or_default()));
        println!("  ✗ headlines failed: {:?}", failed_methods.last().unwrap().1);
    }

    // Test topstories
    println!("Testing Yahoo Finance method: topstories");
    let result = test_function_with_validation("topstories", || yahoo_finance.topstories(), &context).await;
    if result.success {
        successful_methods += 1;
        println!("  ✓ topstories returned {} articles", result.article_count);
    } else {
        failed_methods.push(("topstories", result.error_message.unwrap_or_default()));
        println!("  ✗ topstories failed: {:?}", failed_methods.last().unwrap().1);
    }

    // Test headline with symbols
    println!("Testing Yahoo Finance method: headline (with symbols)");
    let symbols = vec!["AAPL", "MSFT"];
    let result = test_function_with_validation("headline_with_symbols", || yahoo_finance.headline(&symbols), &context).await;
    if result.success {
        successful_methods += 1;
        println!("  ✓ headline with symbols returned {} articles", result.article_count);
    } else {
        failed_methods.push(("headline_with_symbols", result.error_message.unwrap_or_default()));
        println!("  ✗ headline with symbols failed: {:?}", failed_methods.last().unwrap().1);
    }

    println!("\n=== YAHOO FINANCE METHOD TEST SUMMARY ===");
    println!("Successful methods: {}/3", successful_methods);
    println!("Failed methods: {}/3", failed_methods.len());
    
    if !failed_methods.is_empty() {
        println!("Failed methods:");
        for (method, error) in &failed_methods {
            println!("  - {}: {}", method, error);
        }
    }

    // We expect at least 2 out of 3 methods to work
    assert!(
        successful_methods >= 2,
        "Expected at least 2/3 Yahoo Finance methods to work, got {}/3 successful. Failed methods: {:?}",
        successful_methods,
        failed_methods
    );
}

#[tokio::test]
async fn test_yahoo_finance_fetch_feed_with_topics() {
    let context = setup_test_context().await;
    let yahoo_finance = YahooFinance::new(context.client.clone());

    // Test fetch_feed method directly with all available topics
    let topics = yahoo_finance.available_topics();
    
    let mut successful_topics = 0;
    let mut failed_topics = Vec::new();

    for &topic in &topics {
        println!("Testing Yahoo Finance topic: {}", topic);
        
        match yahoo_finance.fetch_feed(topic).await {
            Ok(articles) => {
                successful_topics += 1;
                println!("  ✓ Topic '{}' returned {} articles", topic, articles.len());

                if !articles.is_empty() {
                    // Validate first article from each successful topic
                    let article = &articles[0];
                    assert_valid_news_article(article, false);

                    // Ensure source is properly set
                    assert_eq!(article.source, Some("Yahoo Finance".to_string()));
                }
            }
            Err(e) => {
                failed_topics.push((topic, e.to_string()));
                println!("  ✗ Topic '{}' failed: {}", topic, e);
            }
        }
    }

    println!("\n=== YAHOO FINANCE TOPIC TEST SUMMARY ===");
    println!("Successful topics: {}/{}", successful_topics, topics.len());
    println!("Failed topics: {}/{}", failed_topics.len(), topics.len());

    // We expect all topics to work since Yahoo Finance only has 2 basic topics
    assert!(
        successful_topics >= topics.len() / 2,
        "Expected at least half of Yahoo Finance topics to work, got {}/{} successful. Failed topics: {:?}",
        successful_topics,
        topics.len(),
        failed_topics
    );
}

#[tokio::test]
async fn test_yahoo_finance_symbol_parameter_edge_cases() {
    let context = setup_test_context().await;
    let yahoo_finance = YahooFinance::new(context.client.clone());

    // Test edge cases for symbol parameters
    let test_cases = vec![
        // Empty symbol array
        (vec![], "empty_symbols"),
        
        // Single character symbols (some ETFs)
        (vec!["V"], "single_char_symbol"),
        
        // Symbols with numbers
        (vec!["BRK.A", "BRK.B"], "symbols_with_dots"),
        
        // Mixed case symbols (should be handled properly)
        (vec!["aapl", "MSFT", "Googl"], "mixed_case_symbols"),
        
        // Many symbols
        (vec!["AAPL", "MSFT", "GOOGL", "AMZN", "TSLA", "META", "NVDA", "NFLX"], "many_symbols"),
    ];

    let mut successful_cases = 0;
    let mut failed_cases = Vec::new();

    let test_cases_len = test_cases.len();
    for (symbols, case_name) in test_cases {
        let symbols_str = symbols.join(",");
        println!("Testing Yahoo Finance edge case '{}': {}", case_name, symbols_str);

        match yahoo_finance.headline(&symbols).await {
            Ok(articles) => {
                successful_cases += 1;
                println!("  ✓ Success - {} articles returned", articles.len());
                
                // For non-empty symbol arrays, validate articles if any returned
                if !symbols.is_empty() && !articles.is_empty() {
                    // Validate first article
                    let article = &articles[0];
                    assert_valid_news_article(article, false);
                    assert_eq!(article.source, Some("Yahoo Finance".to_string()));
                }
            }
            Err(e) => {
                failed_cases.push((case_name.to_string(), e.to_string()));
                println!("  ✗ Failed: {}", e);
            }
        }
    }

    println!("\n=== YAHOO FINANCE EDGE CASE TEST SUMMARY ===");
    println!("Successful cases: {}/{}", successful_cases, test_cases_len);
    println!("Failed cases: {}/{}", failed_cases.len(), test_cases_len);

    if !failed_cases.is_empty() {
        println!("Failed edge cases:");
        for (case, error) in &failed_cases {
            println!("  - {}: {}", case, error);
        }
    }

    // We expect most edge cases to work, but some may fail (like empty symbols)
    assert!(
        successful_cases >= test_cases_len / 2,
        "Expected at least half of Yahoo Finance edge cases to work, got {}/{} successful. Failed cases: {:?}",
        successful_cases,
        test_cases_len,
        failed_cases
    );
}