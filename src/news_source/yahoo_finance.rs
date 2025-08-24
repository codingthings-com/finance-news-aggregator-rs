use crate::error::Result;
use crate::news_source::NewsSource;
use crate::parser::NewsParser;
use crate::types::NewsArticle;
use async_trait::async_trait;
use log::{debug, info};
use reqwest::Client;

/// Yahoo Finance news client
pub struct YahooFinance {
    base_url: String,
    client: Client,
    parser: NewsParser,
}

impl YahooFinance {
    /// Create a new Yahoo Finance client
    pub fn new(client: Client) -> Self {
        Self {
            base_url: "https://feeds.finance.yahoo.com/rss/2.0".to_string(),
            client,
            parser: NewsParser::new("yahoo"),
        }
    }

    /// Get general news feed
    pub async fn news(&self) -> Result<Vec<NewsArticle>> {
        let url = format!("{}/headline", self.base_url);
        info!("Fetching Yahoo Finance news: {}", url);

        let response = self.client.get(&url).send().await?;
        let content = response.text().await?;

        debug!("Received {} bytes of content", content.len());

        let mut articles = self.parser.parse_response(&content)?;

        // Set source for all articles
        for article in &mut articles {
            article.source = Some(self.name().to_string());
        }

        info!("Parsed {} articles from Yahoo Finance news", articles.len());
        Ok(articles)
    }

    /// Get headlines for specific symbols
    pub async fn headlines(&self, symbols: &[&str]) -> Result<Vec<NewsArticle>> {
        let symbols_str = symbols.join(",");
        let url = format!("{}/headline?s={}", self.base_url, symbols_str);
        info!(
            "Fetching Yahoo Finance headlines for symbols: {}",
            symbols_str
        );

        let response = self.client.get(&url).send().await?;
        let content = response.text().await?;

        debug!("Received {} bytes of content", content.len());

        let mut articles = self.parser.parse_response(&content)?;

        // Set source for all articles
        for article in &mut articles {
            article.source = Some(self.name().to_string());
        }

        info!(
            "Parsed {} articles from Yahoo Finance headlines",
            articles.len()
        );
        Ok(articles)
    }

    /// Get market summary
    pub async fn market_summary(&self) -> Result<Vec<NewsArticle>> {
        let url = format!("{}/topstories", self.base_url);
        info!("Fetching Yahoo Finance market summary: {}", url);

        let response = self.client.get(&url).send().await?;
        let content = response.text().await?;

        debug!("Received {} bytes of content", content.len());

        let mut articles = self.parser.parse_response(&content)?;

        // Set source for all articles
        for article in &mut articles {
            article.source = Some(self.name().to_string());
        }

        info!(
            "Parsed {} articles from Yahoo Finance market summary",
            articles.len()
        );
        Ok(articles)
    }

    /// Get industry news
    pub async fn industry_news(&self, industry: &str) -> Result<Vec<NewsArticle>> {
        let url = format!("{}/industry?s={}", self.base_url, industry);
        info!("Fetching Yahoo Finance industry news for: {}", industry);

        let response = self.client.get(&url).send().await?;
        let content = response.text().await?;

        debug!("Received {} bytes of content", content.len());

        let mut articles = self.parser.parse_response(&content)?;

        // Set source for all articles
        for article in &mut articles {
            article.source = Some(self.name().to_string());
        }

        info!(
            "Parsed {} articles from Yahoo Finance industry {}",
            articles.len(),
            industry
        );
        Ok(articles)
    }
}

#[async_trait]
impl NewsSource for YahooFinance {
    fn name(&self) -> &'static str {
        "Yahoo Finance"
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }

    async fn fetch_feed(&self, category: &str) -> Result<Vec<NewsArticle>> {
        match category {
            "news" => self.news().await,
            "market_summary" => self.market_summary().await,
            _ => {
                let url = format!("{}/{}", self.base_url, category);
                info!("Fetching Yahoo Finance feed: {}", url);

                let response = self.client.get(&url).send().await?;
                let content = response.text().await?;

                debug!("Received {} bytes of content", content.len());

                let mut articles = self.parser.parse_response(&content)?;

                // Set source for all articles
                for article in &mut articles {
                    article.source = Some(self.name().to_string());
                }

                info!(
                    "Parsed {} articles from Yahoo Finance {}",
                    articles.len(),
                    category
                );
                Ok(articles)
            }
        }
    }

    fn available_topics(&self) -> Vec<&'static str> {
        vec!["news", "market_summary", "topstories", "headline"]
    }
}
