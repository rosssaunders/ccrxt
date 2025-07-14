//! Private endpoint testing infrastructure and credential management
//!
//! This module provides utilities for safely testing private endpoints with proper
//! credential handling and test environment management.

use super::test_env::TestEnv;
use secrecy::SecretString;
use std::env;

/// Credentials for Binance API (all markets)
#[derive(Debug, Clone)]
pub struct BinanceCredentials {
    pub api_key: SecretString,
    pub secret_key: SecretString,
}

/// Credentials for OKX API
#[derive(Debug, Clone)]
pub struct OkxCredentials {
    pub api_key: SecretString,
    pub secret_key: SecretString,
    pub passphrase: SecretString,
}

/// Credentials for Deribit API
#[derive(Debug, Clone)]
pub struct DeribitCredentials {
    pub client_id: SecretString,
    pub client_secret: SecretString,
}

/// Credentials for Crypto.com API
#[derive(Debug, Clone)]
pub struct CryptocomCredentials {
    pub api_key: SecretString,
    pub secret_key: SecretString,
}

/// Credentials for Coinbase Exchange API
#[derive(Debug, Clone)]
pub struct CoinbaseCredentials {
    pub api_key: SecretString,
    pub secret_key: SecretString,
    pub passphrase: SecretString,
}

/// Credentials for Bybit API
#[derive(Debug, Clone)]
pub struct BybitCredentials {
    pub api_key: SecretString,
    pub secret_key: SecretString,
}

/// Credentials for Bullish API
#[derive(Debug, Clone)]
pub struct BullishCredentials {
    pub api_key: SecretString,
    pub secret_key: SecretString,
}

/// Credentials for BingX API
#[derive(Debug, Clone)]
pub struct BingxCredentials {
    pub api_key: SecretString,
    pub secret_key: SecretString,
}

/// Credentials for Bitget API
#[derive(Debug, Clone)]
pub struct BitgetCredentials {
    pub api_key: SecretString,
    pub secret_key: SecretString,
    pub passphrase: SecretString,
}

/// Credentials for Gate.io API
#[derive(Debug, Clone)]
pub struct GateioCredentials {
    pub api_key: SecretString,
    pub secret_key: SecretString,
}

/// Credentials for KuCoin API
#[derive(Debug, Clone)]
pub struct KucoinCredentials {
    pub api_key: SecretString,
    pub secret_key: SecretString,
    pub passphrase: SecretString,
}

/// Credentials for BitMart API
#[derive(Debug, Clone)]
pub struct BitmartCredentials {
    pub api_key: SecretString,
    pub secret_key: SecretString,
    pub memo: SecretString,
}

/// Private test configuration and utilities
pub struct PrivateTestConfig {
    pub env: TestEnv,
}

impl PrivateTestConfig {
    /// Create a new private test configuration
    pub fn new() -> Self {
        Self {
            env: TestEnv::new(),
        }
    }

    /// Check if private tests should be skipped and print skip message if needed
    pub fn skip_if_no_credentials(&self, venue: &str) -> bool {
        if self.env.should_skip_private_tests() {
            println!(
                "⚠️ Skipping {} private tests - set RUN_PRIVATE_TESTS=true and provide credentials to enable",
                venue
            );
            return true;
        }
        false
    }
}

/// Macro to skip private tests if credentials are not available
#[macro_export]
macro_rules! skip_private_test {
    ($venue:expr) => {
        let config = PrivateTestConfig::new();
        if config.skip_if_no_credentials($venue) {
            return;
        }
    };
}

/// Trait for loading credentials from environment variables
pub trait CredentialLoader {
    /// Load credentials from environment variables
    /// Returns None if required credentials are not available
    fn load_from_env() -> Option<Self>
    where
        Self: Sized;

    /// Get the venue name for logging
    fn venue_name() -> &'static str;
}

impl CredentialLoader for BinanceCredentials {
    fn load_from_env() -> Option<Self> {
        let api_key = env::var("BINANCE_API_KEY").ok()?;
        let secret_key = env::var("BINANCE_SECRET_KEY").ok()?;

        Some(Self {
            api_key: SecretString::new(api_key.into()),
            secret_key: SecretString::new(secret_key.into()),
        })
    }

    fn venue_name() -> &'static str {
        "Binance"
    }
}

impl CredentialLoader for OkxCredentials {
    fn load_from_env() -> Option<Self> {
        let api_key = env::var("OKX_API_KEY").ok()?;
        let secret_key = env::var("OKX_SECRET_KEY").ok()?;
        let passphrase = env::var("OKX_PASSPHRASE").ok()?;

        Some(Self {
            api_key: SecretString::new(api_key.into()),
            secret_key: SecretString::new(secret_key.into()),
            passphrase: SecretString::new(passphrase.into()),
        })
    }

    fn venue_name() -> &'static str {
        "OKX"
    }
}

impl CredentialLoader for DeribitCredentials {
    fn load_from_env() -> Option<Self> {
        let client_id = env::var("DERIBIT_CLIENT_ID").ok()?;
        let client_secret = env::var("DERIBIT_CLIENT_SECRET").ok()?;

        Some(Self {
            client_id: SecretString::new(client_id.into()),
            client_secret: SecretString::new(client_secret.into()),
        })
    }

    fn venue_name() -> &'static str {
        "Deribit"
    }
}

impl CredentialLoader for CryptocomCredentials {
    fn load_from_env() -> Option<Self> {
        let api_key = env::var("CRYPTOCOM_API_KEY").ok()?;
        let secret_key = env::var("CRYPTOCOM_SECRET_KEY").ok()?;

        Some(Self {
            api_key: SecretString::new(api_key.into()),
            secret_key: SecretString::new(secret_key.into()),
        })
    }

    fn venue_name() -> &'static str {
        "Crypto.com"
    }
}

impl CredentialLoader for CoinbaseCredentials {
    fn load_from_env() -> Option<Self> {
        let api_key = env::var("COINBASE_API_KEY").ok()?;
        let secret_key = env::var("COINBASE_SECRET_KEY").ok()?;
        let passphrase = env::var("COINBASE_PASSPHRASE").ok()?;

        Some(Self {
            api_key: SecretString::new(api_key.into()),
            secret_key: SecretString::new(secret_key.into()),
            passphrase: SecretString::new(passphrase.into()),
        })
    }

    fn venue_name() -> &'static str {
        "Coinbase"
    }
}

impl CredentialLoader for BybitCredentials {
    fn load_from_env() -> Option<Self> {
        let api_key = env::var("BYBIT_API_KEY").ok()?;
        let secret_key = env::var("BYBIT_SECRET_KEY").ok()?;

        Some(Self {
            api_key: SecretString::new(api_key.into()),
            secret_key: SecretString::new(secret_key.into()),
        })
    }

    fn venue_name() -> &'static str {
        "Bybit"
    }
}

impl CredentialLoader for BullishCredentials {
    fn load_from_env() -> Option<Self> {
        let api_key = env::var("BULLISH_API_KEY").ok()?;
        let secret_key = env::var("BULLISH_SECRET_KEY").ok()?;

        Some(Self {
            api_key: SecretString::new(api_key.into()),
            secret_key: SecretString::new(secret_key.into()),
        })
    }

    fn venue_name() -> &'static str {
        "Bullish"
    }
}

impl CredentialLoader for BingxCredentials {
    fn load_from_env() -> Option<Self> {
        let api_key = env::var("BINGX_API_KEY").ok()?;
        let secret_key = env::var("BINGX_SECRET_KEY").ok()?;

        Some(Self {
            api_key: SecretString::new(api_key.into()),
            secret_key: SecretString::new(secret_key.into()),
        })
    }

    fn venue_name() -> &'static str {
        "BingX"
    }
}

impl CredentialLoader for BitgetCredentials {
    fn load_from_env() -> Option<Self> {
        let api_key = env::var("BITGET_API_KEY").ok()?;
        let secret_key = env::var("BITGET_SECRET_KEY").ok()?;
        let passphrase = env::var("BITGET_PASSPHRASE").ok()?;

        Some(Self {
            api_key: SecretString::new(api_key.into()),
            secret_key: SecretString::new(secret_key.into()),
            passphrase: SecretString::new(passphrase.into()),
        })
    }

    fn venue_name() -> &'static str {
        "Bitget"
    }
}

impl CredentialLoader for GateioCredentials {
    fn load_from_env() -> Option<Self> {
        let api_key = env::var("GATEIO_API_KEY").ok()?;
        let secret_key = env::var("GATEIO_SECRET_KEY").ok()?;

        Some(Self {
            api_key: SecretString::new(api_key.into()),
            secret_key: SecretString::new(secret_key.into()),
        })
    }

    fn venue_name() -> &'static str {
        "Gate.io"
    }
}

impl CredentialLoader for KucoinCredentials {
    fn load_from_env() -> Option<Self> {
        let api_key = env::var("KUCOIN_API_KEY").ok()?;
        let secret_key = env::var("KUCOIN_SECRET_KEY").ok()?;
        let passphrase = env::var("KUCOIN_PASSPHRASE").ok()?;

        Some(Self {
            api_key: SecretString::new(api_key.into()),
            secret_key: SecretString::new(secret_key.into()),
            passphrase: SecretString::new(passphrase.into()),
        })
    }

    fn venue_name() -> &'static str {
        "KuCoin"
    }
}

impl CredentialLoader for BitmartCredentials {
    fn load_from_env() -> Option<Self> {
        let api_key = env::var("BITMART_API_KEY").ok()?;
        let secret_key = env::var("BITMART_SECRET_KEY").ok()?;
        let memo = env::var("BITMART_MEMO").ok()?;

        Some(Self {
            api_key: SecretString::new(api_key.into()),
            secret_key: SecretString::new(secret_key.into()),
            memo: SecretString::new(memo.into()),
        })
    }

    fn venue_name() -> &'static str {
        "BitMart"
    }
}

/// Helper macro to create a private test function that loads credentials and skips if unavailable
#[macro_export]
macro_rules! private_test {
    (
        $(#[$meta:meta])*
        async fn $test_name:ident<$cred_type:ty>($test_body:expr)
    ) => {
        $(#[$meta])*
        async fn $test_name() {
            use $crate::common::{CredentialLoader, PrivateTestConfig};

            let config = PrivateTestConfig::new();
            if config.skip_if_no_credentials(<$cred_type>::venue_name()) {
                return;
            }

            let credentials = match <$cred_type>::load_from_env() {
                Some(creds) => creds,
                None => {
                    println!("⚠️ Skipping {} private test - credentials not available", <$cred_type>::venue_name());
                    return;
                }
            };

            config.env.print_env_info();

            // Execute the test body
            $test_body(credentials, config).await;
        }
    };
}

impl Default for PrivateTestConfig {
    fn default() -> Self {
        Self::new()
    }
}
