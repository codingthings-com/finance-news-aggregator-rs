use crate::error::Result;
use crate::news_source::NewsSource;
use crate::parser::NewsParser;
use crate::types::NewsArticle;
use async_trait::async_trait;
use reqwest::Client;
use std::collections::HashMap;

/// NASDAQ news client
/// 
/// Provides access to NASDAQ RSS feeds covering stocks, commodities, cryptocurrency,
/// earnings, economics, and technology news.
pub struct NASDAQ {
    url_map: HashMap<String, String>,
    client: Client,
    parser: NewsParser,
}

impl NASDAQ {
    /// Create a new NASDAQ client
    /// 
    /// Initializes the client with NASDAQ RSS feed URLs.
    pub fn new(client: Client) -> Self {
        let mut url_map = HashMap::new();
        url_map.insert("base".to_string(), "https://www.nasdaq.com/feed/rssoutbound".to_string());
        url_map.insert("original".to_string(), "https://www.nasdaq.com/feed/nasdaq-original/rss.xml".to_string());
        
        Self {
            url_map,
            client,
            parser: NewsParser::new("nasdaq"),
        }
    }

    /// Get original content feed
    pub async fn original_content(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("original").await
    }

    /// Get commodities feed
    pub async fn commodities(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("commodities").await
    }

    /// Get cryptocurrency feed
    pub async fn cryptocurrency(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("cryptocurrency").await
    }

    /// Get dividends feed
    pub async fn dividends(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("dividends").await
    }

    /// Get earnings feed
    pub async fn earnings(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("earnings").await
    }

    /// Get economics feed
    pub async fn economics(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("economics").await
    }

    /// Get financial advisors feed
    pub async fn financial_advisors(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("financial-advisors").await
    }

    /// Get innovation feed
    pub async fn innovation(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("innovation").await
    }

    /// Get stocks feed
    pub async fn stocks(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("stocks").await
    }

    /// Get technology feed
    pub async fn technology(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("technology").await
    }
}

#[async_trait]
impl NewsSource for NASDAQ {
    fn name(&self) -> &'static str {
        "NASDAQ"
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

    // Override build_topic_url to handle special "original" endpoint and query parameters
    fn build_topic_url(&self, topic: &str) -> Result<String> {
        if topic == "original" {
            // Special case: original content has its own dedicated URL
            self.url_map()
                .get("original")
                .ok_or_else(|| crate::error::FanError::InvalidUrl("Original URL not found".to_string()))
                .map(|s| s.clone())
        } else {
            // Standard topics use the base URL with category parameter
            let base_url = self.url_map()
                .get("base")
                .ok_or_else(|| crate::error::FanError::InvalidUrl("Base URL not found".to_string()))?;
            Ok(format!("{}?category={}", base_url, topic))
        }
    }

    // Uses default fetch_topic implementation

    fn available_topics(&self) -> Vec<&'static str> {
        vec![
            "original",
            "commodities",
            "cryptocurrency",
            "dividends",
            "earnings",
            "economics",
            "financial-advisors",
            "innovation",
            "stocks",
            "technology",
        ]
    }
}
