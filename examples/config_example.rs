use finance_news_aggregator_rs::{NewsClient, Result};
use finance_news_aggregator_rs::types::SourceConfig;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    env_logger::init();
    
    println!("Finance News Aggregator - Configuration Example\n");
    
    // Example 1: Default configuration
    println!("=== Default Configuration ===");
    let mut default_client = NewsClient::new();
    println!("Default timeout: {} seconds", default_client.config().timeout_seconds);
    println!("Default user agent: {}", default_client.config().user_agent);
    println!("Default max retries: {}", default_client.config().max_retries);
    
    // Example 2: Custom configuration
    println!("\n=== Custom Configuration ===");
    let custom_config = SourceConfig::default()
        .with_timeout(60)
        .with_user_agent("Custom Finance News Bot 1.0")
        .with_retries(5, 2000);
        
    let mut custom_client = NewsClient::with_config(custom_config);
    println!("Custom timeout: {} seconds", custom_client.config().timeout_seconds);
    println!("Custom user agent: {}", custom_client.config().user_agent);
    println!("Custom max retries: {}", custom_client.config().max_retries);
    println!("Custom retry delay: {} ms", custom_client.config().retry_delay_ms);
    
    // Example 3: Using the configured client
    println!("\n=== Testing with Custom Config ===");
    let wsj = custom_client.wsj();
    
    match wsj.opinions().await {
        Ok(articles) => {
            println!("Successfully fetched {} WSJ opinion articles with custom config", articles.len());
            if let Some(first) = articles.first() {
                println!("Latest: {}", first.title.as_deref().unwrap_or("No title"));
            }
        }
        Err(e) => {
            eprintln!("Error fetching WSJ opinions: {}", e);
        }
    }
    
    // Example 4: Configuration methods
    println!("\n=== Configuration Utilities ===");
    let config = SourceConfig::new("https://example.com/rss/{topic}.xml")
        .with_timeout(45)
        .with_retries(3, 1500);
        
    println!("Base URL: {}", config.base_url);
    println!("Timeout duration: {:?}", config.timeout_duration());
    println!("Retry delay duration: {:?}", config.retry_delay_duration());
    
    Ok(())
}