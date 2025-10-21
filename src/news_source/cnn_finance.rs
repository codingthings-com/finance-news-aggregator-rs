use crate::error::Result;
use crate::news_source::NewsSource;
use crate::parser::NewsParser;
use crate::types::NewsArticle;
use async_trait::async_trait;
use reqwest::Client;
use std::collections::HashMap;

/// CNN Finance news client
/// 
/// Provides access to CNN's financial news RSS feeds across multiple categories
/// including latest news, companies, economy, markets, and more.
pub struct CNNFinance {
    url_map: HashMap<String, String>,
    client: Client,
    parser: NewsParser,
}

impl CNNFinance {
    /// Create a new CNN Finance client
    /// 
    /// Initializes the client with predefined URL patterns for CNN Finance RSS feeds.
    pub fn new(client: Client) -> Self {
        let mut url_map = HashMap::new();
        url_map.insert("base".to_string(), "http://rss.cnn.com/rss/{topic}.rss".to_string());
        url_map.insert("buzz".to_string(), "http://rss.cnn.com/cnnmoneymorningbuzz".to_string());
        
        Self {
            url_map,
            client,
            parser: NewsParser::new("cnn_finance"),
        }
    }

    /// Get all stories (latest financial news)
    pub async fn all_stories(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("money_latest").await
    }

    /// Get companies news
    pub async fn companies(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("money_news_companies").await
    }

    /// Get economy news
    pub async fn economy(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("money_news_economy").await
    }

    /// Get international news
    pub async fn international(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("money_news_international").await
    }

    /// Get investing news
    pub async fn investing(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("money_news_investing").await
    }

    /// Get markets news
    pub async fn markets(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("money_markets").await
    }

    /// Get media news
    pub async fn media(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("money_media").await
    }

    /// Get personal finance news
    pub async fn personal_finance(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("money_pf").await
    }

    /// Get real estate news
    pub async fn real_estate(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("money_real_estate").await
    }

    /// Get technology news
    pub async fn technology(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("money_technology").await
    }

    /// Get morning buzz (special feed)
    pub async fn morning_buzz(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("morning_buzz").await
    }
}

#[async_trait]
impl NewsSource for CNNFinance {
    fn name(&self) -> &'static str {
        "CNN Finance"
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

    // Override build_topic_url to handle special "morning_buzz" endpoint
    fn build_topic_url(&self, topic: &str) -> Result<String> {
        if topic == "morning_buzz" {
            // Special case: morning buzz has its own dedicated URL
            self.url_map()
                .get("buzz")
                .ok_or_else(|| crate::error::FanError::InvalidUrl("Buzz URL not found".to_string()))
                .map(|s| s.clone())
        } else {
            // Standard topics use the base URL pattern
            let base_url = self.url_map()
                .get("base")
                .ok_or_else(|| crate::error::FanError::InvalidUrl("Base URL not found".to_string()))?;
            Ok(base_url.replace("{topic}", topic))
        }
    }

    // Uses default fetch_topic implementation

    fn available_topics(&self) -> Vec<&'static str> {
        vec![
            "money_latest",
            "money_news_companies",
            "money_news_economy",
            "money_news_international",
            "money_news_investing",
            "money_markets",
            "money_media",
            "money_pf",
            "money_real_estate",
            "money_technology",
            "morning_buzz",
        ]
    }
}
