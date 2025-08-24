use crate::error::Result;
use crate::news_source::NewsSource;
use crate::parser::NewsParser;
use crate::types::NewsArticle;
use async_trait::async_trait;
use log::{debug, info};
use reqwest::Client;
use std::collections::HashMap;

/// MarketWatch news client
pub struct MarketWatch {
    base_url: String,
    client: Client,
    parser: NewsParser,
    topic_categories: HashMap<&'static str, &'static str>,
}

impl MarketWatch {
    /// Create a new MarketWatch client
    pub fn new(client: Client) -> Self {
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
            base_url: "http://feeds.marketwatch.com/marketwatch/{topic}/".to_string(),
            client,
            parser: NewsParser::new("market_watch"),
            topic_categories,
        }
    }

    /// Get news feed by topic
    pub async fn news_feed(&self, topic: &str) -> Result<Vec<NewsArticle>> {
        if let Some(&topic_id) = self.topic_categories.get(topic) {
            self.fetch_feed(topic_id).await
        } else {
            Err(crate::error::FanError::InvalidUrl(format!(
                "Invalid topic: {}",
                topic
            )))
        }
    }

    /// Get top stories
    pub async fn top_stories(&self) -> Result<Vec<NewsArticle>> {
        self.news_feed("top_stories").await
    }

    /// Get real time headlines
    pub async fn real_time_headlines(&self) -> Result<Vec<NewsArticle>> {
        self.news_feed("real_time_headlines").await
    }

    /// Get market pulse
    pub async fn market_pulse(&self) -> Result<Vec<NewsArticle>> {
        self.news_feed("market_pulse").await
    }

    /// Get bulletins
    pub async fn bulletins(&self) -> Result<Vec<NewsArticle>> {
        self.news_feed("bulletins").await
    }

    /// Get personal finance
    pub async fn personal_finance(&self) -> Result<Vec<NewsArticle>> {
        self.news_feed("personal_finance").await
    }

    /// Get stocks to watch
    pub async fn stocks_to_watch(&self) -> Result<Vec<NewsArticle>> {
        self.news_feed("stocks_to_watch").await
    }

    /// Get internet stories
    pub async fn internet_stories(&self) -> Result<Vec<NewsArticle>> {
        self.news_feed("internet_stories").await
    }

    /// Get mutual funds
    pub async fn mutual_funds(&self) -> Result<Vec<NewsArticle>> {
        self.news_feed("mutual_funds").await
    }

    /// Get software stories
    pub async fn software_stories(&self) -> Result<Vec<NewsArticle>> {
        self.news_feed("software_stories").await
    }

    /// Get banking and finance
    pub async fn banking_and_finance(&self) -> Result<Vec<NewsArticle>> {
        self.news_feed("banking_and_finance").await
    }

    /// Get commentary
    pub async fn commentary(&self) -> Result<Vec<NewsArticle>> {
        self.news_feed("commentary").await
    }

    /// Get newsletter and research
    pub async fn newsletter_and_research(&self) -> Result<Vec<NewsArticle>> {
        self.news_feed("newsletter_and_research").await
    }

    /// Get auto reviews
    pub async fn auto_reviews(&self) -> Result<Vec<NewsArticle>> {
        self.news_feed("auto_reviews").await
    }
}

#[async_trait]
impl NewsSource for MarketWatch {
    fn name(&self) -> &'static str {
        "MarketWatch"
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }

    async fn fetch_feed(&self, topic: &str) -> Result<Vec<NewsArticle>> {
        let url = self.base_url.replace("{topic}", topic);
        info!("Fetching MarketWatch feed: {}", url);

        let response = self.client.get(&url).send().await?;
        let content = response.text().await?;

        debug!("Received {} bytes of content", content.len());

        let mut articles = self.parser.parse_response(&content)?;

        // Set source for all articles
        for article in &mut articles {
            article.source = Some(self.name().to_string());
        }

        info!(
            "Parsed {} articles from MarketWatch topic {}",
            articles.len(),
            topic
        );
        Ok(articles)
    }

    fn available_topics(&self) -> Vec<&'static str> {
        self.topic_categories.keys().copied().collect()
    }
}
