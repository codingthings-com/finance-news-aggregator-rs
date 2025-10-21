use finance_news_aggregator_rs::news_source::NewsSource;
use finance_news_aggregator_rs::news_source::nasdaq::NASDAQ;
use tokio;

mod integration;
use integration::utils::client_factory::ClientFactory;

#[tokio::test]
async fn test_nasdaq_basic_functionality() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let nasdaq = NASDAQ::new(client);

    assert_eq!(nasdaq.name(), "NASDAQ");

    let topics = nasdaq.available_topics();
    assert!(!topics.is_empty());
    assert!(topics.contains(&"original"));
    assert!(topics.contains(&"technology"));
}

#[tokio::test]
async fn test_nasdaq_original_content() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let nasdaq = NASDAQ::new(client);

    match nasdaq.original_content().await {
        Ok(articles) => {
            println!("✓ original_content returned {} articles", articles.len());
            for article in &articles {
                assert_eq!(article.source, Some("NASDAQ".to_string()));
            }
        }
        Err(e) => println!("✗ original_content failed: {}", e),
    }
}

#[tokio::test]
async fn test_nasdaq_commodities() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let nasdaq = NASDAQ::new(client);

    match nasdaq.commodities().await {
        Ok(articles) => {
            println!("✓ commodities returned {} articles", articles.len());
        }
        Err(e) => println!("✗ commodities failed: {}", e),
    }
}

#[tokio::test]
async fn test_nasdaq_cryptocurrency() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let nasdaq = NASDAQ::new(client);

    match nasdaq.cryptocurrency().await {
        Ok(articles) => {
            println!("✓ cryptocurrency returned {} articles", articles.len());
        }
        Err(e) => println!("✗ cryptocurrency failed: {}", e),
    }
}

#[tokio::test]
async fn test_nasdaq_technology() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let nasdaq = NASDAQ::new(client);

    match nasdaq.technology().await {
        Ok(articles) => {
            println!("✓ technology returned {} articles", articles.len());
        }
        Err(e) => println!("✗ technology failed: {}", e),
    }
}

#[tokio::test]
async fn test_nasdaq_all_topics() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let nasdaq = NASDAQ::new(client);

    let topics = nasdaq.available_topics();
    let mut successful = 0;
    let mut _failed = 0;

    for &topic in &topics {
        match nasdaq.fetch_topic(topic).await {
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
        "\nNASDAQ Summary: {}/{} topics accessible",
        successful,
        topics.len()
    );
    assert!(
        successful > 0,
        "At least one NASDAQ feed should be accessible"
    );
}
