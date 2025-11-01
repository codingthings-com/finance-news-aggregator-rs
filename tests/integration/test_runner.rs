use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::time::timeout;
// use futures::future::join_all; // Not used yet

use crate::integration::utils::{
    IntegrationTestConfig,
    TestContext,
    TestResult,
    client_factory::ClientFactory,
    // deprecation_tracker::DeprecationTracker, // Not used directly
    environment::{EnvironmentConfig, TestMode},
};

use finance_news_aggregator_rs::news_source::{
    NewsSource, cnbc::CNBC, market_watch::MarketWatch, nasdaq::NASDAQ,
    seeking_alpha::SeekingAlpha, wsj::WallStreetJournal, yahoo_finance::YahooFinance,
};

/// Comprehensive test runner for all news sources
pub struct IntegrationTestRunner {
    config: EnvironmentConfig,
    context: TestContext,
    results: Vec<TestResult>,
    source_results: HashMap<String, Vec<TestResult>>,
    start_time: Instant,
}

/// Summary of test execution results
#[derive(Debug, Clone)]
pub struct TestSummary {
    pub total_tests: usize,
    pub successful_tests: usize,
    pub failed_tests: usize,
    pub total_articles: usize,
    pub total_execution_time: Duration,
    pub source_summaries: HashMap<String, SourceSummary>,
    pub deprecation_report: String,
    pub performance_report: Option<String>,
}

/// Summary for individual news source
#[derive(Debug, Clone)]
pub struct SourceSummary {
    pub source_name: String,
    pub tests_run: usize,
    pub tests_passed: usize,
    pub tests_failed: usize,
    pub total_articles: usize,
    pub average_response_time: Duration,
    pub success_rate: f64,
    pub failed_functions: Vec<String>,
}

impl IntegrationTestRunner {
    /// Create a new test runner with environment-based configuration
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let env_config = EnvironmentConfig::from_env();
        let client = ClientFactory::create_test_client()?;

        let integration_config = IntegrationTestConfig {
            test_timeout_seconds: env_config.timeout_seconds,
            network_retry_attempts: env_config.max_retries,
            deprecation_tracking_enabled: env_config.enable_deprecation_tracking,
            ci_mode: env_config.test_mode == TestMode::CI,
            ..Default::default()
        };

        let context = TestContext::new(client, integration_config);

        Ok(Self {
            config: env_config,
            context,
            results: Vec::new(),
            source_results: HashMap::new(),
            start_time: Instant::now(),
        })
    }

    /// Run all integration tests
    pub async fn run_all_tests(&mut self) -> Result<TestSummary, Box<dyn std::error::Error>> {
        println!("🚀 Starting comprehensive integration test suite");
        println!("Environment: {:?}", self.config.test_mode);
        println!("Configuration: {:?}", self.config);
        println!();

        let sources_to_test = self.get_sources_to_test();

        if self.config.parallel_execution {
            self.run_tests_parallel(sources_to_test).await?;
        } else {
            self.run_tests_sequential(sources_to_test).await?;
        }

        let summary = self.generate_summary();
        self.print_final_report(&summary);

        Ok(summary)
    }

    /// Get list of sources to test based on configuration
    fn get_sources_to_test(&self) -> Vec<&'static str> {
        let all_sources = vec![
            "CNBC",
            "MarketWatch",
            "NASDAQ",
            "SeekingAlpha",
            "WallStreetJournal",
            "YahooFinance",
        ];

        all_sources
            .into_iter()
            .filter(|source| self.config.should_test_source(source))
            .collect()
    }

    /// Run tests in parallel for better performance
    async fn run_tests_parallel(
        &mut self,
        sources: Vec<&'static str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔄 Running tests in parallel mode");

        let mut tasks = Vec::new();

        for source in sources {
            let client = self.context.client.clone();
            let config = self.config.clone();

            let task =
                tokio::spawn(async move { Self::test_source_async(source, client, config).await });

            tasks.push((source, task));
        }

        for (source, task) in tasks {
            match task.await {
                Ok(results) => {
                    println!("✅ Completed tests for {}", source);
                    self.source_results
                        .insert(source.to_string(), results.clone());
                    self.results.extend(results);
                }
                Err(e) => {
                    println!("❌ Failed to complete tests for {}: {}", source, e);
                }
            }
        }

        Ok(())
    }

    /// Run tests sequentially for more controlled execution
    async fn run_tests_sequential(
        &mut self,
        sources: Vec<&'static str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔄 Running tests in sequential mode");

        for source in sources {
            println!("📊 Testing {} source...", source);

            let results =
                Self::test_source_async(source, self.context.client.clone(), self.config.clone())
                    .await;

            println!("✅ Completed {} tests for {}", results.len(), source);
            self.source_results
                .insert(source.to_string(), results.clone());
            self.results.extend(results);
        }

        Ok(())
    }

    /// Test a specific news source asynchronously
    async fn test_source_async(
        source_name: &str,
        client: reqwest::Client,
        config: EnvironmentConfig,
    ) -> Vec<TestResult> {
        let timeout_duration = Duration::from_secs(config.timeout_seconds);

        let test_future = async {
            match source_name {
                "CNBC" => Self::test_cnbc_source(client).await,
                "MarketWatch" => Self::test_market_watch_source(client).await,
                "NASDAQ" => Self::test_nasdaq_source(client).await,
                "SeekingAlpha" => Self::test_seeking_alpha_source(client).await,
                "WallStreetJournal" => Self::test_wsj_source(client).await,
                "YahooFinance" => Self::test_yahoo_finance_source(client).await,
                _ => {
                    println!("⚠️  Unknown source: {}", source_name);
                    Vec::new()
                }
            }
        };

        match timeout(timeout_duration, test_future).await {
            Ok(results) => results,
            Err(_) => {
                println!(
                    "⏰ Timeout testing {} after {:?}",
                    source_name, timeout_duration
                );
                vec![TestResult::failure(
                    &format!("{}_timeout", source_name),
                    format!("Source test timed out after {:?}", timeout_duration),
                    timeout_duration,
                )]
            }
        }
    }

    /// Test CNBC news source
    async fn test_cnbc_source(client: reqwest::Client) -> Vec<TestResult> {
        let cnbc = CNBC::new(client);
        let mut results = Vec::new();

        // Test basic functionality
        results.push(Self::test_basic_functionality(&cnbc, "CNBC").await);

        // Test main functions
        results.extend(vec![
            Self::test_function("top_news", || cnbc.top_news()).await,
            Self::test_function("business", || cnbc.business()).await,
            Self::test_function("technology", || cnbc.technology()).await,
            Self::test_function("investing", || cnbc.investing()).await,
            Self::test_function("world_news", || cnbc.world_news()).await,
        ]);

        // Test topic-based functions
        let topics = vec!["economy", "finance", "politics", "health_care"];
        for topic in topics {
            results.push(
                Self::test_function(&format!("fetch_topic({})", topic), || {
                    cnbc.fetch_topic(topic)
                })
                .await,
            );
        }

        results
    }

    /// Test Market Watch news source
    async fn test_market_watch_source(client: reqwest::Client) -> Vec<TestResult> {
        let mw = MarketWatch::new(client);
        let mut results = Vec::new();

        results.push(Self::test_basic_functionality(&mw, "MarketWatch").await);

        // Only test working feeds (many MarketWatch feeds are broken)
        results.extend(vec![
            Self::test_function("top_stories", || mw.top_stories()).await,
            Self::test_function("real_time_headlines", || mw.real_time_headlines()).await,
            Self::test_function("market_pulse", || mw.market_pulse()).await,
            Self::test_function("bulletins", || mw.bulletins()).await,
        ]);

        results
    }

    /// Test NASDAQ news source
    async fn test_nasdaq_source(client: reqwest::Client) -> Vec<TestResult> {
        let nasdaq = NASDAQ::new(client);
        let mut results = Vec::new();

        results.push(Self::test_basic_functionality(&nasdaq, "NASDAQ").await);

        results.extend(vec![
            Self::test_function("commodities", || nasdaq.commodities()).await,
            Self::test_function("cryptocurrency", || nasdaq.cryptocurrency()).await,
            Self::test_function("dividends", || nasdaq.dividends()).await,
            Self::test_function("earnings", || nasdaq.earnings()).await,
            Self::test_function("economics", || nasdaq.economics()).await,
            Self::test_function("innovation", || nasdaq.innovation()).await,
            Self::test_function("original_content", || nasdaq.original_content()).await,
            Self::test_function("financial_advisors", || nasdaq.financial_advisors()).await,
            Self::test_function("stocks", || nasdaq.stocks()).await,
        ]);

        // Test category-based function
        let categories = vec!["commodities", "cryptocurrency", "earnings"];
        for category in categories {
            results.push(
                Self::test_function(&format!("fetch_topic({})", category), || {
                    nasdaq.fetch_topic(category)
                })
                .await,
            );
        }

        results
    }

    /// Test Seeking Alpha news source
    async fn test_seeking_alpha_source(client: reqwest::Client) -> Vec<TestResult> {
        let sa = SeekingAlpha::new(client);
        let mut results = Vec::new();

        results.push(Self::test_basic_functionality(&sa, "SeekingAlpha").await);

        results.extend(vec![
            Self::test_function("all_news", || sa.all_news()).await,
            Self::test_function("editors_picks", || sa.editors_picks()).await,
            Self::test_function("etfs", || sa.etfs()).await,
            Self::test_function("forex", || sa.forex()).await,
            Self::test_function("ipo_analysis", || sa.ipo_analysis()).await,
            Self::test_function("latest_articles", || sa.latest_articles()).await,
            Self::test_function("long_ideas", || sa.long_ideas()).await,
            Self::test_function("short_ideas", || sa.short_ideas()).await,
            Self::test_function("transcripts", || sa.transcripts()).await,
            Self::test_function("wall_street_breakfast", || sa.wall_street_breakfast()).await,
            Self::test_function("most_popular_articles", || sa.most_popular_articles()).await,
        ]);

        // Test parameterized functions
        let countries = vec!["US", "UK", "Germany"];
        for country in countries {
            results.push(
                Self::test_function(&format!("global_markets({})", country), || {
                    sa.global_markets(country)
                })
                .await,
            );
        }

        let sectors = vec!["technology", "healthcare", "finance"];
        for sector in sectors {
            results.push(
                Self::test_function(&format!("sectors({})", sector), || sa.sectors(sector)).await,
            );
        }

        let symbols = vec!["AAPL", "MSFT", "GOOGL"];
        for symbol in symbols {
            results.push(
                Self::test_function(&format!("stocks({})", symbol), || sa.stocks(symbol)).await,
            );
        }

        results
    }

    /// Test Wall Street Journal news source
    async fn test_wsj_source(client: reqwest::Client) -> Vec<TestResult> {
        let wsj = WallStreetJournal::new(client.clone());
        let mut results = Vec::new();

        results.push(Self::test_basic_functionality(&wsj, "WallStreetJournal").await);

        results.extend(vec![
            Self::test_function("lifestyle", || wsj.lifestyle()).await,
            Self::test_function("market_news", || wsj.market_news()).await,
            Self::test_function("opinions", || wsj.opinions()).await,
            Self::test_function("technology_news", || wsj.technology_news()).await,
            Self::test_function("us_business_news", || wsj.us_business_news()).await,
            Self::test_function("world_news", || wsj.world_news()).await,
        ]);

        // Test with custom configuration
        let config = finance_news_aggregator_rs::types::SourceConfig::default();
        let wsj_with_config = WallStreetJournal::with_config(client, config);
        results.push(
            Self::test_basic_functionality(&wsj_with_config, "WallStreetJournal_with_config").await,
        );

        results
    }

    /// Test Yahoo Finance news source
    async fn test_yahoo_finance_source(client: reqwest::Client) -> Vec<TestResult> {
        let yf = YahooFinance::new(client.clone());
        let mut results = Vec::new();

        results.push(Self::test_basic_functionality(&yf, "YahooFinance").await);

        results.extend(vec![
            Self::test_function("headlines", || yf.headlines()).await,
            Self::test_function("topstories", || yf.topstories()).await,
        ]);

        // Test symbol-based functions
        let test_symbols = ["AAPL", "MSFT", "TSLA"];
        for symbol in test_symbols {
            let yf_for_test = YahooFinance::new(client.clone());
            let symbol_vec = vec![symbol];
            let result = Self::test_function_with_symbols(
                &format!("headline({})", symbol),
                yf_for_test,
                symbol_vec,
            )
            .await;
            results.push(result);
        }

        // Test with symbol arrays
        let symbol_arrays: Vec<Vec<&str>> =
            vec![vec!["AAPL", "MSFT"], vec!["GOOGL", "AMZN", "TSLA"]];
        for (i, symbols) in symbol_arrays.iter().enumerate() {
            let yf_for_test = YahooFinance::new(client.clone());
            let result = Self::test_function_with_symbols(
                &format!("headline(array_{})", i),
                yf_for_test,
                symbols.clone(),
            )
            .await;
            results.push(result);
        }

        results
    }

    /// Test a Yahoo Finance function with symbols
    async fn test_function_with_symbols(
        function_name: &str,
        yf: YahooFinance,
        symbols: Vec<&str>,
    ) -> TestResult {
        let start_time = Instant::now();

        match yf.headline(&symbols).await {
            Ok(articles) => {
                TestResult::success(function_name, articles.len(), start_time.elapsed())
            }
            Err(e) => TestResult::failure(function_name, e.to_string(), start_time.elapsed()),
        }
    }

    /// Test basic functionality common to all sources
    async fn test_basic_functionality<T: NewsSource>(source: &T, source_name: &str) -> TestResult {
        let start_time = Instant::now();

        // Test name() function
        let name = source.name();
        if name.is_empty() {
            return TestResult::failure(
                &format!("{}_basic", source_name),
                "name() returned empty string".to_string(),
                start_time.elapsed(),
            );
        }

        // Test available_topics() function
        let topics = source.available_topics();
        if topics.is_empty() {
            return TestResult::failure(
                &format!("{}_basic", source_name),
                "available_topics() returned empty list".to_string(),
                start_time.elapsed(),
            );
        }

        TestResult::success(&format!("{}_basic", source_name), 0, start_time.elapsed())
    }

    /// Test a specific function with error handling
    async fn test_function<F, Fut>(function_name: &str, test_fn: F) -> TestResult
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<
                Output = Result<
                    Vec<finance_news_aggregator_rs::types::NewsArticle>,
                    finance_news_aggregator_rs::error::FanError,
                >,
            >,
    {
        let start_time = Instant::now();

        match test_fn().await {
            Ok(articles) => {
                TestResult::success(function_name, articles.len(), start_time.elapsed())
            }
            Err(e) => TestResult::failure(function_name, e.to_string(), start_time.elapsed()),
        }
    }

    /// Generate comprehensive test summary
    fn generate_summary(&self) -> TestSummary {
        let total_tests = self.results.len();
        let successful_tests = self.results.iter().filter(|r| r.success).count();
        let failed_tests = total_tests - successful_tests;
        let total_articles: usize = self.results.iter().map(|r| r.article_count).sum();
        let total_execution_time = self.start_time.elapsed();

        let mut source_summaries = HashMap::new();

        for (source_name, results) in &self.source_results {
            let tests_run = results.len();
            let tests_passed = results.iter().filter(|r| r.success).count();
            let tests_failed = tests_run - tests_passed;
            let total_articles: usize = results.iter().map(|r| r.article_count).sum();
            let avg_time = if tests_run > 0 {
                let total_ms = results.iter().map(|r| r.execution_time_ms).sum::<u128>();
                let avg_ms = (total_ms / tests_run as u128) as u64;
                Duration::from_millis(avg_ms)
            } else {
                Duration::from_millis(0)
            };
            let success_rate = if tests_run > 0 {
                tests_passed as f64 / tests_run as f64
            } else {
                0.0
            };
            let failed_functions: Vec<String> = results
                .iter()
                .filter(|r| !r.success)
                .map(|r| r.function_name.clone())
                .collect();

            source_summaries.insert(
                source_name.clone(),
                SourceSummary {
                    source_name: source_name.clone(),
                    tests_run,
                    tests_passed,
                    tests_failed,
                    total_articles,
                    average_response_time: avg_time,
                    success_rate,
                    failed_functions,
                },
            );
        }

        let deprecation_report = if self.config.enable_deprecation_tracking {
            self.context
                .deprecation_tracker
                .generate_report()
                .to_string()
        } else {
            "Deprecation tracking disabled".to_string()
        };

        let performance_report = if self.config.enable_performance_tracking {
            Some(self.generate_performance_report())
        } else {
            None
        };

        TestSummary {
            total_tests,
            successful_tests,
            failed_tests,
            total_articles,
            total_execution_time,
            source_summaries,
            deprecation_report,
            performance_report,
        }
    }

    /// Generate performance analysis report
    fn generate_performance_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== PERFORMANCE ANALYSIS ===\n");

        for (source_name, results) in &self.source_results {
            let successful_results: Vec<_> = results.iter().filter(|r| r.success).collect();

            if successful_results.is_empty() {
                continue;
            }

            let times: Vec<u128> = successful_results
                .iter()
                .map(|r| r.execution_time_ms)
                .collect();
            let avg_time = times.iter().sum::<u128>() / times.len() as u128;
            let min_time = *times.iter().min().unwrap_or(&0);
            let max_time = *times.iter().max().unwrap_or(&0);

            report.push_str(&format!(
                "{}: avg={}ms, min={}ms, max={}ms\n",
                source_name, avg_time, min_time, max_time
            ));

            // Identify slow functions (> 5 seconds)
            let slow_functions: Vec<_> = successful_results
                .iter()
                .filter(|r| r.execution_time_ms > 5000)
                .collect();

            if !slow_functions.is_empty() {
                report.push_str(&format!("  Slow functions (>5s): "));
                for func in slow_functions {
                    report.push_str(&format!(
                        "{}({}ms) ",
                        func.function_name, func.execution_time_ms
                    ));
                }
                report.push('\n');
            }
        }

        report
    }

    /// Print comprehensive final report
    fn print_final_report(&self, summary: &TestSummary) {
        println!("\n🎯 ===== INTEGRATION TEST SUMMARY =====");
        println!(
            "⏱️  Total execution time: {:?}",
            summary.total_execution_time
        );
        println!("📊 Total tests: {}", summary.total_tests);
        println!(
            "✅ Successful: {} ({:.1}%)",
            summary.successful_tests,
            summary.successful_tests as f64 / summary.total_tests as f64 * 100.0
        );
        println!(
            "❌ Failed: {} ({:.1}%)",
            summary.failed_tests,
            summary.failed_tests as f64 / summary.total_tests as f64 * 100.0
        );
        println!("📰 Total articles fetched: {}", summary.total_articles);
        println!();

        println!("📈 === SOURCE BREAKDOWN ===");
        for (source_name, source_summary) in &summary.source_summaries {
            println!(
                "🔸 {}: {}/{} passed ({:.1}%) - {} articles - avg {:?}",
                source_name,
                source_summary.tests_passed,
                source_summary.tests_run,
                source_summary.success_rate * 100.0,
                source_summary.total_articles,
                source_summary.average_response_time
            );

            if !source_summary.failed_functions.is_empty() && self.config.verbose_output {
                println!("   Failed functions: {:?}", source_summary.failed_functions);
            }
        }
        println!();

        if self.config.enable_deprecation_tracking {
            println!("🔍 === DEPRECATION REPORT ===");
            println!("{}", summary.deprecation_report);
            println!();
        }

        if let Some(ref performance_report) = summary.performance_report {
            println!("⚡ === PERFORMANCE REPORT ===");
            println!("{}", performance_report);
            println!();
        }

        // Overall health assessment
        let overall_success_rate = summary.successful_tests as f64 / summary.total_tests as f64;
        if overall_success_rate >= 0.9 {
            println!(
                "🎉 Overall Status: EXCELLENT ({:.1}% success rate)",
                overall_success_rate * 100.0
            );
        } else if overall_success_rate >= 0.75 {
            println!(
                "✅ Overall Status: GOOD ({:.1}% success rate)",
                overall_success_rate * 100.0
            );
        } else if overall_success_rate >= 0.5 {
            println!(
                "⚠️  Overall Status: NEEDS ATTENTION ({:.1}% success rate)",
                overall_success_rate * 100.0
            );
        } else {
            println!(
                "🚨 Overall Status: CRITICAL ({:.1}% success rate)",
                overall_success_rate * 100.0
            );
        }

        println!("=====================================\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_runner_creation() {
        let runner = IntegrationTestRunner::new().await;
        assert!(runner.is_ok());
    }

    #[tokio::test]
    async fn test_basic_functionality_test() {
        let client = ClientFactory::create_test_client().unwrap();
        let cnbc = CNBC::new(client);
        let result = IntegrationTestRunner::test_basic_functionality(&cnbc, "CNBC").await;
        assert!(result.success);
    }
}
