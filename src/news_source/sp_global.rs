use crate::error::Result;
use crate::news_source::NewsSource;
use crate::parser::NewsParser;
use crate::types::NewsArticle;
use async_trait::async_trait;
use log::{debug, info};
use reqwest::Client;

/// S&P Global news client
pub struct SPGlobal {
    base_url: String,
    client: Client,
    parser: NewsParser,
}

impl SPGlobal {
    /// Create a new S&P Global client
    pub fn new(client: Client) -> Self {
        Self {
            base_url: "https://www.spglobal.com/spdji/en/rss/rss-details/".to_string(),
            client,
            parser: NewsParser::new("sp_global"),
        }
    }

    /// Get methodologies feed
    pub async fn methodologies(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed_with_params("methodologies").await
    }

    /// Get all indices feed
    pub async fn all_indices(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed_with_params("all-indicies").await
    }

    /// Get research feed
    pub async fn research(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed_with_params("research").await
    }

    /// Get market commentary feed
    pub async fn market_commentary(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed_with_params("market-commentary").await
    }

    /// Get education feed
    pub async fn education(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed_with_params("education").await
    }

    /// Get performance reports feed
    pub async fn performance_reports(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed_with_params("performance-reports").await
    }

    /// Get SPIVA feed
    pub async fn spiva(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed_with_params("spiva").await
    }

    /// Get index TV feed
    pub async fn index_tv(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed_with_params("index-tv").await
    }

    /// Get corporate news feed
    pub async fn corporate_news(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed_with_params("corporate-news").await
    }

    /// Get index launches feed
    pub async fn index_launches(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed_with_params("index-launches").await
    }

    /// Get index announcements feed
    pub async fn index_announcements(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed_with_params("index-announcements").await
    }

    /// Get new consultations feed
    pub async fn new_consultations(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed_with_params("new-consultations").await
    }

    /// Fetch feed with RSS feed name parameter
    async fn fetch_feed_with_params(&self, rss_feed_name: &str) -> Result<Vec<NewsArticle>> {
        let url = format!("{}?rssFeedName={}", self.base_url, rss_feed_name);
        info!("Fetching S&P Global feed: {}", url);
        
        let response = self.client.get(&url).send().await?;
        let content = response.text().await?;
        
        debug!("Received {} bytes of content", content.len());
        
        let mut articles = self.parser.parse_response(&content)?;
        
        // Set source for all articles
        for article in &mut articles {
            article.source = Some(self.name().to_string());
        }
        
        info!("Parsed {} articles from S&P Global {}", articles.len(), rss_feed_name);
        Ok(articles)
    }
}

#[async_trait]
impl NewsSource for SPGlobal {
    fn name(&self) -> &'static str {
        "S&P Global"
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }

    async fn fetch_feed(&self, rss_feed_name: &str) -> Result<Vec<NewsArticle>> {
        self.fetch_feed_with_params(rss_feed_name).await
    }

    fn available_topics(&self) -> Vec<&'static str> {
        vec![
            "methodologies",
            "all-indicies",
            "research",
            "market-commentary",
            "education",
            "performance-reports",
            "spiva",
            "index-tv",
            "corporate-news",
            "index-launches",
            "index-announcements",
            "new-consultations"
        ]
    }
}