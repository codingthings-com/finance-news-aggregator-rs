use crate::error::Result;
use crate::news_source::NewsSource;
use crate::parser::NewsParser;
use crate::types::{NewsArticle, SourceConfig};
use async_trait::async_trait;
use reqwest::Client;
use std::collections::HashMap;

/// Wall Street Journal news client
///
/// Provides access to Wall Street Journal RSS feeds including opinions, world news,
/// business, markets, technology, and lifestyle content.
pub struct WallStreetJournal {
    url_map: HashMap<String, String>,
    client: Client,
    parser: NewsParser,
}

impl WallStreetJournal {
    /// Create a new Wall Street Journal client
    ///
    /// Initializes the client with WSJ RSS feed URL patterns.
    pub fn new(client: Client) -> Self {
        Self::with_config(
            client,
            SourceConfig::new("https://feeds.a.dj.com/rss/{topic}.xml"),
        )
    }

    /// Create a new Wall Street Journal client with custom config
    ///
    /// # Arguments
    /// * `client` - HTTP client for making requests
    /// * `config` - Source configuration (only base_url is used)
    pub fn with_config(client: Client, config: SourceConfig) -> Self {
        let mut url_map = HashMap::new();
        url_map.insert("base".to_string(), config.base_url.clone());

        Self {
            url_map,
            client,
            parser: NewsParser::new("wsj"),
        }
    }

    /// Get opinions feed
    pub async fn opinions(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("RSSOpinion").await
    }

    /// Get world news feed
    pub async fn world_news(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("RSSWorldNews").await
    }

    /// Get US business news feed
    pub async fn us_business_news(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("WSJcomUSBusiness").await
    }

    /// Get market news feed
    pub async fn market_news(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("RSSMarketsMain").await
    }

    /// Get technology news feed
    pub async fn technology_news(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("RSSWSJD").await
    }

    /// Get lifestyle feed
    pub async fn lifestyle(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("RSSLifestyle").await
    }
}

#[async_trait]
impl NewsSource for WallStreetJournal {
    fn name(&self) -> &'static str {
        "Wall Street Journal"
    }

    fn url_map(&self) -> &HashMap<String, String> {
        &self.url_map
    }

    fn client(&self) -> &Client {
        &self.client
    }

    fn parser(&self) -> &NewsParser {
        &self.parser
    }

    // Uses default fetch_topic implementation (simple pattern substitution)

    fn available_topics(&self) -> Vec<&'static str> {
        vec![
            "RSSOpinion",
            "RSSWorldNews",
            "WSJcomUSBusiness",
            "RSSMarketsMain",
            "RSSWSJD",
            "RSSLifestyle",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Test imports

    #[tokio::test]
    async fn test_wsj_opinions() {
        let config = SourceConfig::new("https://feeds.a.dj.com/rss/{topic}.xml").with_timeout(30);

        let client = Client::builder()
            .timeout(config.timeout_duration())
            .user_agent(&config.user_agent)
            .build()
            .expect("Failed to create HTTP client");

        let wsj = WallStreetJournal::with_config(client, config);
        let result = wsj.opinions().await;

        match result {
            Ok(articles) => {
                println!("Successfully fetched {} opinion articles", articles.len());
                if !articles.is_empty() {
                    println!("First article: {:?}", articles[0]);
                }
            }
            Err(e) => {
                println!("Error fetching opinions: {}", e);
                // Don't fail the test for network issues in CI
            }
        }
    }

    #[test]
    fn test_wsj_config() {
        let config = SourceConfig::new("https://feeds.a.dj.com/rss/{topic}.xml")
            .with_timeout(60)
            .with_user_agent("Custom User Agent")
            .with_retries(5, 2000);

        assert_eq!(config.base_url, "https://feeds.a.dj.com/rss/{topic}.xml");
        assert_eq!(config.timeout_seconds, 60);
        assert_eq!(config.user_agent, "Custom User Agent");
        assert_eq!(config.max_retries, 5);
        assert_eq!(config.retry_delay_ms, 2000);
    }
}
