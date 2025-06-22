use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bitmart::RestResult;
use crate::bitmart::rate_limit::EndpointType;

/// Request parameters for getting margin account details (isolated)
#[derive(Debug, Serialize, Default)]
pub struct GetMarginIsolatedAccountRequest {
    /// Trading pair (e.g. BMX_USDT), no symbol is passed, and all isolated margin assets are returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

/// Base currency details for isolated margin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginAssetBase {
    /// Currency
    pub currency: String,
    /// Whether open to borrow
    pub borrow_enabled: bool,
    /// Borrowed assets (precision: 8 decimal places)
    pub borrowed: String,
    /// Outstanding principal amount (precision: 8 decimal places)
    pub borrow_unpaid: String,
    /// Interest outstanding (precision: 8 decimal places)
    pub interest_unpaid: String,
    /// Available assets (precision: 8 decimal places)
    pub available: String,
    /// Trading frozen assets (precision: 8 decimal places)
    pub frozen: String,
    /// Net assets (precision: 8 decimal places)
    pub net_asset: String,
    /// Converted BTC net assets (precision: 8 decimal places)
    #[serde(rename = "netAssetBTC")]
    pub net_asset_btc: String,
    /// Total assets (precision: 8 decimal places)
    pub total_asset: String,
}

/// Quote currency details for isolated margin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginAssetQuote {
    /// Currency
    pub currency: String,
    /// Whether open to borrow
    pub borrow_enabled: bool,
    /// Borrowed assets (precision: 8 decimal places)
    pub borrowed: String,
    /// Outstanding principal amount (precision: 8 decimal places)
    pub borrow_unpaid: String,
    /// Interest outstanding (precision: 8 decimal places)
    pub interest_unpaid: String,
    /// Available assets (precision: 8 decimal places)
    pub available: String,
    /// Trading frozen assets (precision: 8 decimal places)
    pub frozen: String,
    /// Net assets (precision: 8 decimal places)
    pub net_asset: String,
    /// Converted BTC net assets (precision: 8 decimal places)
    #[serde(rename = "netAssetBTC")]
    pub net_asset_btc: String,
    /// Total assets (precision: 8 decimal places)
    pub total_asset: String,
}

/// Isolated margin symbol details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginIsolatedSymbol {
    /// Trading pair
    pub symbol: String,
    /// Current risk rate
    pub risk_rate: String,
    /// Risk level
    pub risk_level: String,
    /// Whether open to buy
    pub buy_enabled: bool,
    /// Whether open to sell
    pub sell_enabled: bool,
    /// Liquidation price (precision: 8 decimal places)
    pub liquidate_price: String,
    /// Liquidation rate
    pub liquidate_rate: String,
    /// Base currency details
    pub base: MarginAssetBase,
    /// Quote currency details
    pub quote: MarginAssetQuote,
}

/// Response for margin isolated account endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMarginIsolatedAccountResponse {
    /// Array of isolated margin symbols
    pub symbols: Vec<MarginIsolatedSymbol>,
}

impl GetMarginIsolatedAccountRequest {
    /// Create a new request for all isolated margin assets
    pub fn new() -> Self {
        Self { symbol: None }
    }

    /// Create a new request for a specific trading pair
    pub fn new_with_symbol(symbol: String) -> Self {
        Self {
            symbol: Some(symbol),
        }
    }
}

impl RestClient {
    /// Get margin account details (isolated)
    ///
    /// Applicable for isolated margin account inquiries
    ///
    /// See: https://raw.githubusercontent.com/rosssaunders/coincise/refs/heads/main/docs/bitmart/spot/funding_account.md
    ///
    /// Rate limit: 12 times/2 sec per API key
    ///
    /// # Arguments
    /// * `request` - The request parameters
    ///
    /// # Returns
    /// Margin isolated account information
    pub async fn get_margin_isolated_account(&self, request: GetMarginIsolatedAccountRequest) -> RestResult<GetMarginIsolatedAccountResponse> {
        self.send_request(
            "/spot/v1/margin/isolated/account",
            reqwest::Method::GET,
            Some(&request),
            EndpointType::MarginLoan,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_margin_isolated_account_request_new() {
        let request = GetMarginIsolatedAccountRequest::new();
        assert!(request.symbol.is_none());
    }

    #[test]
    fn test_get_margin_isolated_account_request_with_symbol() {
        let request = GetMarginIsolatedAccountRequest::new_with_symbol("BTC_USDT".to_string());
        assert_eq!(request.symbol, Some("BTC_USDT".to_string()));
    }

    #[test]
    fn test_get_margin_isolated_account_request_default() {
        let request = GetMarginIsolatedAccountRequest::default();
        assert!(request.symbol.is_none());
    }

    #[test]
    fn test_request_serialization() {
        let request = GetMarginIsolatedAccountRequest {
            symbol: Some("BTC_USDT".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("BTC_USDT"));
        assert!(serialized.contains("symbol"));
    }

    #[test]
    fn test_margin_asset_base_structure() {
        let base = MarginAssetBase {
            currency: "BTC".to_string(),
            borrow_enabled: false,
            borrowed: "2.00000000".to_string(),
            borrow_unpaid: "0.84478234".to_string(),
            interest_unpaid: "0.01385763".to_string(),
            available: "112.89603334".to_string(),
            frozen: "0.00000000".to_string(),
            net_asset: "110.89603334".to_string(),
            net_asset_btc: "0.00000000".to_string(),
            total_asset: "112.89603334".to_string(),
        };

        assert_eq!(base.currency, "BTC");
        assert!(!base.borrow_enabled);
        assert_eq!(base.borrowed, "2.00000000");
        assert_eq!(base.borrow_unpaid, "0.84478234");
        assert_eq!(base.interest_unpaid, "0.01385763");
        assert_eq!(base.available, "112.89603334");
        assert_eq!(base.frozen, "0.00000000");
        assert_eq!(base.net_asset, "110.89603334");
        assert_eq!(base.net_asset_btc, "0.00000000");
        assert_eq!(base.total_asset, "112.89603334");
    }

    #[test]
    fn test_margin_asset_quote_structure() {
        let quote = MarginAssetQuote {
            currency: "USDT".to_string(),
            borrow_enabled: true,
            borrowed: "0.00000000".to_string(),
            borrow_unpaid: "0.84478234".to_string(),
            interest_unpaid: "0.01385763".to_string(),
            available: "10.00000000".to_string(),
            frozen: "0.00000000".to_string(),
            net_asset: "10.00000000".to_string(),
            net_asset_btc: "0.00000000".to_string(),
            total_asset: "10.00000000".to_string(),
        };

        assert_eq!(quote.currency, "USDT");
        assert!(quote.borrow_enabled);
        assert_eq!(quote.borrowed, "0.00000000");
        assert_eq!(quote.available, "10.00000000");
    }

    #[test]
    fn test_margin_isolated_symbol_structure() {
        let symbol = MarginIsolatedSymbol {
            symbol: "BTC_USDT".to_string(),
            risk_rate: "18.77".to_string(),
            risk_level: "1".to_string(),
            buy_enabled: true,
            sell_enabled: true,
            liquidate_price: "-0.09408905".to_string(),
            liquidate_rate: "1.1".to_string(),
            base: MarginAssetBase {
                currency: "BTC".to_string(),
                borrow_enabled: false,
                borrowed: "2.00000000".to_string(),
                borrow_unpaid: "0.84478234".to_string(),
                interest_unpaid: "0.01385763".to_string(),
                available: "112.89603334".to_string(),
                frozen: "0.00000000".to_string(),
                net_asset: "110.89603334".to_string(),
                net_asset_btc: "0.00000000".to_string(),
                total_asset: "112.89603334".to_string(),
            },
            quote: MarginAssetQuote {
                currency: "USDT".to_string(),
                borrow_enabled: true,
                borrowed: "0.00000000".to_string(),
                borrow_unpaid: "0.84478234".to_string(),
                interest_unpaid: "0.01385763".to_string(),
                available: "10.00000000".to_string(),
                frozen: "0.00000000".to_string(),
                net_asset: "10.00000000".to_string(),
                net_asset_btc: "0.00000000".to_string(),
                total_asset: "10.00000000".to_string(),
            },
        };

        assert_eq!(symbol.symbol, "BTC_USDT");
        assert_eq!(symbol.risk_rate, "18.77");
        assert_eq!(symbol.risk_level, "1");
        assert!(symbol.buy_enabled);
        assert!(symbol.sell_enabled);
        assert_eq!(symbol.liquidate_price, "-0.09408905");
        assert_eq!(symbol.liquidate_rate, "1.1");
        assert_eq!(symbol.base.currency, "BTC");
        assert_eq!(symbol.quote.currency, "USDT");
    }

    #[test]
    fn test_symbol_serialization_roundtrip() {
        let symbol = MarginIsolatedSymbol {
            symbol: "BTC_USDT".to_string(),
            risk_rate: "18.77".to_string(),
            risk_level: "1".to_string(),
            buy_enabled: true,
            sell_enabled: true,
            liquidate_price: "-0.09408905".to_string(),
            liquidate_rate: "1.1".to_string(),
            base: MarginAssetBase {
                currency: "BTC".to_string(),
                borrow_enabled: false,
                borrowed: "2.00000000".to_string(),
                borrow_unpaid: "0.84478234".to_string(),
                interest_unpaid: "0.01385763".to_string(),
                available: "112.89603334".to_string(),
                frozen: "0.00000000".to_string(),
                net_asset: "110.89603334".to_string(),
                net_asset_btc: "0.00000000".to_string(),
                total_asset: "112.89603334".to_string(),
            },
            quote: MarginAssetQuote {
                currency: "USDT".to_string(),
                borrow_enabled: true,
                borrowed: "0.00000000".to_string(),
                borrow_unpaid: "0.84478234".to_string(),
                interest_unpaid: "0.01385763".to_string(),
                available: "10.00000000".to_string(),
                frozen: "0.00000000".to_string(),
                net_asset: "10.00000000".to_string(),
                net_asset_btc: "0.00000000".to_string(),
                total_asset: "10.00000000".to_string(),
            },
        };

        let serialized = serde_json::to_string(&symbol).unwrap();
        let deserialized: MarginIsolatedSymbol = serde_json::from_str(&serialized).unwrap();

        assert_eq!(symbol.symbol, deserialized.symbol);
        assert_eq!(symbol.risk_rate, deserialized.risk_rate);
        assert_eq!(symbol.base.currency, deserialized.base.currency);
        assert_eq!(symbol.quote.currency, deserialized.quote.currency);
    }

    #[test]
    fn test_get_margin_isolated_account_response_structure() {
        let response = GetMarginIsolatedAccountResponse {
            symbols: vec![MarginIsolatedSymbol {
                symbol: "BTC_USDT".to_string(),
                risk_rate: "18.77".to_string(),
                risk_level: "1".to_string(),
                buy_enabled: true,
                sell_enabled: true,
                liquidate_price: "-0.09408905".to_string(),
                liquidate_rate: "1.1".to_string(),
                base: MarginAssetBase {
                    currency: "BTC".to_string(),
                    borrow_enabled: false,
                    borrowed: "2.00000000".to_string(),
                    borrow_unpaid: "0.84478234".to_string(),
                    interest_unpaid: "0.01385763".to_string(),
                    available: "112.89603334".to_string(),
                    frozen: "0.00000000".to_string(),
                    net_asset: "110.89603334".to_string(),
                    net_asset_btc: "0.00000000".to_string(),
                    total_asset: "112.89603334".to_string(),
                },
                quote: MarginAssetQuote {
                    currency: "USDT".to_string(),
                    borrow_enabled: true,
                    borrowed: "0.00000000".to_string(),
                    borrow_unpaid: "0.84478234".to_string(),
                    interest_unpaid: "0.01385763".to_string(),
                    available: "10.00000000".to_string(),
                    frozen: "0.00000000".to_string(),
                    net_asset: "10.00000000".to_string(),
                    net_asset_btc: "0.00000000".to_string(),
                    total_asset: "10.00000000".to_string(),
                },
            }],
        };

        assert_eq!(response.symbols.len(), 1);
        assert_eq!(response.symbols[0].symbol, "BTC_USDT");
    }
}
