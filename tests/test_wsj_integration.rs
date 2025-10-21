use finance_news_aggregator_rs::news_source::NewsSource;
use finance_news_aggregator_rs::news_source::wsj::WallStreetJournal;
use finance_news_aggregator_rs::types::SourceConfig;
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
    deprecation_tracker::DeprecationTracker,
};

/// Setup test context for WSJ integration tests
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

// Task 7.1: Add WSJ configuration and client tests

#[tokio::test]
async fn test_wsj_new_client_initialization() {
    let context = setup_test_context().await;
    
    // Test new() method
    let wsj = WallStreetJournal::new(context.client.clone());
    
    // Validate basic properties
    assert_eq!(wsj.name(), "Wall Street Journal");
    
    let base_url = wsj.base_url();
    assert!(!base_url.is_empty());
    assert!(base_url.contains("feeds.a.dj.com"));
    assert!(base_url.contains("{topic}"));
    
    // Test available_topics() function
    let topics = wsj.available_topics();
    assert!(!topics.is_empty());
    
    // Verify expected topics are present
    let expected_topics = vec![
        "RSSOpinion",
        "RSSWorldNews", 
        "WSJcomUSBusiness",
        "RSSMarketsMain",
        "RSSWSJD",
        "RSSLifestyle",
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
async fn test_wsj_with_config_client_initialization() {
    let context = setup_test_context().await;
    
    // Create custom configuration
    let custom_config = SourceConfig::new("https://feeds.a.dj.com/rss/{topic}.xml")
        .with_timeout(60)
        .with_user_agent("Custom WSJ Test Agent")
        .with_retries(5, 2000);
    
    // Test with_config() method
    let wsj = WallStreetJournal::with_config(context.client.clone(), custom_config.clone());
    
    // Validate basic properties
    assert_eq!(wsj.name(), "Wall Street Journal");
    
    let base_url = wsj.base_url();
    assert_eq!(base_url, custom_config.base_url);
    assert!(base_url.contains("feeds.a.dj.com"));
    assert!(base_url.contains("{topic}"));
    
    // Test available_topics() function
    let topics = wsj.available_topics();
    assert!(!topics.is_empty());
    assert_eq!(topics.len(), 6); // Should have 6 topics as defined in the source
}

#[tokio::test]
async fn test_wsj_source_config_integration() {
    let context = setup_test_context().await;
    
    // Test various SourceConfig configurations
    let configs = vec![
        // Default configuration
        SourceConfig::new("https://feeds.a.dj.com/rss/{topic}.xml"),
        
        // Configuration with custom timeout
        SourceConfig::new("https://feeds.a.dj.com/rss/{topic}.xml")
            .with_timeout(45),
            
        // Configuration with custom user agent
        SourceConfig::new("https://feeds.a.dj.com/rss/{topic}.xml")
            .with_user_agent("WSJ Integration Test Client"),
            
        // Configuration with retry settings
        SourceConfig::new("https://feeds.a.dj.com/rss/{topic}.xml")
            .with_retries(3, 1500),
            
        // Comprehensive configuration
        SourceConfig::new("https://feeds.a.dj.com/rss/{topic}.xml")
            .with_timeout(30)
            .with_user_agent("WSJ Comprehensive Test")
            .with_retries(2, 1000),
    ];
    
    for (i, config) in configs.into_iter().enumerate() {
        println!("Testing WSJ configuration variant {}", i + 1);
        
        let wsj = WallStreetJournal::with_config(context.client.clone(), config.clone());
        
        // Validate configuration is properly applied
        assert_eq!(wsj.base_url(), config.base_url);
        assert_eq!(wsj.name(), "Wall Street Journal");
        
        // Test that the client can be used for basic operations
        let topics = wsj.available_topics();
        assert!(!topics.is_empty());
        
        // Test basic functionality with this configuration
        // Use a simple topic that's likely to work
        match wsj.opinions().await {
            Ok(articles) => {
                println!("  ✓ Configuration {} working ({} articles)", i + 1, articles.len());
                
                if !articles.is_empty() {
                    // Validate first article
                    let article = &articles[0];
                    assert_valid_news_article(article, false);
                    assert_eq!(article.source, Some("Wall Street Journal".to_string()));
                }
            }
            Err(e) => {
                println!("  ⚠ Configuration {} failed: {}", i + 1, e);
                // Don't fail the test for network issues, just log
            }
        }
    }
}

#[tokio::test]
async fn test_wsj_custom_configuration_handling() {
    let context = setup_test_context().await;
    
    // Test edge cases in configuration
    
    // Test with minimal configuration
    let minimal_config = SourceConfig::new("https://feeds.a.dj.com/rss/{topic}.xml");
    let wsj_minimal = WallStreetJournal::with_config(context.client.clone(), minimal_config);
    
    assert_eq!(wsj_minimal.name(), "Wall Street Journal");
    assert!(!wsj_minimal.available_topics().is_empty());
    
    // Test with maximum timeout configuration
    let max_timeout_config = SourceConfig::new("https://feeds.a.dj.com/rss/{topic}.xml")
        .with_timeout(120); // 2 minutes
    let wsj_max_timeout = WallStreetJournal::with_config(context.client.clone(), max_timeout_config);
    
    assert_eq!(wsj_max_timeout.name(), "Wall Street Journal");
    
    // Test with custom base URL (edge case)
    let custom_url_config = SourceConfig::new("https://custom.example.com/rss/{topic}.xml");
    let wsj_custom_url = WallStreetJournal::with_config(context.client.clone(), custom_url_config.clone());
    
    assert_eq!(wsj_custom_url.base_url(), custom_url_config.base_url);
    assert_eq!(wsj_custom_url.name(), "Wall Street Journal"); // Name should remain constant
    
    // Test configuration immutability
    let original_config = SourceConfig::new("https://feeds.a.dj.com/rss/{topic}.xml")
        .with_timeout(30);
    let wsj1 = WallStreetJournal::with_config(context.client.clone(), original_config.clone());
    let wsj2 = WallStreetJournal::with_config(context.client.clone(), original_config.clone());
    
    // Both instances should have the same configuration
    assert_eq!(wsj1.base_url(), wsj2.base_url());
    assert_eq!(wsj1.name(), wsj2.name());
}

#[tokio::test]
async fn test_wsj_config_vs_new_comparison() {
    let context = setup_test_context().await;
    
    // Create WSJ instance using new()
    let wsj_new = WallStreetJournal::new(context.client.clone());
    
    // Create WSJ instance using with_config() with default-like settings
    let default_config = SourceConfig::new("https://feeds.a.dj.com/rss/{topic}.xml");
    let wsj_config = WallStreetJournal::with_config(context.client.clone(), default_config);
    
    // Both should have the same basic properties
    assert_eq!(wsj_new.name(), wsj_config.name());
    assert_eq!(wsj_new.base_url(), wsj_config.base_url());
    
    // Both should have the same available topics
    let topics_new = wsj_new.available_topics();
    let topics_config = wsj_config.available_topics();
    assert_eq!(topics_new.len(), topics_config.len());
    
    for topic in &topics_new {
        assert!(topics_config.contains(topic));
    }
    
    // Test that both can fetch the same content (if network allows)
    match (wsj_new.opinions().await, wsj_config.opinions().await) {
        (Ok(articles_new), Ok(articles_config)) => {
            println!("Both new() and with_config() successfully fetched opinions");
            println!("  new(): {} articles", articles_new.len());
            println!("  with_config(): {} articles", articles_config.len());
            
            // Both should return articles with proper source attribution
            if !articles_new.is_empty() {
                assert_eq!(articles_new[0].source, Some("Wall Street Journal".to_string()));
            }
            if !articles_config.is_empty() {
                assert_eq!(articles_config[0].source, Some("Wall Street Journal".to_string()));
            }
        }
        (Err(e1), Err(e2)) => {
            println!("Both methods failed (likely network issue): {} / {}", e1, e2);
        }
        (Ok(_), Err(e)) => {
            println!("new() succeeded but with_config() failed: {}", e);
        }
        (Err(e), Ok(_)) => {
            println!("with_config() succeeded but new() failed: {}", e);
        }
    }
}

// Task 7: Implement Wall Street Journal integration tests

#[tokio::test]
async fn test_wsj_basic_functionality() {
    let context = setup_test_context().await;
    let wsj = WallStreetJournal::new(context.client.clone());

    // Test name() function
    assert_eq!(wsj.name(), "Wall Street Journal");

    // Test base_url() function
    let base_url = wsj.base_url();
    assert!(!base_url.is_empty());
    assert!(base_url.contains("feeds.a.dj.com"));
    assert!(base_url.contains("{topic}"));

    // Test available_topics() function
    let topics = wsj.available_topics();
    assert!(!topics.is_empty());
    assert_eq!(topics.len(), 6); // WSJ has 6 defined topics

    // Verify expected topics are present
    let expected_topics = vec![
        "RSSOpinion",
        "RSSWorldNews",
        "WSJcomUSBusiness", 
        "RSSMarketsMain",
        "RSSWSJD",
        "RSSLifestyle",
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
async fn test_wsj_opinions() {
    let context = setup_test_context().await;
    let wsj = WallStreetJournal::new(context.client.clone());

    let result = test_function_with_validation("opinions", || wsj.opinions(), &context).await;

    assert!(
        result.success,
        "opinions() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_wsj_world_news() {
    let context = setup_test_context().await;
    let wsj = WallStreetJournal::new(context.client.clone());

    let result = test_function_with_validation("world_news", || wsj.world_news(), &context).await;

    assert!(
        result.success,
        "world_news() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_wsj_us_business_news() {
    let context = setup_test_context().await;
    let wsj = WallStreetJournal::new(context.client.clone());

    let result = test_function_with_validation("us_business_news", || wsj.us_business_news(), &context).await;

    assert!(
        result.success,
        "us_business_news() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_wsj_market_news() {
    let context = setup_test_context().await;
    let wsj = WallStreetJournal::new(context.client.clone());

    let result = test_function_with_validation("market_news", || wsj.market_news(), &context).await;

    assert!(
        result.success,
        "market_news() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_wsj_technology_news() {
    let context = setup_test_context().await;
    let wsj = WallStreetJournal::new(context.client.clone());

    let result = test_function_with_validation("technology_news", || wsj.technology_news(), &context).await;

    assert!(
        result.success,
        "technology_news() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_wsj_lifestyle() {
    let context = setup_test_context().await;
    let wsj = WallStreetJournal::new(context.client.clone());

    let result = test_function_with_validation("lifestyle", || wsj.lifestyle(), &context).await;

    assert!(
        result.success,
        "lifestyle() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_wsj_all_public_methods_comprehensive() {
    let context = setup_test_context().await;
    let wsj = WallStreetJournal::new(context.client.clone());

    let mut successful_methods = 0;
    let mut failed_methods = Vec::new();

    // Test opinions
    println!("Testing WSJ method: opinions");
    let result = test_function_with_validation("opinions", || wsj.opinions(), &context).await;
    if result.success {
        successful_methods += 1;
        println!("  ✓ opinions returned {} articles", result.article_count);
    } else {
        failed_methods.push(("opinions", result.error_message.unwrap_or_default()));
        println!("  ✗ opinions failed: {:?}", failed_methods.last().unwrap().1);
    }

    // Test world_news
    println!("Testing WSJ method: world_news");
    let result = test_function_with_validation("world_news", || wsj.world_news(), &context).await;
    if result.success {
        successful_methods += 1;
        println!("  ✓ world_news returned {} articles", result.article_count);
    } else {
        failed_methods.push(("world_news", result.error_message.unwrap_or_default()));
        println!("  ✗ world_news failed: {:?}", failed_methods.last().unwrap().1);
    }

    // Test us_business_news
    println!("Testing WSJ method: us_business_news");
    let result = test_function_with_validation("us_business_news", || wsj.us_business_news(), &context).await;
    if result.success {
        successful_methods += 1;
        println!("  ✓ us_business_news returned {} articles", result.article_count);
    } else {
        failed_methods.push(("us_business_news", result.error_message.unwrap_or_default()));
        println!("  ✗ us_business_news failed: {:?}", failed_methods.last().unwrap().1);
    }

    // Test market_news
    println!("Testing WSJ method: market_news");
    let result = test_function_with_validation("market_news", || wsj.market_news(), &context).await;
    if result.success {
        successful_methods += 1;
        println!("  ✓ market_news returned {} articles", result.article_count);
    } else {
        failed_methods.push(("market_news", result.error_message.unwrap_or_default()));
        println!("  ✗ market_news failed: {:?}", failed_methods.last().unwrap().1);
    }

    // Test technology_news
    println!("Testing WSJ method: technology_news");
    let result = test_function_with_validation("technology_news", || wsj.technology_news(), &context).await;
    if result.success {
        successful_methods += 1;
        println!("  ✓ technology_news returned {} articles", result.article_count);
    } else {
        failed_methods.push(("technology_news", result.error_message.unwrap_or_default()));
        println!("  ✗ technology_news failed: {:?}", failed_methods.last().unwrap().1);
    }

    // Test lifestyle
    println!("Testing WSJ method: lifestyle");
    let result = test_function_with_validation("lifestyle", || wsj.lifestyle(), &context).await;
    if result.success {
        successful_methods += 1;
        println!("  ✓ lifestyle returned {} articles", result.article_count);
    } else {
        failed_methods.push(("lifestyle", result.error_message.unwrap_or_default()));
        println!("  ✗ lifestyle failed: {:?}", failed_methods.last().unwrap().1);
    }

    println!("\n=== WSJ METHOD TEST SUMMARY ===");
    println!("Successful methods: {}/6", successful_methods);
    println!("Failed methods: {}/6", failed_methods.len());
    
    if !failed_methods.is_empty() {
        println!("Failed methods:");
        for (method, error) in &failed_methods {
            println!("  - {}: {}", method, error);
        }
    }

    // We expect at least half of the methods to work
    assert!(
        successful_methods >= 3,
        "Expected at least 3/6 WSJ methods to work, got {}/6 successful. Failed methods: {:?}",
        successful_methods,
        failed_methods
    );
}

#[tokio::test]
async fn test_wsj_data_validation() {
    let context = setup_test_context().await;
    let wsj = WallStreetJournal::new(context.client.clone());

    // Test with opinions as it's likely to have good data
    match wsj.opinions().await {
        Ok(articles) => {
            assert!(!articles.is_empty(), "Should receive at least one article");

            // Validate article structure integrity
            for (i, article) in articles.iter().take(5).enumerate() {
                println!("Validating WSJ article {}: {:?}", i + 1, article.title);

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
                    Some("Wall Street Journal".to_string()),
                    "Article source should be set to Wall Street Journal"
                );
            }
        }
        Err(e) => {
            panic!("Failed to fetch WSJ opinions for validation: {}", e);
        }
    }
}

#[tokio::test]
async fn test_wsj_fetch_feed_with_topics() {
    let context = setup_test_context().await;
    let wsj = WallStreetJournal::new(context.client.clone());

    // Test fetch_feed method directly with all available topics
    let topics = wsj.available_topics();
    
    let mut successful_topics = 0;
    let mut failed_topics = Vec::new();

    for &topic in &topics {
        println!("Testing WSJ topic: {}", topic);
        
        match wsj.fetch_feed(topic).await {
            Ok(articles) => {
                successful_topics += 1;
                println!("  ✓ Topic '{}' returned {} articles", topic, articles.len());

                if !articles.is_empty() {
                    // Validate first article from each successful topic
                    let article = &articles[0];
                    assert_valid_news_article(article, false);

                    // Ensure source is properly set
                    assert_eq!(article.source, Some("Wall Street Journal".to_string()));
                }
            }
            Err(e) => {
                failed_topics.push((topic, e.to_string()));
                println!("  ✗ Topic '{}' failed: {}", topic, e);
            }
        }
    }

    println!("\n=== WSJ TOPIC TEST SUMMARY ===");
    println!("Successful topics: {}/{}", successful_topics, topics.len());
    println!("Failed topics: {}/{}", failed_topics.len(), topics.len());

    // We expect at least half of the topics to work
    assert!(
        successful_topics >= topics.len() / 2,
        "Expected at least half of WSJ topics to work, got {}/{} successful. Failed topics: {:?}",
        successful_topics,
        topics.len(),
        failed_topics
    );
}

#[tokio::test]
async fn test_wsj_publication_date_format() {
    let context = setup_test_context().await;
    let wsj = WallStreetJournal::new(context.client.clone());

    match wsj.market_news().await {
        Ok(articles) => {
            let articles_with_dates: Vec<_> = articles
                .iter()
                .filter(|article| article.pub_date.is_some())
                .collect();

            if !articles_with_dates.is_empty() {
                for article in articles_with_dates.iter().take(3) {
                    if let Some(ref pub_date) = article.pub_date {
                        assert!(
                            !pub_date.trim().is_empty(),
                            "Publication date should not be empty"
                        );

                        // Basic validation - should contain some date-like patterns
                        let date_lower = pub_date.to_lowercase();
                        let has_date_indicators = date_lower.contains("mon")
                            || date_lower.contains("tue")
                            || date_lower.contains("wed")
                            || date_lower.contains("thu")
                            || date_lower.contains("fri")
                            || date_lower.contains("sat")
                            || date_lower.contains("sun")
                            || date_lower.contains("jan")
                            || date_lower.contains("feb")
                            || date_lower.contains("mar")
                            || date_lower.contains("apr")
                            || date_lower.contains("may")
                            || date_lower.contains("jun")
                            || date_lower.contains("jul")
                            || date_lower.contains("aug")
                            || date_lower.contains("sep")
                            || date_lower.contains("oct")
                            || date_lower.contains("nov")
                            || date_lower.contains("dec")
                            || pub_date.chars().any(|c| c.is_ascii_digit());

                        assert!(
                            has_date_indicators,
                            "Publication date '{}' should contain recognizable date patterns",
                            pub_date
                        );
                    }
                }
            }
        }
        Err(e) => {
            println!(
                "Warning: Could not fetch WSJ articles for date validation: {}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_wsj_url_construction() {
    let context = setup_test_context().await;
    let wsj = WallStreetJournal::new(context.client.clone());

    let base_url = wsj.base_url();
    let topics = wsj.available_topics();

    // Test URL construction for each topic
    for &topic in &topics {
        let constructed_url = base_url.replace("{topic}", topic);
        
        println!("Testing WSJ URL construction for topic '{}': {}", topic, constructed_url);
        
        // Validate URL format
        assert_valid_url(&constructed_url);
        assert!(constructed_url.contains("feeds.a.dj.com"));
        assert!(constructed_url.contains(topic));
        assert!(constructed_url.ends_with(".xml"));
        
        // Test that the URL doesn't contain the placeholder anymore
        assert!(!constructed_url.contains("{topic}"));
    }
}

// Task 7.2: Create WSJ endpoint monitoring

#[tokio::test]
async fn test_wsj_endpoint_availability_monitoring() {
    let context = setup_test_context().await;
    let wsj = WallStreetJournal::new(context.client.clone());
    let mut deprecation_tracker = DeprecationTracker::new();

    let topics = wsj.available_topics();
    let base_url = wsj.base_url();
    
    println!("=== WSJ ENDPOINT AVAILABILITY MONITORING ===");
    println!("Base URL: {}", base_url);
    println!("Testing {} topics for availability", topics.len());

    let mut available_endpoints = 0;
    let mut deprecated_endpoints = 0;
    let mut error_endpoints = 0;

    for &topic in &topics {
        let constructed_url = base_url.replace("{topic}", topic);
        println!("\nTesting endpoint: {} -> {}", topic, constructed_url);

        match wsj.fetch_feed(topic).await {
            Ok(articles) => {
                available_endpoints += 1;
                println!("  ✓ Available - {} articles returned", articles.len());
                
                // Validate URL construction worked correctly
                assert_valid_url(&constructed_url);
                assert!(!constructed_url.contains("{topic}"));
                
                // If we got articles, validate they have proper source attribution
                if !articles.is_empty() {
                    assert_eq!(articles[0].source, Some("Wall Street Journal".to_string()));
                }
            }
            Err(e) => {
                println!("  ✗ Failed - {}", e);
                
                // Record failure for deprecation tracking
                deprecation_tracker.record_failure_with_url(
                    "Wall Street Journal",
                    &format!("fetch_feed({})", topic),
                    &constructed_url,
                    &e,
                );

                // Classify the error type
                let error_msg = e.to_string().to_lowercase();
                if error_msg.contains("404") || error_msg.contains("not found") {
                    deprecated_endpoints += 1;
                    println!("    → Likely DEPRECATED (404 Not Found)");
                } else if error_msg.contains("403") || error_msg.contains("forbidden") {
                    deprecated_endpoints += 1;
                    println!("    → Likely DEPRECATED (403 Forbidden)");
                } else if error_msg.contains("timeout") || error_msg.contains("timed out") {
                    error_endpoints += 1;
                    println!("    → Network timeout (may be temporary)");
                } else if error_msg.contains("dns") || error_msg.contains("resolve") {
                    deprecated_endpoints += 1;
                    println!("    → DNS resolution failed (likely deprecated)");
                } else {
                    error_endpoints += 1;
                    println!("    → Other error (may be temporary)");
                }
            }
        }
    }

    println!("\n=== WSJ ENDPOINT MONITORING SUMMARY ===");
    println!("Available endpoints: {}/{}", available_endpoints, topics.len());
    println!("Deprecated endpoints: {}/{}", deprecated_endpoints, topics.len());
    println!("Error endpoints: {}/{}", error_endpoints, topics.len());

    // Generate deprecation report
    let report = deprecation_tracker.generate_report();
    if !report.deprecated_endpoints.is_empty() || !report.removal_candidates.is_empty() {
        println!("\n=== DEPRECATION REPORT ===");
        println!("{}", report);
    }

    // We expect at least half of the endpoints to be available
    assert!(
        available_endpoints >= topics.len() / 2,
        "Expected at least half of WSJ endpoints to be available, got {}/{} available",
        available_endpoints,
        topics.len()
    );

    // If we have too many deprecated endpoints, warn about it
    if deprecated_endpoints > topics.len() / 2 {
        println!(
            "WARNING: More than half of WSJ endpoints appear deprecated ({}/{})",
            deprecated_endpoints, topics.len()
        );
    }
}

#[tokio::test]
async fn test_wsj_configuration_url_generation_monitoring() {
    let context = setup_test_context().await;
    let mut deprecation_tracker = DeprecationTracker::new();

    println!("=== WSJ CONFIGURATION URL GENERATION MONITORING ===");

    // Test various configuration scenarios
    let test_configs = vec![
        // Default configuration
        ("default", SourceConfig::new("https://feeds.a.dj.com/rss/{topic}.xml")),
        
        // Configuration with different base URL patterns
        ("alt_pattern", SourceConfig::new("https://feeds.a.dj.com/rss/{topic}.xml")),
        
        // Configuration with custom timeout
        ("custom_timeout", SourceConfig::new("https://feeds.a.dj.com/rss/{topic}.xml").with_timeout(45)),
        
        // Configuration with custom user agent
        ("custom_ua", SourceConfig::new("https://feeds.a.dj.com/rss/{topic}.xml")
            .with_user_agent("WSJ Endpoint Monitor")),
        
        // Configuration with retry settings
        ("with_retries", SourceConfig::new("https://feeds.a.dj.com/rss/{topic}.xml")
            .with_retries(2, 1000)),
    ];

    for (config_name, config) in test_configs {
        println!("\nTesting configuration: {}", config_name);
        println!("  Base URL: {}", config.base_url);
        
        let wsj = WallStreetJournal::with_config(context.client.clone(), config.clone());
        
        // Test URL generation for each topic
        let topics = wsj.available_topics();
        let mut config_successful = 0;
        let mut config_failed = 0;

        for &topic in &topics {
            let constructed_url = config.base_url.replace("{topic}", topic);
            
            // Validate URL construction
            if constructed_url.contains("{topic}") {
                println!("    ✗ URL construction failed for topic '{}': still contains placeholder", topic);
                config_failed += 1;
                continue;
            }

            // Validate URL format (basic validation)
            if constructed_url.starts_with("https://") && constructed_url.contains(".") {
                println!("    ✓ Valid URL generated for '{}': {}", topic, constructed_url);
                config_successful += 1;
            } else {
                println!("    ✗ Invalid URL generated for '{}': {}", topic, constructed_url);
                config_failed += 1;
                
                // Create a simple error for tracking
                let url_error = std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid URL format: {}", constructed_url)
                );
                
                deprecation_tracker.record_failure_with_url(
                    "Wall Street Journal",
                    &format!("url_generation_{}_{}", config_name, topic),
                    &constructed_url,
                    &url_error,
                );
            }
        }

        println!("  Configuration '{}' results: {}/{} URLs generated successfully", 
                config_name, config_successful, topics.len());

        // Test actual endpoint functionality with this configuration
        println!("  Testing endpoint functionality with configuration '{}'...", config_name);
        match wsj.opinions().await {
            Ok(articles) => {
                println!("    ✓ Configuration '{}' works - {} articles fetched", config_name, articles.len());
            }
            Err(e) => {
                println!("    ✗ Configuration '{}' failed: {}", config_name, e);
                deprecation_tracker.record_failure(
                    "Wall Street Journal",
                    &format!("config_test_{}", config_name),
                    &e,
                );
            }
        }
    }

    // Test edge cases in URL generation
    println!("\n=== TESTING URL GENERATION EDGE CASES ===");
    
    // Test with malformed base URL
    let malformed_config = SourceConfig::new("not-a-valid-url/{topic}");
    let wsj_malformed = WallStreetJournal::with_config(context.client.clone(), malformed_config);
    
    for &topic in wsj_malformed.available_topics().iter().take(2) {
        let constructed_url = wsj_malformed.base_url().replace("{topic}", topic);
        println!("Testing malformed URL construction: {}", constructed_url);
        
        if constructed_url.starts_with("https://") && constructed_url.contains(".") {
            println!("  Unexpectedly valid URL: {}", constructed_url);
        } else {
            println!("  ✓ Correctly detected malformed URL: {}", constructed_url);
        }
    }

    // Test with missing topic placeholder
    let no_placeholder_config = SourceConfig::new("https://feeds.a.dj.com/rss/fixed.xml");
    let wsj_no_placeholder = WallStreetJournal::with_config(context.client.clone(), no_placeholder_config);
    
    println!("Testing URL without topic placeholder: {}", wsj_no_placeholder.base_url());
    for &topic in wsj_no_placeholder.available_topics().iter().take(2) {
        let constructed_url = wsj_no_placeholder.base_url().replace("{topic}", topic);
        println!("  Topic '{}' -> {}", topic, constructed_url);
        
        // Should be the same URL regardless of topic since no placeholder
        assert_eq!(constructed_url, wsj_no_placeholder.base_url());
    }

    // Generate final report
    let report = deprecation_tracker.generate_report();
    if report.total_failures > 0 {
        println!("\n=== CONFIGURATION MONITORING REPORT ===");
        println!("{}", report);
    } else {
        println!("\n✓ All WSJ configuration URL generation tests passed");
    }
}

#[tokio::test]
async fn test_wsj_deprecated_topic_detection() {
    let context = setup_test_context().await;
    let wsj = WallStreetJournal::new(context.client.clone());
    let mut deprecation_tracker = DeprecationTracker::new();

    println!("=== WSJ DEPRECATED TOPIC DETECTION ===");

    // Test known good topics first
    let known_topics = wsj.available_topics();
    println!("Testing {} known topics for deprecation", known_topics.len());

    let mut working_topics = Vec::new();
    let mut deprecated_topics = Vec::new();

    for &topic in &known_topics {
        println!("Testing topic: {}", topic);
        
        match wsj.fetch_feed(topic).await {
            Ok(articles) => {
                working_topics.push(topic);
                println!("  ✓ Working - {} articles", articles.len());
            }
            Err(e) => {
                let error_msg = e.to_string().to_lowercase();
                
                // Check if this looks like a deprecated endpoint
                if error_msg.contains("404") || error_msg.contains("not found") ||
                   error_msg.contains("403") || error_msg.contains("forbidden") {
                    deprecated_topics.push(topic);
                    println!("  ✗ DEPRECATED - {}", e);
                    
                    deprecation_tracker.record_failure(
                        "Wall Street Journal",
                        &format!("deprecated_topic_{}", topic),
                        &e,
                    );
                } else {
                    println!("  ⚠ Error (may be temporary) - {}", e);
                }
            }
        }
    }

    // Test some potentially deprecated topics that might have existed historically
    let potentially_deprecated_topics = vec![
        "RSSMarketsMainOld",
        "RSSOpinionOld", 
        "WSJcomUSBusinessOld",
        "RSSWorldNewsOld",
        "RSSLifestyleOld",
        "RSSMarketsLegacy",
        "WSJcomTechOld",
        "RSSPersonalFinance",
        "RSSRealEstate",
        "RSSCareerJournal",
    ];

    println!("\nTesting {} potentially deprecated topics", potentially_deprecated_topics.len());
    
    let mut confirmed_deprecated = 0;
    
    for topic in &potentially_deprecated_topics {
        println!("Testing potentially deprecated topic: {}", topic);
        
        match wsj.fetch_feed(topic).await {
            Ok(articles) => {
                println!("  ✓ Unexpectedly working - {} articles", articles.len());
                println!("    → This topic might need to be added to available_topics()");
            }
            Err(e) => {
                let error_msg = e.to_string().to_lowercase();
                
                if error_msg.contains("404") || error_msg.contains("not found") {
                    confirmed_deprecated += 1;
                    println!("  ✓ Confirmed deprecated (404) - {}", topic);
                } else if error_msg.contains("403") || error_msg.contains("forbidden") {
                    confirmed_deprecated += 1;
                    println!("  ✓ Confirmed deprecated (403) - {}", topic);
                } else {
                    println!("  ? Other error - {}", e);
                }
                
                deprecation_tracker.record_failure(
                    "Wall Street Journal",
                    &format!("test_deprecated_{}", topic),
                    &e,
                );
            }
        }
    }

    println!("\n=== WSJ TOPIC DEPRECATION SUMMARY ===");
    println!("Working topics: {}/{}", working_topics.len(), known_topics.len());
    println!("Deprecated known topics: {}/{}", deprecated_topics.len(), known_topics.len());
    println!("Confirmed deprecated test topics: {}/{}", confirmed_deprecated, potentially_deprecated_topics.len());

    // Generate deprecation report
    let report = deprecation_tracker.generate_report();
    println!("\n=== TOPIC DEPRECATION REPORT ===");
    println!("{}", report);

    // Recommendations based on findings
    if !deprecated_topics.is_empty() {
        println!("\n=== REMOVAL RECOMMENDATIONS ===");
        println!("The following topics from available_topics() appear to be deprecated:");
        for topic in &deprecated_topics {
            println!("  - Remove '{}' from available_topics() in wsj.rs", topic);
        }
    }

    if working_topics.len() < known_topics.len() / 2 {
        println!("\nWARNING: More than half of known WSJ topics appear deprecated!");
        println!("This may indicate a major change in WSJ's RSS feed structure.");
    }

    // We expect at least some topics to be working
    assert!(
        !working_topics.is_empty(),
        "Expected at least one WSJ topic to be working, but all {} topics failed",
        known_topics.len()
    );
}

#[tokio::test]
async fn test_wsj_rss_feed_availability_comprehensive() {
    let context = setup_test_context().await;
    let wsj = WallStreetJournal::new(context.client.clone());
    let mut deprecation_tracker = DeprecationTracker::new();

    println!("=== WSJ RSS FEED AVAILABILITY COMPREHENSIVE TEST ===");

    // Test base URL accessibility
    let base_url_template = wsj.base_url();
    println!("Base URL template: {}", base_url_template);

    // Validate base URL format
    assert!(base_url_template.contains("{topic}"), "Base URL should contain topic placeholder");
    assert!(base_url_template.starts_with("https://"), "Base URL should use HTTPS");
    assert!(base_url_template.contains("feeds.a.dj.com"), "Base URL should point to WSJ feeds domain");

    // Test each topic endpoint systematically
    let topics = wsj.available_topics();
    let mut endpoint_status = std::collections::HashMap::new();

    for &topic in &topics {
        let url = base_url_template.replace("{topic}", topic);
        println!("\nTesting RSS feed: {} -> {}", topic, url);

        // Test URL validity first (basic validation)
        if url.starts_with("https://") && url.contains(".") && url.contains("feeds.a.dj.com") {
            println!("  ✓ Valid URL format");
            println!("    URL: {}", url);
        } else {
            println!("  ✗ Invalid URL format: {}", url);
            endpoint_status.insert(topic, "INVALID_URL");
            continue;
        }

        // Test actual feed fetching
        let start_time = Instant::now();
        match wsj.fetch_feed(topic).await {
            Ok(articles) => {
                let duration = start_time.elapsed();
                println!("  ✓ Feed accessible - {} articles in {:?}", articles.len(), duration);
                
                // Analyze feed quality
                if articles.is_empty() {
                    println!("    ⚠ Warning: Feed returned no articles");
                    endpoint_status.insert(topic, "EMPTY_FEED");
                } else {
                    // Check article quality
                    let articles_with_titles = articles.iter().filter(|a| a.title.is_some()).count();
                    let articles_with_links = articles.iter().filter(|a| a.link.is_some()).count();
                    
                    println!("    Articles with titles: {}/{}", articles_with_titles, articles.len());
                    println!("    Articles with links: {}/{}", articles_with_links, articles.len());
                    
                    if articles_with_titles == 0 {
                        println!("    ⚠ Warning: No articles have titles");
                        endpoint_status.insert(topic, "NO_TITLES");
                    } else if articles_with_links == 0 {
                        println!("    ⚠ Warning: No articles have links");
                        endpoint_status.insert(topic, "NO_LINKS");
                    } else {
                        endpoint_status.insert(topic, "HEALTHY");
                    }

                    // Validate first article in detail
                    if let Some(article) = articles.first() {
                        println!("    Sample article:");
                        if let Some(ref title) = article.title {
                            println!("      Title: {}", title.chars().take(60).collect::<String>());
                        }
                        if let Some(ref link) = article.link {
                            println!("      Link: {}", link);
                        }
                        println!("      Source: {:?}", article.source);
                    }
                }
            }
            Err(e) => {
                let duration = start_time.elapsed();
                println!("  ✗ Feed failed in {:?}: {}", duration, e);
                
                // Classify the error for deprecation tracking
                let error_msg = e.to_string().to_lowercase();
                let status = if error_msg.contains("404") || error_msg.contains("not found") {
                    "DEPRECATED_404"
                } else if error_msg.contains("403") || error_msg.contains("forbidden") {
                    "DEPRECATED_403"
                } else if error_msg.contains("timeout") {
                    "TIMEOUT"
                } else if error_msg.contains("dns") {
                    "DNS_ERROR"
                } else {
                    "OTHER_ERROR"
                };
                
                endpoint_status.insert(topic, status);
                
                deprecation_tracker.record_failure_with_url(
                    "Wall Street Journal",
                    &format!("rss_availability_{}", topic),
                    &url,
                    &e,
                );
            }
        }
    }

    // Generate comprehensive status report
    println!("\n=== WSJ RSS FEED STATUS SUMMARY ===");
    let mut status_counts = std::collections::HashMap::new();
    
    for (topic, status) in &endpoint_status {
        *status_counts.entry(status).or_insert(0) += 1;
        println!("  {}: {}", topic, status);
    }

    println!("\n=== STATUS DISTRIBUTION ===");
    for (status, count) in &status_counts {
        println!("  {}: {} endpoints", status, count);
    }

    // Generate deprecation report
    let report = deprecation_tracker.generate_report();
    if report.total_failures > 0 {
        println!("\n=== RSS AVAILABILITY REPORT ===");
        println!("{}", report);
    }

    // Health check assertions
    let healthy_count = status_counts.get(&"HEALTHY").unwrap_or(&0);
    let total_count = topics.len();
    
    println!("\n=== HEALTH ASSESSMENT ===");
    println!("Healthy endpoints: {}/{}", healthy_count, total_count);
    
    if *healthy_count == 0 {
        panic!("No WSJ RSS feeds are healthy - this indicates a major service issue");
    } else if *healthy_count < total_count / 2 {
        println!("WARNING: Less than half of WSJ RSS feeds are healthy ({}/{})", healthy_count, total_count);
        println!("This may indicate service degradation or API changes");
    } else {
        println!("✓ WSJ RSS feed availability is acceptable ({}/{} healthy)", healthy_count, total_count);
    }

    // Ensure we have at least one working endpoint
    assert!(
        *healthy_count > 0,
        "Expected at least one healthy WSJ RSS endpoint, but found none"
    );
}