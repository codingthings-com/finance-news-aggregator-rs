use crate::error::Result;
use crate::news_source::NewsSource;
use crate::parser::NewsParser;
use crate::types::{NewsArticle, SourceConfig};
use async_trait::async_trait;
use log::{debug, info};
use reqwest::Client;
use std::collections::HashMap;

/// CNBC news client
pub struct CNBC {
    config: SourceConfig,
    client: Client,
    parser: NewsParser,
    topic_categories: HashMap<&'static str, u32>,
}

impl CNBC {
    /// Create a new CNBC client
    pub fn new(client: Client) -> Self {
        let config = SourceConfig::new("https://www.cnbc.com/id/{topic_id}/device/rss/rss.html");
        Self::with_config(client, config)
    }

    /// Create a new CNBC client with custom config
    pub fn with_config(client: Client, config: SourceConfig) -> Self {
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
            config,
            client,
            parser: NewsParser::new("cnbc"),
            topic_categories,
        }
    }

    /// Get news feed by topic
    pub async fn news_feed(&self, topic: &str) -> Result<Vec<NewsArticle>> {
        if let Some(&topic_id) = self.topic_categories.get(topic) {
            self.fetch_feed(&topic_id.to_string()).await
        } else {
            Err(crate::error::FanError::InvalidUrl(format!(
                "Invalid topic: {}",
                topic
            )))
        }
    }

    /// Get top news
    pub async fn top_news(&self) -> Result<Vec<NewsArticle>> {
        self.news_feed("top_news").await
    }

    /// Get world news
    pub async fn world_news(&self) -> Result<Vec<NewsArticle>> {
        self.news_feed("world_news").await
    }

    /// Get business news
    pub async fn business(&self) -> Result<Vec<NewsArticle>> {
        self.news_feed("business").await
    }

    /// Get technology news
    pub async fn technology(&self) -> Result<Vec<NewsArticle>> {
        self.news_feed("technology").await
    }

    /// Get investing news
    pub async fn investing(&self) -> Result<Vec<NewsArticle>> {
        self.news_feed("investing").await
    }
}

#[async_trait]
impl NewsSource for CNBC {
    fn name(&self) -> &'static str {
        "CNBC"
    }

    fn base_url(&self) -> &str {
        &self.config.base_url
    }

    async fn fetch_feed(&self, topic_id: &str) -> Result<Vec<NewsArticle>> {
        let url = self.config.base_url.replace("{topic_id}", topic_id);
        info!("Fetching CNBC feed: {}", url);

        let response = self.client.get(&url).send().await?;
        let content = response.text().await?;

        debug!("Received {} bytes of content", content.len());

        let mut articles = self.parser.parse_response(&content)?;

        // Set source for all articles
        for article in &mut articles {
            article.source = Some(self.name().to_string());
        }

        info!(
            "Parsed {} articles from CNBC topic {}",
            articles.len(),
            topic_id
        );
        Ok(articles)
    }

    fn available_topics(&self) -> Vec<&'static str> {
        self.topic_categories.keys().copied().collect()
    }
}
