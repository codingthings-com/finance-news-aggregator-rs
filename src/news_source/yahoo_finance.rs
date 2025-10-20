use crate::error::Result;
use crate::news_source::NewsSource;
use crate::parser::NewsParser;
use crate::types::NewsArticle;
use async_trait::async_trait;
use log::debug;
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
            base_url: "https://finance.yahoo.com/news/rssindex".to_string(),
            client,
            parser: NewsParser::new("yahoo"),
        }
        // https://feeds.finance.yahoo.com/rss/2.0 no longer available
    }

    /// Get general news feed
    pub async fn headlines(&self) -> Result<Vec<NewsArticle>> {
        let url = format!("{}/headlines", self.base_url);
        debug!("Fetching Yahoo Finance Headlines: {}", url);

        let response = self.client.get(&url).send().await?;
        let content = response.text().await?;

        debug!("Received {} bytes of content", content.len());

        let mut articles = self.parser.parse_response(&content)?;

        // Set source for all articles
        for article in &mut articles {
            article.source = Some(self.name().to_string());
        }

        debug!("Parsed {} articles from Yahoo Finance news", articles.len());
        Ok(articles)
    }

    /// Get headlines for specific symbols
    pub async fn headline(&self, symbols: &[&str]) -> Result<Vec<NewsArticle>> {
        let symbols_str = symbols.join(",");
        let url = format!("{}/headline?s={}", self.base_url, symbols_str);
        debug!(
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

        debug!(
            "Parsed {} articles from Yahoo Finance headlines",
            articles.len()
        );
        Ok(articles)
    }

    /// Get market summary
    pub async fn topstories(&self) -> Result<Vec<NewsArticle>> {
        let url = format!("{}/topstories", self.base_url);
        debug!("Fetching Yahoo Finance market summary: {}", url);

        let response = self.client.get(&url).send().await?;
        let content = response.text().await?;

        debug!("Received {} bytes of content", content.len());

        let mut articles = self.parser.parse_response(&content)?;

        // Set source for all articles
        for article in &mut articles {
            article.source = Some(self.name().to_string());
        }

        debug!(
            "Parsed {} articles from Yahoo Finance market summary",
            articles.len()
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
            "headlines" => self.headlines().await,
            "topstories" => self.topstories().await,
            _ => {
                let url = format!("{}/{}", self.base_url, category);
                debug!("Fetching Yahoo Finance feed: {}", url);

                let response = self.client.get(&url).send().await?;
                let content = response.text().await?;

                debug!("Received {} bytes of content", content.len());

                let mut articles = self.parser.parse_response(&content)?;

                // Set source for all articles
                for article in &mut articles {
                    article.source = Some(self.name().to_string());
                }

                debug!(
                    "Parsed {} articles from Yahoo Finance {}",
                    articles.len(),
                    category
                );
                Ok(articles)
            }
        }
    }

    fn available_topics(&self) -> Vec<&'static str> {
        vec!["topstories", "headlines"]
    }
}
