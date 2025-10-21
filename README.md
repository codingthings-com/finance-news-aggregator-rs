# Finance News Aggregator (finance-news-aggregator-rs)

A Rust port of the Python finance-news-aggregator ([link](https://github.com/areed1192/finance-news-aggregator)) library for aggregating financial news from various sources.

Available at: https://crates.io/crates/finance-news-aggregator-rs


## Currently Supported Sources

- **Wall Street Journal (WSJ)**: Opinions, World News, US Business, Market News, Technology, Lifestyle
- **CNBC**: Top News, World News, Business, Technology, Investing, and 20+ other categories
- **NASDAQ**: Original Content, Commodities, Cryptocurrency, Dividends, Earnings, Economics, etc.
- **MarketWatch**: Top Stories, Real-time Headlines, Market Pulse, Bulletins, Personal Finance, etc.
- **Seeking Alpha**: Latest Articles, Market News, Long/Short Ideas, IPO Analysis, Transcripts, etc.
- **CNN Finance**: All Stories, Companies, Economy, Markets, Media, Technology, etc.
- **Yahoo Finance**: General News

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
finance-news-aggregator-rs = "0.1.4"
```

or

```toml
[dependencies]
finance-news-aggregator-rs = { git = "https://github.com/codingthings-com/finance-news-aggregator-rs.git" }
```

## Usage

### Library Usage

```rust
use finance_news_aggregator_rs::{NewsClient, Result};
use finance_news_aggregator_rs::types::SourceConfig;
use finance_news_aggregator_rs::news_source::NewsSource;

#[tokio::main]
async fn main() -> Result<()> {
    // Default configuration
    let mut client = NewsClient::new();
    
    // Or with custom configuration
    let config = SourceConfig::default()
        .with_timeout(60)
        .with_user_agent("My News Bot 1.0")
        .with_retries(5, 2000);
    let mut custom_client = NewsClient::with_config(config);
    
    // Get WSJ client and use convenience methods
    let wsj = client.wsj();
    let opinions = wsj.opinions().await?;
    println!("Found {} WSJ opinion articles", opinions.len());
    
    // Get CNBC client
    let cnbc = client.cnbc();
    let top_news = cnbc.top_news().await?;
    println!("Found {} CNBC top news articles", top_news.len());
    
    // Use the generic topic-based API
    // First, get available topics
    let topics = cnbc.available_topics();
    println!("Available CNBC topics: {:?}", topics);
    
    // Then fetch by topic name
    let tech_news = cnbc.fetch_topic("technology").await?;
    println!("Found {} technology articles", tech_news.len());
    
    // Or fetch from any RSS URL directly
    let custom_url = "https://feeds.a.dj.com/rss/RSSOpinion.xml";
    let articles = wsj.fetch_feed_by_url(custom_url).await?;
    println!("Found {} articles from custom URL", articles.len());
    
    // Save to file
    client.save_to_file(&opinions, "wsj_opinions").await?;
    
    Ok(())
}
```

### New Architecture Features

The library now provides three ways to fetch news:

1. **Convenience Methods**: Source-specific methods like `wsj.opinions()`, `cnbc.top_news()`
2. **Topic-Based Fetching**: Generic `fetch_topic(topic)` method that works across all sources
3. **Direct URL Fetching**: Generic `fetch_feed_by_url(url)` for fetching any RSS feed

Each news source maintains a URL map with named endpoints, making it easy to extend and customize feed sources.

See src/news_source/*.rs for the available feeds for each source.


## Examples

Run the examples:

```bash
# All sources example
cargo run --example all_sources_example

# Topic-based API example (demonstrates new features)
cargo run --example topic_based_example

# Configuration example
cargo run --example config_example
```

## Development

### Running Tests

#### Unit Tests
Run all unit tests:
```bash
cargo test --lib
```

#### Integration Tests
Run all integration tests:
```bash
cargo test --test '*'
```

Run specific integration test files:
```bash
# CNN Finance integration tests
cargo test --test test_cnn_finance_integration

# CNBC integration tests  
cargo test --test test_cnbc_integration
```

#### Deprecation Detection Tests
Run CNN Finance deprecation detection tests specifically:
```bash
# Test base URL endpoint availability
cargo test test_cnn_finance_base_url_endpoint_availability --test test_cnn_finance_integration

# Test buzz URL endpoint availability
cargo test test_cnn_finance_buzz_url_endpoint_availability --test test_cnn_finance_integration

# Test for deprecated feed categories
cargo test test_cnn_finance_deprecated_feed_categories --test test_cnn_finance_integration

# Comprehensive endpoint monitoring with deprecation tracker
cargo test test_cnn_finance_endpoint_monitoring_with_deprecation_tracker --test test_cnn_finance_integration
```

#### Data Validation Tests
Run data validation and quality tests:
```bash
# CNN Finance data validation
cargo test test_cnn_finance_article_structure_validation --test test_cnn_finance_integration
cargo test test_cnn_finance_newsarticle_data_quality --test test_cnn_finance_integration
```

#### Run All Tests
Run both unit and integration tests:
```bash
cargo test
```

#### Test Output
The integration tests provide detailed output including:
- Endpoint availability status
- Article count per feed
- Deprecation warnings and recommendations
- Error classification (404/403 vs temporary failures)
- Comprehensive deprecation reports

Example output:
```
✓ CNN Finance base_url endpoint is available
✓ Category 'money_latest' is working (15 articles)
✗ Category 'deprecated_topic' appears deprecated (404)
WARNING: CNN Finance has critical failures that may indicate deprecated endpoints
```

### Building

```bash
cargo build --release
```

### Adding New Sources

1. Create a new module in `src/news_source/`
2. Implement the news source client
3. Add parser support in `src/parser.rs`
4. Export the new source in `src/news_source/mod.rs`
5. Add client method in `src/news_client.rs`

### Testing Framework

The project includes a comprehensive integration testing framework with:

#### Deprecation Detection
- **Endpoint Monitoring**: Automatically detects when news source endpoints become unavailable
- **Error Classification**: Distinguishes between permanent deprecation (404/403/DNS errors) and temporary issues
- **Feed Category Tracking**: Monitors individual topic categories for deprecation
- **Deprecation Reports**: Generates detailed reports with removal recommendations

#### Data Validation
- **Article Structure Validation**: Ensures news articles have proper structure and required fields
- **Content Quality Checks**: Validates article titles, descriptions, and URLs
- **Source Attribution**: Verifies correct source attribution for all articles

#### Test Utilities
- **Integration Test Config**: Configurable test parameters and timeouts
- **Deprecation Tracker**: Utility for tracking and reporting endpoint failures
- **Article Validation Rules**: Flexible validation rules for different data quality requirements
- **Client Factory**: Standardized HTTP client creation for consistent testing

## Architecture

### NewsSource Trait

All news sources implement the `NewsSource` trait, which provides:

- `name()` - Returns the source name
- `url_map()` - Returns a HashMap of named URLs (e.g., "base", "buzz", "original")
- `client()` - Returns the HTTP client
- `parser()` - Returns the RSS parser
- `fetch_feed_by_url(url)` - Generic method to fetch any RSS feed (default implementation)
- `fetch_topic(topic)` - Fetch news by topic name (source-specific implementation)
- `available_topics()` - List all available topics for the source

### URL Mapping

Each news source maintains a `HashMap<String, String>` for URL management:

```rust
// Example: CNN Finance URL map
{
    "base": "http://rss.cnn.com/rss/{topic}.rss",
    "buzz": "http://rss.cnn.com/cnnmoneymorningbuzz"
}
```

This design allows:
- Multiple URL patterns per source
- Easy addition of new endpoints
- Clear separation between URL templates and topic logic

### Topic-Based Fetching

The `fetch_topic()` method maps friendly topic names to actual feed URLs:

```rust
// Client code
let articles = cnn.fetch_topic("money_latest").await?;

// Internally maps to:
// http://rss.cnn.com/rss/money_latest.rss
```

Each source implements its own topic-to-URL mapping logic, handling:
- URL template substitution
- Query parameter construction
- Special case endpoints (e.g., "morning_buzz" for CNN)

## Project Structure

```
finance-news-aggregator-rs/
├── src/
│   ├── lib.rs               # Library root
│   ├── news_client.rs       # Main news client
│   ├── error.rs             # Error types
│   ├── parser.rs            # RSS/XML parser
│   ├── types.rs             # Common types
│   └── news_source/         # News source implementations
│       ├── mod.rs           # NewsSource trait with default implementations
│       ├── wsj.rs           # Wall Street Journal
│       ├── cnbc.rs          # CNBC
│       ├── cnn_finance.rs   # CNN Finance
│       ├── yahoo_finance.rs # Yahoo Finance
│       ├── nasdaq.rs        # NASDAQ
│       ├── market_watch.rs  # MarketWatch
│       └── seeking_alpha.rs # Seeking Alpha
├── examples/
│   ├── all_sources_example.rs  # Usage examples
│   └── config_example.rs       # Configuration examples
└── Cargo.toml
```

## Error Handling

The library uses a custom `Result<T>` type with comprehensive error variants:

- `Http` - HTTP request errors
- `XmlParsing` - RSS/XML parsing errors with detailed context
- `JsonSerialization` - JSON serialization errors
- `Io` - File I/O errors
- `InvalidUrl` - URL validation errors
- `FeedParsing` - Feed parsing errors

The XML parser gracefully handles malformed feeds and invalid UTF-8 sequences, ensuring robust operation across different news sources.

## Logging

Logging can be updated with the `RUST_LOG` environment variable.

```
export RUST_LOG=debug 
```


## Features

- **Async/Await Support**: Built with Tokio for efficient async operations
- **Modular Design**: Easy to extend with new news sources
- **Flexible Fetching**: Three ways to fetch news - convenience methods, topic-based, or direct URL
- **URL Mapping**: Each source maintains a HashMap of named URLs for easy customization
- **Generic Feed Fetcher**: Common `fetch_feed_by_url()` implementation shared across all sources
- **Topic Discovery**: `available_topics()` method to discover supported feeds for each source
- **Error Handling**: Comprehensive error types with `thiserror`
- **Robust XML Parsing**: Handles various RSS/XML formats with namespace support
- **UTF-8 Safety**: Graceful handling of invalid UTF-8 sequences in feeds
- **Logging**: Built-in logging with `env_logger`
- **JSON Export**: Save articles to JSON files
- **Type Safety**: Strongly typed with Serde serialization
- **Configurable**: Customizable timeouts, user agents, and retry policies
- **Integration Testing**: Comprehensive test suite with deprecation detection
- **Endpoint Monitoring**: Automated detection of deprecated news feeds and endpoints


## License

MIT License - see LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request
