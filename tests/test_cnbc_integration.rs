use finance_news_aggregator_rs::news_source::NewsSource;
use finance_news_aggregator_rs::news_source::cnbc::CNBC;
use tokio;

mod integration;
use integration::utils::client_factory::ClientFactory;

#[tokio::test]
async fn test_cnbc_basic_functionality() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let cnbc = CNBC::new(client);

    assert_eq!(cnbc.name(), "CNBC");

    let topics = cnbc.available_topics();
    assert!(!topics.is_empty());
}

#[tokio::test]
async fn test_cnbc_top_news() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let cnbc = CNBC::new(client);

    match cnbc.top_news().await {
        Ok(articles) => {
            println!("✓ top_news returned {} articles", articles.len());
            for article in &articles {
                assert_eq!(article.source, Some("CNBC".to_string()));
            }
        }
        Err(e) => println!("✗ top_news failed: {}", e),
    }
}

#[tokio::test]
async fn test_cnbc_business() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let cnbc = CNBC::new(client);

    match cnbc.business().await {
        Ok(articles) => {
            println!("✓ business returned {} articles", articles.len());
        }
        Err(e) => println!("✗ business failed: {}", e),
    }
}

#[tokio::test]
async fn test_cnbc_technology() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let cnbc = CNBC::new(client);

    match cnbc.technology().await {
        Ok(articles) => {
            println!("✓ technology returned {} articles", articles.len());
        }
        Err(e) => println!("✗ technology failed: {}", e),
    }
}

#[tokio::test]
async fn test_cnbc_investing() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let cnbc = CNBC::new(client);

    match cnbc.investing().await {
        Ok(articles) => {
            println!("✓ investing returned {} articles", articles.len());
        }
        Err(e) => println!("✗ investing failed: {}", e),
    }
}

#[tokio::test]
async fn test_cnbc_world_news() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let cnbc = CNBC::new(client);

    match cnbc.world_news().await {
        Ok(articles) => {
            println!("✓ world_news returned {} articles", articles.len());
        }
        Err(e) => println!("✗ world_news failed: {}", e),
    }
}

#[tokio::test]
async fn test_cnbc_all_topics() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let cnbc = CNBC::new(client);

    let topics = cnbc.available_topics();
    let mut successful = 0;
    let mut _failed = 0;

    for &topic in &topics {
        match cnbc.fetch_topic(topic).await {
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
        "\nCNBC Summary: {}/{} topics accessible",
        successful,
        topics.len()
    );
    assert!(
        successful > 0,
        "At least one CNBC feed should be accessible"
    );
}
