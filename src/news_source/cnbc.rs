use crate::error::Result;
use crate::news_source::NewsSource;
use crate::parser::NewsParser;
use crate::types::{NewsArticle, SourceConfig};
use async_trait::async_trait;
use reqwest::Client;
use std::collections::HashMap;

/// CNBC news client
/// 
/// Provides access to CNBC RSS feeds covering business news, markets, technology,
/// politics, healthcare, and more across global markets.
pub struct CNBC {
    url_map: HashMap<String, String>,
    client: Client,
    parser: NewsParser,
    topic_categories: HashMap<&'static str, u32>,
}

impl CNBC {
    /// Create a new CNBC client
    /// 
    /// Initializes the client with CNBC RSS feed URL patterns and topic ID mappings.
    pub fn new(client: Client) -> Self {
        Self::with_config(client, SourceConfig::new("https://www.cnbc.com/id/{topic_id}/device/rss/rss.html"))
    }

    /// Create a new CNBC client with custom config
    /// 
    /// # Arguments
    /// * `client` - HTTP client for making requests
    /// * `config` - Source configuration (only base_url is used)
    pub fn with_config(client: Client, config: SourceConfig) -> Self {
        let mut url_map = HashMap::new();
        url_map.insert("base".to_string(), config.base_url.clone());
        
        let mut topic_categories = HashMap::new();
        // RSS feed IDs for CNBC topics
        topic_categories.insert("top_news", 100003114);
        topic_categories.insert("world_news", 100727362);
        topic_categories.insert("us_news", 15837362);
        topic_categories.insert("asia_news", 19832390);
        topic_categories.insert("europe_news", 19794221);
        topic_categories.insert("business", 10001147);
        topic_categories.insert("earnings", 15839135);
        topic_categories.insert("commentary", 100370673);
        topic_categories.insert("economy", 20910258);
        topic_categories.insert("finance", 10000664);
        topic_categories.insert("technology", 19854910);
        topic_categories.insert("politics", 10000113);
        topic_categories.insert("health_care", 10000108);
        topic_categories.insert("real_estate", 10000115);
        topic_categories.insert("wealth", 10001054);
        topic_categories.insert("autos", 10000101);
        topic_categories.insert("energy", 19836768);
        topic_categories.insert("media", 10000110);
        topic_categories.insert("retail", 10000116);
        topic_categories.insert("travel", 10000739);
        topic_categories.insert("small_business", 44877279);
        topic_categories.insert("investing", 15839069);
        topic_categories.insert("financial_advisors", 100646281);
        topic_categories.insert("personal_finance", 21324812);

        Self {
            url_map,
            client,
            parser: NewsParser::new("cnbc"),
            topic_categories,
        }
    }

    /// Get top news
    pub async fn top_news(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("top_news").await
    }

    /// Get world news
    pub async fn world_news(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("world_news").await
    }

    /// Get business news
    pub async fn business(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("business").await
    }

    /// Get technology news
    pub async fn technology(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("technology").await
    }

    /// Get investing news
    pub async fn investing(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("investing").await
    }
}

#[async_trait]
impl NewsSource for CNBC {
    fn name(&self) -> &'static str {
        "CNBC"
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

    // Override build_topic_url to map topic names to numeric IDs
    fn build_topic_url(&self, topic: &str) -> Result<String> {
        let topic_id = self.topic_categories
            .get(topic)
            .ok_or_else(|| crate::error::FanError::InvalidUrl(format!("Invalid topic: {}", topic)))?;
        
        let base_url = self.url_map()
            .get("base")
            .ok_or_else(|| crate::error::FanError::InvalidUrl("Base URL not found".to_string()))?;
        
        Ok(base_url.replace("{topic_id}", &topic_id.to_string()))
    }

    // Uses default fetch_topic implementation

    fn available_topics(&self) -> Vec<&'static str> {
        self.topic_categories.keys().copied().collect()
    }
}
