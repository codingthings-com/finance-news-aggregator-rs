use finance_news_aggregator_rs::news_source::NewsSource;
use finance_news_aggregator_rs::{NewsClient, Result};

/// Example demonstrating the new topic-based and URL-based fetching API
///
/// This example shows three ways to fetch news:
/// 1. Using convenience methods (e.g., wsj.opinions())
/// 2. Using the generic fetch_topic() method
/// 3. Using the generic fetch_feed_by_url() method
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let mut client = NewsClient::new();

    println!("=== Finance News Aggregator - Topic-Based API Demo ===\n");

    // Example 1: Discover available topics
    println!("1. Discovering Available Topics");
    println!("--------------------------------");
    {
        let wsj = client.wsj();
        let wsj_topics = wsj.available_topics();
        println!("WSJ available topics: {:?}\n", wsj_topics);
    }
    {
        let cnbc = client.cnbc();
        let cnbc_topics = cnbc.available_topics();
        println!(
            "CNBC available topics (first 5): {:?}\n",
            &cnbc_topics[..5.min(cnbc_topics.len())]
        );
    }

    // Example 2: Fetch using convenience methods (traditional approach)
    println!("2. Using Convenience Methods");
    println!("----------------------------");
    {
        let wsj = client.wsj();
        match wsj.opinions().await {
            Ok(articles) => {
                println!("✓ WSJ Opinions: {} articles", articles.len());
                if let Some(first) = articles.first() {
                    println!(
                        "  First article: {}",
                        first.title.as_ref().unwrap_or(&"No title".to_string())
                    );
                }
            }
            Err(e) => println!("✗ Error fetching WSJ opinions: {}", e),
        }
    }
    println!();

    // Example 3: Fetch using generic topic-based API
    println!("3. Using Generic fetch_topic() Method");
    println!("--------------------------------------");

    // Fetch WSJ world news using topic name
    {
        let wsj = client.wsj();
        match wsj.fetch_topic("RSSWorldNews").await {
            Ok(articles) => {
                println!(
                    "✓ WSJ World News (via fetch_topic): {} articles",
                    articles.len()
                );
                if let Some(first) = articles.first() {
                    println!(
                        "  First article: {}",
                        first.title.as_ref().unwrap_or(&"No title".to_string())
                    );
                }
            }
            Err(e) => println!("✗ Error fetching WSJ world news: {}", e),
        }
    }

    // Fetch CNBC technology news using topic name
    {
        let cnbc = client.cnbc();
        match cnbc.fetch_topic("technology").await {
            Ok(articles) => {
                println!(
                    "✓ CNBC Technology (via fetch_topic): {} articles",
                    articles.len()
                );
                if let Some(first) = articles.first() {
                    println!(
                        "  First article: {}",
                        first.title.as_ref().unwrap_or(&"No title".to_string())
                    );
                }
            }
            Err(e) => println!("✗ Error fetching CNBC technology: {}", e),
        }
    }
    println!();

    // Example 4: Fetch using direct URL
    println!("4. Using Generic fetch_feed_by_url() Method");
    println!("-------------------------------------------");
    {
        let wsj = client.wsj();
        // Fetch from any RSS URL directly
        let custom_url = "https://feeds.a.dj.com/rss/RSSMarketsMain.xml";
        match wsj.fetch_feed_by_url(custom_url).await {
            Ok(articles) => {
                println!("✓ Custom URL fetch: {} articles", articles.len());
                println!("  URL: {}", custom_url);
                if let Some(first) = articles.first() {
                    println!(
                        "  First article: {}",
                        first.title.as_ref().unwrap_or(&"No title".to_string())
                    );
                }
            }
            Err(e) => println!("✗ Error fetching from custom URL: {}", e),
        }
    }
    println!();

    // Example 5: Fetch special endpoints
    println!("5. Fetching Special Endpoints");
    println!("------------------------------");

    // NASDAQ has a special "original" content endpoint
    {
        let nasdaq = client.nasdaq();
        match nasdaq.fetch_topic("original").await {
            Ok(articles) => {
                println!("✓ NASDAQ Original Content: {} articles", articles.len());
                if let Some(first) = articles.first() {
                    println!(
                        "  First article: {}",
                        first.title.as_ref().unwrap_or(&"No title".to_string())
                    );
                }
            }
            Err(e) => println!("✗ Error fetching NASDAQ original content: {}", e),
        }
    }
    println!();

    println!("=== Demo Complete ===");

    Ok(())
}
