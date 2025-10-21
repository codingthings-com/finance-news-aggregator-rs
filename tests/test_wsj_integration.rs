use finance_news_aggregator_rs::news_source::NewsSource;
use finance_news_aggregator_rs::news_source::wsj::WallStreetJournal;
use tokio;

mod integration;
use integration::utils::client_factory::ClientFactory;

#[tokio::test]
async fn test_wsj_basic_functionality() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let wsj = WallStreetJournal::new(client);

    assert_eq!(wsj.name(), "Wall Street Journal");

    let topics = wsj.available_topics();
    assert!(!topics.is_empty());
}

#[tokio::test]
async fn test_wsj_opinions() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let wsj = WallStreetJournal::new(client);

    match wsj.opinions().await {
        Ok(articles) => {
            println!("✓ opinions returned {} articles", articles.len());
            for article in &articles {
                assert_eq!(article.source, Some("Wall Street Journal".to_string()));
            }
        }
        Err(e) => println!("✗ opinions failed: {}", e),
    }
}

#[tokio::test]
async fn test_wsj_world_news() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let wsj = WallStreetJournal::new(client);

    match wsj.world_news().await {
        Ok(articles) => {
            println!("✓ world_news returned {} articles", articles.len());
        }
        Err(e) => println!("✗ world_news failed: {}", e),
    }
}

#[tokio::test]
async fn test_wsj_us_business_news() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let wsj = WallStreetJournal::new(client);

    match wsj.us_business_news().await {
        Ok(articles) => {
            println!("✓ us_business_news returned {} articles", articles.len());
        }
        Err(e) => println!("✗ us_business_news failed: {}", e),
    }
}

#[tokio::test]
async fn test_wsj_market_news() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let wsj = WallStreetJournal::new(client);

    match wsj.market_news().await {
        Ok(articles) => {
            println!("✓ market_news returned {} articles", articles.len());
        }
        Err(e) => println!("✗ market_news failed: {}", e),
    }
}

#[tokio::test]
async fn test_wsj_technology_news() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let wsj = WallStreetJournal::new(client);

    match wsj.technology_news().await {
        Ok(articles) => {
            println!("✓ technology_news returned {} articles", articles.len());
        }
        Err(e) => println!("✗ technology_news failed: {}", e),
    }
}

#[tokio::test]
async fn test_wsj_lifestyle() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let wsj = WallStreetJournal::new(client);

    match wsj.lifestyle().await {
        Ok(articles) => {
            println!("✓ lifestyle returned {} articles", articles.len());
        }
        Err(e) => println!("✗ lifestyle failed: {}", e),
    }
}

#[tokio::test]
async fn test_wsj_all_topics() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let wsj = WallStreetJournal::new(client);

    let topics = wsj.available_topics();
    let mut successful = 0;
    let mut _failed = 0;

    for &topic in &topics {
        match wsj.fetch_topic(topic).await {
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
        "\nWSJ Summary: {}/{} topics accessible",
        successful,
        topics.len()
    );
    assert!(successful > 0, "At least one WSJ feed should be accessible");
}
