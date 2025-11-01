use std::env;
use std::process;

use crate::integration::test_runner::IntegrationTestRunner;
use crate::integration::utils::environment::EnvironmentConfig;

/// Command-line interface for running integration tests
pub struct CliRunner;

impl CliRunner {
    /// Run integration tests from command line
    pub async fn run() {
        let args: Vec<String> = env::args().collect();

        // Parse command line arguments
        let mut sources_filter = None;
        let mut verbose = false;
        let mut help = false;

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--sources" | "-s" => {
                    if i + 1 < args.len() {
                        sources_filter = Some(args[i + 1].clone());
                        i += 2;
                    } else {
                        eprintln!("Error: --sources requires a value");
                        process::exit(1);
                    }
                }
                "--verbose" | "-v" => {
                    verbose = true;
                    i += 1;
                }
                "--help" | "-h" => {
                    help = true;
                    i += 1;
                }
                _ => {
                    eprintln!("Unknown argument: {}", args[i]);
                    help = true;
                    break;
                }
            }
        }

        if help {
            Self::print_help();
            return;
        }

        // Set environment variables based on CLI args
        if let Some(sources) = sources_filter {
            unsafe {
                env::set_var("INTEGRATION_SOURCES", sources);
            }
        }

        if verbose {
            unsafe {
                env::set_var("VERBOSE_OUTPUT", "true");
            }
        }

        // Initialize logging
        let log_level = if verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        };

        let _ = env_logger::builder().filter_level(log_level).try_init();

        println!("🚀 Finance News Aggregator - Integration Test Runner");
        println!("Environment: {:?}", EnvironmentConfig::from_env().test_mode);

        if let Some(ref sources) = env::var("INTEGRATION_SOURCES").ok() {
            println!("Testing sources: {}", sources);
        }

        println!();

        // Create and run tests
        match IntegrationTestRunner::new().await {
            Ok(mut runner) => {
                match runner.run_all_tests().await {
                    Ok(summary) => {
                        println!("✅ Integration tests completed successfully");

                        // Exit with appropriate code based on results
                        let success_rate =
                            summary.successful_tests as f64 / summary.total_tests as f64;
                        if success_rate < 0.5 {
                            println!(
                                "❌ Test suite failed - success rate too low: {:.1}%",
                                success_rate * 100.0
                            );
                            process::exit(1);
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to run integration tests: {}", e);
                        process::exit(1);
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Failed to initialize test runner: {}", e);
                process::exit(1);
            }
        }
    }

    /// Print help message
    fn print_help() {
        println!("Finance News Aggregator - Integration Test Runner");
        println!();
        println!("USAGE:");
        println!("    cargo test --test integration_test_suite");
        println!();
        println!("OPTIONS:");
        println!("    -s, --sources <SOURCES>    Comma-separated list of sources to test");
        println!(
            "                               (CNBC,MarketWatch,NASDAQ,SeekingAlpha,WallStreetJournal,YahooFinance)"
        );
        println!("    -v, --verbose              Enable verbose output");
        println!("    -h, --help                 Print this help message");
        println!();
        println!("ENVIRONMENT VARIABLES:");
        println!("    CI=1                       Run in CI mode (faster, less comprehensive)");
        println!(
            "    NIGHTLY_BUILD=1            Run in nightly mode (comprehensive deprecation scan)"
        );
        println!("    INTEGRATION_SOURCES        Comma-separated list of sources to test");
        println!(
            "    INTEGRATION_TIMEOUT        Timeout in seconds for network operations (default: 30)"
        );
        println!("    SKIP_NETWORK_TESTS=1       Skip network connectivity tests");
        println!("    ENABLE_DEPRECATION_TRACKING=1  Enable deprecation detection");
        println!("    ENABLE_PERFORMANCE_TRACKING=1  Enable performance monitoring");
        println!("    VERBOSE_OUTPUT=1           Enable verbose output");
        println!();
        println!("EXAMPLES:");
        println!("    # Run all tests");
        println!("    cargo test --test integration_test_suite");
        println!();
        println!("    # Test only CNBC and WSJ");
        println!(
            "    INTEGRATION_SOURCES=CNBC,WallStreetJournal cargo test --test integration_test_suite"
        );
        println!();
        println!("    # Run in CI mode");
        println!("    CI=1 cargo test --test integration_test_suite");
        println!();
        println!("    # Run nightly deprecation scan");
        println!("    NIGHTLY_BUILD=1 cargo test --test integration_test_suite");
        println!();
        println!("    # Run specific test categories");
        println!("    cargo test --test integration_test_suite run_cnbc_only_tests -- --ignored");
        println!(
            "    cargo test --test integration_test_suite run_performance_regression_tests -- --ignored"
        );
    }
}

/// Example usage function for documentation
#[allow(dead_code)]
pub fn example_usage() {
    println!("Example integration test runner usage:");
    println!();
    println!("1. Run all tests:");
    println!("   cargo test --test integration_test_suite");
    println!();
    println!("2. Test specific sources:");
    println!("   INTEGRATION_SOURCES=CNBC,WSJ cargo test --test integration_test_suite");
    println!();
    println!("3. Run in CI mode:");
    println!("   CI=1 cargo test --test integration_test_suite");
    println!();
    println!("4. Run nightly comprehensive scan:");
    println!("   NIGHTLY_BUILD=1 cargo test --test integration_test_suite");
    println!();
    println!("5. Run performance tests:");
    println!(
        "   cargo test --test integration_test_suite run_performance_regression_tests -- --ignored"
    );
    println!();
    println!("6. Run deprecation detection:");
    println!(
        "   cargo test --test integration_test_suite run_deprecation_detection_tests -- --ignored"
    );
}
