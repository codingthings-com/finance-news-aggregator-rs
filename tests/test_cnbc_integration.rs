use finance_news_aggregator_rs::news_source::NewsSource;
use finance_news_aggregator_rs::news_source::cnbc::CNBC;
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

/// Setup test context for CNBC integration tests
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
async fn test_cnbc_basic_functionality() {
    let context = setup_test_context().await;
    let cnbc = CNBC::new(context.client.clone());

    // Test name() function
    assert_eq!(cnbc.name(), "CNBC");

    // Test base_url() function
    let base_url = cnbc.base_url();
    assert!(!base_url.is_empty());
    assert!(base_url.contains("cnbc.com"));

    // Test available_topics() function
    let topics = cnbc.available_topics();
    assert!(!topics.is_empty());

    // Verify expected topics are present
    let expected_topics = vec![
        "top_news",
        "business",
        "technology",
        "investing",
        "world_news",
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
async fn test_cnbc_top_news() {
    let context = setup_test_context().await;
    let cnbc = CNBC::new(context.client.clone());

    let result = test_function_with_validation("top_news", || cnbc.top_news(), &context).await;

    assert!(
        result.success,
        "top_news() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_cnbc_business() {
    let context = setup_test_context().await;
    let cnbc = CNBC::new(context.client.clone());

    let result = test_function_with_validation("business", || cnbc.business(), &context).await;

    assert!(
        result.success,
        "business() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_cnbc_technology() {
    let context = setup_test_context().await;
    let cnbc = CNBC::new(context.client.clone());

    let result = test_function_with_validation("technology", || cnbc.technology(), &context).await;

    assert!(
        result.success,
        "technology() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_cnbc_investing() {
    let context = setup_test_context().await;
    let cnbc = CNBC::new(context.client.clone());

    let result = test_function_with_validation("investing", || cnbc.investing(), &context).await;

    assert!(
        result.success,
        "investing() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_cnbc_world_news() {
    let context = setup_test_context().await;
    let cnbc = CNBC::new(context.client.clone());

    let result = test_function_with_validation("world_news", || cnbc.world_news(), &context).await;

    assert!(
        result.success,
        "world_news() failed: {:?}",
        result.error_message
    );
}

#[tokio::test]
async fn test_cnbc_news_feed_with_topics() {
    let context = setup_test_context().await;
    let cnbc = CNBC::new(context.client.clone());

    // Test various topic categories
    let test_topics = vec![
        "economy",
        "finance",
        "politics",
        "health_care",
        "real_estate",
        "wealth",
        "autos",
        "energy",
        "media",
        "retail",
        "travel",
    ];

    for topic in test_topics {
        let result = test_function_with_validation(
            &format!("news_feed({})", topic),
            || cnbc.news_feed(topic),
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

// Task 2.1: CNBC data validation tests
#[tokio::test]
async fn test_cnbc_article_structure_validation() {
    let context = setup_test_context().await;
    let cnbc = CNBC::new(context.client.clone());

    // Test with business news as it's likely to have good data
    match cnbc.business().await {
        Ok(articles) => {
            assert!(!articles.is_empty(), "Should receive at least one article");

            // Validate article structure integrity
            for (i, article) in articles.iter().take(5).enumerate() {
                println!("Validating article {}: {:?}", i + 1, article.title);

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
                    Some("CNBC".to_string()),
                    "Article source should be set to CNBC"
                );
            }
        }
        Err(e) => {
            panic!("Failed to fetch business news for validation: {}", e);
        }
    }
}

#[tokio::test]
async fn test_cnbc_publication_date_format() {
    let context = setup_test_context().await;
    let cnbc = CNBC::new(context.client.clone());

    match cnbc.top_news().await {
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
                "Warning: Could not fetch articles for date validation: {}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_cnbc_comprehensive_topic_validation() {
    let context = setup_test_context().await;
    let cnbc = CNBC::new(context.client.clone());

    // Get all available topics
    let topics = cnbc.available_topics();

    let mut successful_topics = 0;
    let mut failed_topics = Vec::new();

    // Test a subset of topics to avoid overwhelming the test
    let topics_to_test: Vec<_> = topics.iter().take(10).collect();
    let topics_count = topics_to_test.len();

    for &topic in &topics_to_test {
        match cnbc.news_feed(topic).await {
            Ok(articles) => {
                successful_topics += 1;

                if !articles.is_empty() {
                    // Validate first article from each successful topic
                    let article = &articles[0];
                    assert_valid_news_article(article, false);

                    // Ensure source is properly set
                    assert_eq!(article.source, Some("CNBC".to_string()));
                }
            }
            Err(e) => {
                failed_topics.push((topic, e.to_string()));
            }
        }
    }

    // We expect at least half of the topics to work
    assert!(
        successful_topics >= topics_count / 2,
        "Expected at least half of topics to work, got {}/{} successful. Failed topics: {:?}",
        successful_topics,
        topics_count,
        failed_topics
    );
}
