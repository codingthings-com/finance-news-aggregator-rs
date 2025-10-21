use finance_news_aggregator_rs::news_source::NewsSource;
use finance_news_aggregator_rs::news_source::yahoo_finance::YahooFinance;
use tokio;

mod integration;
use integration::utils::client_factory::ClientFactory;

#[tokio::test]
async fn test_yahoo_finance_basic_functionality() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let yahoo_finance = YahooFinance::new(client);

    assert_eq!(yahoo_finance.name(), "Yahoo Finance");

    let topics = yahoo_finance.available_topics();
    assert!(!topics.is_empty());
    assert_eq!(topics.len(), 2);
}

#[tokio::test]
async fn test_yahoo_finance_headlines() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let yahoo_finance = YahooFinance::new(client);

    match yahoo_finance.headlines().await {
        Ok(articles) => {
            println!("✓ headlines returned {} articles", articles.len());
            for article in &articles {
                assert_eq!(article.source, Some("Yahoo Finance".to_string()));
            }
        }
        Err(e) => println!("✗ headlines failed: {}", e),
    }
}

#[tokio::test]
async fn test_yahoo_finance_topstories() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let yahoo_finance = YahooFinance::new(client);

    match yahoo_finance.topstories().await {
        Ok(articles) => {
            println!("✓ topstories returned {} articles", articles.len());
        }
        Err(e) => println!("✗ topstories failed: {}", e),
    }
}

#[tokio::test]
async fn test_yahoo_finance_headline_with_symbols() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let yahoo_finance = YahooFinance::new(client);

    let symbols = vec!["AAPL", "MSFT", "GOOGL"];
    match yahoo_finance.headline(&symbols).await {
        Ok(articles) => {
            println!(
                "✓ headline with symbols returned {} articles",
                articles.len()
            );
        }
        Err(e) => println!("✗ headline with symbols failed: {}", e),
    }
}

#[tokio::test]
async fn test_yahoo_finance_all_topics() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let yahoo_finance = YahooFinance::new(client);

    let topics = yahoo_finance.available_topics();
    let mut successful = 0;
    let mut _failed = 0;

    for &topic in &topics {
        match yahoo_finance.fetch_topic(topic).await {
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
        "\nYahoo Finance Summary: {}/{} topics accessible",
        successful,
        topics.len()
    );
    assert!(
        successful > 0,
        "At least one Yahoo Finance feed should be accessible"
    );
}
