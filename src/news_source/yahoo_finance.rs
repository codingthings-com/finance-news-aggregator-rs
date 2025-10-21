use crate::error::Result;
use crate::news_source::NewsSource;
use crate::parser::NewsParser;
use crate::types::NewsArticle;
use async_trait::async_trait;
use reqwest::Client;
use std::collections::HashMap;

/// Yahoo Finance news client
///
/// Provides access to Yahoo Finance RSS feeds for financial news and market updates.
pub struct YahooFinance {
    url_map: HashMap<String, String>,
    client: Client,
    parser: NewsParser,
}

impl YahooFinance {
    /// Create a new Yahoo Finance client
    ///
    /// Initializes the client with Yahoo Finance RSS feed URLs.
    /// Note: The old feeds.finance.yahoo.com/rss/2.0 endpoint is no longer available.
    pub fn new(client: Client) -> Self {
        let mut url_map = HashMap::new();
        url_map.insert(
            "base".to_string(),
            "https://finance.yahoo.com/news/rssindex".to_string(),
        );

        Self {
            url_map,
            client,
            parser: NewsParser::new("yahoo"),
        }
    }

    /// Get general news headlines
    pub async fn headlines(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("headlines").await
    }

    /// Get headlines for specific stock symbols
    ///
    /// # Arguments
    /// * `symbols` - Array of stock symbols (e.g., ["AAPL", "GOOGL", "MSFT"])
    ///
    /// # Returns
    /// News articles related to the specified symbols
    pub async fn headline(&self, symbols: &[&str]) -> Result<Vec<NewsArticle>> {
        let base_url = self
            .url_map
            .get("base")
            .ok_or_else(|| crate::error::FanError::InvalidUrl("Base URL not found".to_string()))?;

        let symbols_str = symbols.join(",");
        let url = format!("{}/headline?s={}", base_url, symbols_str);

        self.fetch_feed_by_url(&url).await
    }

    /// Get top stories and market summary
    pub async fn topstories(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("topstories").await
    }
}

#[async_trait]
impl NewsSource for YahooFinance {
    fn name(&self) -> &'static str {
        "Yahoo Finance"
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

    // Override build_topic_url for Yahoo's URL structure (base/{topic} instead of pattern substitution)
    fn build_topic_url(&self, topic: &str) -> Result<String> {
        let base_url = self
            .url_map()
            .get("base")
            .ok_or_else(|| crate::error::FanError::InvalidUrl("Base URL not found".to_string()))?;

        Ok(format!("{}/{}", base_url, topic))
    }

    // Uses default fetch_topic implementation

    fn available_topics(&self) -> Vec<&'static str> {
        vec!["topstories", "headlines"]
    }
}
