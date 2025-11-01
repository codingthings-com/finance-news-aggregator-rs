use crate::error::Result;
use crate::parser::NewsParser;
use crate::types::NewsArticle;
use async_trait::async_trait;
use log::debug;
use reqwest::Client;
use std::collections::HashMap;

pub mod cnbc;
pub mod generic;
pub mod market_watch;
pub mod nasdaq;
pub mod seeking_alpha;
pub mod wsj;
pub mod yahoo_finance;

pub use cnbc::CNBC;
pub use generic::GenericSource;
pub use market_watch::MarketWatch;
pub use nasdaq::NASDAQ;
pub use seeking_alpha::SeekingAlpha;
pub use wsj::WallStreetJournal;
pub use yahoo_finance::YahooFinance;

/// Common trait for all news sources
///
/// This trait defines the interface for fetching news from various RSS feed sources.
/// It provides both generic URL-based fetching and topic-based fetching with default
/// implementations that handle common patterns.
#[async_trait]
pub trait NewsSource {
    /// Get the name of the news source
    fn name(&self) -> &'static str;

    /// Get the URL map containing named URLs for this source
    ///
    /// Returns a HashMap where keys are URL identifiers (e.g., "base", "buzz", "original")
    /// and values are the actual URL patterns or endpoints.
    fn url_map(&self) -> &HashMap<String, String>;

    /// Get the HTTP client for making requests
    fn client(&self) -> &Client;

    /// Get the parser for this news source
    fn parser(&self) -> &NewsParser;

    /// Build the URL for a given topic
    ///
    /// This method provides the topic-to-URL mapping logic. The default implementation
    /// uses simple pattern substitution with the base URL. Sources can override this
    /// to implement custom mapping logic (e.g., topic ID mapping, special endpoints).
    ///
    /// # Arguments
    /// * `topic` - The topic identifier
    ///
    /// # Returns
    /// The complete URL for the topic, or an error if the topic is invalid
    fn build_topic_url(&self, topic: &str) -> Result<String> {
        // Default implementation: simple pattern substitution
        let base_url = self
            .url_map()
            .get("base")
            .ok_or_else(|| crate::error::FanError::InvalidUrl("Base URL not found".to_string()))?;

        Ok(base_url.replace("{topic}", topic))
    }

    /// Generic method to fetch a feed from any RSS URL
    ///
    /// This method provides a default implementation that can be used by all news sources.
    /// It fetches the RSS feed from the given URL, parses it, and sets the source attribution.
    ///
    /// # Arguments
    /// * `url` - The complete RSS feed URL to fetch
    ///
    /// # Returns
    /// A vector of parsed NewsArticle objects
    async fn fetch_feed_by_url(&self, url: &str) -> Result<Vec<NewsArticle>> {
        debug!("Fetching {} feed from URL: {}", self.name(), url);

        let response = self.client().get(url).send().await?;
        let content = response.text().await?;

        debug!("Received {} bytes of content", content.len());

        let mut articles = self.parser().parse_response(&content)?;

        // Set source for all articles
        for article in &mut articles {
            article.source = Some(self.name().to_string());
        }

        debug!("Parsed {} articles from {}", articles.len(), self.name());
        Ok(articles)
    }

    /// Fetch news articles for a specific topic
    ///
    /// This method maps topic names to their corresponding feed URLs and fetches them.
    /// The default implementation uses `build_topic_url()` for URL construction.
    /// Sources with complex logic can override this method.
    ///
    /// # Arguments
    /// * `topic` - The topic identifier (e.g., "headlines", "technology", "markets")
    ///
    /// # Returns
    /// A vector of parsed NewsArticle objects for the requested topic
    async fn fetch_topic(&self, topic: &str) -> Result<Vec<NewsArticle>> {
        let url = self.build_topic_url(topic)?;
        debug!("Fetching {} topic '{}': {}", self.name(), topic, url);
        self.fetch_feed_by_url(&url).await
    }

    /// Get available topics/feeds for this source
    ///
    /// Returns a list of topic identifiers that can be used with `fetch_topic()`
    fn available_topics(&self) -> Vec<&'static str>;
}
