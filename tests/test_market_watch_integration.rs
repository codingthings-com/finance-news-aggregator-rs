use finance_news_aggregator_rs::news_source::NewsSource;
use finance_news_aggregator_rs::news_source::market_watch::MarketWatch;
use tokio;

mod integration;
use integration::utils::client_factory::ClientFactory;

#[tokio::test]
async fn test_market_watch_basic_functionality() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let market_watch = MarketWatch::new(client);

    assert_eq!(market_watch.name(), "MarketWatch");

    let topics = market_watch.available_topics();
    assert!(!topics.is_empty());
    assert_eq!(topics.len(), 4); // Only 4 working feeds
}

#[tokio::test]
async fn test_market_watch_top_stories() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let market_watch = MarketWatch::new(client);

    match market_watch.top_stories().await {
        Ok(articles) => {
            println!("✓ top_stories returned {} articles", articles.len());
            for article in &articles {
                assert_eq!(article.source, Some("MarketWatch".to_string()));
            }
        }
        Err(e) => println!("✗ top_stories failed: {}", e),
    }
}

#[tokio::test]
async fn test_market_watch_real_time_headlines() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let market_watch = MarketWatch::new(client);

    match market_watch.real_time_headlines().await {
        Ok(articles) => {
            println!("✓ real_time_headlines returned {} articles", articles.len());
        }
        Err(e) => println!("✗ real_time_headlines failed: {}", e),
    }
}

#[tokio::test]
async fn test_market_watch_market_pulse() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let market_watch = MarketWatch::new(client);

    match market_watch.market_pulse().await {
        Ok(articles) => {
            println!("✓ market_pulse returned {} articles", articles.len());
        }
        Err(e) => println!("✗ market_pulse failed: {}", e),
    }
}

#[tokio::test]
async fn test_market_watch_bulletins() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let market_watch = MarketWatch::new(client);

    match market_watch.bulletins().await {
        Ok(articles) => {
            println!("✓ bulletins returned {} articles", articles.len());
        }
        Err(e) => println!("✗ bulletins failed: {}", e),
    }
}

#[tokio::test]
async fn test_market_watch_all_topics() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let market_watch = MarketWatch::new(client);

    let topics = market_watch.available_topics();
    let mut successful = 0;
    let mut _failed = 0;

    for &topic in &topics {
        match market_watch.fetch_topic(topic).await {
            Ok(articles) => {
                successful += 1;
                println!("✓ {} returned {} articles", topic, articles.len());
            }
            Err(e) => {
                _failed += 1;
                println!("✗ {} failed: {}", topic, e);
            }
        }
    }

    println!(
        "\nMarketWatch Summary: {}/{} topics accessible",
        successful,
        topics.len()
    );
    assert!(
        successful > 0,
        "At least one MarketWatch feed should be accessible"
    );
}
