use crate::error::Result;
use crate::types::NewsArticle;
use async_trait::async_trait;

pub mod cnbc;
pub mod cnn_finance;
pub mod market_watch;
pub mod nasdaq;
pub mod seeking_alpha;
pub mod wsj;
pub mod yahoo_finance;

pub use cnbc::CNBC;
pub use cnn_finance::CNNFinance;
pub use market_watch::MarketWatch;
pub use nasdaq::NASDAQ;
pub use seeking_alpha::SeekingAlpha;
pub use wsj::WallStreetJournal;
pub use yahoo_finance::YahooFinance;

/// Common trait for all news sources
#[async_trait]
pub trait NewsSource {
    /// Get the name of the news source
    fn name(&self) -> &'static str;

    /// Get the base URL pattern for this source
    fn base_url(&self) -> &str;

    /// Make a request to fetch news articles
    async fn fetch_feed(&self, topic: &str) -> Result<Vec<NewsArticle>>;

    /// Get available topics/feeds for this source
    fn available_topics(&self) -> Vec<&'static str>;
}
