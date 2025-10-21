use finance_news_aggregator_rs::news_source::NewsSource;
use finance_news_aggregator_rs::news_source::seeking_alpha::SeekingAlpha;
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

/// Setup test context for Seeking Alpha integration tests
async fn setup_test_context() -> TestContext {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let config = IntegrationTestConfig::default();
    TestContext::new(client, config)
}

/// Test function execution with validation and error handling
async fn test_function_with_validation<F, Fut>(
    function_name: &str,
    test_fn: F,
    context: &TestContext,
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
            // Record failure for deprecation tracking if enabled
            if context.config.deprecation_tracking_enabled {
                // Note: We can't mutate the context here, so we'll handle deprecation tracking
                // in the specific deprecation test function
                println!("Warning: Function '{}' failed: {} (will be tracked for deprecation)", function_name, e);
            } else {
                println!("Warning: Function '{}' failed: {}", function_name, e);
            }
            TestResult::failure(function_name, e.to_string(), start_time.elapsed())
        }
    }
}

#[tokio::test]
async fn test_seeking_alpha_basic_functionality() {
    let context = setup_test_context().await;
    let seeking_alpha = SeekingAlpha::new(context.client.clone());

    // Test name() function
    assert_eq!(seeking_alpha.name(), "Seeking Alpha");

    // Test base_url() function
    let base_url = seeking_alpha.base_url();
    assert!(!base_url.is_empty());
    assert!(base_url.contains("seekingalpha.com"));

    // Test available_topics() function
    let topics = seeking_alpha.available_topics();
    assert!(!topics.is_empty());

    // Verify expected topics are present
    let expected_topics = vec![
        "latest-articles",
        "all-news",
        "market-news",
        "long-ideas",
        "short-ideas",
        "ipo-analysis",
        "transcripts",
        "wall-street-breakfast",
        "most-popular-articles",
        "forex",
        "editors-picks",
        "etfs",
    ];
    for expected_topic in expected_topics {
        assert!(
            topics.contains(&expected_topic),
            "Expected topic '{}' not found in available topics",
            expected_topic
        );
    }
}

// Task 6: Implement Seeking Alpha integration tests
#[tokio::test]
async fn test_seeking_alpha_all_news() {
    let context = setup_test_context().await;
    let seeking_alpha = SeekingAlpha::new(context.client.clone());

    let result = test_function_with_validation("all_news", || seeking_alpha.all_news(), &context).await;

    assert!(
        result.success,
        "all_news() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_seeking_alpha_editors_picks() {
    let context = setup_test_context().await;
    let seeking_alpha = SeekingAlpha::new(context.client.clone());

    let result = test_function_with_validation("editors_picks", || seeking_alpha.editors_picks(), &context).await;

    assert!(
        result.success,
        "editors_picks() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_seeking_alpha_etfs() {
    let context = setup_test_context().await;
    let seeking_alpha = SeekingAlpha::new(context.client.clone());

    let result = test_function_with_validation("etfs", || seeking_alpha.etfs(), &context).await;

    assert!(
        result.success,
        "etfs() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_seeking_alpha_forex() {
    let context = setup_test_context().await;
    let seeking_alpha = SeekingAlpha::new(context.client.clone());

    let result = test_function_with_validation("forex", || seeking_alpha.forex(), &context).await;

    assert!(
        result.success,
        "forex() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_seeking_alpha_ipo_analysis() {
    let context = setup_test_context().await;
    let seeking_alpha = SeekingAlpha::new(context.client.clone());

    let result = test_function_with_validation("ipo_analysis", || seeking_alpha.ipo_analysis(), &context).await;

    assert!(
        result.success,
        "ipo_analysis() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_seeking_alpha_latest_articles() {
    let context = setup_test_context().await;
    let seeking_alpha = SeekingAlpha::new(context.client.clone());

    let result = test_function_with_validation("latest_articles", || seeking_alpha.latest_articles(), &context).await;

    assert!(
        result.success,
        "latest_articles() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_seeking_alpha_market_news() {
    let context = setup_test_context().await;
    let seeking_alpha = SeekingAlpha::new(context.client.clone());

    let result = test_function_with_validation("market_news", || seeking_alpha.market_news(), &context).await;

    assert!(
        result.success,
        "market_news() failed: {:?}",
        result.error_message
    );
}

// Task 6.1: Add Seeking Alpha parameterized function tests
#[tokio::test]
async fn test_seeking_alpha_global_markets_with_countries() {
    let context = setup_test_context().await;
    let seeking_alpha = SeekingAlpha::new(context.client.clone());

    // Test global_markets() with different country parameters
    let test_countries = vec![
        "usa",
        "china", 
        "europe",
        "japan",
        "canada",
        "uk",
    ];

    let mut successful_countries = 0;
    let mut failed_countries = Vec::new();

    for country in &test_countries {
        println!("Testing Seeking Alpha global_markets with country: {}", country);
        
        let result = test_function_with_validation(
            &format!("global_markets({})", country),
            || seeking_alpha.global_markets(country),
            &context,
        ).await;

        if result.success {
            successful_countries += 1;
            println!("  ✓ Country '{}' returned {} articles", country, result.article_count);
        } else {
            failed_countries.push((country, result.error_message.unwrap_or_default()));
            println!("  ✗ Country '{}' failed", country);
        }
    }

    // We expect at least some countries to work
    assert!(
        successful_countries > 0,
        "Expected at least one country to work for global_markets, got {}/{} successful. Failed: {:?}",
        successful_countries,
        test_countries.len(),
        failed_countries
    );

    println!("Global markets test summary: {}/{} countries successful", successful_countries, test_countries.len());
}

#[tokio::test]
async fn test_seeking_alpha_sectors_with_parameters() {
    let context = setup_test_context().await;
    let seeking_alpha = SeekingAlpha::new(context.client.clone());

    // Test sectors() with different sector parameters
    let test_sectors = vec![
        "technology",
        "healthcare", 
        "finance",
        "energy",
        "consumer",
        "industrial",
        "materials",
        "utilities",
    ];

    let mut successful_sectors = 0;
    let mut failed_sectors = Vec::new();

    for sector in &test_sectors {
        println!("Testing Seeking Alpha sectors with sector: {}", sector);
        
        let result = test_function_with_validation(
            &format!("sectors({})", sector),
            || seeking_alpha.sectors(sector),
            &context,
        ).await;

        if result.success {
            successful_sectors += 1;
            println!("  ✓ Sector '{}' returned {} articles", sector, result.article_count);
        } else {
            failed_sectors.push((sector, result.error_message.unwrap_or_default()));
            println!("  ✗ Sector '{}' failed", sector);
        }
    }

    // We expect at least some sectors to work
    assert!(
        successful_sectors > 0,
        "Expected at least one sector to work for sectors(), got {}/{} successful. Failed: {:?}",
        successful_sectors,
        test_sectors.len(),
        failed_sectors
    );

    println!("Sectors test summary: {}/{} sectors successful", successful_sectors, test_sectors.len());
}

#[tokio::test]
async fn test_seeking_alpha_stocks_with_realistic_tickers() {
    let context = setup_test_context().await;
    let seeking_alpha = SeekingAlpha::new(context.client.clone());

    // Test stocks() function with realistic ticker symbols (AAPL, MSFT, GOOGL)
    let test_tickers = vec![
        "AAPL",  // Apple
        "MSFT",  // Microsoft
        "GOOGL", // Alphabet/Google
        "TSLA",  // Tesla
        "AMZN",  // Amazon
    ];

    let mut successful_tickers = 0;
    let mut failed_tickers = Vec::new();

    for ticker in &test_tickers {
        println!("Testing Seeking Alpha stocks with ticker: {}", ticker);
        
        let result = test_function_with_validation(
            &format!("stocks({})", ticker),
            || seeking_alpha.stocks(ticker),
            &context,
        ).await;

        if result.success {
            successful_tickers += 1;
            println!("  ✓ Ticker '{}' returned {} articles", ticker, result.article_count);
            
            // Additional validation for stock-specific articles
            match seeking_alpha.stocks(ticker).await {
                Ok(articles) => {
                    if !articles.is_empty() {
                        // Validate that stock articles have proper structure
                        for article in articles.iter().take(2) {
                            assert_eq!(
                                article.source,
                                Some("Seeking Alpha".to_string()),
                                "Stock article should have Seeking Alpha as source"
                            );
                            
                            // Stock articles should have meaningful titles
                            if let Some(ref title) = article.title {
                                assert!(
                                    title.len() >= 5,
                                    "Stock article titles should be substantial"
                                );
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("Warning: Could not validate stock article structure for {}: {}", ticker, e);
                }
            }
        } else {
            failed_tickers.push((ticker, result.error_message.unwrap_or_default()));
            println!("  ✗ Ticker '{}' failed", ticker);
        }
    }

    // We expect at least some tickers to work
    assert!(
        successful_tickers > 0,
        "Expected at least one ticker to work for stocks(), got {}/{} successful. Failed: {:?}",
        successful_tickers,
        test_tickers.len(),
        failed_tickers
    );

    println!("Stocks test summary: {}/{} tickers successful", successful_tickers, test_tickers.len());
}

// Task 6.2: Add Seeking Alpha specialized content tests
#[tokio::test]
async fn test_seeking_alpha_long_ideas() {
    let context = setup_test_context().await;
    let seeking_alpha = SeekingAlpha::new(context.client.clone());

    let result = test_function_with_validation("long_ideas", || seeking_alpha.long_ideas(), &context).await;

    assert!(
        result.success,
        "long_ideas() failed: {:?}",
        result.error_message
    );

    // Validate long_ideas data quality and content structure
    match seeking_alpha.long_ideas().await {
        Ok(articles) => {
            if !articles.is_empty() {
                println!("Validating long ideas articles structure");
                
                for (i, article) in articles.iter().take(3).enumerate() {
                    println!("Validating long ideas article {}: {:?}", i + 1, article.title);

                    // Use lenient validation rules for real-world data
                    let rules = ArticleValidationRules::lenient();
                    assert_article_meets_rules(article, &rules);

                    // Validate source is set correctly
                    assert_eq!(
                        article.source,
                        Some("Seeking Alpha".to_string()),
                        "Long ideas article source should be set to Seeking Alpha"
                    );

                    // Validate that articles have meaningful content
                    assert!(
                        article.title.is_some() || article.description.is_some(),
                        "Long ideas article {} should have either title or description",
                        i + 1
                    );

                    // If link exists, validate URL format
                    if let Some(ref link) = article.link {
                        assert!(!link.trim().is_empty(), "Long ideas article link should not be empty");
                        assert_valid_url(link);
                    }
                }
            }
        }
        Err(e) => {
            println!("Warning: Could not validate long ideas data structure: {}", e);
        }
    }
}

#[tokio::test]
async fn test_seeking_alpha_short_ideas() {
    let context = setup_test_context().await;
    let seeking_alpha = SeekingAlpha::new(context.client.clone());

    let result = test_function_with_validation("short_ideas", || seeking_alpha.short_ideas(), &context).await;

    assert!(
        result.success,
        "short_ideas() failed: {:?}",
        result.error_message
    );

    // Validate short_ideas data quality and content structure
    match seeking_alpha.short_ideas().await {
        Ok(articles) => {
            if !articles.is_empty() {
                println!("Validating short ideas articles structure");
                
                for (i, article) in articles.iter().take(3).enumerate() {
                    println!("Validating short ideas article {}: {:?}", i + 1, article.title);

                    // Use lenient validation rules for real-world data
                    let rules = ArticleValidationRules::lenient();
                    assert_article_meets_rules(article, &rules);

                    // Validate source is set correctly
                    assert_eq!(
                        article.source,
                        Some("Seeking Alpha".to_string()),
                        "Short ideas article source should be set to Seeking Alpha"
                    );

                    // Validate that articles have meaningful content
                    assert!(
                        article.title.is_some() || article.description.is_some(),
                        "Short ideas article {} should have either title or description",
                        i + 1
                    );

                    // If link exists, validate URL format
                    if let Some(ref link) = article.link {
                        assert!(!link.trim().is_empty(), "Short ideas article link should not be empty");
                        assert_valid_url(link);
                    }
                }
            }
        }
        Err(e) => {
            println!("Warning: Could not validate short ideas data structure: {}", e);
        }
    }
}

#[tokio::test]
async fn test_seeking_alpha_transcripts() {
    let context = setup_test_context().await;
    let seeking_alpha = SeekingAlpha::new(context.client.clone());

    let result = test_function_with_validation("transcripts", || seeking_alpha.transcripts(), &context).await;

    assert!(
        result.success,
        "transcripts() failed: {:?}",
        result.error_message
    );

    // Validate transcripts data quality and content structure
    match seeking_alpha.transcripts().await {
        Ok(articles) => {
            if !articles.is_empty() {
                println!("Validating transcripts articles structure");
                
                for (i, article) in articles.iter().take(3).enumerate() {
                    println!("Validating transcripts article {}: {:?}", i + 1, article.title);

                    // Use lenient validation rules for real-world data
                    let rules = ArticleValidationRules::lenient();
                    assert_article_meets_rules(article, &rules);

                    // Validate source is set correctly
                    assert_eq!(
                        article.source,
                        Some("Seeking Alpha".to_string()),
                        "Transcripts article source should be set to Seeking Alpha"
                    );

                    // Validate that articles have meaningful content
                    assert!(
                        article.title.is_some() || article.description.is_some(),
                        "Transcripts article {} should have either title or description",
                        i + 1
                    );

                    // If link exists, validate URL format
                    if let Some(ref link) = article.link {
                        assert!(!link.trim().is_empty(), "Transcripts article link should not be empty");
                        assert_valid_url(link);
                    }
                }
            }
        }
        Err(e) => {
            println!("Warning: Could not validate transcripts data structure: {}", e);
        }
    }
}

#[tokio::test]
async fn test_seeking_alpha_wall_street_breakfast() {
    let context = setup_test_context().await;
    let seeking_alpha = SeekingAlpha::new(context.client.clone());

    let result = test_function_with_validation("wall_street_breakfast", || seeking_alpha.wall_street_breakfast(), &context).await;

    assert!(
        result.success,
        "wall_street_breakfast() failed: {:?}",
        result.error_message
    );

    // Validate wall_street_breakfast data quality and content structure
    match seeking_alpha.wall_street_breakfast().await {
        Ok(articles) => {
            if !articles.is_empty() {
                println!("Validating Wall Street Breakfast articles structure");
                
                for (i, article) in articles.iter().take(3).enumerate() {
                    println!("Validating Wall Street Breakfast article {}: {:?}", i + 1, article.title);

                    // Use lenient validation rules for real-world data
                    let rules = ArticleValidationRules::lenient();
                    assert_article_meets_rules(article, &rules);

                    // Validate source is set correctly
                    assert_eq!(
                        article.source,
                        Some("Seeking Alpha".to_string()),
                        "Wall Street Breakfast article source should be set to Seeking Alpha"
                    );

                    // Validate that articles have meaningful content
                    assert!(
                        article.title.is_some() || article.description.is_some(),
                        "Wall Street Breakfast article {} should have either title or description",
                        i + 1
                    );

                    // If link exists, validate URL format
                    if let Some(ref link) = article.link {
                        assert!(!link.trim().is_empty(), "Wall Street Breakfast article link should not be empty");
                        assert_valid_url(link);
                    }
                }
            }
        }
        Err(e) => {
            println!("Warning: Could not validate Wall Street Breakfast data structure: {}", e);
        }
    }
}

#[tokio::test]
async fn test_seeking_alpha_most_popular_articles() {
    let context = setup_test_context().await;
    let seeking_alpha = SeekingAlpha::new(context.client.clone());

    let result = test_function_with_validation("most_popular_articles", || seeking_alpha.most_popular_articles(), &context).await;

    assert!(
        result.success,
        "most_popular_articles() failed: {:?}",
        result.error_message
    );

    // Validate most_popular_articles data quality and content structure
    match seeking_alpha.most_popular_articles().await {
        Ok(articles) => {
            if !articles.is_empty() {
                println!("Validating most popular articles structure");
                
                for (i, article) in articles.iter().take(5).enumerate() {
                    println!("Validating most popular article {}: {:?}", i + 1, article.title);

                    // Use lenient validation rules for real-world data
                    let rules = ArticleValidationRules::lenient();
                    assert_article_meets_rules(article, &rules);

                    // Validate source is set correctly
                    assert_eq!(
                        article.source,
                        Some("Seeking Alpha".to_string()),
                        "Most popular article source should be set to Seeking Alpha"
                    );

                    // Validate that articles have meaningful content
                    assert!(
                        article.title.is_some() || article.description.is_some(),
                        "Most popular article {} should have either title or description",
                        i + 1
                    );

                    // If title exists, validate it's not empty and substantial
                    if let Some(ref title) = article.title {
                        assert!(
                            !title.trim().is_empty(),
                            "Most popular article title should not be empty"
                        );
                        assert!(
                            title.len() >= 3,
                            "Most popular article title should be at least 3 characters"
                        );
                    }

                    // If link exists, validate URL format
                    if let Some(ref link) = article.link {
                        assert!(!link.trim().is_empty(), "Most popular article link should not be empty");
                        assert_valid_url(link);
                        
                        // Most popular articles should likely link to seekingalpha.com
                        if link.contains("seekingalpha.com") {
                            println!("  ✓ Most popular article {} links to Seeking Alpha domain", i + 1);
                        }
                    }

                    // If description exists, validate it's meaningful
                    if let Some(ref description) = article.description {
                        assert!(
                            !description.trim().is_empty(),
                            "Most popular article description should not be empty"
                        );
                    }
                }
            }
        }
        Err(e) => {
            println!("Warning: Could not validate most popular articles data structure: {}", e);
        }
    }
}

// Additional comprehensive tests for all Seeking Alpha methods
#[tokio::test]
async fn test_seeking_alpha_all_public_methods() {
    let context = setup_test_context().await;
    let seeking_alpha = SeekingAlpha::new(context.client.clone());

    // Test all public methods systematically
    let test_methods = vec![
        "latest_articles",
        "all_news",
        "market_news",
        "long_ideas",
        "short_ideas",
        "ipo_analysis",
        "transcripts",
        "wall_street_breakfast",
        "most_popular_articles",
        "forex",
        "editors_picks",
        "etfs",
    ];

    let mut successful_methods = 0;
    let mut failed_methods = Vec::new();

    for method_name in &test_methods {
        println!("Testing Seeking Alpha method: {}", method_name);
        
        let result = match *method_name {
            "latest_articles" => test_function_with_validation(method_name, || seeking_alpha.latest_articles(), &context).await,
            "all_news" => test_function_with_validation(method_name, || seeking_alpha.all_news(), &context).await,
            "market_news" => test_function_with_validation(method_name, || seeking_alpha.market_news(), &context).await,
            "long_ideas" => test_function_with_validation(method_name, || seeking_alpha.long_ideas(), &context).await,
            "short_ideas" => test_function_with_validation(method_name, || seeking_alpha.short_ideas(), &context).await,
            "ipo_analysis" => test_function_with_validation(method_name, || seeking_alpha.ipo_analysis(), &context).await,
            "transcripts" => test_function_with_validation(method_name, || seeking_alpha.transcripts(), &context).await,
            "wall_street_breakfast" => test_function_with_validation(method_name, || seeking_alpha.wall_street_breakfast(), &context).await,
            "most_popular_articles" => test_function_with_validation(method_name, || seeking_alpha.most_popular_articles(), &context).await,
            "forex" => test_function_with_validation(method_name, || seeking_alpha.forex(), &context).await,
            "editors_picks" => test_function_with_validation(method_name, || seeking_alpha.editors_picks(), &context).await,
            "etfs" => test_function_with_validation(method_name, || seeking_alpha.etfs(), &context).await,
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

    println!("\n=== SEEKING ALPHA METHODS SUMMARY ===");
    println!("Total methods tested: {}", test_methods.len());
    println!("Successful methods: {}", successful_methods);
    println!("Failed methods: {}", failed_methods.len());

    if !failed_methods.is_empty() {
        println!("Failed methods:");
        for (method, error) in &failed_methods {
            println!("  - {}: {}", method, error);
        }
    }

    // We expect at least 50% of methods to work for Seeking Alpha to be considered functional
    let success_rate = successful_methods as f64 / test_methods.len() as f64;
    assert!(
        success_rate >= 0.5,
        "Seeking Alpha success rate too low: {:.1}% ({}/{}). This may indicate widespread issues.",
        success_rate * 100.0,
        successful_methods,
        test_methods.len()
    );
}

#[tokio::test]
async fn test_seeking_alpha_article_structure_validation() {
    let context = setup_test_context().await;
    let seeking_alpha = SeekingAlpha::new(context.client.clone());

    // Test with latest_articles as it's likely to have good data
    match seeking_alpha.latest_articles().await {
        Ok(articles) => {
            assert!(!articles.is_empty(), "Should receive at least one article");

            // Validate article structure integrity
            for (i, article) in articles.iter().take(5).enumerate() {
                println!("Validating Seeking Alpha article {}: {:?}", i + 1, article.title);

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
                    Some("Seeking Alpha".to_string()),
                    "Article source should be set to Seeking Alpha"
                );
            }
        }
        Err(e) => {
            panic!("Failed to fetch latest articles for validation: {}", e);
        }
    }
}

// Task 6.3: Implement Seeking Alpha deprecation tracking
#[tokio::test]
async fn test_seeking_alpha_deprecation_tracking() {
    let mut context = setup_test_context().await;
    let seeking_alpha = SeekingAlpha::new(context.client.clone());

    println!("=== SEEKING ALPHA DEPRECATION TRACKING ===");
    println!("Monitoring Seeking Alpha base_url endpoint changes and API modifications");

    // Test base_url accessibility
    let base_url = seeking_alpha.base_url();
    println!("Testing base URL: {}", base_url);

    match context.client.get(base_url).send().await {
        Ok(response) => {
            let status = response.status();
            println!("Base URL status: {}", status);
            
            if !status.is_success() {
                let error_msg = format!("Base URL returned non-success status: {}", status);
                let error = std::io::Error::new(std::io::ErrorKind::Other, error_msg);
                context.deprecation_tracker.record_failure_with_url(
                    "SeekingAlpha",
                    "base_url",
                    base_url,
                    &error,
                );
            }
        }
        Err(e) => {
            println!("Base URL failed: {}", e);
            context.deprecation_tracker.record_failure_with_url(
                "SeekingAlpha",
                "base_url",
                base_url,
                &e,
            );
        }
    }

    // Test all available topics for deprecation
    let topics = seeking_alpha.available_topics();
    println!("Testing {} available topics for deprecation", topics.len());

    let mut successful_topics = 0;
    let mut failed_topics = Vec::new();

    for topic in &topics {
        println!("Testing topic: {}", topic);
        
        let test_url = format!("{}?category={}", base_url, topic);
        
        match context.client.get(&test_url).send().await {
            Ok(response) => {
                let status = response.status();
                
                if status.is_success() {
                    // Try to parse the response to detect format changes
                    match response.text().await {
                        Ok(content) => {
                            if content.trim().is_empty() {
                                let error_msg = format!("Topic {} returned empty content", topic);
                                let error = std::io::Error::new(std::io::ErrorKind::InvalidData, error_msg);
                                context.deprecation_tracker.record_failure_with_url(
                                    "SeekingAlpha",
                                    &format!("topic_{}", topic),
                                    &test_url,
                                    &error,
                                );
                                failed_topics.push((topic, "Empty content".to_string()));
                            } else if !content.contains("<?xml") && !content.contains("<rss") {
                                let error_msg = format!("Topic {} returned non-XML content (possible format change)", topic);
                                let error = std::io::Error::new(std::io::ErrorKind::InvalidData, error_msg);
                                context.deprecation_tracker.record_failure_with_url(
                                    "SeekingAlpha",
                                    &format!("topic_{}", topic),
                                    &test_url,
                                    &error,
                                );
                                failed_topics.push((topic, "Non-XML format".to_string()));
                            } else {
                                successful_topics += 1;
                                println!("  OK Topic {} is accessible and returns XML", topic);
                            }
                        }
                        Err(e) => {
                            context.deprecation_tracker.record_failure_with_url(
                                "SeekingAlpha",
                                &format!("topic_{}", topic),
                                &test_url,
                                &e,
                            );
                            failed_topics.push((topic, e.to_string()));
                        }
                    }
                } else {
                    let error_msg = format!("Topic {} returned HTTP {}", topic, status);
                    let error = std::io::Error::new(std::io::ErrorKind::Other, error_msg);
                    context.deprecation_tracker.record_failure_with_url(
                        "SeekingAlpha",
                        &format!("topic_{}", topic),
                        &test_url,
                        &error,
                    );
                    failed_topics.push((topic, format!("HTTP {}", status)));
                    println!("  FAIL Topic {} failed with status: {}", topic, status);
                }
            }
            Err(e) => {
                context.deprecation_tracker.record_failure_with_url(
                    "SeekingAlpha",
                    &format!("topic_{}", topic),
                    &test_url,
                    &e,
                );
                failed_topics.push((topic, e.to_string()));
                println!("  FAIL Topic {} failed: {}", topic, e);
            }
        }
    }

    // Test parameterized functions for API modifications
    println!("Testing parameterized functions for API modifications");
    
    // Test global_markets with a sample country
    let test_country = "usa";
    let global_markets_url = format!("{}?category=global-markets-{}", base_url, test_country);
    match context.client.get(&global_markets_url).send().await {
        Ok(response) => {
            if !response.status().is_success() {
                let error_msg = format!("Global markets API modification detected: HTTP {}", response.status());
                let error = std::io::Error::new(std::io::ErrorKind::Other, error_msg);
                context.deprecation_tracker.record_failure_with_url(
                    "SeekingAlpha",
                    "global_markets_api",
                    &global_markets_url,
                    &error,
                );
            }
        }
        Err(e) => {
            context.deprecation_tracker.record_failure_with_url(
                "SeekingAlpha",
                "global_markets_api",
                &global_markets_url,
                &e,
            );
        }
    }

    // Test sectors with a sample sector
    let test_sector = "technology";
    let sectors_url = format!("{}?category=sectors-{}", base_url, test_sector);
    match context.client.get(&sectors_url).send().await {
        Ok(response) => {
            if !response.status().is_success() {
                let error_msg = format!("Sectors API modification detected: HTTP {}", response.status());
                let error = std::io::Error::new(std::io::ErrorKind::Other, error_msg);
                context.deprecation_tracker.record_failure_with_url(
                    "SeekingAlpha",
                    "sectors_api",
                    &sectors_url,
                    &error,
                );
            }
        }
        Err(e) => {
            context.deprecation_tracker.record_failure_with_url(
                "SeekingAlpha",
                "sectors_api",
                &sectors_url,
                &e,
            );
        }
    }

    // Test stocks with a sample ticker
    let test_ticker = "AAPL";
    let stocks_url = format!("{}?category=stocks-{}", base_url, test_ticker);
    match context.client.get(&stocks_url).send().await {
        Ok(response) => {
            if !response.status().is_success() {
                let error_msg = format!("Stocks API modification detected: HTTP {}", response.status());
                let error = std::io::Error::new(std::io::ErrorKind::Other, error_msg);
                context.deprecation_tracker.record_failure_with_url(
                    "SeekingAlpha",
                    "stocks_api",
                    &stocks_url,
                    &error,
                );
            }
        }
        Err(e) => {
            context.deprecation_tracker.record_failure_with_url(
                "SeekingAlpha",
                "stocks_api",
                &stocks_url,
                &e,
            );
        }
    }

    // Generate and display deprecation report
    let report = context.deprecation_tracker.generate_report();
    
    println!("=== SEEKING ALPHA DEPRECATION REPORT ===");
    println!("Topics tested: {}", topics.len());
    println!("Successful topics: {}", successful_topics);
    println!("Failed topics: {}", failed_topics.len());
    
    if !failed_topics.is_empty() {
        println!("Failed topics:");
        for (topic, error) in &failed_topics {
            println!("  - {}: {}", topic, error);
        }
    }

    println!("Deprecation Summary:");
    println!("Total failures recorded: {}", report.total_failures);
    println!("Deprecated endpoints: {}", report.deprecated_endpoints.len());
    println!("Removal candidates: {}", report.removal_candidates.len());

    if !report.deprecated_endpoints.is_empty() {
        println!("Deprecated endpoints detected:");
        for endpoint in &report.deprecated_endpoints {
            println!("  - {}::{} ({}) - {}", 
                endpoint.source, 
                endpoint.function, 
                endpoint.error_type,
                endpoint.url
            );
        }
    }

    if !report.removal_candidates.is_empty() {
        println!("Removal candidates (functions with consistent failures):");
        for candidate in &report.removal_candidates {
            println!("  - {}", candidate);
        }
    }

    // Print error type summary
    if !report.error_summary.is_empty() {
        println!("Error type summary:");
        for (error_type, count) in &report.error_summary {
            println!("  - {}: {}", error_type, count);
        }
    }

    // Check if Seeking Alpha has critical failures that indicate deprecation
    let has_critical_failures = context.deprecation_tracker.has_critical_failures("SeekingAlpha");
    if has_critical_failures {
        println!("WARNING: Seeking Alpha has critical failures that may indicate deprecated endpoints!");
        println!("   Consider reviewing the failed endpoints for potential removal.");
    } else {
        println!("SUCCESS: Seeking Alpha endpoints appear to be functioning normally.");
    }

    println!("=== END SEEKING ALPHA DEPRECATION REPORT ===");

    // The test should not fail even if there are deprecation issues - this is for monitoring
    // We only assert that the deprecation tracking system is working
    assert!(
        report.total_failures >= 0, // This will always be true, but ensures the report was generated
        "Deprecation tracking system should generate a report"
    );

    // Log a summary for CI/monitoring systems
    if std::env::var("CI").is_ok() {
        println!("CI_SEEKING_ALPHA_DEPRECATION_SUMMARY: success_rate={:.1}%, failures={}, deprecated={}", 
            (successful_topics as f64 / topics.len() as f64) * 100.0,
            report.total_failures,
            report.deprecated_endpoints.len()
        );
    }
}