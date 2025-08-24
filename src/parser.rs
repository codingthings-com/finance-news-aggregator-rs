use crate::error::{FanError, Result};
use crate::types::NewsArticle;
use quick_xml::Reader;
use quick_xml::events::Event;
use std::collections::HashMap;

/// RSS/XML parser for news feeds with namespace support
///
/// The parser handles RSS feeds from different news sources, each with their own
/// XML namespaces and tag structures. It normalizes the content into `NewsArticle` structs.
///
/// # Examples
///
/// ```rust
/// use finance_news_aggregator_rs::parser::NewsParser;
///
/// let parser = NewsParser::new("wsj");
/// let rss_content = r#"
/// <rss>
///   <channel>
///     <item>
///       <title>Market Update</title>
///       <link>https://example.com/article</link>
///       <description>Stock market news</description>
///       <pubDate>Mon, 01 Jan 2024 12:00:00 GMT</pubDate>
///     </item>
///   </channel>
/// </rss>
/// "#;
///
/// let articles = parser.parse_response(rss_content).unwrap();
/// assert_eq!(articles.len(), 1);
/// assert_eq!(articles[0].title.as_ref().unwrap(), "Market Update");
/// ```
pub struct NewsParser {
    client_type: String,
    namespaces: HashMap<String, Vec<String>>,
}

impl NewsParser {
    /// Create a new parser for the specified news source
    ///
    /// # Arguments
    ///
    /// * `client_type` - The news source identifier (e.g., "wsj", "cnbc", "nasdaq")
    ///
    /// # Examples
    ///
    /// ```rust
    /// use finance_news_aggregator_rs::parser::NewsParser;
    ///
    /// let wsj_parser = NewsParser::new("wsj");
    /// let cnbc_parser = NewsParser::new("cnbc");
    /// ```
    pub fn new(client_type: &str) -> Self {
        let mut namespaces = HashMap::new();

        // Define namespaces for different clients (similar to Python version)
        namespaces.insert(
            "wsj".to_string(),
            vec![
                "http://dowjones.net/rss/".to_string(),
                "http://purl.org/rss/1.0/modules/content/".to_string(),
                "http://search.yahoo.com/mrss/".to_string(),
            ],
        );

        namespaces.insert(
            "cnbc".to_string(),
            vec!["http://search.cnbc.com/rss/2.0/modules/siteContentMetadata".to_string()],
        );

        namespaces.insert(
            "nasdaq".to_string(),
            vec![
                "http://purl.org/dc/elements/1.1/".to_string(),
                "http://nasdaq.com/reference/feeds/1.0".to_string(),
            ],
        );

        namespaces.insert(
            "market_watch".to_string(),
            vec!["http://rssnamespace.org/feedburner/ext/1.0".to_string()],
        );

        namespaces.insert("sp_global".to_string(), vec![]);

        namespaces.insert(
            "seeking_alpha".to_string(),
            vec![
                "http://search.yahoo.com/mrss/".to_string(),
                "https://seekingalpha.com/api/1.0".to_string(),
            ],
        );

        namespaces.insert(
            "cnn_finance".to_string(),
            vec![
                "http://rssnamespace.org/feedburner/ext/1.0".to_string(),
                "http://search.yahoo.com/mrss/".to_string(),
            ],
        );

        namespaces.insert(
            "yahoo".to_string(),
            vec!["http://search.yahoo.com/mrss/".to_string()],
        );

        Self {
            client_type: client_type.to_string(),
            namespaces,
        }
    }

    /// Parse RSS/XML content into NewsArticle structs
    ///
    /// Processes RSS feed content and extracts article information, handling
    /// namespace-specific tags and converting them to standardized fields.
    ///
    /// # Arguments
    ///
    /// * `content` - Raw RSS/XML content as a string
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `NewsArticle` structs on success,
    /// or a `FanError` if parsing fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use finance_news_aggregator_rs::parser::NewsParser;
    ///
    /// let parser = NewsParser::new("wsj");
    /// let rss_content = r#"
    /// <rss>
    ///   <channel>
    ///     <item>
    ///       <title>Breaking News</title>
    ///       <link>https://wsj.com/article</link>
    ///     </item>
    ///   </channel>
    /// </rss>
    /// "#;
    ///
    /// let articles = parser.parse_response(rss_content)?;
    /// # Ok::<(), finance_news_aggregator_rs::error::FanError>(())
    /// ```
    pub fn parse_response(&self, content: &str) -> Result<Vec<NewsArticle>> {
        let mut reader = Reader::from_str(content);
        reader.config_mut().trim_text(true);

        let mut articles = Vec::new();
        let mut current_article = NewsArticle::new();
        let mut current_tag = String::new();
        let mut in_item = false;
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let tag_name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    current_tag = self.clean_tag_name(&tag_name);

                    if current_tag == "item" {
                        in_item = true;
                        current_article = NewsArticle::new();
                    }
                }
                Ok(Event::Text(e)) => {
                    if in_item && !current_tag.is_empty() {
                        // Convert bytes to UTF-8 string, replacing invalid sequences
                        let text = String::from_utf8_lossy(&e).to_string();
                        self.set_article_field(&mut current_article, &current_tag, text);
                    }
                }
                Ok(Event::End(ref e)) => {
                    let tag_name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    let clean_tag = self.clean_tag_name(&tag_name);

                    if clean_tag == "item" && in_item {
                        articles.push(current_article.clone());
                        in_item = false;
                    }
                    current_tag.clear();
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(FanError::XmlParsing(e)),
                _ => {}
            }
            buf.clear();
        }

        Ok(articles)
    }

    /// Clean tag names by removing namespaces and prefixes
    ///
    /// Removes source-specific XML namespaces and namespace prefixes to normalize
    /// tag names across different news sources.
    fn clean_tag_name(&self, tag: &str) -> String {
        let mut clean_tag = tag.to_string();

        if let Some(namespaces) = self.namespaces.get(&self.client_type) {
            for namespace in namespaces {
                clean_tag = clean_tag.replace(namespace, "");
            }
        }

        // Remove any remaining namespace prefixes
        if let Some(colon_pos) = clean_tag.rfind(':') {
            clean_tag = clean_tag[colon_pos + 1..].to_string();
        }

        clean_tag
    }

    /// Set the appropriate field in NewsArticle based on tag name
    ///
    /// Maps XML tag names to NewsArticle fields. Standard RSS tags like "title",
    /// "link", "description" are mapped to their corresponding fields, while
    /// unknown tags are stored in the `extra_fields` HashMap.
    fn set_article_field(&self, article: &mut NewsArticle, tag: &str, value: String) {
        match tag.to_lowercase().as_str() {
            "title" => article.title = Some(value),
            "link" => article.link = Some(value),
            "description" => article.description = Some(value),
            "pubdate" => article.pub_date = Some(value),
            "guid" => article.guid = Some(value),
            "category" => article.category = Some(value),
            "author" | "creator" => article.author = Some(value),
            _ => {
                article.extra_fields.insert(tag.to_string(), value);
            }
        }
    }
}
