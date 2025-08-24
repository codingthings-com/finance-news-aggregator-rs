//! # Finance News Aggregator (finance-news-aggregator-rs)
//!
//! A Rust library for aggregating financial news from various sources.
//! This is a port of the Python finance-news-aggregator project.

pub mod error;
pub mod news_client;
pub mod news_source;
pub mod parser;
pub mod types;

pub use error::{FanError, Result};
pub use news_client::NewsClient;
pub use types::NewsArticle;
