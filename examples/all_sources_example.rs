use finance_news_aggregator_rs::{NewsClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    env_logger::init();
    
    // Create a new instance of the News Client
    let mut news_client = NewsClient::new();
    
    println!("Finance News Aggregator - All Sources Example\n");
    
    // Wall Street Journal
    println!("=== Wall Street Journal ===");
    let wsj = news_client.wsj();
    match wsj.opinions().await {
        Ok(articles) => {
            println!("WSJ Opinions: {} articles", articles.len());
            if let Some(first) = articles.first() {
                println!("  Latest: {}", first.title.as_deref().unwrap_or("No title"));
            }
        }
        Err(e) => eprintln!("WSJ Error: {}", e),
    }
    
    // CNBC
    println!("\n=== CNBC ===");
    let cnbc = news_client.cnbc();
    match cnbc.top_news().await {
        Ok(articles) => {
            println!("CNBC Top News: {} articles", articles.len());
            if let Some(first) = articles.first() {
                println!("  Latest: {}", first.title.as_deref().unwrap_or("No title"));
            }
        }
        Err(e) => eprintln!("CNBC Error: {}", e),
    }
    
    // NASDAQ
    println!("\n=== NASDAQ ===");
    let nasdaq = news_client.nasdaq();
    match nasdaq.original_content().await {
        Ok(articles) => {
            println!("NASDAQ Original Content: {} articles", articles.len());
            if let Some(first) = articles.first() {
                println!("  Latest: {}", first.title.as_deref().unwrap_or("No title"));
            }
        }
        Err(e) => eprintln!("NASDAQ Error: {}", e),
    }
    
    // MarketWatch
    println!("\n=== MarketWatch ===");
    let market_watch = news_client.market_watch();
    match market_watch.top_stories().await {
        Ok(articles) => {
            println!("MarketWatch Top Stories: {} articles", articles.len());
            if let Some(first) = articles.first() {
                println!("  Latest: {}", first.title.as_deref().unwrap_or("No title"));
            }
        }
        Err(e) => eprintln!("MarketWatch Error: {}", e),
    }
    
    // Seeking Alpha
    println!("\n=== Seeking Alpha ===");
    let seeking_alpha = news_client.seeking_alpha();
    match seeking_alpha.latest_articles().await {
        Ok(articles) => {
            println!("Seeking Alpha Latest: {} articles", articles.len());
            if let Some(first) = articles.first() {
                println!("  Latest: {}", first.title.as_deref().unwrap_or("No title"));
            }
        }
        Err(e) => eprintln!("Seeking Alpha Error: {}", e),
    }
    
    // CNN Finance
    println!("\n=== CNN Finance ===");
    let cnn_finance = news_client.cnn_finance();
    match cnn_finance.all_stories().await {
        Ok(articles) => {
            println!("CNN Finance All Stories: {} articles", articles.len());
            if let Some(first) = articles.first() {
                println!("  Latest: {}", first.title.as_deref().unwrap_or("No title"));
            }
        }
        Err(e) => eprintln!("CNN Finance Error: {}", e),
    }
    
    // Yahoo Finance
    println!("\n=== Yahoo Finance ===");
    let yahoo_finance = news_client.yahoo_finance();
    match yahoo_finance.news().await {
        Ok(articles) => {
            println!("Yahoo Finance News: {} articles", articles.len());
            if let Some(first) = articles.first() {
                println!("  Latest: {}", first.title.as_deref().unwrap_or("No title"));
            }
            
            // Save a sample to file
            news_client.save_to_file(&articles, "yahoo_finance_sample").await?;
            println!("  Saved sample to examples/responses/yahoo_finance_sample.json");
        }
        Err(e) => eprintln!("Yahoo Finance Error: {}", e),
    }
    
    println!("\nExample completed! Check examples/responses/ for saved files.");
    
    Ok(())
}