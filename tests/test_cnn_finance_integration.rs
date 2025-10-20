use finance_news_aggregator_rs::news_source::NewsSource;
use finance_news_aggregator_rs::news_source::cnn_finance::CNNFinance;
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

/// Setup test context for CNN Finance integration tests
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
async fn test_cnn_finance_basic_functionality() {
    let context = setup_test_context().await;
    let cnn_finance = CNNFinance::new(context.client.clone());

    // Test name() function
    assert_eq!(cnn_finance.name(), "CNN Finance");

    // Test base_url() function
    let base_url = cnn_finance.base_url();
    assert!(!base_url.is_empty());
    assert!(base_url.contains("rss.cnn.com"));
    assert!(base_url.contains("{topic}"));

    // Test available_topics() function
    let topics = cnn_finance.available_topics();
    assert!(!topics.is_empty());

    // Verify expected topics are present
    let expected_topics = vec![
        "money_latest",
        "money_news_companies",
        "money_news_economy",
        "money_news_international",
        "money_news_investing",
        "money_markets",
        "money_media",
        "money_pf",
        "money_real_estate",
        "money_technology",
        "morning_buzz",
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
async fn test_cnn_finance_all_stories() {
    let context = setup_test_context().await;
    let cnn_finance = CNNFinance::new(context.client.clone());

    let result = test_function_with_validation("all_stories", || cnn_finance.all_stories(), &context).await;

    assert!(
        result.success,
        "all_stories() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_cnn_finance_companies() {
    let context = setup_test_context().await;
    let cnn_finance = CNNFinance::new(context.client.clone());

    let result = test_function_with_validation("companies", || cnn_finance.companies(), &context).await;

    assert!(
        result.success,
        "companies() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_cnn_finance_economy() {
    let context = setup_test_context().await;
    let cnn_finance = CNNFinance::new(context.client.clone());

    let result = test_function_with_validation("economy", || cnn_finance.economy(), &context).await;

    assert!(
        result.success,
        "economy() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_cnn_finance_international() {
    let context = setup_test_context().await;
    let cnn_finance = CNNFinance::new(context.client.clone());

    let result = test_function_with_validation("international", || cnn_finance.international(), &context).await;

    assert!(
        result.success,
        "international() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_cnn_finance_investing() {
    let context = setup_test_context().await;
    let cnn_finance = CNNFinance::new(context.client.clone());

    let result = test_function_with_validation("investing", || cnn_finance.investing(), &context).await;

    assert!(
        result.success,
        "investing() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_cnn_finance_markets() {
    let context = setup_test_context().await;
    let cnn_finance = CNNFinance::new(context.client.clone());

    let result = test_function_with_validation("markets", || cnn_finance.markets(), &context).await;

    assert!(
        result.success,
        "markets() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_cnn_finance_media() {
    let context = setup_test_context().await;
    let cnn_finance = CNNFinance::new(context.client.clone());

    let result = test_function_with_validation("media", || cnn_finance.media(), &context).await;

    assert!(
        result.success,
        "media() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_cnn_finance_personal_finance() {
    let context = setup_test_context().await;
    let cnn_finance = CNNFinance::new(context.client.clone());

    let result = test_function_with_validation("personal_finance", || cnn_finance.personal_finance(), &context).await;

    assert!(
        result.success,
        "personal_finance() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_cnn_finance_real_estate() {
    let context = setup_test_context().await;
    let cnn_finance = CNNFinance::new(context.client.clone());

    let result = test_function_with_validation("real_estate", || cnn_finance.real_estate(), &context).await;

    assert!(
        result.success,
        "real_estate() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_cnn_finance_technology() {
    let context = setup_test_context().await;
    let cnn_finance = CNNFinance::new(context.client.clone());

    let result = test_function_with_validation("technology", || cnn_finance.technology(), &context).await;

    assert!(
        result.success,
        "technology() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_cnn_finance_morning_buzz() {
    let context = setup_test_context().await;
    let cnn_finance = CNNFinance::new(context.client.clone());

    let result = test_function_with_validation("morning_buzz", || cnn_finance.morning_buzz(), &context).await;

    assert!(
        result.success,
        "morning_buzz() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_cnn_finance_fetch_feed_with_topics() {
    let context = setup_test_context().await;
    let cnn_finance = CNNFinance::new(context.client.clone());

    // Test various topic categories using fetch_feed directly
    let test_topics = vec![
        "money_latest",
        "money_news_companies", 
        "money_news_economy",
        "money_news_international",
        "money_news_investing",
        "money_markets",
        "money_media",
        "money_pf",
        "money_real_estate",
        "money_technology",
    ];

    for topic in test_topics {
        let result = test_function_with_validation(
            &format!("fetch_feed({})", topic),
            || cnn_finance.fetch_feed(topic),
            &context,
        )
        .await;

        // Allow some topics to fail (they might be deprecated)
        if !result.success {
            println!(
                "Warning: Topic '{}' failed: {:?}",
                topic, result.error_message
            );
        }
    }
}

// Task 3.1: CNN Finance data validation tests
#[tokio::test]
async fn test_cnn_finance_article_structure_validation() {
    let context = setup_test_context().await;
    let cnn_finance = CNNFinance::new(context.client.clone());

    // Test with all_stories as it's likely to have good data
    match cnn_finance.all_stories().await {
        Ok(articles) => {
            assert!(!articles.is_empty(), "Should receive at least one article");

            // Validate article structure integrity
            for (i, article) in articles.iter().take(5).enumerate() {
                println!("Validating CNN Finance article {}: {:?}", i + 1, article.title);

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
                    Some("CNN Finance".to_string()),
                    "Article source should be set to CNN Finance"
                );
            }
        }
        Err(e) => {
            panic!("Failed to fetch all_stories for validation: {}", e);
        }
    }
}

#[tokio::test]
async fn test_cnn_finance_morning_buzz_specific_functionality() {
    let context = setup_test_context().await;
    let cnn_finance = CNNFinance::new(context.client.clone());

    // Test morning_buzz() specific functionality
    match cnn_finance.morning_buzz().await {
        Ok(articles) => {
            println!("Morning buzz returned {} articles", articles.len());

            if !articles.is_empty() {
                // Validate that morning buzz articles have proper structure
                for (i, article) in articles.iter().take(3).enumerate() {
                    println!("Validating morning buzz article {}: {:?}", i + 1, article.title);

                    // Morning buzz should have meaningful content
                    assert!(
                        article.title.is_some() || article.description.is_some(),
                        "Morning buzz article {} should have title or description",
                        i + 1
                    );

                    // Validate source is set correctly for morning buzz
                    assert_eq!(
                        article.source,
                        Some("CNN Finance".to_string()),
                        "Morning buzz article source should be set to CNN Finance"
                    );

                    // If title exists, validate it's substantial (morning buzz should have good titles)
                    if let Some(ref title) = article.title {
                        assert!(
                            title.len() >= 5,
                            "Morning buzz title should be at least 5 characters: '{}'",
                            title
                        );
                        
                        // Morning buzz titles should not be just placeholder text
                        let title_lower = title.to_lowercase();
                        assert!(
                            !title_lower.contains("lorem ipsum") && !title_lower.contains("test"),
                            "Morning buzz title should not be placeholder text: '{}'",
                            title
                        );
                    }

                    // If link exists, validate it's a CNN link
                    if let Some(ref link) = article.link {
                        assert_valid_url(link);
                        assert!(
                            link.contains("cnn.com") || link.contains("money.cnn.com"),
                            "Morning buzz link should be from CNN domain: '{}'",
                            link
                        );
                    }
                }

                // Morning buzz should typically have multiple articles
                if articles.len() == 1 {
                    println!("Warning: Morning buzz only returned 1 article, expected more");
                }
            } else {
                println!("Warning: Morning buzz returned no articles - this may indicate an issue");
            }
        }
        Err(e) => {
            println!("Warning: Morning buzz failed: {}", e);
            
            // Check if this is a deprecation issue
            let error_msg = e.to_string().to_lowercase();
            if error_msg.contains("404") || error_msg.contains("not found") {
                panic!("Morning buzz endpoint appears to be deprecated (404): {}", e);
            } else if error_msg.contains("403") || error_msg.contains("forbidden") {
                panic!("Morning buzz endpoint access forbidden (403): {}", e);
            }
            
            // For other errors, just log but don't fail the test
            println!("Morning buzz test failed with non-critical error: {}", e);
        }
    }
}

#[tokio::test]
async fn test_cnn_finance_personal_finance_specific_functionality() {
    let context = setup_test_context().await;
    let cnn_finance = CNNFinance::new(context.client.clone());

    // Test personal_finance() specific functionality
    match cnn_finance.personal_finance().await {
        Ok(articles) => {
            println!("Personal finance returned {} articles", articles.len());

            if !articles.is_empty() {
                // Validate that personal finance articles have proper structure
                for (i, article) in articles.iter().take(3).enumerate() {
                    println!("Validating personal finance article {}: {:?}", i + 1, article.title);

                    // Personal finance should have meaningful content
                    assert!(
                        article.title.is_some() || article.description.is_some(),
                        "Personal finance article {} should have title or description",
                        i + 1
                    );

                    // Validate source is set correctly
                    assert_eq!(
                        article.source,
                        Some("CNN Finance".to_string()),
                        "Personal finance article source should be set to CNN Finance"
                    );

                    // If title exists, validate it's relevant to personal finance
                    if let Some(ref title) = article.title {
                        assert!(
                            title.len() >= 5,
                            "Personal finance title should be at least 5 characters: '{}'",
                            title
                        );
                        
                        // Check for personal finance related keywords (optional validation)
                        let title_lower = title.to_lowercase();
                        let has_finance_keywords = title_lower.contains("money") ||
                            title_lower.contains("finance") ||
                            title_lower.contains("invest") ||
                            title_lower.contains("saving") ||
                            title_lower.contains("budget") ||
                            title_lower.contains("retirement") ||
                            title_lower.contains("debt") ||
                            title_lower.contains("credit") ||
                            title_lower.contains("loan") ||
                            title_lower.contains("tax") ||
                            title_lower.contains("income") ||
                            title_lower.contains("spending") ||
                            title_lower.contains("financial") ||
                            title_lower.contains("wealth") ||
                            title_lower.contains("dollar") ||
                            title_lower.contains("cost") ||
                            title_lower.contains("price") ||
                            title_lower.contains("pay") ||
                            title_lower.contains("buy") ||
                            title_lower.contains("sell");
                        
                        // Note: We don't assert this as CNN might include general business news
                        // in personal finance feed, but we log it for awareness
                        if !has_finance_keywords {
                            println!("Note: Personal finance article may not have finance keywords: '{}'", title);
                        }
                    }

                    // If link exists, validate it's a CNN link
                    if let Some(ref link) = article.link {
                        assert_valid_url(link);
                        assert!(
                            link.contains("cnn.com") || link.contains("money.cnn.com"),
                            "Personal finance link should be from CNN domain: '{}'",
                            link
                        );
                    }
                }
            } else {
                println!("Warning: Personal finance returned no articles - this may indicate an issue");
            }
        }
        Err(e) => {
            println!("Warning: Personal finance failed: {}", e);
            
            // Check if this is a deprecation issue
            let error_msg = e.to_string().to_lowercase();
            if error_msg.contains("404") || error_msg.contains("not found") {
                panic!("Personal finance endpoint appears to be deprecated (404): {}", e);
            } else if error_msg.contains("403") || error_msg.contains("forbidden") {
                panic!("Personal finance endpoint access forbidden (403): {}", e);
            }
            
            // For other errors, just log but don't fail the test
            println!("Personal finance test failed with non-critical error: {}", e);
        }
    }
}

#[tokio::test]
async fn test_cnn_finance_publication_date_format() {
    let context = setup_test_context().await;
    let cnn_finance = CNNFinance::new(context.client.clone());

    match cnn_finance.all_stories().await {
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
                        let has_date_indicators = date_lower.contains("mon") ||
                            date_lower.contains("tue") ||
                            date_lower.contains("wed") ||
                            date_lower.contains("thu") ||
                            date_lower.contains("fri") ||
                            date_lower.contains("sat") ||
                            date_lower.contains("sun") ||
                            date_lower.contains("jan") ||
                            date_lower.contains("feb") ||
                            date_lower.contains("mar") ||
                            date_lower.contains("apr") ||
                            date_lower.contains("may") ||
                            date_lower.contains("jun") ||
                            date_lower.contains("jul") ||
                            date_lower.contains("aug") ||
                            date_lower.contains("sep") ||
                            date_lower.contains("oct") ||
                            date_lower.contains("nov") ||
                            date_lower.contains("dec") ||
                            pub_date.chars().any(|c| c.is_ascii_digit());

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
                "Warning: Could not fetch articles for date validation: {}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_cnn_finance_comprehensive_topic_validation() {
    let context = setup_test_context().await;
    let cnn_finance = CNNFinance::new(context.client.clone());

    // Get all available topics
    let topics = cnn_finance.available_topics();

    let mut successful_topics = 0;
    let mut failed_topics = Vec::new();

    // Test all topics to validate comprehensive functionality
    for &topic in &topics {
        match cnn_finance.fetch_feed(topic).await {
            Ok(articles) => {
                successful_topics += 1;
                println!("✓ Topic '{}' returned {} articles", topic, articles.len());

                if !articles.is_empty() {
                    // Validate first article from each successful topic
                    let article = &articles[0];
                    assert_valid_news_article(article, false);

                    // Ensure source is properly set
                    assert_eq!(article.source, Some("CNN Finance".to_string()));
                }
            }
            Err(e) => {
                failed_topics.push((topic, e.to_string()));
                println!("✗ Topic '{}' failed: {}", topic, e);
            }
        }
    }

    // We expect at least half of the topics to work
    let topics_count = topics.len();
    assert!(
        successful_topics >= topics_count / 2,
        "Expected at least half of topics to work, got {}/{} successful. Failed topics: {:?}",
        successful_topics,
        topics_count,
        failed_topics
    );

    println!(
        "CNN Finance topic validation: {}/{} topics successful",
        successful_topics, topics_count
    );
}

#[tokio::test]
async fn test_cnn_finance_newsarticle_data_quality() {
    let context = setup_test_context().await;
    let cnn_finance = CNNFinance::new(context.client.clone());

    // Test data quality across different feed types
    let mut total_articles_tested = 0;
    let mut feeds_with_good_data = 0;

    // Test all_stories
    match cnn_finance.all_stories().await {
        Ok(articles) => {
            if !articles.is_empty() {
                feeds_with_good_data += 1;
                
                // Test first few articles for data quality
                for article in articles.iter().take(2) {
                    total_articles_tested += 1;

                    // Validate NewsArticle data structure and content quality
                    assert!(
                        article.title.is_some() || article.description.is_some(),
                        "Article from all_stories should have title or description"
                    );

                    // If title exists, check quality
                    if let Some(ref title) = article.title {
                        assert!(
                            title.len() >= 3,
                            "Title from all_stories should be at least 3 characters"
                        );
                        
                        // Check for common quality issues
                        assert!(
                            !title.trim().is_empty(),
                            "Title from all_stories should not be empty or whitespace"
                        );
                    }

                    // If link exists, validate it
                    if let Some(ref link) = article.link {
                        assert_valid_url(link);
                        assert!(
                            link.contains("cnn.com"),
                            "Link from all_stories should be from CNN domain"
                        );
                    }

                    // Validate source attribution
                    assert_eq!(
                        article.source,
                        Some("CNN Finance".to_string()),
                        "Article from all_stories should have correct source attribution"
                    );
                }
                
                println!("✓ all_stories returned {} articles with good data quality", articles.len());
            } else {
                println!("⚠ all_stories returned no articles");
            }
        }
        Err(e) => {
            println!("✗ all_stories failed: {}", e);
        }
    }

    // Test companies
    match cnn_finance.companies().await {
        Ok(articles) => {
            if !articles.is_empty() {
                feeds_with_good_data += 1;
                
                // Test first few articles for data quality
                for article in articles.iter().take(2) {
                    total_articles_tested += 1;

                    // Validate NewsArticle data structure and content quality
                    assert!(
                        article.title.is_some() || article.description.is_some(),
                        "Article from companies should have title or description"
                    );

                    // Validate source attribution
                    assert_eq!(
                        article.source,
                        Some("CNN Finance".to_string()),
                        "Article from companies should have correct source attribution"
                    );
                }
                
                println!("✓ companies returned {} articles with good data quality", articles.len());
            } else {
                println!("⚠ companies returned no articles");
            }
        }
        Err(e) => {
            println!("✗ companies failed: {}", e);
        }
    }

    // Test economy
    match cnn_finance.economy().await {
        Ok(articles) => {
            if !articles.is_empty() {
                feeds_with_good_data += 1;
                
                // Test first few articles for data quality
                for article in articles.iter().take(2) {
                    total_articles_tested += 1;

                    // Validate NewsArticle data structure and content quality
                    assert!(
                        article.title.is_some() || article.description.is_some(),
                        "Article from economy should have title or description"
                    );

                    // Validate source attribution
                    assert_eq!(
                        article.source,
                        Some("CNN Finance".to_string()),
                        "Article from economy should have correct source attribution"
                    );
                }
                
                println!("✓ economy returned {} articles with good data quality", articles.len());
            } else {
                println!("⚠ economy returned no articles");
            }
        }
        Err(e) => {
            println!("✗ economy failed: {}", e);
        }
    }

    // Test markets
    match cnn_finance.markets().await {
        Ok(articles) => {
            if !articles.is_empty() {
                feeds_with_good_data += 1;
                
                // Test first few articles for data quality
                for article in articles.iter().take(2) {
                    total_articles_tested += 1;

                    // Validate NewsArticle data structure and content quality
                    assert!(
                        article.title.is_some() || article.description.is_some(),
                        "Article from markets should have title or description"
                    );

                    // Validate source attribution
                    assert_eq!(
                        article.source,
                        Some("CNN Finance".to_string()),
                        "Article from markets should have correct source attribution"
                    );
                }
                
                println!("✓ markets returned {} articles with good data quality", articles.len());
            } else {
                println!("⚠ markets returned no articles");
            }
        }
        Err(e) => {
            println!("✗ markets failed: {}", e);
        }
    }

    // Ensure we tested some articles and most feeds worked
    assert!(
        total_articles_tested > 0,
        "Should have tested at least some articles for data quality"
    );
    
    assert!(
        feeds_with_good_data >= 2,
        "Expected at least 2 feeds to return good data, got {}",
        feeds_with_good_data
    );

    println!(
        "CNN Finance data quality validation: tested {} articles from {} feeds",
        total_articles_tested, feeds_with_good_data
    );
}

// Task 3.2: CNN Finance deprecation detection tests
#[tokio::test]
async fn test_cnn_finance_base_url_endpoint_availability() {
    let context = setup_test_context().await;
    let cnn_finance = CNNFinance::new(context.client.clone());

    // Test base_url endpoint availability by checking a known working topic
    let base_url_template = cnn_finance.base_url();
    assert!(
        base_url_template.contains("{topic}"),
        "Base URL should contain topic placeholder: {}",
        base_url_template
    );

    // Test base URL with a simple topic to verify endpoint availability
    let test_url = base_url_template.replace("{topic}", "money_latest");
    
    match context.client.get(&test_url).send().await {
        Ok(response) => {
            let status = response.status();
            if status.is_success() {
                println!("✓ CNN Finance base_url endpoint is available: {}", test_url);
                
                // Verify we get valid RSS content
                match response.text().await {
                    Ok(content) => {
                        assert!(
                            content.contains("<rss") || content.contains("<?xml"),
                            "Base URL should return RSS/XML content"
                        );
                        println!("✓ Base URL returns valid RSS content");
                    }
                    Err(e) => {
                        panic!("Failed to read response content from base URL: {}", e);
                    }
                }
            } else if status.as_u16() == 404 {
                panic!(
                    "CNN Finance base_url endpoint appears to be deprecated (404): {}",
                    test_url
                );
            } else if status.as_u16() == 403 {
                panic!(
                    "CNN Finance base_url endpoint access forbidden (403): {}",
                    test_url
                );
            } else {
                println!(
                    "Warning: CNN Finance base_url returned status {}: {}",
                    status, test_url
                );
            }
        }
        Err(e) => {
            let error_msg = e.to_string().to_lowercase();
            if error_msg.contains("dns") || error_msg.contains("resolve") {
                panic!(
                    "CNN Finance base_url DNS resolution failed (likely deprecated): {}",
                    e
                );
            } else if error_msg.contains("timeout") {
                println!(
                    "Warning: CNN Finance base_url timeout (may be temporary): {}",
                    e
                );
            } else {
                panic!("CNN Finance base_url endpoint failed: {}", e);
            }
        }
    }
}

#[tokio::test]
async fn test_cnn_finance_buzz_url_endpoint_availability() {
    let context = setup_test_context().await;
    let cnn_finance = CNNFinance::new(context.client.clone());

    // Test buzz_url endpoint availability directly
    // We need to access the buzz_url, but it's private, so we'll test via morning_buzz()
    match cnn_finance.morning_buzz().await {
        Ok(articles) => {
            println!("✓ CNN Finance buzz_url endpoint is available");
            println!("✓ Morning buzz returned {} articles", articles.len());
            
            // Verify articles have proper structure
            if !articles.is_empty() {
                for article in articles.iter().take(2) {
                    assert_eq!(
                        article.source,
                        Some("CNN Finance".to_string()),
                        "Morning buzz articles should have correct source"
                    );
                }
            }
        }
        Err(e) => {
            let error_msg = e.to_string().to_lowercase();
            if error_msg.contains("404") || error_msg.contains("not found") {
                panic!(
                    "CNN Finance buzz_url endpoint appears to be deprecated (404): {}",
                    e
                );
            } else if error_msg.contains("403") || error_msg.contains("forbidden") {
                panic!(
                    "CNN Finance buzz_url endpoint access forbidden (403): {}",
                    e
                );
            } else if error_msg.contains("dns") || error_msg.contains("resolve") {
                panic!(
                    "CNN Finance buzz_url DNS resolution failed (likely deprecated): {}",
                    e
                );
            } else if error_msg.contains("timeout") {
                println!(
                    "Warning: CNN Finance buzz_url timeout (may be temporary): {}",
                    e
                );
            } else {
                println!(
                    "Warning: CNN Finance buzz_url failed with error: {}",
                    e
                );
            }
        }
    }
}

#[tokio::test]
async fn test_cnn_finance_deprecated_feed_categories() {
    let context = setup_test_context().await;
    let cnn_finance = CNNFinance::new(context.client.clone());

    // Get all available topics
    let topics = cnn_finance.available_topics();
    
    let mut working_categories = Vec::new();
    let mut deprecated_categories = Vec::new();
    let mut temporary_failures = Vec::new();

    println!("Testing CNN Finance feed categories for deprecation...");

    // Test each category to identify deprecated ones
    for &topic in &topics {
        match cnn_finance.fetch_feed(topic).await {
            Ok(articles) => {
                working_categories.push(topic);
                println!("✓ Category '{}' is working ({} articles)", topic, articles.len());
            }
            Err(e) => {
                let error_msg = e.to_string().to_lowercase();
                
                if error_msg.contains("404") || error_msg.contains("not found") {
                    deprecated_categories.push((topic, "HTTP_404_NOT_FOUND", e.to_string()));
                    println!("✗ Category '{}' appears deprecated (404): {}", topic, e);
                } else if error_msg.contains("403") || error_msg.contains("forbidden") {
                    deprecated_categories.push((topic, "HTTP_403_FORBIDDEN", e.to_string()));
                    println!("✗ Category '{}' access forbidden (403): {}", topic, e);
                } else if error_msg.contains("dns") || error_msg.contains("resolve") {
                    deprecated_categories.push((topic, "DNS_ERROR", e.to_string()));
                    println!("✗ Category '{}' DNS error (likely deprecated): {}", topic, e);
                } else if error_msg.contains("timeout") {
                    temporary_failures.push((topic, "NETWORK_TIMEOUT", e.to_string()));
                    println!("⚠ Category '{}' timeout (may be temporary): {}", topic, e);
                } else if error_msg.contains("500") || error_msg.contains("502") || error_msg.contains("503") {
                    temporary_failures.push((topic, "SERVER_ERROR", e.to_string()));
                    println!("⚠ Category '{}' server error (may be temporary): {}", topic, e);
                } else {
                    temporary_failures.push((topic, "UNKNOWN_ERROR", e.to_string()));
                    println!("⚠ Category '{}' unknown error: {}", topic, e);
                }
            }
        }
    }

    // Generate deprecation report
    println!("\n=== CNN FINANCE DEPRECATION REPORT ===");
    println!("Total categories tested: {}", topics.len());
    println!("Working categories: {}", working_categories.len());
    println!("Deprecated categories: {}", deprecated_categories.len());
    println!("Temporary failures: {}", temporary_failures.len());

    if !working_categories.is_empty() {
        println!("\nWorking Categories:");
        for category in &working_categories {
            println!("  ✓ {}", category);
        }
    }

    if !deprecated_categories.is_empty() {
        println!("\nDeprecated Categories (removal candidates):");
        for (category, error_type, error_msg) in &deprecated_categories {
            println!("  ✗ {} - {} ({})", category, error_type, error_msg);
        }
    }

    if !temporary_failures.is_empty() {
        println!("\nTemporary Failures (monitor):");
        for (category, error_type, error_msg) in &temporary_failures {
            println!("  ⚠ {} - {} ({})", category, error_type, error_msg);
        }
    }

    println!("=== END REPORT ===\n");

    // Ensure we have at least some working categories
    assert!(
        !working_categories.is_empty(),
        "Expected at least some CNN Finance categories to work, but all failed"
    );

    // Ensure we have more working than deprecated categories
    assert!(
        working_categories.len() >= deprecated_categories.len(),
        "Too many deprecated categories ({}) compared to working ones ({}). This may indicate a systemic issue.",
        deprecated_categories.len(),
        working_categories.len()
    );

    // If we have deprecated categories, log them for removal consideration
    if !deprecated_categories.is_empty() {
        println!(
            "RECOMMENDATION: Consider removing {} deprecated CNN Finance categories from available_topics()",
            deprecated_categories.len()
        );
        
        for (category, _, _) in &deprecated_categories {
            println!("  - Remove topic: '{}'", category);
        }
    }
}

#[tokio::test]
async fn test_cnn_finance_endpoint_monitoring_with_deprecation_tracker() {
    use integration::utils::deprecation_tracker::DeprecationTracker;
    
    let context = setup_test_context().await;
    let cnn_finance = CNNFinance::new(context.client.clone());
    let mut deprecation_tracker = DeprecationTracker::new();

    let mut successful_functions = 0;
    let mut failed_functions = 0;

    // Test all CNN Finance functions individually and track failures
    
    // Test all_stories
    match cnn_finance.all_stories().await {
        Ok(articles) => {
            successful_functions += 1;
            println!("✓ Function 'all_stories' succeeded ({} articles)", articles.len());
        }
        Err(e) => {
            failed_functions += 1;
            println!("✗ Function 'all_stories' failed: {}", e);
            deprecation_tracker.record_failure("CNN Finance", "all_stories", &e);
        }
    }

    // Test companies
    match cnn_finance.companies().await {
        Ok(articles) => {
            successful_functions += 1;
            println!("✓ Function 'companies' succeeded ({} articles)", articles.len());
        }
        Err(e) => {
            failed_functions += 1;
            println!("✗ Function 'companies' failed: {}", e);
            deprecation_tracker.record_failure("CNN Finance", "companies", &e);
        }
    }

    // Test economy
    match cnn_finance.economy().await {
        Ok(articles) => {
            successful_functions += 1;
            println!("✓ Function 'economy' succeeded ({} articles)", articles.len());
        }
        Err(e) => {
            failed_functions += 1;
            println!("✗ Function 'economy' failed: {}", e);
            deprecation_tracker.record_failure("CNN Finance", "economy", &e);
        }
    }

    // Test international
    match cnn_finance.international().await {
        Ok(articles) => {
            successful_functions += 1;
            println!("✓ Function 'international' succeeded ({} articles)", articles.len());
        }
        Err(e) => {
            failed_functions += 1;
            println!("✗ Function 'international' failed: {}", e);
            deprecation_tracker.record_failure("CNN Finance", "international", &e);
        }
    }

    // Test investing
    match cnn_finance.investing().await {
        Ok(articles) => {
            successful_functions += 1;
            println!("✓ Function 'investing' succeeded ({} articles)", articles.len());
        }
        Err(e) => {
            failed_functions += 1;
            println!("✗ Function 'investing' failed: {}", e);
            deprecation_tracker.record_failure("CNN Finance", "investing", &e);
        }
    }

    // Test markets
    match cnn_finance.markets().await {
        Ok(articles) => {
            successful_functions += 1;
            println!("✓ Function 'markets' succeeded ({} articles)", articles.len());
        }
        Err(e) => {
            failed_functions += 1;
            println!("✗ Function 'markets' failed: {}", e);
            deprecation_tracker.record_failure("CNN Finance", "markets", &e);
        }
    }

    // Test media
    match cnn_finance.media().await {
        Ok(articles) => {
            successful_functions += 1;
            println!("✓ Function 'media' succeeded ({} articles)", articles.len());
        }
        Err(e) => {
            failed_functions += 1;
            println!("✗ Function 'media' failed: {}", e);
            deprecation_tracker.record_failure("CNN Finance", "media", &e);
        }
    }

    // Test personal_finance
    match cnn_finance.personal_finance().await {
        Ok(articles) => {
            successful_functions += 1;
            println!("✓ Function 'personal_finance' succeeded ({} articles)", articles.len());
        }
        Err(e) => {
            failed_functions += 1;
            println!("✗ Function 'personal_finance' failed: {}", e);
            deprecation_tracker.record_failure("CNN Finance", "personal_finance", &e);
        }
    }

    // Test real_estate
    match cnn_finance.real_estate().await {
        Ok(articles) => {
            successful_functions += 1;
            println!("✓ Function 'real_estate' succeeded ({} articles)", articles.len());
        }
        Err(e) => {
            failed_functions += 1;
            println!("✗ Function 'real_estate' failed: {}", e);
            deprecation_tracker.record_failure("CNN Finance", "real_estate", &e);
        }
    }

    // Test technology
    match cnn_finance.technology().await {
        Ok(articles) => {
            successful_functions += 1;
            println!("✓ Function 'technology' succeeded ({} articles)", articles.len());
        }
        Err(e) => {
            failed_functions += 1;
            println!("✗ Function 'technology' failed: {}", e);
            deprecation_tracker.record_failure("CNN Finance", "technology", &e);
        }
    }

    // Test morning_buzz
    match cnn_finance.morning_buzz().await {
        Ok(articles) => {
            successful_functions += 1;
            println!("✓ Function 'morning_buzz' succeeded ({} articles)", articles.len());
        }
        Err(e) => {
            failed_functions += 1;
            println!("✗ Function 'morning_buzz' failed: {}", e);
            deprecation_tracker.record_failure("CNN Finance", "morning_buzz", &e);
        }
    }

    // Generate and display deprecation report
    let report = deprecation_tracker.generate_report();
    println!("\n{}", report);

    // Ensure we have more successful than failed functions
    assert!(
        successful_functions > failed_functions,
        "Too many CNN Finance functions failed ({}) compared to successful ones ({})",
        failed_functions,
        successful_functions
    );

    // If we have critical failures, report them
    if deprecation_tracker.has_critical_failures("CNN Finance") {
        println!("WARNING: CNN Finance has critical failures that may indicate deprecated endpoints");
        
        let cnn_failures = deprecation_tracker.get_source_failures("CNN Finance");
        for failure in cnn_failures {
            if matches!(
                failure.error_type.as_str(),
                "HTTP_404_NOT_FOUND" | "HTTP_403_FORBIDDEN" | "DNS_ERROR"
            ) {
                println!(
                    "CRITICAL: Function '{}' has critical failure: {} ({})",
                    failure.function, failure.error_type, failure.error_message
                );
            }
        }
    }

    println!(
        "CNN Finance endpoint monitoring complete: {}/{} functions successful",
        successful_functions,
        successful_functions + failed_functions
    );
}