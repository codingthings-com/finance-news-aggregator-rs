use crate::error::Result;
use crate::news_source::NewsSource;
use crate::parser::NewsParser;
use crate::types::NewsArticle;
use async_trait::async_trait;
use log::{debug, info};
use reqwest::Client;

/// Seeking Alpha news client
pub struct SeekingAlpha {
    base_url: String,
    client: Client,
    parser: NewsParser,
}

impl SeekingAlpha {
    /// Create a new Seeking Alpha client
    pub fn new(client: Client) -> Self {
        Self {
            base_url: "https://seekingalpha.com/feed.xml".to_string(),
            client,
            parser: NewsParser::new("seeking_alpha"),
        }
    }

    /// Get latest articles
    pub async fn latest_articles(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed("latest-articles").await
    }

    /// Get all news
    pub async fn all_news(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed("all-news").await
    }

    /// Get market news
    pub async fn market_news(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed("market-news").await
    }

    /// Get long ideas
    pub async fn long_ideas(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed("long-ideas").await
    }

    /// Get short ideas
    pub async fn short_ideas(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed("short-ideas").await
    }

    /// Get IPO analysis
    pub async fn ipo_analysis(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed("ipo-analysis").await
    }

    /// Get transcripts
    pub async fn transcripts(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed("transcripts").await
    }

    /// Get Wall Street breakfast
    pub async fn wall_street_breakfast(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed("wall-street-breakfast").await
    }

    /// Get most popular articles
    pub async fn most_popular_articles(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed("most-popular-articles").await
    }

    /// Get forex articles
    pub async fn forex(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed("forex").await
    }

    /// Get editor picks
    pub async fn editors_picks(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed("editors-picks").await
    }

    /// Get ETFs
    pub async fn etfs(&self) -> Result<Vec<NewsArticle>> {
        self.fetch_feed("etfs").await
    }

    /// Get global markets by country
    pub async fn global_markets(&self, country: &str) -> Result<Vec<NewsArticle>> {
        self.fetch_feed(&format!("global-markets-{}", country)).await
    }

    /// Get sectors by sector name
    pub async fn sectors(&self, sector: &str) -> Result<Vec<NewsArticle>> {
        self.fetch_feed(&format!("sectors-{}", sector)).await
    }

    /// Get stocks by ticker
    pub async fn stocks(&self, ticker: &str) -> Result<Vec<NewsArticle>> {
        self.fetch_feed(&format!("stocks-{}", ticker)).await
    }
}

#[async_trait]
impl NewsSource for SeekingAlpha {
    fn name(&self) -> &'static str {
        "Seeking Alpha"
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }

    async fn fetch_feed(&self, category: &str) -> Result<Vec<NewsArticle>> {
        let url = format!("{}?category={}", self.base_url, category);
        info!("Fetching Seeking Alpha feed: {}", url);
        
        let response = self.client.get(&url).send().await?;
        let content = response.text().await?;
        
        debug!("Received {} bytes of content", content.len());
        
        let mut articles = self.parser.parse_response(&content)?;
        
        // Set source for all articles
        for article in &mut articles {
            article.source = Some(self.name().to_string());
        }
        
        info!("Parsed {} articles from Seeking Alpha {}", articles.len(), category);
        Ok(articles)
    }

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
            "etfs"
        ]
    }
}