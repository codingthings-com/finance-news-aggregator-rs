use crate::error::Result;
use crate::news_source::NewsSource;
use crate::parser::NewsParser;
use crate::types::NewsArticle;
use async_trait::async_trait;
use reqwest::Client;
use std::collections::HashMap;

/// MarketWatch news client
/// 
/// Provides access to MarketWatch RSS feeds covering market news, personal finance,
/// stocks, mutual funds, and more.
pub struct MarketWatch {
    url_map: HashMap<String, String>,
    client: Client,
    parser: NewsParser,
    topic_categories: HashMap<&'static str, &'static str>,
}

impl MarketWatch {
    /// Create a new MarketWatch client
    /// 
    /// Initializes the client with MarketWatch RSS feed URL patterns and topic mappings.
    pub fn new(client: Client) -> Self {
        let mut url_map = HashMap::new();
        url_map.insert("base".to_string(), "http://feeds.marketwatch.com/marketwatch/{topic}/".to_string());
        
        let mut topic_categories = HashMap::new();
        // RSS feed IDs for MarketWatch topics
        topic_categories.insert("top_stories", "topstories");
        topic_categories.insert("real_time_headlines", "realtimeheadlines");
        topic_categories.insert("market_pulse", "marketpulse");
        topic_categories.insert("bulletins", "bulletins");
        topic_categories.insert("personal_finance", "pf");
        topic_categories.insert("stocks_to_watch", "StockstoWatch");
        topic_categories.insert("internet_stories", "Internet");
        topic_categories.insert("mutual_funds", "mutualfunds");
        topic_categories.insert("software_stories", "software");
        topic_categories.insert("banking_and_finance", "financial");
        topic_categories.insert("commentary", "commentary");
        topic_categories.insert("newsletter_and_research", "newslettersandresearch");
        topic_categories.insert("auto_reviews", "autoreviews");

        Self {
            url_map,
            client,
            parser: NewsParser::new("market_watch"),
            topic_categories,
        }
    }

    /// Get top stories
    pub async fn top_stories(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("top_stories").await
    }

    /// Get real time headlines
    pub async fn real_time_headlines(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("real_time_headlines").await
    }

    /// Get market pulse
    pub async fn market_pulse(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("market_pulse").await
    }

    /// Get bulletins
    pub async fn bulletins(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("bulletins").await
    }

    /// Get personal finance
    pub async fn personal_finance(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("personal_finance").await
    }

    /// Get stocks to watch
    pub async fn stocks_to_watch(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("stocks_to_watch").await
    }

    /// Get internet stories
    pub async fn internet_stories(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("internet_stories").await
    }

    /// Get mutual funds
    pub async fn mutual_funds(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("mutual_funds").await
    }

    /// Get software stories
    pub async fn software_stories(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("software_stories").await
    }

    /// Get banking and finance
    pub async fn banking_and_finance(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("banking_and_finance").await
    }

    /// Get commentary
    pub async fn commentary(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("commentary").await
    }

    /// Get newsletter and research
    pub async fn newsletter_and_research(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("newsletter_and_research").await
    }

    /// Get auto reviews
    pub async fn auto_reviews(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("auto_reviews").await
    }
}

#[async_trait]
impl NewsSource for MarketWatch {
    fn name(&self) -> &'static str {
        "MarketWatch"
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

    // Override build_topic_url to map topic names to feed IDs
    fn build_topic_url(&self, topic: &str) -> Result<String> {
        let topic_id = self.topic_categories
            .get(topic)
            .ok_or_else(|| crate::error::FanError::InvalidUrl(format!("Invalid topic: {}", topic)))?;
        
        let base_url = self.url_map()
            .get("base")
            .ok_or_else(|| crate::error::FanError::InvalidUrl("Base URL not found".to_string()))?;
        
        Ok(base_url.replace("{topic}", topic_id))
    }

    // Uses default fetch_topic implementation

    fn available_topics(&self) -> Vec<&'static str> {
        self.topic_categories.keys().copied().collect()
    }
}
