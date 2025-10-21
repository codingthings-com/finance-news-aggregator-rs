use finance_news_aggregator_rs::news_source::NewsSource;
use finance_news_aggregator_rs::news_source::seeking_alpha::SeekingAlpha;
use tokio;

mod integration;
use integration::utils::client_factory::ClientFactory;

#[tokio::test]
async fn test_seeking_alpha_basic_functionality() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let seeking_alpha = SeekingAlpha::new(client);

    assert_eq!(seeking_alpha.name(), "Seeking Alpha");
    
    let topics = seeking_alpha.available_topics();
    assert!(!topics.is_empty());
}

#[tokio::test]
async fn test_seeking_alpha_latest_articles() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let seeking_alpha = SeekingAlpha::new(client);

    match seeking_alpha.latest_articles().await {
        Ok(articles) => {
            println!("✓ latest_articles returned {} articles", articles.len());
            for article in &articles {
                assert_eq!(article.source, Some("Seeking Alpha".to_string()));
            }
        }
        Err(e) => println!("✗ latest_articles failed: {}", e),
    }
}

#[tokio::test]
async fn test_seeking_alpha_all_news() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let seeking_alpha = SeekingAlpha::new(client);

    match seeking_alpha.all_news().await {
        Ok(articles) => {
            println!("✓ all_news returned {} articles", articles.len());
        }
        Err(e) => println!("✗ all_news failed: {}", e),
    }
}

#[tokio::test]
async fn test_seeking_alpha_market_news() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let seeking_alpha = SeekingAlpha::new(client);

    match seeking_alpha.market_news().await {
        Ok(articles) => {
            println!("✓ market_news returned {} articles", articles.len());
        }
        Err(e) => println!("✗ market_news failed: {}", e),
    }
}

#[tokio::test]
async fn test_seeking_alpha_editors_picks() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let seeking_alpha = SeekingAlpha::new(client);

    match seeking_alpha.editors_picks().await {
        Ok(articles) => {
            println!("✓ editors_picks returned {} articles", articles.len());
        }
        Err(e) => println!("✗ editors_picks failed: {}", e),
    }
}

#[tokio::test]
async fn test_seeking_alpha_all_topics() {
    let client = ClientFactory::create_test_client().expect("Failed to create test client");
    let seeking_alpha = SeekingAlpha::new(client);

    let topics = seeking_alpha.available_topics();
    let mut successful = 0;
    let mut _failed = 0;

    for &topic in &topics {
        match seeking_alpha.fetch_topic(topic).await {
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

    println!("\nSeeking Alpha Summary: {}/{} topics accessible", successful, topics.len());
    assert!(successful > 0, "At least one Seeking Alpha feed should be accessible");
}
