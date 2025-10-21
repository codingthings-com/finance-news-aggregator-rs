use finance_news_aggregator_rs::NewsClient;
use finance_news_aggregator_rs::news_source::NewsSource;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let mut client = NewsClient::new();

    // Use the generic source to fetch any RSS feed
    let generic = client.generic();

    // Example: Fetch WSJ Opinion feed directly
    println!("Fetching WSJ Opinion feed...");
    let wsj_url = "https://feeds.a.dj.com/rss/RSSOpinion.xml";
    let articles = generic.fetch_feed_by_url(wsj_url).await?;
    println!("Found {} articles from WSJ Opinion\n", articles.len());

    // Example: Fetch any other RSS feed
    println!("Fetching CNBC Top News feed...");
    let cnbc_url = "https://www.cnbc.com/id/100003114/device/rss/rss.html";
    let cnbc_articles = generic.fetch_feed_by_url(cnbc_url).await?;
    println!("Found {} articles from CNBC\n", cnbc_articles.len());

    // Display first article from each
    if let Some(article) = articles.first() {
        println!("First WSJ article:");
        println!(
            "  Title: {}",
            article.title.as_ref().unwrap_or(&"N/A".to_string())
        );
        println!(
            "  Link: {}",
            article.link.as_ref().unwrap_or(&"N/A".to_string())
        );
    }

    if let Some(article) = cnbc_articles.first() {
        println!("\nFirst CNBC article:");
        println!(
            "  Title: {}",
            article.title.as_ref().unwrap_or(&"N/A".to_string())
        );
        println!(
            "  Link: {}",
            article.link.as_ref().unwrap_or(&"N/A".to_string())
        );
    }

    Ok(())
}
