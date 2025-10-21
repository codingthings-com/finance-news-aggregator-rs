use crate::error::Result;
use crate::news_source::NewsSource;
use crate::parser::NewsParser;
use crate::types::NewsArticle;
use async_trait::async_trait;
use reqwest::Client;
use std::collections::HashMap;

/// Seeking Alpha news client
///
/// Provides access to Seeking Alpha RSS feeds for investment research, market analysis,
/// stock ideas, IPO analysis, earnings transcripts, and more.
pub struct SeekingAlpha {
    url_map: HashMap<String, String>,
    client: Client,
    parser: NewsParser,
}

impl SeekingAlpha {
    /// Create a new Seeking Alpha client
    ///
    /// Initializes the client with Seeking Alpha RSS feed URL.
    pub fn new(client: Client) -> Self {
        let mut url_map = HashMap::new();
        url_map.insert(
            "base".to_string(),
            "https://seekingalpha.com/feed.xml".to_string(),
        );

        Self {
            url_map,
            client,
            parser: NewsParser::new("seeking_alpha"),
        }
    }

    /// Get latest articles
    pub async fn latest_articles(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("latest-articles").await
    }

    /// Get all news
    pub async fn all_news(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("all-news").await
    }

    /// Get market news
    pub async fn market_news(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("market-news").await
    }

    /// Get long ideas
    pub async fn long_ideas(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("long-ideas").await
    }

    /// Get short ideas
    pub async fn short_ideas(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("short-ideas").await
    }

    /// Get IPO analysis
    pub async fn ipo_analysis(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("ipo-analysis").await
    }

    /// Get transcripts
    pub async fn transcripts(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("transcripts").await
    }

    /// Get Wall Street breakfast
    pub async fn wall_street_breakfast(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("wall-street-breakfast").await
    }

    /// Get most popular articles
    pub async fn most_popular_articles(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("most-popular-articles").await
    }

    /// Get forex articles
    pub async fn forex(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("forex").await
    }

    /// Get editor picks
    pub async fn editors_picks(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("editors-picks").await
    }

    /// Get ETFs
    pub async fn etfs(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_topic("etfs").await
    }

    /// Get global markets by country
    ///
    /// # Arguments
    /// * `country` - Country code or name (e.g., "china", "india", "brazil")
    pub async fn global_markets(&self, country: &str) -> Result<Vec<NewsArticle>> {
        self.fetch_topic(&format!("global-markets-{}", country))
            .await
    }

    /// Get sectors by sector name
    ///
    /// # Arguments
    /// * `sector` - Sector name (e.g., "technology", "healthcare", "energy")
    pub async fn sectors(&self, sector: &str) -> Result<Vec<NewsArticle>> {
        self.fetch_topic(&format!("sectors-{}", sector)).await
    }

    /// Get stocks by ticker symbol
    ///
    /// # Arguments
    /// * `ticker` - Stock ticker symbol (e.g., "AAPL", "GOOGL", "MSFT")
    pub async fn stocks(&self, ticker: &str) -> Result<Vec<NewsArticle>> {
        self.fetch_topic(&format!("stocks-{}", ticker)).await
    }
}

#[async_trait]
impl NewsSource for SeekingAlpha {
    fn name(&self) -> &'static str {
        "Seeking Alpha"
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

    // Override build_topic_url for Seeking Alpha's query parameter structure
    fn build_topic_url(&self, topic: &str) -> Result<String> {
        let base_url = self
            .url_map()
            .get("base")
            .ok_or_else(|| crate::error::FanError::InvalidUrl("Base URL not found".to_string()))?;

        Ok(format!("{}?category={}", base_url, topic))
    }

    // Uses default fetch_topic implementation

    fn available_topics(&self) -> Vec<&'static str> {
        vec![
            "latest-articles",
            "all-news",
            "market-news",
            "long-ideas",
            "short-ideas",
            "ipo-analysis",
            "transcripts",
            "wall-street-breakfast",
            "most-popular-articles",
            "forex",
            "editors-picks",
            "etfs",
        ]
    }
}
