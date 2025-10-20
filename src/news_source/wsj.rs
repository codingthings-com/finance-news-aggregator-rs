use crate::error::Result;
use crate::news_source::NewsSource;
use crate::parser::NewsParser;
use crate::types::{NewsArticle, SourceConfig};
use async_trait::async_trait;
use log::debug;
use reqwest::Client;

/// Wall Street Journal news client
pub struct WallStreetJournal {
    config: SourceConfig,
    client: Client,
    parser: NewsParser,
}

impl WallStreetJournal {
    /// Create a new Wall Street Journal client
    pub fn new(client: Client) -> Self {
        let config = SourceConfig::new("https://feeds.a.dj.com/rss/{topic}.xml");

        Self {
            config,
            client,
            parser: NewsParser::new("wsj"),
        }
    }

    /// Create a new Wall Street Journal client with custom config
    pub fn with_config(client: Client, config: SourceConfig) -> Self {
        Self {
            config,
            client,
            parser: NewsParser::new("wsj"),
        }
    }

    /// Get opinions feed
    pub async fn opinions(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed("RSSOpinion").await
    }

    /// Get world news feed
    pub async fn world_news(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed("RSSWorldNews").await
    }

    /// Get US business news feed
    pub async fn us_business_news(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed("WSJcomUSBusiness").await
    }

    /// Get market news feed
    pub async fn market_news(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed("RSSMarketsMain").await
    }

    /// Get technology news feed
    pub async fn technology_news(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed("RSSWSJD").await
    }

    /// Get lifestyle feed
    pub async fn lifestyle(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed("RSSLifestyle").await
    }
}

#[async_trait]
impl NewsSource for WallStreetJournal {
    fn name(&self) -> &'static str {
        "Wall Street Journal"
    }

    fn base_url(&self) -> &str {
        &self.config.base_url
    }

    async fn fetch_feed(&self, topic: &str) -> Result<Vec<NewsArticle>> {
        let url = self.config.base_url.replace("{topic}", topic);
        debug!("Fetching WSJ feed: {}", url);

        let response = self.client.get(&url).send().await?;
        let content = response.text().await?;

        debug!("Received {} bytes of content", content.len());

        let mut articles = self.parser.parse_response(&content)?;

        // Set source for all articles
        for article in &mut articles {
            article.source = Some(self.name().to_string());
        }

        debug!("Parsed {} articles from WSJ {}", articles.len(), topic);
        Ok(articles)
    }

    fn available_topics(&self) -> Vec<&'static str> {
        vec![
            "RSSOpinion",
            "RSSWorldNews",
            "WSJcomUSBusiness",
            "RSSMarketsMain",
            "RSSWSJD",
            "RSSLifestyle",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Test imports

    #[tokio::test]
    async fn test_wsj_opinions() {
        let config = SourceConfig::new("https://feeds.a.dj.com/rss/{topic}.xml").with_timeout(30);

        let client = Client::builder()
            .timeout(config.timeout_duration())
            .user_agent(&config.user_agent)
            .build()
            .expect("Failed to create HTTP client");

        let wsj = WallStreetJournal::with_config(client, config);
        let result = wsj.opinions().await;

        match result {
            Ok(articles) => {
                println!("Successfully fetched {} opinion articles", articles.len());
                if !articles.is_empty() {
                    println!("First article: {:?}", articles[0]);
                }
            }
            Err(e) => {
                println!("Error fetching opinions: {}", e);
                // Don't fail the test for network issues in CI
            }
        }
    }

    #[test]
    fn test_wsj_config() {
        let config = SourceConfig::new("https://feeds.a.dj.com/rss/{topic}.xml")
            .with_timeout(60)
            .with_user_agent("Custom User Agent")
            .with_retries(5, 2000);

        assert_eq!(config.base_url, "https://feeds.a.dj.com/rss/{topic}.xml");
        assert_eq!(config.timeout_seconds, 60);
        assert_eq!(config.user_agent, "Custom User Agent");
        assert_eq!(config.max_retries, 5);
        assert_eq!(config.retry_delay_ms, 2000);
    }
}
