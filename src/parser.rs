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
        // Pre-process the content to handle Unicode entities before XML parsing
        let preprocessed_content = self.preprocess_unicode_entities(content);

        let mut reader = Reader::from_str(&preprocessed_content);
        reader.config_mut().trim_text(true);

        let mut articles = Vec::new();
        let mut current_article = NewsArticle::new();
        let mut current_tag = String::new();
        let mut in_item = false;
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let tag_name = e.name();
                    let tag_str = match std::str::from_utf8(tag_name.as_ref()) {
                        Ok(s) => s,
                        Err(_) => {
                            log::warn!("Invalid UTF-8 in tag name");
                            continue;
                        }
                    };
                    current_tag = self.clean_tag_name(tag_str);

                    if current_tag == "item" {
                        in_item = true;
                        current_article = NewsArticle::new();
                    }
                }
                Ok(Event::Text(e)) => {
                    if in_item && !current_tag.is_empty() {
                        // Use the reader to decode entities properly
                        let mut text = match reader.decoder().decode(&e) {
                            Ok(cow_str) => cow_str.into_owned(),
                            Err(err) => {
                                log::warn!("Failed to decode text: {}", err);
                                // Fallback to raw UTF-8 conversion
                                match std::str::from_utf8(&e) {
                                    Ok(s) => s.to_string(),
                                    Err(_) => {
                                        log::warn!("Invalid UTF-8 in text content");
                                        continue;
                                    }
                                }
                            }
                        };

                        // Handle Unicode entities that the decoder might miss
                        text = self.decode_unicode_entities(&text);

                        self.set_article_field(&mut current_article, &current_tag, text);
                    }
                }
                Ok(Event::CData(e)) => {
                    if in_item && !current_tag.is_empty() {
                        // Handle CDATA sections
                        let text = match std::str::from_utf8(&e) {
                            Ok(s) => s.to_string(),
                            Err(_) => {
                                log::warn!("Invalid UTF-8 in CDATA section");
                                continue;
                            }
                        };
                        self.set_article_field(&mut current_article, &current_tag, text);
                    }
                }
                Ok(Event::End(ref e)) => {
                    let tag_name = e.name();
                    let tag_str = match std::str::from_utf8(tag_name.as_ref()) {
                        Ok(s) => s,
                        Err(_) => {
                            log::warn!("Invalid UTF-8 in end tag name");
                            continue;
                        }
                    };
                    let clean_tag = self.clean_tag_name(tag_str);

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

    /// Preprocess the entire XML content to handle Unicode entities before parsing
    ///
    /// This ensures that Unicode quotation marks are converted to regular apostrophes
    /// before the XML parser splits them into separate text nodes
    fn preprocess_unicode_entities(&self, content: &str) -> String {
        content
            .replace("&#x2018;", "'") // Left single quotation mark
            .replace("&#x2019;", "'") // Right single quotation mark
            .replace("&#x201C;", "\"") // Left double quotation mark
            .replace("&#x201D;", "\"") // Right double quotation mark
            .replace("&#8216;", "'") // Left single quotation mark (decimal)
            .replace("&#8217;", "'") // Right single quotation mark (decimal)
            .replace("&#8220;", "\"") // Left double quotation mark (decimal)
            .replace("&#8221;", "\"") // Right double quotation mark (decimal)
    }

    /// Decode Unicode entities that might not be handled by the XML decoder
    ///
    /// Handles numeric character references like &#x2018; and &#x2019; for proper apostrophes
    fn decode_unicode_entities(&self, text: &str) -> String {
        let mut result = text.to_string();

        // Handle hexadecimal numeric character references
        while let Some(start) = result.find("&#x") {
            if let Some(end) = result[start..].find(';') {
                let entity = &result[start..start + end + 1];
                let hex_part = &entity[3..entity.len() - 1]; // Remove &#x and ;

                if let Ok(code_point) = u32::from_str_radix(hex_part, 16)
                    && let Some(character) = char::from_u32(code_point) {
                        result = result.replace(entity, &character.to_string());
                        continue;
                    }
                // If we can't decode it, just remove the entity to avoid infinite loop
                result = result.replace(entity, "");
            } else {
                break;
            }
        }

        // Handle decimal numeric character references
        while let Some(start) = result.find("&#") {
            if result.chars().nth(start + 2) == Some('x') {
                // Skip hex entities (already handled above)
                if let Some(next_entity) = result[start + 1..].find("&#") {
                    let new_start = start + 1 + next_entity;
                    if new_start <= start {
                        break; // Avoid infinite loop
                    }
                    continue;
                } else {
                    break;
                }
            }

            if let Some(end) = result[start..].find(';') {
                let entity = &result[start..start + end + 1];
                let decimal_part = &entity[2..entity.len() - 1]; // Remove &# and ;

                if let Ok(code_point) = decimal_part.parse::<u32>()
                    && let Some(character) = char::from_u32(code_point) {
                        result = result.replace(entity, &character.to_string());
                        continue;
                    }
                // If we can't decode it, just remove the entity to avoid infinite loop
                result = result.replace(entity, "");
            } else {
                break;
            }
        }

        result
    }

    /// Set the appropriate field in NewsArticle based on tag name
    ///
    /// Maps XML tag names to NewsArticle fields. Standard RSS tags like "title",
    /// "link", "description" are mapped to their corresponding fields, while
    /// unknown tags are stored in the `extra_fields` HashMap.
    ///
    /// This method handles text accumulation for cases where XML content spans multiple text nodes.
    fn set_article_field(&self, article: &mut NewsArticle, tag: &str, value: String) {
        match tag.to_lowercase().as_str() {
            "title" => {
                if let Some(existing) = &article.title {
                    article.title = Some(format!("{}{}", existing, value));
                } else {
                    article.title = Some(value);
                }
            }
            "link" => {
                if let Some(existing) = &article.link {
                    article.link = Some(format!("{}{}", existing, value));
                } else {
                    article.link = Some(value);
                }
            }
            "description" => {
                if let Some(existing) = &article.description {
                    article.description = Some(format!("{}{}", existing, value));
                } else {
                    article.description = Some(value);
                }
            }
            "pubdate" => article.pub_date = Some(value),
            "guid" => article.guid = Some(value),
            "category" => article.category = Some(value),
            "author" | "creator" => article.author = Some(value),
            _ => {
                if let Some(existing) = article.extra_fields.get(tag) {
                    article
                        .extra_fields
                        .insert(tag.to_string(), format!("{}{}", existing, value));
                } else {
                    article.extra_fields.insert(tag.to_string(), value);
                }
            }
        }
    }
}
