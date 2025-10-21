use finance_news_aggregator_rs::news_source::NewsSource;
use finance_news_aggregator_rs::news_source::nasdaq::NASDAQ;
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
};

/// Setup test context for NASDAQ integration tests
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

#[tokio::test]
async fn test_nasdaq_basic_functionality() {
    let context = setup_test_context().await;
    let nasdaq = NASDAQ::new(context.client.clone());

    // Test name() function
    assert_eq!(nasdaq.name(), "NASDAQ");

    // Test base_url() function
    let base_url = nasdaq.base_url();
    assert!(!base_url.is_empty());
    assert!(base_url.contains("nasdaq.com"));

    // Test available_topics() function
    let topics = nasdaq.available_topics();
    assert!(!topics.is_empty());

    // Verify expected topics are present
    let expected_topics = vec![
        "original",
        "commodities",
        "cryptocurrency",
        "dividends",
        "earnings",
        "economics",
        "financial-advisors",
        "innovation",
        "stocks",
        "technology",
    ];
    for expected_topic in expected_topics {
        assert!(
            topics.contains(&expected_topic),
            "Expected topic '{}' not found in available topics",
            expected_topic
        );
    }
}

#[tokio::test]
async fn test_nasdaq_commodities() {
    let context = setup_test_context().await;
    let nasdaq = NASDAQ::new(context.client.clone());

    let result = test_function_with_validation("commodities", || nasdaq.commodities(), &context).await;

    assert!(
        result.success,
        "commodities() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_nasdaq_cryptocurrency() {
    let context = setup_test_context().await;
    let nasdaq = NASDAQ::new(context.client.clone());

    let result = test_function_with_validation("cryptocurrency", || nasdaq.cryptocurrency(), &context).await;

    assert!(
        result.success,
        "cryptocurrency() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_nasdaq_dividends() {
    let context = setup_test_context().await;
    let nasdaq = NASDAQ::new(context.client.clone());

    let result = test_function_with_validation("dividends", || nasdaq.dividends(), &context).await;

    assert!(
        result.success,
        "dividends() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_nasdaq_earnings() {
    let context = setup_test_context().await;
    let nasdaq = NASDAQ::new(context.client.clone());

    let result = test_function_with_validation("earnings", || nasdaq.earnings(), &context).await;

    assert!(
        result.success,
        "earnings() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_nasdaq_economics() {
    let context = setup_test_context().await;
    let nasdaq = NASDAQ::new(context.client.clone());

    let result = test_function_with_validation("economics", || nasdaq.economics(), &context).await;

    assert!(
        result.success,
        "economics() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_nasdaq_innovation() {
    let context = setup_test_context().await;
    let nasdaq = NASDAQ::new(context.client.clone());

    let result = test_function_with_validation("innovation", || nasdaq.innovation(), &context).await;

    assert!(
        result.success,
        "innovation() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_nasdaq_technology() {
    let context = setup_test_context().await;
    let nasdaq = NASDAQ::new(context.client.clone());

    let result = test_function_with_validation("technology", || nasdaq.technology(), &context).await;

    assert!(
        result.success,
        "technology() failed: {:?}",
        result.error_message
    );
}

// Task 5.1: NASDAQ original content and category tests
#[tokio::test]
async fn test_nasdaq_original_content() {
    let context = setup_test_context().await;
    let nasdaq = NASDAQ::new(context.client.clone());

    let result = test_function_with_validation("original_content", || nasdaq.original_content(), &context).await;

    assert!(
        result.success,
        "original_content() failed: {:?}",
        result.error_message
    );

    // Additional validation for original content
    match nasdaq.original_content().await {
        Ok(articles) => {
            if !articles.is_empty() {
                // Validate that original content has proper structure
                for article in articles.iter().take(3) {
                    assert_eq!(
                        article.source,
                        Some("NASDAQ".to_string()),
                        "Original content should have NASDAQ as source"
                    );
                    
                    // Original content should have meaningful titles
                    if let Some(ref title) = article.title {
                        assert!(
                            title.len() >= 5,
                            "Original content titles should be substantial"
                        );
                    }
                }
            }
        }
        Err(e) => {
            println!("Warning: Could not validate original content structure: {}", e);
        }
    }
}

#[tokio::test]
async fn test_nasdaq_feed_by_category() {
    let context = setup_test_context().await;
    let nasdaq = NASDAQ::new(context.client.clone());

    // Test feed_by_category with various category parameters
    let test_categories = vec![
        "commodities",
        "cryptocurrency", 
        "dividends",
        "earnings",
        "economics",
        "financial-advisors",
        "innovation",
        "stocks",
        "technology",
    ];

    let mut successful_categories = 0;
    let mut failed_categories = Vec::new();

    for category in &test_categories {
        println!("Testing NASDAQ category: {}", category);
        
        let result = test_function_with_validation(
            &format!("feed_by_category({})", category),
            || nasdaq.feed_by_category(category),
            &context,
        ).await;

        if result.success {
            successful_categories += 1;
            println!("  ✓ Category '{}' returned {} articles", category, result.article_count);
        } else {
            failed_categories.push((category, result.error_message.unwrap_or_default()));
            println!("  ✗ Category '{}' failed", category);
        }
    }

    // We expect at least half of the categories to work
    assert!(
        successful_categories >= test_categories.len() / 2,
        "Expected at least half of NASDAQ categories to work, got {}/{} successful. Failed: {:?}",
        successful_categories,
        test_categories.len(),
        failed_categories
    );
}

#[tokio::test]
async fn test_nasdaq_financial_advisors() {
    let context = setup_test_context().await;
    let nasdaq = NASDAQ::new(context.client.clone());

    let result = test_function_with_validation("financial_advisors", || nasdaq.financial_advisors(), &context).await;

    assert!(
        result.success,
        "financial_advisors() failed: {:?}",
        result.error_message
    );

    // Validate financial_advisors data structure integrity
    match nasdaq.financial_advisors().await {
        Ok(articles) => {
            if !articles.is_empty() {
                println!("Validating financial advisors articles structure");
                
                for (i, article) in articles.iter().take(3).enumerate() {
                    println!("Validating financial advisors article {}: {:?}", i + 1, article.title);

                    // Use lenient validation rules for real-world data
                    let rules = ArticleValidationRules::lenient();
                    assert_article_meets_rules(article, &rules);

                    // Validate source is set correctly
                    assert_eq!(
                        article.source,
                        Some("NASDAQ".to_string()),
                        "Financial advisors article source should be set to NASDAQ"
                    );

                    // Validate that articles have meaningful content
                    assert!(
                        article.title.is_some() || article.description.is_some(),
                        "Financial advisors article {} should have either title or description",
                        i + 1
                    );

                    // If link exists, validate URL format
                    if let Some(ref link) = article.link {
                        assert!(!link.trim().is_empty(), "Financial advisors article link should not be empty");
                        assert_valid_url(link);
                    }
                }
            }
        }
        Err(e) => {
            println!("Warning: Could not validate financial advisors data structure: {}", e);
        }
    }
}

#[tokio::test]
async fn test_nasdaq_stocks() {
    let context = setup_test_context().await;
    let nasdaq = NASDAQ::new(context.client.clone());

    let result = test_function_with_validation("stocks", || nasdaq.stocks(), &context).await;

    assert!(
        result.success,
        "stocks() failed: {:?}",
        result.error_message
    );

    // Validate stocks data structure integrity
    match nasdaq.stocks().await {
        Ok(articles) => {
            if !articles.is_empty() {
                println!("Validating stocks articles structure");
                
                for (i, article) in articles.iter().take(3).enumerate() {
                    println!("Validating stocks article {}: {:?}", i + 1, article.title);

                    // Use lenient validation rules for real-world data
                    let rules = ArticleValidationRules::lenient();
                    assert_article_meets_rules(article, &rules);

                    // Validate source is set correctly
                    assert_eq!(
                        article.source,
                        Some("NASDAQ".to_string()),
                        "Stocks article source should be set to NASDAQ"
                    );

                    // Validate that articles have meaningful content
                    assert!(
                        article.title.is_some() || article.description.is_some(),
                        "Stocks article {} should have either title or description",
                        i + 1
                    );

                    // If title exists, validate it's not empty and substantial
                    if let Some(ref title) = article.title {
                        assert!(
                            !title.trim().is_empty(),
                            "Stocks article title should not be empty"
                        );
                        assert!(
                            title.len() >= 3,
                            "Stocks article title should be at least 3 characters"
                        );
                    }

                    // If link exists, validate URL format
                    if let Some(ref link) = article.link {
                        assert!(!link.trim().is_empty(), "Stocks article link should not be empty");
                        assert_valid_url(link);
                        
                        // Stocks articles should likely link to nasdaq.com
                        if link.contains("nasdaq.com") {
                            println!("  ✓ Stocks article {} links to NASDAQ domain", i + 1);
                        }
                    }

                    // If description exists, validate it's meaningful
                    if let Some(ref description) = article.description {
                        assert!(
                            !description.trim().is_empty(),
                            "Stocks article description should not be empty"
                        );
                    }
                }
            }
        }
        Err(e) => {
            println!("Warning: Could not validate stocks data structure: {}", e);
        }
    }
}

// Task 5.2: NASDAQ endpoint validation
#[tokio::test]
async fn test_nasdaq_endpoint_validation() {
    let context = setup_test_context().await;
    let nasdaq = NASDAQ::new(context.client.clone());

    println!("=== NASDAQ ENDPOINT VALIDATION ===");

    // Test base_url availability
    let base_url = nasdaq.base_url();
    println!("Testing NASDAQ base_url: {}", base_url);
    
    // Test base URL with a simple category to validate endpoint
    match nasdaq.feed_by_category("economics").await {
        Ok(articles) => {
            println!("  ✓ Base URL endpoint is accessible (returned {} articles)", articles.len());
        }
        Err(e) => {
            println!("  ✗ Base URL endpoint failed: {}", e);
            
            // Record this as a potential deprecation
            let error_type = classify_endpoint_error(&e);
            if matches!(error_type.as_str(), "HTTP_404_NOT_FOUND" | "HTTP_403_FORBIDDEN" | "DNS_ERROR") {
                println!("  ⚠️  Base URL may be deprecated: {}", error_type);
            }
        }
    }

    // Test original_content_url availability
    println!("Testing NASDAQ original_content_url");
    match nasdaq.original_content().await {
        Ok(articles) => {
            println!("  ✓ Original content URL is accessible (returned {} articles)", articles.len());
        }
        Err(e) => {
            println!("  ✗ Original content URL failed: {}", e);
            
            // Record this as a potential deprecation
            let error_type = classify_endpoint_error(&e);
            if matches!(error_type.as_str(), "HTTP_404_NOT_FOUND" | "HTTP_403_FORBIDDEN" | "DNS_ERROR") {
                println!("  ⚠️  Original content URL may be deprecated: {}", error_type);
            }
        }
    }

    // Test all category endpoints for deprecation
    let categories = nasdaq.available_topics();
    let mut working_categories = Vec::new();
    let mut deprecated_categories = Vec::new();
    let mut temporary_failures = Vec::new();

    println!("Testing NASDAQ category endpoints for deprecation...");
    
    for category in &categories {
        if *category == "original" {
            continue; // Already tested above
        }
        
        print!("  Testing category '{}': ", category);
        
        match nasdaq.feed_by_category(category).await {
            Ok(articles) => {
                println!("✓ Working ({} articles)", articles.len());
                working_categories.push(*category);
            }
            Err(e) => {
                let error_type = classify_endpoint_error(&e);
                
                match error_type.as_str() {
                    "HTTP_404_NOT_FOUND" | "HTTP_403_FORBIDDEN" | "DNS_ERROR" => {
                        println!("✗ DEPRECATED ({})", error_type);
                        deprecated_categories.push((*category, error_type));
                    }
                    _ => {
                        println!("⚠️  Temporary failure ({})", error_type);
                        temporary_failures.push((*category, error_type));
                    }
                }
            }
        }
    }

    // Generate removal recommendations
    println!("\n=== NASDAQ ENDPOINT VALIDATION REPORT ===");
    println!("Total categories tested: {}", categories.len());
    println!("Working categories: {}", working_categories.len());
    println!("Deprecated categories: {}", deprecated_categories.len());
    println!("Temporary failures: {}", temporary_failures.len());

    if !working_categories.is_empty() {
        println!("\nWorking categories:");
        for category in &working_categories {
            println!("  ✓ {}", category);
        }
    }

    if !deprecated_categories.is_empty() {
        println!("\n⚠️  DEPRECATED CATEGORIES (removal candidates):");
        for (category, error_type) in &deprecated_categories {
            println!("  ✗ {} ({})", category, error_type);
        }
        
        println!("\nREMOVAL RECOMMENDATIONS:");
        println!("The following NASDAQ categories should be considered for removal:");
        for (category, _) in &deprecated_categories {
            println!("  - Remove '{}' method and category from available_topics()", category);
        }
    }

    if !temporary_failures.is_empty() {
        println!("\nTemporary failures (monitor for patterns):");
        for (category, error_type) in &temporary_failures {
            println!("  ⚠️  {} ({})", category, error_type);
        }
    }

    // Validate that at least some categories are working
    let working_percentage = working_categories.len() as f64 / (categories.len() - 1) as f64; // -1 for "original"
    
    if working_percentage < 0.5 {
        println!("\n🚨 WARNING: Less than 50% of NASDAQ categories are working!");
        println!("This may indicate widespread API changes or deprecation.");
    } else {
        println!("\n✅ NASDAQ endpoint validation completed successfully");
        println!("Working endpoint percentage: {:.1}%", working_percentage * 100.0);
    }

    // Assert that we have at least some working endpoints
    assert!(
        !working_categories.is_empty(),
        "No NASDAQ categories are working - this indicates complete API failure"
    );
}

/// Classify endpoint errors for deprecation tracking
fn classify_endpoint_error(error: &finance_news_aggregator_rs::error::FanError) -> String {
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

#[tokio::test]
async fn test_nasdaq_deprecation_tracking_integration() {
    use integration::utils::deprecation_tracker::DeprecationTracker;
    
    let context = setup_test_context().await;
    let nasdaq = NASDAQ::new(context.client.clone());
    let mut tracker = DeprecationTracker::new();

    println!("=== NASDAQ DEPRECATION TRACKING ===");

    // Test all NASDAQ endpoints and track failures
    let categories = nasdaq.available_topics();
    
    for category in &categories {
        let result = if *category == "original" {
            nasdaq.original_content().await
        } else {
            nasdaq.feed_by_category(category).await
        };

        match result {
            Ok(articles) => {
                println!("✓ Category '{}': {} articles", category, articles.len());
            }
            Err(e) => {
                println!("✗ Category '{}': {}", category, e);
                
                // Record failure in deprecation tracker
                let url = if *category == "original" {
                    "https://www.nasdaq.com/feed/nasdaq-original/rss.xml".to_string()
                } else {
                    format!("https://www.nasdaq.com/feed/rssoutbound?category={}", category)
                };
                
                tracker.record_failure_with_url(
                    "NASDAQ",
                    &format!("{}()", category),
                    &url,
                    &e,
                );
            }
        }
    }

    // Generate and display deprecation report
    let report = tracker.generate_report();
    println!("\n{}", report);

    // Check for critical failures that indicate deprecation
    if tracker.has_critical_failures("NASDAQ") {
        println!("🚨 NASDAQ has critical failures that may indicate deprecated endpoints!");
        
        let nasdaq_failures = tracker.get_source_failures("NASDAQ");
        for failure in nasdaq_failures {
            if matches!(
                failure.error_type.as_str(),
                "HTTP_404_NOT_FOUND" | "HTTP_403_FORBIDDEN" | "DNS_ERROR"
            ) {
                println!("  Critical failure: {}::{} - {}", 
                    failure.source, failure.function, failure.error_type);
            }
        }
    }

    // Validate that we don't have complete failure
    assert!(
        report.total_failures < categories.len(),
        "All NASDAQ endpoints failed - this indicates complete API deprecation"
    );
}

// Additional comprehensive tests for all NASDAQ methods
#[tokio::test]
async fn test_nasdaq_all_public_methods() {
    let context = setup_test_context().await;
    let nasdaq = NASDAQ::new(context.client.clone());

    // Test all public methods systematically
    let test_methods = vec![
        "original_content",
        "commodities",
        "cryptocurrency", 
        "dividends",
        "earnings",
        "economics",
        "financial_advisors",
        "innovation",
        "stocks",
        "technology",
    ];

    let mut successful_methods = 0;
    let mut failed_methods = Vec::new();

    for method_name in &test_methods {
        println!("Testing NASDAQ method: {}", method_name);
        
        let result = match *method_name {
            "original_content" => test_function_with_validation(method_name, || nasdaq.original_content(), &context).await,
            "commodities" => test_function_with_validation(method_name, || nasdaq.commodities(), &context).await,
            "cryptocurrency" => test_function_with_validation(method_name, || nasdaq.cryptocurrency(), &context).await,
            "dividends" => test_function_with_validation(method_name, || nasdaq.dividends(), &context).await,
            "earnings" => test_function_with_validation(method_name, || nasdaq.earnings(), &context).await,
            "economics" => test_function_with_validation(method_name, || nasdaq.economics(), &context).await,
            "financial_advisors" => test_function_with_validation(method_name, || nasdaq.financial_advisors(), &context).await,
            "innovation" => test_function_with_validation(method_name, || nasdaq.innovation(), &context).await,
            "stocks" => test_function_with_validation(method_name, || nasdaq.stocks(), &context).await,
            "technology" => test_function_with_validation(method_name, || nasdaq.technology(), &context).await,
            _ => unreachable!(),
        };

        if result.success {
            successful_methods += 1;
            println!("  ✓ Method '{}' returned {} articles", method_name, result.article_count);
        } else {
            failed_methods.push((method_name, result.error_message.unwrap_or_default()));
            println!("  ✗ Method '{}' failed", method_name);
        }
    }

    println!("\n=== NASDAQ METHODS SUMMARY ===");
    println!("Total methods tested: {}", test_methods.len());
    println!("Successful methods: {}", successful_methods);
    println!("Failed methods: {}", failed_methods.len());

    if !failed_methods.is_empty() {
        println!("Failed methods:");
        for (method, error) in &failed_methods {
            println!("  - {}: {}", method, error);
        }
    }

    // We expect at least 70% of methods to work for NASDAQ to be considered functional
    let success_rate = successful_methods as f64 / test_methods.len() as f64;
    assert!(
        success_rate >= 0.7,
        "NASDAQ success rate too low: {:.1}% ({}/{}). This may indicate widespread issues.",
        success_rate * 100.0,
        successful_methods,
        test_methods.len()
    );
}

#[tokio::test]
async fn test_nasdaq_article_structure_validation() {
    let context = setup_test_context().await;
    let nasdaq = NASDAQ::new(context.client.clone());

    // Test with economics news as it's likely to have good data
    match nasdaq.economics().await {
        Ok(articles) => {
            assert!(!articles.is_empty(), "Should receive at least one article");

            // Validate article structure integrity
            for (i, article) in articles.iter().take(5).enumerate() {
                println!("Validating NASDAQ article {}: {:?}", i + 1, article.title);

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
                    Some("NASDAQ".to_string()),
                    "Article source should be set to NASDAQ"
                );
            }
        }
        Err(e) => {
            panic!("Failed to fetch economics news for validation: {}", e);
        }
    }
}