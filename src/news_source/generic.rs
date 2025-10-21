use crate::news_source::NewsSource;
use crate::parser::NewsParser;
use async_trait::async_trait;
use reqwest::Client;
use std::collections::HashMap;

/// Generic news source for fetching arbitrary RSS feeds
///
/// This source doesn't have predefined feeds or topics. It's designed
/// for fetching any RSS feed URL directly using `fetch_feed_by_url()`.
pub struct GenericSource {
    client: Client,
    parser: NewsParser,
    url_map: HashMap<String, String>,
}

impl GenericSource {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            parser: NewsParser::new("generic"),
            url_map: HashMap::new(),
        }
    }
}

#[async_trait]
impl NewsSource for GenericSource {
    fn name(&self) -> &'static str {
        "Generic"
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

    fn available_topics(&self) -> Vec<&'static str> {
        // Generic source doesn't have predefined topics
        vec![]
    }
}
