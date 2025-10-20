use crate::error::Result;
use crate::news_source::NewsSource;
use crate::parser::NewsParser;
use crate::types::NewsArticle;
use async_trait::async_trait;
use log::debug;
use reqwest::Client;

/// NASDAQ news client
pub struct NASDAQ {
    base_url: String,
    original_content_url: String,
    client: Client,
    parser: NewsParser,
}

impl NASDAQ {
    /// Create a new NASDAQ client
    pub fn new(client: Client) -> Self {
        Self {
            base_url: "https://www.nasdaq.com/feed/rssoutbound".to_string(),
            original_content_url: "https://www.nasdaq.com/feed/nasdaq-original/rss.xml".to_string(),
            client,
            parser: NewsParser::new("nasdaq"),
        }
    }

    /// Get original content feed
    pub async fn original_content(&self) -> Result<Vec<NewsArticle>> {
        let url = &self.original_content_url;
        debug!("Fetching NASDAQ original content: {}", url);

        let response = self.client.get(url).send().await?;
        let content = response.text().await?;

        debug!("Received {} bytes of content", content.len());

        let mut articles = self.parser.parse_response(&content)?;

        // Set source for all articles
        for article in &mut articles {
            article.source = Some(self.name().to_string());
        }

        debug!(
            "Parsed {} articles from NASDAQ original content",
            articles.len()
        );
        Ok(articles)
    }

    /// Get feed by category
    pub async fn feed_by_category(&self, category: &str) -> Result<Vec<NewsArticle>> {
        let url = format!("{}?category={}", self.base_url, category);
        debug!("Fetching NASDAQ feed: {}", url);

        let response = self.client.get(&url).send().await?;
        let content = response.text().await?;

        debug!("Received {} bytes of content", content.len());

        let mut articles = self.parser.parse_response(&content)?;

        // Set source for all articles
        for article in &mut articles {
            article.source = Some(self.name().to_string());
        }

        debug!(
            "Parsed {} articles from NASDAQ category {}",
            articles.len(),
            category
        );
        Ok(articles)
    }

    /// Get commodities feed
    pub async fn commodities(&self) -> Result<Vec<NewsArticle>> {
        self.feed_by_category("commodities").await
    }

    /// Get cryptocurrency feed
    pub async fn cryptocurrency(&self) -> Result<Vec<NewsArticle>> {
        self.feed_by_category("cryptocurrency").await
    }

    /// Get dividends feed
    pub async fn dividends(&self) -> Result<Vec<NewsArticle>> {
        self.feed_by_category("dividends").await
    }

    /// Get earnings feed
    pub async fn earnings(&self) -> Result<Vec<NewsArticle>> {
        self.feed_by_category("earnings").await
    }

    /// Get economics feed
    pub async fn economics(&self) -> Result<Vec<NewsArticle>> {
        self.feed_by_category("economics").await
    }

    /// Get financial advisors feed
    pub async fn financial_advisors(&self) -> Result<Vec<NewsArticle>> {
        self.feed_by_category("financial-advisors").await
    }

    /// Get innovation feed
    pub async fn innovation(&self) -> Result<Vec<NewsArticle>> {
        self.feed_by_category("innovation").await
    }

    /// Get stocks feed
    pub async fn stocks(&self) -> Result<Vec<NewsArticle>> {
        self.feed_by_category("stocks").await
    }

    /// Get technology feed
    pub async fn technology(&self) -> Result<Vec<NewsArticle>> {
        self.feed_by_category("technology").await
    }
}

#[async_trait]
impl NewsSource for NASDAQ {
    fn name(&self) -> &'static str {
        "NASDAQ"
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }

    async fn fetch_feed(&self, category: &str) -> Result<Vec<NewsArticle>> {
        if category == "original" {
            self.original_content().await
        } else {
            self.feed_by_category(category).await
        }
    }

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
