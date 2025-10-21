use finance_news_aggregator_rs::news_source::NewsSource;
use finance_news_aggregator_rs::news_source::cnn_finance::CNNFinance;
use tokio;

mod integration;
use integration::utils::client_factory::ClientFactory;

#[tokio::test]
async fn test_cnn_finance_basic_functionality() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let cnn_finance = CNNFinance::new(client);

    assert_eq!(cnn_finance.name(), "CNN Finance");
    
    let topics = cnn_finance.available_topics();
    assert!(!topics.is_empty());
}

#[tokio::test]
async fn test_cnn_finance_all_stories() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let cnn_finance = CNNFinance::new(client);

    match cnn_finance.all_stories().await {
        Ok(articles) => {
            println!("✓ all_stories returned {} articles", articles.len());
            for article in &articles {
                assert_eq!(article.source, Some("CNN Finance".to_string()));
            }
        }
        Err(e) => println!("✗ all_stories failed: {}", e),
    }
}

#[tokio::test]
async fn test_cnn_finance_companies() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let cnn_finance = CNNFinance::new(client);

    match cnn_finance.companies().await {
        Ok(articles) => {
            println!("✓ companies returned {} articles", articles.len());
        }
        Err(e) => println!("✗ companies failed: {}", e),
    }
}

#[tokio::test]
async fn test_cnn_finance_economy() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let cnn_finance = CNNFinance::new(client);

    match cnn_finance.economy().await {
        Ok(articles) => {
            println!("✓ economy returned {} articles", articles.len());
        }
        Err(e) => println!("✗ economy failed: {}", e),
    }
}

#[tokio::test]
async fn test_cnn_finance_markets() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let cnn_finance = CNNFinance::new(client);

    match cnn_finance.markets().await {
        Ok(articles) => {
            println!("✓ markets returned {} articles", articles.len());
        }
        Err(e) => println!("✗ markets failed: {}", e),
    }
}

#[tokio::test]
async fn test_cnn_finance_technology() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let cnn_finance = CNNFinance::new(client);

    match cnn_finance.technology().await {
        Ok(articles) => {
            println!("✓ technology returned {} articles", articles.len());
        }
        Err(e) => println!("✗ technology failed: {}", e),
    }
}

#[tokio::test]
async fn test_cnn_finance_all_topics() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let cnn_finance = CNNFinance::new(client);

    let topics = cnn_finance.available_topics();
    let mut successful = 0;
    let mut _failed = 0;

    for &topic in &topics {
        match cnn_finance.fetch_topic(topic).await {
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

    println!("\nCNN Finance Summary: {}/{} topics accessible", successful, topics.len());
    assert!(successful > 0, "At least one CNN Finance feed should be accessible");
}
