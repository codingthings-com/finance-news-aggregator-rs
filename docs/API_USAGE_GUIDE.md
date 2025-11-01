# API Usage Guide

## Quick Start

```rust
use finance_news_aggregator_rs::NewsClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = NewsClient::new();
    
    let wsj = client.wsj();
    let articles = wsj.opinions().await?;
    
    println!("Found {} articles", articles.len());
    Ok(())
}
```

## Three Ways to Fetch News

### 1. Convenience Methods

```rust
let mut client = NewsClient::new();

let wsj = client.wsj();
let opinions = wsj.opinions().await?;

let cnbc = client.cnbc();
let top_news = cnbc.top_news().await?;
```

### 2. Topic-Based Fetching

```rust
use finance_news_aggregator_rs::news_source::NewsSource;

let cnbc = client.cnbc();
let topics = cnbc.available_topics();
let articles = cnbc.fetch_topic("technology").await?;
```

### 3. Direct URL Fetching (Generic Source)

```rust
use finance_news_aggregator_rs::news_source::NewsSource;

// Use the generic source for any RSS feed
let generic = client.generic();
let url = "https://feeds.a.dj.com/rss/RSSOpinion.xml";
let articles = generic.fetch_feed_by_url(url).await?;

// Works with any RSS feed
let custom_feed = generic.fetch_feed_by_url("https://example.com/feed.xml").await?;
```

## Available Feeds

### Wall Street Journal (6)
`opinions()`, `world_news()`, `us_business_news()`, `market_news()`, `technology_news()`, `lifestyle()`

### CNBC (24)
`top_news()`, `world_news()`, `business()`, `technology()`, `investing()`, and 19 more

### NASDAQ (10)
`original_content()`, `commodities()`, `cryptocurrency()`, `dividends()`, `earnings()`, `economics()`, `financial_advisors()`, `innovation()`, `stocks()`, `technology()`

### MarketWatch (4)
`top_stories()`, `real_time_headlines()`, `market_pulse()`, `bulletins()`

### Seeking Alpha (12)
`latest_articles()`, `all_news()`, `market_news()`, `editors_picks()`, `etfs()`, `forex()`, `ipo_analysis()`, `long_ideas()`, `short_ideas()`, `transcripts()`, `wall_street_breakfast()`, `most_popular_articles()`

### Yahoo Finance (2 + symbols)
`headlines()`, `topstories()`, `headline(&["AAPL", "MSFT"])`

## Configuration

```rust
use finance_news_aggregator_rs::types::SourceConfig;

let config = SourceConfig::default()
    .with_timeout(60)
    .with_user_agent("My Bot 1.0")
    .with_retries(5, 2000);

let mut client = NewsClient::with_config(config);
```

## Error Handling

```rust
use finance_news_aggregator_rs::error::FanError;

match wsj.opinions().await {
    Ok(articles) => println!("Got {} articles", articles.len()),
    Err(FanError::Http(e)) => eprintln!("Network error: {}", e),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Advanced Usage

### Parallel Fetching

```rust
let (r1, r2, r3) = tokio::join!(
    wsj.opinions(),
    cnbc.top_news(),
    nasdaq.technology()
);
```

### Multi-Source Aggregation

```rust
let mut all_articles = Vec::new();
all_articles.extend(client.wsj().opinions().await?);
all_articles.extend(client.cnbc().top_news().await?);
client.save_to_file(&all_articles, "all_news").await?;
```

## Best Practices

1. **Reuse the NewsClient** - Create once, use many times
2. **Handle errors gracefully** - Feeds can be temporarily unavailable
3. **Use parallel fetching** - Speed up multi-source requests
4. **Check available topics** - Use `available_topics()` before dynamic fetching
5. **Set appropriate timeouts** - Balance reliability and speed

## Logging

```bash
RUST_LOG=debug cargo run --example all_sources_example
```
