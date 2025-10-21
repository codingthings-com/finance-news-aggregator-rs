# Implementation Plan

- [x] 1. Set up integration test infrastructure

  - Create tests/integration directory structure with mod.rs and utils subdirectory
  - Set up Cargo.toml dev-dependencies for integration testing (tokio-test, reqwest test features)
  - _Requirements: 1.4, 3.4_

- [x] 1.1 Create core test utilities and configuration

  - Implement TestConfig, TestResult, and TestContext structs in tests/integration/utils/mod.rs
  - Create IntegrationTestConfig with source filtering and timeout settings
  - _Requirements: 3.1, 3.3_

- [x] 1.2 Implement HTTP client factory

  - Create client_factory.rs with standardized client creation (30s timeout, retry logic)
  - Add user agent rotation and connection pooling configuration
  - _Requirements: 3.1, 3.2_

- [x] 1.3 Build custom assertion helpers

  - Implement assertions.rs with assert_valid_news_article, assert_valid_url, assert_non_empty_collection functions
  - Add article validation for title, link, description fields and URL format checking
  - _Requirements: 2.1, 2.2, 2.3_

- [x] 1.4 Create deprecation tracking system

  - Implement deprecation_tracker.rs with DeprecationTracker struct and error classification
  - Add methods to record failures, categorize errors (404/403 vs timeout vs parse), and generate reports
  - _Requirements: 5.1, 5.2, 5.3, 5.4, 5.5_

- [x] 2. Implement CNBC integration tests

  - Create tests/integration/test_cnbc.rs with comprehensive tests for all CNBC public functions
  - Test new(), name(), base_url(), available_topics(), and all topic-specific methods (business, investing, technology, etc.)
  - _Requirements: 1.1, 1.2, 1.3, 4.1, 4.3_

- [x] 2.1 Add CNBC data validation tests

  - Validate NewsArticle structure integrity for CNBC feeds
  - Test that articles contain valid titles, links, and properly formatted publication dates
  - _Requirements: 2.1, 2.2, 2.3_

- [x] 2.2 Create CNBC deprecation tests

  - Implement deprecation detection for outdated CNBC RSS feed IDs
  - _Requirements: 3.3, 5.1, 5.2_

- [x] 3. Implement CNN Finance integration tests

  - Create tests/integration/test_cnn_finance.rs testing all CNNFinance public methods
  - Test all_stories(), companies(), economy(), international(), investing(), markets(), media(), etc.
  - _Requirements: 1.1, 1.2, 1.3, 4.1_

- [x] 3.1 Add CNN Finance data validation

  - Validate CNNFinance NewsArticle data structure and content quality
  - Test morning_buzz() and personal_finance() specific functionality
  - _Requirements: 2.1, 2.2, 2.3_

- [x] 3.2 Create CNN Finance deprecation detection

  - Monitor CNN Finance base_url and buzz_url endpoint availability
  - Track and report deprecated CNN Finance feed categories
  - _Requirements: 5.1, 5.3, 5.4_

- [x] 4. Implement Market Watch integration tests

  - Create tests/integration/test_market_watch.rs for all MarketWatch public functions
  - Test auto_reviews(), banking_and_finance(), bulletins(), commentary(), market_pulse(), etc.
  - _Requirements: 1.1, 1.2, 1.3, 4.1_

- [x] 4.1 Add Market Watch specialized function tests

  - Test internet_stories(), software_stories(), newsletter_and_research() functions
  - Validate stocks_to_watch() and real_time_headlines() data quality
  - _Requirements: 2.1, 4.5_

- [x] 4.2 Implement Market Watch availability monitoring

  - Create deprecation reports for outdated Market Watch RSS feeds
  - _Requirements: 3.3, 5.2, 5.5_

- [x] 5. Implement NASDAQ integration tests

  - Create tests/integration/test_nasdaq.rs testing all NASDAQ public methods
  - Test commodities(), cryptocurrency(), dividends(), earnings(), economics(), innovation(), etc.
  - _Requirements: 1.1, 1.2, 1.3, 4.1_

- [x] 5.1 Add NASDAQ original content and category tests

  - Test original_content() and feed_by_category() functions with various category parameters
  - Validate financial_advisors() and stocks() data structure integrity
  - _Requirements: 2.1, 4.1, 4.5_

- [x] 5.2 Create NASDAQ endpoint validation

  - Monitor NASDAQ base_url and original_content_url availability
  - Track deprecated NASDAQ category endpoints and generate removal recommendations
  - _Requirements: 5.1, 5.3, 5.5_

- [x] 6. Implement Seeking Alpha integration tests

  - Create tests/integration/test_seeking_alpha.rs for all SeekingAlpha public functions
  - Test all_news(), editors_picks(), etfs(), forex(), ipo_analysis(), latest_articles(), etc.
  - _Requirements: 1.1, 1.2, 1.3, 4.1_

- [x] 6.1 Add Seeking Alpha parameterized function tests

  - Test global_markets() with different country parameters and sectors() with sector parameters
  - Test stocks() function with realistic ticker symbols (AAPL, MSFT, GOOGL)
  - _Requirements: 4.2, 4.5_

- [x] 6.2 Add Seeking Alpha specialized content tests

  - Test long_ideas(), short_ideas(), transcripts(), wall_street_breakfast() functions
  - Validate most_popular_articles() data quality and content structure
  - _Requirements: 2.1, 2.2_

- [x] 6.3 Implement Seeking Alpha deprecation tracking

  - Monitor Seeking Alpha base_url endpoint changes and API modifications
  - _Requirements: 5.1, 5.4, 5.5_

- [x] 7. Implement Wall Street Journal integration tests

  - Create tests/integration/test_wsj.rs testing all WallStreetJournal public methods
  - Test lifestyle(), market_news(), opinions(), technology_news(), us_business_news(), world_news()
  - _Requirements: 1.1, 1.2, 1.3, 4.1_

- [x] 7.1 Add WSJ configuration and client tests

  - Test both new() and with_config() client initialization methods
  - Validate WSJ SourceConfig integration and custom configuration handling
  - _Requirements: 4.4_

- [x] 7.2 Create WSJ endpoint monitoring

  - Track WSJ RSS feed availability and detect deprecated topic endpoints
  - Monitor WSJ configuration-based URL generation for errors
  - _Requirements: 5.1, 5.2, 5.3_

- [x] 8. Implement Yahoo Finance integration tests

  - Create tests/integration/test_yahoo_finance.rs for all YahooFinance public methods
  - Test headlines(), topstories(), and headline() with symbol parameters
  - _Requirements: 1.1, 1.2, 1.3, 4.1_

- [x] 8.1 Add Yahoo Finance symbol-based testing

  - Test headline() function with various stock ticker symbols and symbol arrays
  - Validate Yahoo Finance base_url endpoint functionality and data structure
  - _Requirements: 4.2, 2.1_

- [x] 8.2 Create Yahoo Finance deprecation detection

  - Monitor Yahoo Finance RSS index availability and detect feed changes
  - Track deprecated Yahoo Finance endpoints (like feeds.finance.yahoo.com/rss/2.0)
  - _Requirements: 5.1, 5.3, 5.4_

- [x] 9. Create comprehensive test runner and reporting

  - Implement main integration test runner that executes all source tests
  - Create test result aggregation and comprehensive reporting system
  - _Requirements: 3.4, 5.5_

- [x] 9.1 Add CI/CD integration and environment configuration

  - Implement environment-based test configuration (CI vs local vs nightly)
  - Add test execution controls and conditional test running based on features
  - _Requirements: 3.5_

- [ ]\* 9.2 Create performance regression detection

  - Implement baseline performance tracking and regression detection
  - Generate performance reports and identify slow endpoints over time
  - _Requirements: 3.3_

- [ ] 10. Add comprehensive error handling and recovery

  - Implement retry logic with exponential backoff for transient failures
  - Add error classification system distinguishing temporary vs permanent failures
  - _Requirements: 3.1, 3.2, 5.4_

- [ ] 10.1 Create final integration and validation
  - Run complete test suite against all news sources and validate end-to-end functionality
  - Generate final deprecation report with removal recommendations
  - _Requirements: 1.5, 5.5_
