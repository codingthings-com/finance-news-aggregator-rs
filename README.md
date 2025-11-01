# Finance News Aggregator (Rust)

A Rust library for aggregating financial news from various RSS feed sources. Port of the Python [finance-news-aggregator](https://github.com/areed1192/finance-news-aggregator).

[![Crates.io](https://img.shields.io/crates/v/finance-news-aggregator-rs)](https://crates.io/crates/finance-news-aggregator-rs)

## Supported Sources

| Source | Working Feeds | Status |
|--------|--------------|--------|
| **CNBC** | 24 topics | ✅ 100% |
| **MarketWatch** | 4 topics | ✅ 100% |
| **NASDAQ** | 10 topics | ✅ 100% |
| **Seeking Alpha** | 12 topics | ✅ 100% |
| **Wall Street Journal** | 6 topics | ✅ 100% |
| **Yahoo Finance** | 2 topics + symbols | ✅ 100% |

## Installation

```toml
[dependencies]
finance-news-aggregator-rs = "0.2.2"
```

## Quick Start

```rust
use finance_news_aggregator_rs::NewsClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = NewsClient::new();
    
    // Get news from any source
    let wsj = client.wsj();
    let articles = wsj.opinions().await?;
    
    println!("Found {} articles", articles.len());
    for article in articles.iter().take(5) {
        println!("- {}", article.title.as_ref().unwrap_or(&"No title".to_string()));
    }
    
    Ok(())
}
```

## Usage

### Basic Usage

```rust
let mut client = NewsClient::new();

// Wall Street Journal
let wsj = client.wsj();
let opinions = wsj.opinions().await?;
let world_news = wsj.world_news().await?;

// CNBC
let cnbc = client.cnbc();
let top_news = cnbc.top_news().await?;
let tech = cnbc.technology().await?;

// NASDAQ
let nasdaq = client.nasdaq();
let tech_news = nasdaq.technology().await?;
let crypto = nasdaq.cryptocurrency().await?;

// Yahoo Finance (with stock symbols)
let yahoo = client.yahoo_finance();
let headlines = yahoo.headlines().await?;
let aapl_news = yahoo.headline(&["AAPL", "MSFT"]).await?;
```

### Generic Source (Any RSS Feed)

Fetch any RSS feed directly without using a specific source:

```rust
use finance_news_aggregator_rs::news_source::NewsSource;

let generic = client.generic();
let articles = generic.fetch_feed_by_url("https://example.com/feed.xml").await?;
```

### Topic-Based API

All sources support a generic topic-based API:

```rust
let cnbc = client.cnbc();

// List available topics
let topics = cnbc.available_topics();
println!("Available topics: {:?}", topics);

// Fetch by topic name
let articles = cnbc.fetch_topic("technology").await?;
```

### Custom Configuration

```rust
use finance_news_aggregator_rs::types::SourceConfig;

let config = SourceConfig::default()
    .with_timeout(60)
    .with_user_agent("My News Bot 1.0")
    .with_retries(5, 2000);

let mut client = NewsClient::with_config(config);
```

### Direct URL Fetching

```rust
let wsj = client.wsj();
let articles = wsj.fetch_feed_by_url("https://feeds.a.dj.com/rss/RSSOpinion.xml").await?;
```

### Save to File

```rust
client.save_to_file(&articles, "news_articles").await?;
// Saves to: examples/responses/news_articles.json
```

## Examples

```bash
# Run all sources example
cargo run --example all_sources_example

# Topic-based API example
cargo run --example topic_based_example

# Configuration example
cargo run --example config_example
```

## Testing

```bash
# Run all tests
cargo test

# Run only unit tests
cargo test --lib

# Run integration tests
cargo test --tests

# Run specific source tests
cargo test --test test_nasdaq_integration
cargo test --test test_cnbc_integration
cargo test --test test_wsj_integration
```

## Available Feeds

### Wall Street Journal (6 feeds)
- `opinions()`, `world_news()`, `us_business_news()`, `market_news()`, `technology_news()`, `lifestyle()`

### CNBC (24 feeds)
- `top_news()`, `world_news()`, `business()`, `technology()`, `investing()`
- Plus: economy, finance, politics, health_care, real_estate, wealth, energy, media, retail, travel, and more

### NASDAQ (10 feeds)
- `original_content()`, `commodities()`, `cryptocurrency()`, `dividends()`, `earnings()`
- `economics()`, `financial_advisors()`, `innovation()`, `stocks()`, `technology()`

### MarketWatch (4 feeds)
- `top_stories()`, `real_time_headlines()`, `market_pulse()`, `bulletins()`

### Seeking Alpha (12 feeds)
- `latest_articles()`, `all_news()`, `market_news()`, `editors_picks()`, `etfs()`, `forex()`
- `ipo_analysis()`, `long_ideas()`, `short_ideas()`, `transcripts()`, `wall_street_breakfast()`, `most_popular_articles()`

### Yahoo Finance (2 feeds + symbols)
- `headlines()`, `topstories()`
- `headline(&["AAPL", "MSFT", ...])` - Get news for specific stock symbols

## Architecture

### NewsSource Trait

All sources implement the `NewsSource` trait:

```rust
pub trait NewsSource {
    fn name(&self) -> &'static str;
    fn url_map(&self) -> &HashMap<String, String>;
    fn client(&self) -> &Client;
    fn parser(&self) -> &NewsParser;
    fn available_topics(&self) -> Vec<&'static str>;
    
    async fn fetch_feed_by_url(&self, url: &str) -> Result<Vec<NewsArticle>>;
    async fn fetch_topic(&self, topic: &str) -> Result<Vec<NewsArticle>>;
}
```

### NewsArticle Structure

```rust
pub struct NewsArticle {
    pub title: Option<String>,
    pub link: Option<String>,
    pub description: Option<String>,
    pub pub_date: Option<String>,
    pub source: Option<String>,
}
```

## Error Handling

```rust
use finance_news_aggregator_rs::error::FanError;

match client.wsj().opinions().await {
    Ok(articles) => println!("Got {} articles", articles.len()),
    Err(FanError::Http(e)) => eprintln!("Network error: {}", e),
    Err(FanError::XmlParsing(e)) => eprintln!("Parse error: {}", e),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Logging

Enable logging with the `RUST_LOG` environment variable:

```bash
RUST_LOG=debug cargo run --example all_sources_example
```

## Project Structure

```
src/
├── lib.rs              # Library root
├── news_client.rs      # Main client
├── error.rs            # Error types
├── parser.rs           # RSS parser
├── types.rs            # Data types
└── news_source/        # Source implementations
    ├── mod.rs          # NewsSource trait
    ├── cnbc.rs
    ├── market_watch.rs
    ├── nasdaq.rs
    ├── seeking_alpha.rs
    ├── wsj.rs
    └── yahoo_finance.rs
```

## License

MIT License - see LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass: `cargo test`
5. Submit a pull request
