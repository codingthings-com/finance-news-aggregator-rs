use crate::Result;
use crate::news_source::*;
use crate::types::{NewsArticle, SourceConfig};
use log::info;
use reqwest::Client;
use serde_json;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Main news client that provides access to different news sources
pub struct NewsClient {
    http_client: Client,
    default_config: SourceConfig,
    wsj_client: Option<WallStreetJournal>,
    cnbc_client: Option<CNBC>,
    nasdaq_client: Option<NASDAQ>,
    market_watch_client: Option<MarketWatch>,
    seeking_alpha_client: Option<SeekingAlpha>,
    cnn_finance_client: Option<CNNFinance>,
    yahoo_finance_client: Option<YahooFinance>,
}

impl NewsClient {
    /// Create a new NewsClient instance
    pub fn new() -> Self {
        Self::with_config(SourceConfig::default())
    }

    /// Create a new NewsClient instance with custom configuration
    pub fn with_config(config: SourceConfig) -> Self {
        info!("Creating new NewsClient with config");

        let http_client = Client::builder()
            .timeout(config.timeout_duration())
            .user_agent(&config.user_agent)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            http_client,
            default_config: config,
            wsj_client: None,
            cnbc_client: None,
            nasdaq_client: None,
            market_watch_client: None,
            seeking_alpha_client: None,
            cnn_finance_client: None,
            yahoo_finance_client: None,
        }
    }

    /// Get the default configuration
    pub fn config(&self) -> &SourceConfig {
        &self.default_config
    }

    /// Get Wall Street Journal client
    ///
    /// # Example
    /// ```rust
    /// use finance_news_aggregator_rs::NewsClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut client = NewsClient::new();
    ///     let wsj = client.wsj();
    ///     let opinions = wsj.opinions().await?;
    ///     println!("{:#?}", opinions);
    ///     Ok(())
    /// }
    /// ```
    pub fn wsj(&mut self) -> &WallStreetJournal {
        if self.wsj_client.is_none() {
            self.wsj_client = Some(WallStreetJournal::new(self.http_client.clone()));
        }
        self.wsj_client.as_ref().unwrap()
    }

    /// Get CNBC client
    ///
    /// # Example
    /// ```rust
    /// use finance_news_aggregator_rs::NewsClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut client = NewsClient::new();
    ///     let cnbc = client.cnbc();
    ///     let top_news = cnbc.top_news().await?;
    ///     println!("Found {} articles", top_news.len());
    ///     Ok(())
    /// }
    /// ```
    pub fn cnbc(&mut self) -> &CNBC {
        if self.cnbc_client.is_none() {
            self.cnbc_client = Some(CNBC::new(self.http_client.clone()));
        }
        self.cnbc_client.as_ref().unwrap()
    }

    /// Get NASDAQ client
    ///
    /// # Example
    /// ```rust
    /// use finance_news_aggregator_rs::NewsClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut client = NewsClient::new();
    ///     let nasdaq = client.nasdaq();
    ///     let tech_news = nasdaq.technology().await?;
    ///     println!("Found {} articles", tech_news.len());
    ///     Ok(())
    /// }
    /// ```
    pub fn nasdaq(&mut self) -> &NASDAQ {
        if self.nasdaq_client.is_none() {
            self.nasdaq_client = Some(NASDAQ::new(self.http_client.clone()));
        }
        self.nasdaq_client.as_ref().unwrap()
    }

    /// Get MarketWatch client
    ///
    /// # Example
    /// ```rust
    /// use finance_news_aggregator_rs::NewsClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut client = NewsClient::new();
    ///     let mw = client.market_watch();
    ///     let top_stories = mw.top_stories().await?;
    ///     println!("Found {} articles", top_stories.len());
    ///     Ok(())
    /// }
    /// ```
    pub fn market_watch(&mut self) -> &MarketWatch {
        if self.market_watch_client.is_none() {
            self.market_watch_client = Some(MarketWatch::new(self.http_client.clone()));
        }
        self.market_watch_client.as_ref().unwrap()
    }

    /// Get Seeking Alpha client
    ///
    /// # Example
    /// ```rust
    /// use finance_news_aggregator_rs::NewsClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut client = NewsClient::new();
    ///     let sa = client.seeking_alpha();
    ///     let latest = sa.latest_articles().await?;
    ///     println!("Found {} articles", latest.len());
    ///     Ok(())
    /// }
    /// ```
    pub fn seeking_alpha(&mut self) -> &SeekingAlpha {
        if self.seeking_alpha_client.is_none() {
            self.seeking_alpha_client = Some(SeekingAlpha::new(self.http_client.clone()));
        }
        self.seeking_alpha_client.as_ref().unwrap()
    }

    /// Get CNN Finance client
    ///
    /// # Example
    /// ```rust
    /// use finance_news_aggregator_rs::NewsClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut client = NewsClient::new();
    ///     let cnn = client.cnn_finance();
    ///     let stories = cnn.all_stories().await?;
    ///     println!("Found {} articles", stories.len());
    ///     Ok(())
    /// }
    /// ```
    pub fn cnn_finance(&mut self) -> &CNNFinance {
        if self.cnn_finance_client.is_none() {
            self.cnn_finance_client = Some(CNNFinance::new(self.http_client.clone()));
        }
        self.cnn_finance_client.as_ref().unwrap()
    }

    /// Get Yahoo Finance client
    ///
    /// # Example
    /// ```rust
    /// use finance_news_aggregator_rs::NewsClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut client = NewsClient::new();
    ///     let yahoo = client.yahoo_finance();
    ///     let news = yahoo.news().await?;
    ///     println!("Found {} articles", news.len());
    ///     Ok(())
    /// }
    /// ```
    pub fn yahoo_finance(&mut self) -> &YahooFinance {
        if self.yahoo_finance_client.is_none() {
            self.yahoo_finance_client = Some(YahooFinance::new(self.http_client.clone()));
        }
        self.yahoo_finance_client.as_ref().unwrap()
    }

    /// Save news articles to a JSON file
    ///
    /// # Arguments
    /// * `articles` - Vector of news articles to save
    /// * `filename` - Name of the file (without extension)
    ///
    /// # Example
    /// ```rust
    /// use finance_news_aggregator_rs::NewsClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut client = NewsClient::new();
    ///     let wsj = client.wsj();
    ///     let opinions = wsj.opinions().await?;
    ///     client.save_to_file(&opinions, "wsj_opinions").await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn save_to_file(&self, articles: &[NewsArticle], filename: &str) -> Result<()> {
        // Create examples/responses directory if it doesn't exist
        let dir_path = Path::new("examples/responses");
        std::fs::create_dir_all(dir_path)?;

        let file_path = dir_path.join(format!("{}.json", filename));
        let json_content = serde_json::to_string_pretty(articles)?;

        let mut file = File::create(&file_path)?;
        file.write_all(json_content.as_bytes())?;

        info!("Saved {} articles to {:?}", articles.len(), file_path);
        Ok(())
    }
}

impl Default for NewsClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = NewsClient::new();
        assert!(client.wsj_client.is_none());
        assert!(client.cnbc_client.is_none());
        assert!(client.nasdaq_client.is_none());
        assert!(client.market_watch_client.is_none());
        assert!(client.seeking_alpha_client.is_none());
        assert!(client.cnn_finance_client.is_none());
        assert!(client.yahoo_finance_client.is_none());
    }

    #[tokio::test]
    async fn test_wsj_client_access() {
        let mut client = NewsClient::new();
        let _wsj = client.wsj();
        assert!(client.wsj_client.is_some());
    }

    #[tokio::test]
    async fn test_cnbc_client_access() {
        let mut client = NewsClient::new();
        let _cnbc = client.cnbc();
        assert!(client.cnbc_client.is_some());
    }

    #[tokio::test]
    async fn test_nasdaq_client_access() {
        let mut client = NewsClient::new();
        let _nasdaq = client.nasdaq();
        assert!(client.nasdaq_client.is_some());
    }

    #[tokio::test]
    async fn test_market_watch_client_access() {
        let mut client = NewsClient::new();
        let _mw = client.market_watch();
        assert!(client.market_watch_client.is_some());
    }


    #[tokio::test]
    async fn test_seeking_alpha_client_access() {
        let mut client = NewsClient::new();
        let _sa = client.seeking_alpha();
        assert!(client.seeking_alpha_client.is_some());
    }

    #[tokio::test]
    async fn test_cnn_finance_client_access() {
        let mut client = NewsClient::new();
        let _cnn = client.cnn_finance();
        assert!(client.cnn_finance_client.is_some());
    }

    #[tokio::test]
    async fn test_yahoo_finance_client_access() {
        let mut client = NewsClient::new();
        let _yahoo = client.yahoo_finance();
        assert!(client.yahoo_finance_client.is_some());
    }

    #[tokio::test]
    async fn test_all_clients_independent() {
        let mut client = NewsClient::new();

        // Access all clients
        let _wsj = client.wsj();
        let _cnbc = client.cnbc();
        let _nasdaq = client.nasdaq();
        let _mw = client.market_watch();
        let _sa = client.seeking_alpha();
        let _cnn = client.cnn_finance();
        let _yahoo = client.yahoo_finance();

        // Verify all are initialized
        assert!(client.wsj_client.is_some());
        assert!(client.cnbc_client.is_some());
        assert!(client.nasdaq_client.is_some());
        assert!(client.market_watch_client.is_some());
        assert!(client.seeking_alpha_client.is_some());
        assert!(client.cnn_finance_client.is_some());
        assert!(client.yahoo_finance_client.is_some());
    }
}
