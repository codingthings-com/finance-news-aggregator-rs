# API Usage Guide

## Three Ways to Fetch News

The finance-news-aggregator-rs library now provides three complementary ways to fetch news articles, giving you flexibility based on your use case.

### 1. Convenience Methods (Traditional)

Use source-specific methods for common feeds. This is the simplest approach.

```rust
use finance_news_aggregator_rs::NewsClient;

let mut client = NewsClient::new();

// Wall Street Journal
let wsj = client.wsj();
let opinions = wsj.opinions().await?;
let world_news = wsj.world_news().await?;
let tech_news = wsj.technology_news().await?;

// CNBC
let cnbc = client.cnbc();
let top_news = cnbc.top_news().await?;
let technology = cnbc.technology().await?;

// CNN Finance
let cnn = client.cnn_finance();
let all_stories = cnn.all_stories().await?;
let markets = cnn.markets().await?;
let morning_buzz = cnn.morning_buzz().await?;
```

**When to use:**
- You know exactly which feed you want
- You prefer readable, self-documenting code
- You want IDE autocomplete support

### 2. Topic-Based Fetching (New)

Use the generic `fetch_topic()` method with topic names. Great for dynamic or configurable applications.

```rust
use finance_news_aggregator_rs::NewsClient;
use finance_news_aggregator_rs::news_source::NewsSource;

let mut client = NewsClient::new();

// Discover available topics
let wsj = client.wsj();
let topics = wsj.available_topics();
println!("Available topics: {:?}", topics);
// Output: ["RSSOpinion", "RSSWorldNews", "WSJcomUSBusiness", ...]

// Fetch by topic name
let articles = wsj.fetch_topic("RSSOpinion").await?;

// Works with any source
let cnbc = client.cnbc();
let tech_articles = cnbc.fetch_topic("technology").await?;

let cnn = client.cnn_finance();
let economy_articles = cnn.fetch_topic("money_news_economy").await?;
```

**When to use:**
- Building configurable applications (e.g., user selects topics)
- Iterating over multiple topics programmatically
- Need to discover available topics at runtime
- Building generic news aggregation tools

**Example: User-configurable news fetcher**
```rust
// Load user preferences
let user_topics = vec!["technology", "investing", "economy"];

let cnbc = client.cnbc();
for topic in user_topics {
    if cnbc.available_topics().contains(&topic) {
        let articles = cnbc.fetch_topic(topic).await?;
        println!("Found {} articles for {}", articles.len(), topic);
    }
}
```

### 3. Direct URL Fetching (New)

Use `fetch_feed_by_url()` to fetch from any RSS URL. Perfect for custom feeds or external sources.

```rust
use finance_news_aggregator_rs::NewsClient;
use finance_news_aggregator_rs::news_source::NewsSource;

let mut client = NewsClient::new();
let wsj = client.wsj();

// Fetch from any RSS URL
let custom_url = "https://feeds.a.dj.com/rss/RSSMarketsMain.xml";
let articles = wsj.fetch_feed_by_url(custom_url).await?;

// Works with external feeds too
let external_url = "https://example.com/custom-feed.xml";
let external_articles = wsj.fetch_feed_by_url(external_url).await?;
```

**When to use:**
- Fetching from custom or external RSS feeds
- Testing new feed URLs before adding them to the library
- Building feed aggregators that accept user-provided URLs
- Bypassing topic mapping for direct URL access

## URL Map Inspection

Each news source maintains a map of named URLs. You can inspect these to understand the source's structure.

```rust
use finance_news_aggregator_rs::NewsClient;
use finance_news_aggregator_rs::news_source::NewsSource;

let mut client = NewsClient::new();

// Inspect CNN Finance URLs
let cnn = client.cnn_finance();
let url_map = cnn.url_map();

for (name, url) in url_map {
    println!("{}: {}", name, url);
}
// Output:
// base: http://rss.cnn.com/rss/{topic}.rss
// buzz: http://rss.cnn.com/cnnmoneymorningbuzz
```

## Complete Example: Multi-Source News Aggregator

Here's a complete example that combines all three approaches:

```rust
use finance_news_aggregator_rs::{NewsClient, Result};
use finance_news_aggregator_rs::news_source::NewsSource;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = NewsClient::new();
    let mut all_articles = Vec::new();

    // 1. Use convenience methods for known feeds
    let wsj = client.wsj();
    all_articles.extend(wsj.opinions().await?);
    
    // 2. Use topic-based fetching for dynamic selection
    let cnbc = client.cnbc();
    let user_selected_topics = vec!["technology", "investing"];
    
    for topic in user_selected_topics {
        if cnbc.available_topics().contains(&topic) {
            all_articles.extend(cnbc.fetch_topic(topic).await?);
        }
    }
    
    // 3. Use direct URL fetching for custom feeds
    let custom_feeds = vec![
        "https://feeds.a.dj.com/rss/RSSMarketsMain.xml",
        "https://www.nasdaq.com/feed/nasdaq-original/rss.xml",
    ];
    
    for url in custom_feeds {
        all_articles.extend(wsj.fetch_feed_by_url(url).await?);
    }
    
    println!("Total articles collected: {}", all_articles.len());
    
    // Save to file
    client.save_to_file(&all_articles, "aggregated_news").await?;
    
    Ok(())
}
```

## Topic Names Reference

### Wall Street Journal (WSJ)
- `RSSOpinion` - Opinion articles
- `RSSWorldNews` - World news
- `WSJcomUSBusiness` - US business news
- `RSSMarketsMain` - Market news
- `RSSWSJD` - Technology news
- `RSSLifestyle` - Lifestyle articles

### CNBC
- `top_news` - Top news stories
- `world_news` - World news
- `business` - Business news
- `technology` - Technology news
- `investing` - Investment news
- `economy` - Economic news
- `finance` - Financial news
- And 17 more topics (use `available_topics()` to see all)

### CNN Finance
- `money_latest` - Latest financial news
- `money_news_companies` - Company news
- `money_news_economy` - Economic news
- `money_news_investing` - Investment news
- `money_markets` - Market news
- `money_technology` - Technology news
- `morning_buzz` - Morning buzz (special endpoint)
- And more...

### NASDAQ
- `original` - Original NASDAQ content (special endpoint)
- `commodities` - Commodities news
- `cryptocurrency` - Cryptocurrency news
- `earnings` - Earnings reports
- `stocks` - Stock news
- `technology` - Technology news
- And more...

### Yahoo Finance
- `topstories` - Top stories
- `headlines` - Headlines

### MarketWatch
- `top_stories` - Top stories
- `real_time_headlines` - Real-time headlines
- `market_pulse` - Market pulse
- `personal_finance` - Personal finance
- And more...

### Seeking Alpha
- `latest-articles` - Latest articles
- `market-news` - Market news
- `long-ideas` - Long investment ideas
- `short-ideas` - Short investment ideas
- `ipo-analysis` - IPO analysis
- `transcripts` - Earnings transcripts
- And more...

## Best Practices

1. **Use convenience methods** for simple, known use cases
2. **Use topic-based fetching** for configurable or dynamic applications
3. **Use direct URL fetching** for custom feeds or testing
4. **Always check `available_topics()`** before using topic names
5. **Handle errors gracefully** - feeds can be temporarily unavailable
6. **Respect rate limits** - don't hammer the RSS feeds
7. **Cache results** when appropriate to reduce load

## Error Handling

All fetch methods return `Result<Vec<NewsArticle>>`, so always handle potential errors:

```rust
match wsj.fetch_topic("RSSOpinion").await {
    Ok(articles) => {
        println!("Successfully fetched {} articles", articles.len());
        // Process articles...
    }
    Err(e) => {
        eprintln!("Error fetching articles: {}", e);
        // Handle error (retry, log, fallback, etc.)
    }
}
```

## Performance Tips

1. **Reuse the NewsClient**: Create one client and reuse it
2. **Parallel fetching**: Use `tokio::join!` for concurrent requests
3. **Batch processing**: Collect articles before processing
4. **Timeout configuration**: Set appropriate timeouts for your use case

```rust
// Parallel fetching example
let (wsj_result, cnbc_result) = tokio::join!(
    wsj.fetch_topic("RSSOpinion"),
    cnbc.fetch_topic("technology")
);
```
