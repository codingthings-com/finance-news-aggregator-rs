# Finance News Aggregator (finance-news-aggregator-rs)

A Rust port of the Python `finnews` library for aggregating financial news from various sources.

## Features

- **Async/Await Support**: Built with Tokio for efficient async operations
- **Modular Design**: Easy to extend with new news sources
- **Error Handling**: Comprehensive error types with `thiserror`
- **Robust XML Parsing**: Handles various RSS/XML formats with namespace support
- **UTF-8 Safety**: Graceful handling of invalid UTF-8 sequences in feeds
- **Logging**: Built-in logging with `env_logger`
- **JSON Export**: Save articles to JSON files
- **Type Safety**: Strongly typed with Serde serialization
- **Configurable**: Customizable timeouts, user agents, and retry policies

## Currently Supported Sources

- **Wall Street Journal (WSJ)**: Opinions, World News, US Business, Market News, Technology, Lifestyle
- **CNBC**: Top News, World News, Business, Technology, Investing, and 20+ other categories
- **NASDAQ**: Original Content, Commodities, Cryptocurrency, Dividends, Earnings, Economics, etc.
- **MarketWatch**: Top Stories, Real-time Headlines, Market Pulse, Bulletins, Personal Finance, etc.
- **S&P Global**: Research, Market Commentary, Index Launches, Methodologies, Performance Reports, etc.
- **Seeking Alpha**: Latest Articles, Market News, Long/Short Ideas, IPO Analysis, Transcripts, etc.
- **CNN Finance**: All Stories, Companies, Economy, Markets, Media, Technology, etc.
- **Yahoo Finance**: General News, Market Summary, Headlines by Symbol, Industry News

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
finance-news-aggregator-rs = "0.1.0"
```

## Usage

### Library Usage

```rust
use fan_rs::{NewsClient, Result};
use fan_rs::types::SourceConfig;

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
    
    // Get WSJ client
    let wsj = client.wsj();
    let opinions = wsj.opinions().await?;
    println!("Found {} WSJ opinion articles", opinions.len());
    
    // Get CNBC client
    let cnbc = client.cnbc();
    let top_news = cnbc.top_news().await?;
    println!("Found {} CNBC top news articles", top_news.len());
    
    // Save to file
    client.save_to_file(&opinions, "wsj_opinions").await?;
    
    Ok(())
}
```

### Command Line Usage

```bash
# Fetch WSJ opinions
cargo run --example all_sources_example


# Enable logging
RUST_LOG=info cargo run --bin fan wsj opinions
```

### Available WSJ Feeds

- `opinions` - Opinion articles
- `world` - World news
- `business` - US business news  
- `market` - Market news
- `tech` - Technology news
- `lifestyle` - Lifestyle articles

## Examples

Run the examples:

```bash
# All sources example
cargo run --example all_sources_example
```

## Development

### Running Tests

```bash
cargo test
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

## Project Structure

```
finance-news-aggregator-rs/
├── src/
│   ├── lib.rs          # Library root
│   ├── news_client.rs       # Main news client
│   ├── error.rs        # Error types
│   ├── parser.rs       # RSS/XML parser
│   ├── types.rs        # Common types
│   └── news_source/        # News source implementations
│       ├── mod.rs
│       └── wsj.rs      # Wall Street Journal
│       └── ...         # ... others
├── examples/
│   └── all_sources_example.rs  # Usage examples
│   └── config_example.rs  # Usage examples
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


## License

MIT License - see LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## Roadmap

- [x] Add CNBC support
- [x] Add NASDAQ support  
- [x] Add MarketWatch support
- [x] Add CNN Finance support
- [x] Add Seeking Alpha support
- [x] Add Yahoo Finance support
- [x] Add S&P Global support
- [ ] Add configuration file support
- [ ] Add filtering and search capabilities
- [ ] Add caching support
- [ ] Add rate limiting
- [ ] Add async streaming support
- [ ] Add WebSocket feeds for real-time data