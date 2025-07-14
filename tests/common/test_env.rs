//! Test environment utilities for managing environment variables and configuration

use std::env;

/// Environment variable configuration for private endpoint testing
pub struct TestEnv {
    /// Whether to run private tests (based on credential availability)
    pub run_private_tests: bool,
    /// Use testnet/sandbox environments when available
    pub use_testnet: bool,
    /// Verbose logging for debugging test issues
    pub verbose: bool,
}

impl TestEnv {
    /// Create a new test environment configuration
    pub fn new() -> Self {
        Self {
            run_private_tests: env::var("RUN_PRIVATE_TESTS").unwrap_or_default() == "true",
            use_testnet: env::var("USE_TESTNET").unwrap_or("true".to_string()) == "true",
            verbose: env::var("VERBOSE_TESTS").unwrap_or_default() == "true",
        }
    }

    /// Check if private tests should be skipped
    pub fn should_skip_private_tests(&self) -> bool {
        !self.run_private_tests
    }

    /// Get the appropriate API base URL for a venue (testnet vs mainnet)
    pub fn get_base_url(&self, venue: &str) -> String {
        if self.use_testnet {
            self.get_testnet_url(venue)
        } else {
            self.get_mainnet_url(venue)
        }
    }

    /// Get testnet/sandbox URL for a venue
    fn get_testnet_url(&self, venue: &str) -> String {
        match venue.to_lowercase().as_str() {
            "binance" => "https://testnet.binance.vision".to_string(),
            "binance_spot" => "https://testnet.binance.vision".to_string(),
            "binance_usdm" => "https://testnet.binancefuture.com".to_string(),
            "binance_coinm" => "https://testnet.binancefuture.com".to_string(),
            "okx" => "https://www.okx.com".to_string(), // OKX doesn't have separate testnet URLs
            "deribit" => "https://test.deribit.com".to_string(),
            "cryptocom" => "https://uat-api.3ona.co".to_string(),
            "coinbase" => "https://api-public.sandbox.exchange.coinbase.com".to_string(),
            "bybit" => "https://api-testnet.bybit.com".to_string(),
            _ => self.get_mainnet_url(venue), // Fallback to mainnet if no testnet
        }
    }

    /// Get mainnet URL for a venue
    fn get_mainnet_url(&self, venue: &str) -> String {
        match venue.to_lowercase().as_str() {
            "binance" | "binance_spot" => "https://api.binance.com".to_string(),
            "binance_usdm" => "https://fapi.binance.com".to_string(),
            "binance_coinm" => "https://dapi.binance.com".to_string(),
            "okx" => "https://www.okx.com".to_string(),
            "deribit" => "https://www.deribit.com".to_string(),
            "cryptocom" => "https://api.crypto.com".to_string(),
            "coinbase" => "https://api.exchange.coinbase.com".to_string(),
            "bybit" => "https://api.bybit.com".to_string(),
            "bullish" => "https://api.bullish.com".to_string(),
            "bingx" => "https://open-api.bingx.com".to_string(),
            "bitget" => "https://api.bitget.com".to_string(),
            "gateio" => "https://api.gateio.ws".to_string(),
            "kucoin" => "https://api.kucoin.com".to_string(),
            "bitmart" => "https://api-cloud.bitmart.com".to_string(),
            _ => panic!("Unknown venue: {}", venue),
        }
    }

    /// Print test environment info if verbose mode is enabled
    pub fn print_env_info(&self) {
        if self.verbose {
            println!("Test Environment Configuration:");
            println!("  Run private tests: {}", self.run_private_tests);
            println!("  Use testnet: {}", self.use_testnet);
            println!("  Verbose logging: {}", self.verbose);
        }
    }
}

impl Default for TestEnv {
    fn default() -> Self {
        Self::new()
    }
}
