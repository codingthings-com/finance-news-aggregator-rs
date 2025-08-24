use crate::error::Result;
use crate::news_source::NewsSource;
use crate::parser::NewsParser;
use crate::types::NewsArticle;
use async_trait::async_trait;
use log::{debug, info};
use reqwest::Client;

/// CNN Finance news client
pub struct CNNFinance {
    base_url: String,
    buzz_url: String,
    client: Client,
    parser: NewsParser,
}

impl CNNFinance {
    /// Create a new CNN Finance client
    pub fn new(client: Client) -> Self {
        Self {
            base_url: "http://rss.cnn.com/rss/{topic}.rss".to_string(),
            buzz_url: "http://rss.cnn.com/cnnmoneymorningbuzz".to_string(),
            client,
            parser: NewsParser::new("cnn_finance"),
        }
    }

    /// Get all stories
    pub async fn all_stories(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed("money_latest").await
    }

    /// Get companies news
    pub async fn companies(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed("money_news_companies").await
    }

    /// Get economy news
    pub async fn economy(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed("money_news_economy").await
    }

    /// Get international news
    pub async fn international(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed("money_news_international").await
    }

    /// Get investing news
    pub async fn investing(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed("money_news_investing").await
    }

    /// Get markets news
    pub async fn markets(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed("money_markets").await
    }

    /// Get media news
    pub async fn media(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed("money_media").await
    }

    /// Get personal finance news
    pub async fn personal_finance(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed("money_pf").await
    }

    /// Get real estate news
    pub async fn real_estate(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed("money_real_estate").await
    }

    /// Get technology news
    pub async fn technology(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed("money_technology").await
    }

    /// Get morning buzz
    pub async fn morning_buzz(&self) -> Result<Vec<NewsArticle>> {
        let url = &self.buzz_url;
        info!("Fetching CNN Finance morning buzz: {}", url);

        let response = self.client.get(url).send().await?;
        let content = response.text().await?;

        debug!("Received {} bytes of content", content.len());

        let mut articles = self.parser.parse_response(&content)?;

        // Set source for all articles
        for article in &mut articles {
            article.source = Some(self.name().to_string());
        }

        info!(
            "Parsed {} articles from CNN Finance morning buzz",
            articles.len()
        );
        Ok(articles)
    }
}

#[async_trait]
impl NewsSource for CNNFinance {
    fn name(&self) -> &'static str {
        "CNN Finance"
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }

    async fn fetch_feed(&self, topic: &str) -> Result<Vec<NewsArticle>> {
        let url = self.base_url.replace("{topic}", topic);
        info!("Fetching CNN Finance feed: {}", url);

        let response = self.client.get(&url).send().await?;
        let content = response.text().await?;

        debug!("Received {} bytes of content", content.len());

        let mut articles = self.parser.parse_response(&content)?;

        // Set source for all articles
        for article in &mut articles {
            article.source = Some(self.name().to_string());
        }

        info!(
            "Parsed {} articles from CNN Finance topic {}",
            articles.len(),
            topic
        );
        Ok(articles)
    }

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
