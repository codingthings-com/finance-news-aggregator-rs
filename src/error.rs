use thiserror::Error;

/// Result type alias for the FAN library
pub type Result<T> = std::result::Result<T, FanError>;

/// Error types for the FAN library
#[derive(Error, Debug)]
pub enum FanError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("XML parsing failed: {0}")]
    XmlParsing(#[from] quick_xml::Error),
    
    #[error("JSON serialization failed: {0}")]
    JsonSerialization(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    
    #[error("Feed parsing error: {0}")]
    FeedParsing(String),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}