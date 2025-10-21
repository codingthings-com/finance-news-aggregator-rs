use finance_news_aggregator_rs::types::NewsArticle;
use reqwest::Url;
use std::collections::HashMap;

/// Custom assertion helpers for integration testing

/// Assert that a NewsArticle contains valid data
pub fn assert_valid_news_article(article: &NewsArticle, require_all_fields: bool) {
    if require_all_fields {
        assert!(article.title.is_some(), "Article title should not be None");
        assert!(article.link.is_some(), "Article link should not be None");
        assert!(
            article.description.is_some(),
            "Article description should not be None"
        );
    } else {
        // At minimum, we expect either title or description to be present
        assert!(
            article.title.is_some() || article.description.is_some(),
            "Article should have at least a title or description"
        );
    }

    // If title exists, it should not be empty
    if let Some(ref title) = article.title {
        assert!(
            !title.trim().is_empty(),
            "Article title should not be empty"
        );
    }

    // If link exists, it should be a valid URL
    if let Some(ref link) = article.link {
        assert!(!link.trim().is_empty(), "Article link should not be empty");
        assert_valid_url(link);
    }

    // If description exists, it should not be empty
    if let Some(ref description) = article.description {
        assert!(
            !description.trim().is_empty(),
            "Article description should not be empty"
        );
    }

    // If pub_date exists, it should not be empty
    if let Some(ref pub_date) = article.pub_date {
        assert!(
            !pub_date.trim().is_empty(),
            "Article pub_date should not be empty"
        );
    }
}

/// Assert that a URL string is valid and properly formatted
pub fn assert_valid_url(url_str: &str) {
    assert!(!url_str.trim().is_empty(), "URL should not be empty");

    match Url::parse(url_str) {
        Ok(url) => {
            assert!(
                url.scheme() == "http" || url.scheme() == "https",
                "URL should use HTTP or HTTPS scheme, got: {}",
                url.scheme()
            );
            assert!(url.host().is_some(), "URL should have a valid host");
        }
        Err(e) => panic!("Invalid URL format '{}': {}", url_str, e),
    }
}

/// Assert that a collection is not empty and contains valid data
pub fn assert_non_empty_collection<T>(collection: &[T], collection_name: &str) {
    assert!(
        !collection.is_empty(),
        "{} should not be empty",
        collection_name
    );
}

/// Assert that a collection of NewsArticles contains at least one valid article
pub fn assert_valid_news_collection(articles: &[NewsArticle], min_count: usize) {
    assert_non_empty_collection(articles, "News articles collection");
    assert!(
        articles.len() >= min_count,
        "Expected at least {} articles, got {}",
        min_count,
        articles.len()
    );

    // Check that at least one article has meaningful content
    let valid_articles = articles
        .iter()
        .filter(|article| {
            article
                .title
                .as_ref()
                .map_or(false, |t| !t.trim().is_empty())
                || article
                    .description
                    .as_ref()
                    .map_or(false, |d| !d.trim().is_empty())
        })
        .count();

    assert!(
        valid_articles > 0,
        "At least one article should have a non-empty title or description"
    );
}

/// Assert that execution time is within reasonable bounds
pub fn assert_reasonable_execution_time(execution_time_ms: u128, max_time_ms: u128) {
    assert!(
        execution_time_ms <= max_time_ms,
        "Execution time {}ms exceeds maximum allowed time {}ms",
        execution_time_ms,
        max_time_ms
    );
}

/// Assert that a string collection contains expected values
pub fn assert_contains_expected_values(
    collection: &[String],
    expected_values: &[&str],
    collection_name: &str,
) {
    assert_non_empty_collection(collection, collection_name);

    for expected in expected_values {
        assert!(
            collection.iter().any(|item| item.contains(expected)),
            "{} should contain an item with '{}'",
            collection_name,
            expected
        );
    }
}

/// Assert that a HashMap contains expected keys
pub fn assert_contains_keys<V>(map: &HashMap<String, V>, expected_keys: &[&str], map_name: &str) {
    for key in expected_keys {
        assert!(
            map.contains_key(*key),
            "{} should contain key '{}'",
            map_name,
            key
        );
    }
}

/// Validate article fields for specific quality requirements
pub struct ArticleValidationRules {
    pub require_title: bool,
    pub require_link: bool,
    pub require_description: bool,
    pub validate_url_format: bool,
    pub validate_date_format: bool,
    pub minimum_title_length: usize,
    pub minimum_description_length: usize,
}

impl Default for ArticleValidationRules {
    fn default() -> Self {
        Self {
            require_title: true,
            require_link: true,
            require_description: false,
            validate_url_format: true,
            validate_date_format: false,
            minimum_title_length: 5,
            minimum_description_length: 10,
        }
    }
}

impl ArticleValidationRules {
    /// Create lenient validation rules for testing
    pub fn lenient() -> Self {
        Self {
            require_title: false,
            require_link: false,
            require_description: false,
            validate_url_format: true,
            validate_date_format: false,
            minimum_title_length: 1,
            minimum_description_length: 1,
        }
    }

    /// Create strict validation rules for testing
    pub fn strict() -> Self {
        Self {
            require_title: true,
            require_link: true,
            require_description: true,
            validate_url_format: true,
            validate_date_format: true,
            minimum_title_length: 10,
            minimum_description_length: 20,
        }
    }
}

/// Assert that an article meets specific validation rules
pub fn assert_article_meets_rules(article: &NewsArticle, rules: &ArticleValidationRules) {
    if rules.require_title {
        assert!(article.title.is_some(), "Article title is required");
        let title = article.title.as_ref().unwrap();
        assert!(
            title.len() >= rules.minimum_title_length,
            "Title length {} is below minimum {}",
            title.len(),
            rules.minimum_title_length
        );
    }

    if rules.require_link {
        assert!(article.link.is_some(), "Article link is required");
        if rules.validate_url_format {
            assert_valid_url(article.link.as_ref().unwrap());
        }
    }

    if rules.require_description {
        assert!(
            article.description.is_some(),
            "Article description is required"
        );
        let description = article.description.as_ref().unwrap();
        assert!(
            description.len() >= rules.minimum_description_length,
            "Description length {} is below minimum {}",
            description.len(),
            rules.minimum_description_length
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_url_assertion() {
        assert_valid_url("https://www.example.com");
        assert_valid_url("http://example.com/path?query=value");
    }

    #[test]
    #[should_panic(expected = "Invalid URL format")]
    fn test_invalid_url_assertion() {
        assert_valid_url("not-a-url");
    }

    #[test]
    fn test_non_empty_collection() {
        let collection = vec!["item1", "item2"];
        assert_non_empty_collection(&collection, "test collection");
    }

    #[test]
    #[should_panic(expected = "should not be empty")]
    fn test_empty_collection_assertion() {
        let collection: Vec<String> = vec![];
        assert_non_empty_collection(&collection, "test collection");
    }

    #[test]
    fn test_valid_news_article() {
        let mut article = NewsArticle::new();
        article.title = Some("Test Title".to_string());
        article.link = Some("https://example.com".to_string());
        article.description = Some("Test description".to_string());

        assert_valid_news_article(&article, false);
    }
}
