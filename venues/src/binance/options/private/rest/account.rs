use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::options::{OptionsRiskLevel, RestResult};

const GET_ACCOUNT_ENDPOINT: &str = "/eapi/v1/account";

/// Request parameters for the account information endpoint
#[derive(Debug, Clone, Serialize, Default)]
pub struct AccountRequest {
    /// Request timeout window in milliseconds (max 60000)
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Account asset information
#[derive(Debug, Clone, Deserialize)]
pub struct AccountAsset {
    /// Asset symbol (e.g., "USDT")
    #[serde(rename = "asset")]
    pub asset: String,

    /// Account balance
    #[serde(rename = "marginBalance")]
    pub margin_balance: Decimal,

    /// Account equity
    #[serde(rename = "equity")]
    pub equity: Decimal,

    /// Available funds
    #[serde(rename = "available")]
    pub available: Decimal,

    /// Locked balance for orders and positions
    #[serde(rename = "locked")]
    pub locked: Decimal,

    /// Unrealized profit/loss
    #[serde(rename = "unrealizedPNL")]
    pub unrealized_pnl: Decimal,
}

/// Account Greeks information for an underlying
#[derive(Debug, Clone, Deserialize)]
pub struct AccountGreeks {
    /// Option underlying (e.g., "BTCUSDT")
    #[serde(rename = "underlying")]
    pub underlying: String,

    /// Account delta
    #[serde(rename = "delta")]
    pub delta: Decimal,

    /// Account gamma
    #[serde(rename = "gamma")]
    pub gamma: Decimal,

    /// Account theta
    #[serde(rename = "theta")]
    pub theta: Decimal,

    /// Account vega
    #[serde(rename = "vega")]
    pub vega: Decimal,
}

/// Account information response
#[derive(Debug, Clone, Deserialize)]
pub struct AccountResponse {
    /// Asset information array
    #[serde(rename = "asset")]
    pub assets: Vec<AccountAsset>,

    /// Greeks information array
    #[serde(rename = "greek")]
    pub greeks: Vec<AccountGreeks>,

    /// Response timestamp
    #[serde(rename = "time")]
    pub time: u64,

    /// Account risk level
    #[serde(rename = "riskLevel")]
    pub risk_level: OptionsRiskLevel,
}

impl RestClient {
    /// Get current account information
    ///
    /// Returns account balance, equity, available funds, Greeks, and risk level.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/account)
    /// Method: GET /eapi/v1/account
    /// Weight: 3
    /// Requires: API key and signature
    pub async fn get_account_info(&self, params: AccountRequest) -> RestResult<AccountResponse> {
        self.send_signed_request(
            GET_ACCOUNT_ENDPOINT,
            reqwest::Method::GET,
            params,
            3,
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_account_request_minimal_serialization() {
        // Test minimal request with only required timestamp
        let request = AccountRequest {
            recv_window: None,
            timestamp: 1640995200000,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["timestamp"], 1640995200000u64);
        
        // Ensure optional fields are not serialized when None
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_account_request_with_recv_window_serialization() {
        // Test request with recv_window
        let request = AccountRequest {
            recv_window: Some(60000),
            timestamp: 1640995200000,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["timestamp"], 1640995200000u64);
        assert_eq!(json["recvWindow"], 60000);
    }

    #[test]
    fn test_account_request_default() {
        // Test default implementation creates a valid request
        let request = AccountRequest {
            timestamp: 1640995200000,
            ..Default::default()
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["timestamp"], 1640995200000u64);
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_account_request_edge_cases() {
        // Test edge cases for numeric values
        let request = AccountRequest {
            recv_window: Some(1), // Minimum value
            timestamp: 0,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["recvWindow"], 1);
        assert_eq!(json["timestamp"], 0);

        // Test maximum recv_window
        let request = AccountRequest {
            recv_window: Some(60000), // Maximum value
            timestamp: u64::MAX,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["recvWindow"], 60000);
        assert_eq!(json["timestamp"], u64::MAX);
    }

    #[test]
    fn test_account_asset_deserialization() {
        let json = r#"{
            "asset": "USDT",
            "marginBalance": "1000.50000000",
            "equity": "1050.75000000",
            "available": "500.25000000",
            "locked": "450.25000000",
            "unrealizedPNL": "50.25000000"
        }"#;

        let asset: AccountAsset = serde_json::from_str(json).unwrap();
        assert_eq!(asset.asset, "USDT");
        assert_eq!(asset.margin_balance, dec!(1000.50000000));
        assert_eq!(asset.equity, dec!(1050.75000000));
        assert_eq!(asset.available, dec!(500.25000000));
        assert_eq!(asset.locked, dec!(450.25000000));
        assert_eq!(asset.unrealized_pnl, dec!(50.25000000));
    }

    #[test]
    fn test_account_asset_negative_values_deserialization() {
        // Test negative values which are valid for PnL
        let json = r#"{
            "asset": "BTC",
            "marginBalance": "0.01000000",
            "equity": "0.00950000",
            "available": "0.00500000",
            "locked": "0.00450000",
            "unrealizedPNL": "-0.00050000"
        }"#;

        let asset: AccountAsset = serde_json::from_str(json).unwrap();
        assert_eq!(asset.asset, "BTC");
        assert_eq!(asset.margin_balance, dec!(0.01000000));
        assert_eq!(asset.equity, dec!(0.00950000));
        assert_eq!(asset.available, dec!(0.00500000));
        assert_eq!(asset.locked, dec!(0.00450000));
        assert_eq!(asset.unrealized_pnl, dec!(-0.00050000));
    }

    #[test]
    fn test_account_asset_zero_values_deserialization() {
        // Test zero values
        let json = r#"{
            "asset": "ETH",
            "marginBalance": "0.00000000",
            "equity": "0.00000000",
            "available": "0.00000000",
            "locked": "0.00000000",
            "unrealizedPNL": "0.00000000"
        }"#;

        let asset: AccountAsset = serde_json::from_str(json).unwrap();
        assert_eq!(asset.asset, "ETH");
        assert_eq!(asset.margin_balance, dec!(0.00000000));
        assert_eq!(asset.equity, dec!(0.00000000));
        assert_eq!(asset.available, dec!(0.00000000));
        assert_eq!(asset.locked, dec!(0.00000000));
        assert_eq!(asset.unrealized_pnl, dec!(0.00000000));
    }

    #[test]
    fn test_account_asset_high_precision_deserialization() {
        // Test high precision decimal values
        let json = r#"{
            "asset": "USDT",
            "marginBalance": "12345.67890123",
            "equity": "98765.43210987",
            "available": "11111.11111111",
            "locked": "22222.22222222",
            "unrealizedPNL": "33333.33333333"
        }"#;

        let asset: AccountAsset = serde_json::from_str(json).unwrap();
        assert_eq!(asset.asset, "USDT");
        assert_eq!(asset.margin_balance, dec!(12345.67890123));
        assert_eq!(asset.equity, dec!(98765.43210987));
        assert_eq!(asset.available, dec!(11111.11111111));
        assert_eq!(asset.locked, dec!(22222.22222222));
        assert_eq!(asset.unrealized_pnl, dec!(33333.33333333));
    }

    #[test]
    fn test_account_greeks_deserialization() {
        let json = r#"{
            "underlying": "BTCUSDT",
            "delta": "0.12345678",
            "gamma": "0.87654321",
            "theta": "-0.00012345",
            "vega": "0.00098765"
        }"#;

        let greeks: AccountGreeks = serde_json::from_str(json).unwrap();
        assert_eq!(greeks.underlying, "BTCUSDT");
        assert_eq!(greeks.delta, dec!(0.12345678));
        assert_eq!(greeks.gamma, dec!(0.87654321));
        assert_eq!(greeks.theta, dec!(-0.00012345));
        assert_eq!(greeks.vega, dec!(0.00098765));
    }

    #[test]
    fn test_account_greeks_negative_values_deserialization() {
        // Test negative Greeks values which are common
        let json = r#"{
            "underlying": "ETHUSDT",
            "delta": "-0.75000000",
            "gamma": "0.00500000",
            "theta": "-0.05000000",
            "vega": "0.10000000"
        }"#;

        let greeks: AccountGreeks = serde_json::from_str(json).unwrap();
        assert_eq!(greeks.underlying, "ETHUSDT");
        assert_eq!(greeks.delta, dec!(-0.75000000));
        assert_eq!(greeks.gamma, dec!(0.00500000));
        assert_eq!(greeks.theta, dec!(-0.05000000));
        assert_eq!(greeks.vega, dec!(0.10000000));
    }

    #[test]
    fn test_account_greeks_zero_values_deserialization() {
        // Test zero Greeks values
        let json = r#"{
            "underlying": "ADAUSDT",
            "delta": "0.00000000",
            "gamma": "0.00000000",
            "theta": "0.00000000",
            "vega": "0.00000000"
        }"#;

        let greeks: AccountGreeks = serde_json::from_str(json).unwrap();
        assert_eq!(greeks.underlying, "ADAUSDT");
        assert_eq!(greeks.delta, dec!(0.00000000));
        assert_eq!(greeks.gamma, dec!(0.00000000));
        assert_eq!(greeks.theta, dec!(0.00000000));
        assert_eq!(greeks.vega, dec!(0.00000000));
    }

    #[test]
    fn test_account_greeks_extreme_values_deserialization() {
        // Test extreme values for Greeks
        let json = r#"{
            "underlying": "BTCUSDT",
            "delta": "1.00000000",
            "gamma": "999.99999999",
            "theta": "-999.99999999",
            "vega": "999.99999999"
        }"#;

        let greeks: AccountGreeks = serde_json::from_str(json).unwrap();
        assert_eq!(greeks.underlying, "BTCUSDT");
        assert_eq!(greeks.delta, dec!(1.00000000));
        assert_eq!(greeks.gamma, dec!(999.99999999));
        assert_eq!(greeks.theta, dec!(-999.99999999));
        assert_eq!(greeks.vega, dec!(999.99999999));
    }

    #[test]
    fn test_account_response_single_asset_deserialization() {
        let json = r#"{
            "asset": [
                {
                    "asset": "USDT",
                    "marginBalance": "1000.00000000",
                    "equity": "1000.00000000",
                    "available": "500.00000000",
                    "locked": "500.00000000",
                    "unrealizedPNL": "0.00000000"
                }
            ],
            "greek": [
                {
                    "underlying": "BTCUSDT",
                    "delta": "0.50000000",
                    "gamma": "0.01000000",
                    "theta": "-0.00100000",
                    "vega": "0.05000000"
                }
            ],
            "time": 1640995200000,
            "riskLevel": "NORMAL"
        }"#;

        let response: AccountResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.assets.len(), 1);
        assert_eq!(response.greeks.len(), 1);
        assert_eq!(response.time, 1640995200000);
        assert_eq!(response.risk_level, OptionsRiskLevel::Normal);

        // Verify asset details
        let asset = &response.assets[0];
        assert_eq!(asset.asset, "USDT");
        assert_eq!(asset.margin_balance, dec!(1000.00000000));
        assert_eq!(asset.equity, dec!(1000.00000000));
        assert_eq!(asset.available, dec!(500.00000000));
        assert_eq!(asset.locked, dec!(500.00000000));
        assert_eq!(asset.unrealized_pnl, dec!(0.00000000));

        // Verify Greeks details
        let greeks = &response.greeks[0];
        assert_eq!(greeks.underlying, "BTCUSDT");
        assert_eq!(greeks.delta, dec!(0.50000000));
        assert_eq!(greeks.gamma, dec!(0.01000000));
        assert_eq!(greeks.theta, dec!(-0.00100000));
        assert_eq!(greeks.vega, dec!(0.05000000));
    }

    #[test]
    fn test_account_response_multiple_assets_deserialization() {
        let json = r#"{
            "asset": [
                {
                    "asset": "USDT",
                    "marginBalance": "1000.00000000",
                    "equity": "1000.00000000",
                    "available": "500.00000000",
                    "locked": "500.00000000",
                    "unrealizedPNL": "0.00000000"
                },
                {
                    "asset": "BTC",
                    "marginBalance": "0.02000000",
                    "equity": "0.01950000",
                    "available": "0.01000000",
                    "locked": "0.00950000",
                    "unrealizedPNL": "-0.00050000"
                }
            ],
            "greek": [
                {
                    "underlying": "BTCUSDT",
                    "delta": "0.50000000",
                    "gamma": "0.01000000",
                    "theta": "-0.00100000",
                    "vega": "0.05000000"
                },
                {
                    "underlying": "ETHUSDT",
                    "delta": "-0.25000000",
                    "gamma": "0.00500000",
                    "theta": "-0.00050000",
                    "vega": "0.02500000"
                }
            ],
            "time": 1640995200000,
            "riskLevel": "MEDIUM"
        }"#;

        let response: AccountResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.assets.len(), 2);
        assert_eq!(response.greeks.len(), 2);
        assert_eq!(response.time, 1640995200000);
        assert_eq!(response.risk_level, OptionsRiskLevel::Medium);

        // Verify first asset
        let asset1 = &response.assets[0];
        assert_eq!(asset1.asset, "USDT");
        assert_eq!(asset1.margin_balance, dec!(1000.00000000));

        // Verify second asset
        let asset2 = &response.assets[1];
        assert_eq!(asset2.asset, "BTC");
        assert_eq!(asset2.margin_balance, dec!(0.02000000));
        assert_eq!(asset2.unrealized_pnl, dec!(-0.00050000));

        // Verify first Greeks
        let greeks1 = &response.greeks[0];
        assert_eq!(greeks1.underlying, "BTCUSDT");
        assert_eq!(greeks1.delta, dec!(0.50000000));

        // Verify second Greeks
        let greeks2 = &response.greeks[1];
        assert_eq!(greeks2.underlying, "ETHUSDT");
        assert_eq!(greeks2.delta, dec!(-0.25000000));
    }

    #[test]
    fn test_account_response_empty_arrays_deserialization() {
        // Test response with empty asset and Greeks arrays
        let json = r#"{
            "asset": [],
            "greek": [],
            "time": 1640995200000,
            "riskLevel": "HIGH"
        }"#;

        let response: AccountResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.assets.len(), 0);
        assert_eq!(response.greeks.len(), 0);
        assert_eq!(response.time, 1640995200000);
        assert_eq!(response.risk_level, OptionsRiskLevel::High);
    }

    #[test]
    fn test_account_response_all_risk_levels_deserialization() {
        // Test all risk level variants
        let risk_levels = vec![
            ("NORMAL", OptionsRiskLevel::Normal),
            ("MEDIUM", OptionsRiskLevel::Medium),
            ("HIGH", OptionsRiskLevel::High),
        ];

        for (risk_str, expected_risk) in risk_levels {
            let json = format!(r#"{{
                "asset": [],
                "greek": [],
                "time": 1640995200000,
                "riskLevel": "{}"
            }}"#, risk_str);

            let response: AccountResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(response.risk_level, expected_risk);
        }
    }

    #[test]
    fn test_account_response_large_time_values() {
        // Test with large timestamp values
        let json = r#"{
            "asset": [],
            "greek": [],
            "time": 9223372036854775807,
            "riskLevel": "NORMAL"
        }"#;

        let response: AccountResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.time, 9223372036854775807u64);
    }

    #[test]
    fn test_account_response_mixed_scenarios() {
        // Test realistic mixed scenario with positive and negative values
        let json = r#"{
            "asset": [
                {
                    "asset": "USDT",
                    "marginBalance": "10000.00000000",
                    "equity": "10250.50000000",
                    "available": "5000.25000000",
                    "locked": "5000.25000000",
                    "unrealizedPNL": "250.50000000"
                },
                {
                    "asset": "BTC",
                    "marginBalance": "0.50000000",
                    "equity": "0.48500000",
                    "available": "0.25000000",
                    "locked": "0.23500000",
                    "unrealizedPNL": "-0.01500000"
                }
            ],
            "greek": [
                {
                    "underlying": "BTCUSDT",
                    "delta": "0.65000000",
                    "gamma": "0.00800000",
                    "theta": "-0.00200000",
                    "vega": "0.08000000"
                },
                {
                    "underlying": "ETHUSDT",
                    "delta": "-0.35000000",
                    "gamma": "0.01200000",
                    "theta": "-0.00150000",
                    "vega": "0.06500000"
                }
            ],
            "time": 1640995200000,
            "riskLevel": "MEDIUM"
        }"#;

        let response: AccountResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.assets.len(), 2);
        assert_eq!(response.greeks.len(), 2);
        assert_eq!(response.risk_level, OptionsRiskLevel::Medium);

        // Verify profit scenario
        let usdt_asset = &response.assets[0];
        assert_eq!(usdt_asset.asset, "USDT");
        assert_eq!(usdt_asset.unrealized_pnl, dec!(250.50000000));

        // Verify loss scenario
        let btc_asset = &response.assets[1];
        assert_eq!(btc_asset.asset, "BTC");
        assert_eq!(btc_asset.unrealized_pnl, dec!(-0.01500000));

        // Verify mixed Greeks
        let btc_greeks = &response.greeks[0];
        assert_eq!(btc_greeks.delta, dec!(0.65000000));
        assert_eq!(btc_greeks.theta, dec!(-0.00200000));

        let eth_greeks = &response.greeks[1];
        assert_eq!(eth_greeks.delta, dec!(-0.35000000));
        assert_eq!(eth_greeks.vega, dec!(0.06500000));
    }

    #[test]
    fn test_account_response_edge_case_precision() {
        // Test edge case with very small and very large precision values
        let json = r#"{
            "asset": [
                {
                    "asset": "USDT",
                    "marginBalance": "0.00000001",
                    "equity": "999999999.99999999",
                    "available": "0.00000001",
                    "locked": "999999999.99999998",
                    "unrealizedPNL": "-0.00000001"
                }
            ],
            "greek": [
                {
                    "underlying": "BTCUSDT",
                    "delta": "0.00000001",
                    "gamma": "999999999.99999999",
                    "theta": "-999999999.99999999",
                    "vega": "0.00000001"
                }
            ],
            "time": 1640995200000,
            "riskLevel": "HIGH"
        }"#;

        let response: AccountResponse = serde_json::from_str(json).unwrap();
        let asset = &response.assets[0];
        let greeks = &response.greeks[0];

        assert_eq!(asset.margin_balance, dec!(0.00000001));
        assert_eq!(asset.equity, dec!(999999999.99999999));
        assert_eq!(asset.unrealized_pnl, dec!(-0.00000001));

        assert_eq!(greeks.delta, dec!(0.00000001));
        assert_eq!(greeks.gamma, dec!(999999999.99999999));
        assert_eq!(greeks.theta, dec!(-999999999.99999999));
        assert_eq!(greeks.vega, dec!(0.00000001));
    }

    #[test]
    fn test_account_response_different_underlying_assets() {
        // Test with various underlying assets
        let json = r#"{
            "asset": [
                {
                    "asset": "USDT",
                    "marginBalance": "1000.00000000",
                    "equity": "1000.00000000",
                    "available": "500.00000000",
                    "locked": "500.00000000",
                    "unrealizedPNL": "0.00000000"
                }
            ],
            "greek": [
                {
                    "underlying": "BTCUSDT",
                    "delta": "0.50000000",
                    "gamma": "0.01000000",
                    "theta": "-0.00100000",
                    "vega": "0.05000000"
                },
                {
                    "underlying": "ETHUSDT",
                    "delta": "-0.25000000",
                    "gamma": "0.00500000",
                    "theta": "-0.00050000",
                    "vega": "0.02500000"
                },
                {
                    "underlying": "ADAUSDT",
                    "delta": "0.10000000",
                    "gamma": "0.00100000",
                    "theta": "-0.00010000",
                    "vega": "0.01000000"
                }
            ],
            "time": 1640995200000,
            "riskLevel": "NORMAL"
        }"#;

        let response: AccountResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.greeks.len(), 3);

        let underlying_assets: Vec<&str> = response.greeks.iter().map(|g| g.underlying.as_str()).collect();
        assert_eq!(underlying_assets, vec!["BTCUSDT", "ETHUSDT", "ADAUSDT"]);
    }

    #[test]
    fn test_account_response_real_world_scenario() {
        // Test a realistic account response scenario
        let json = r#"{
            "asset": [
                {
                    "asset": "USDT",
                    "marginBalance": "50000.00000000",
                    "equity": "52500.75000000",
                    "available": "25000.00000000",
                    "locked": "25000.00000000",
                    "unrealizedPNL": "2500.75000000"
                },
                {
                    "asset": "BTC",
                    "marginBalance": "1.00000000",
                    "equity": "0.98750000",
                    "available": "0.50000000",
                    "locked": "0.48750000",
                    "unrealizedPNL": "-0.01250000"
                }
            ],
            "greek": [
                {
                    "underlying": "BTCUSDT",
                    "delta": "0.65432100",
                    "gamma": "0.00123456",
                    "theta": "-0.00045678",
                    "vega": "0.09876543"
                },
                {
                    "underlying": "ETHUSDT",
                    "delta": "-0.34567890",
                    "gamma": "0.00098765",
                    "theta": "-0.00012345",
                    "vega": "0.05432109"
                }
            ],
            "time": 1640995200000,
            "riskLevel": "MEDIUM"
        }"#;

        let response: AccountResponse = serde_json::from_str(json).unwrap();
        
        // Verify overall structure
        assert_eq!(response.assets.len(), 2);
        assert_eq!(response.greeks.len(), 2);
        assert_eq!(response.time, 1640995200000);
        assert_eq!(response.risk_level, OptionsRiskLevel::Medium);

        // Verify profitability calculations are consistent
        let usdt_asset = &response.assets[0];
        assert!(usdt_asset.unrealized_pnl > dec!(0)); // Positive PnL
        assert!(usdt_asset.equity > usdt_asset.margin_balance); // Equity > margin when profitable

        let btc_asset = &response.assets[1];
        assert!(btc_asset.unrealized_pnl < dec!(0)); // Negative PnL
        assert!(btc_asset.equity < btc_asset.margin_balance); // Equity < margin when losing

        // Verify Greeks have realistic values
        let btc_greeks = &response.greeks[0];
        assert!(btc_greeks.delta > dec!(0) && btc_greeks.delta < dec!(1)); // Delta between 0 and 1
        assert!(btc_greeks.gamma > dec!(0)); // Gamma is positive
        assert!(btc_greeks.theta < dec!(0)); // Theta is negative (time decay)
        assert!(btc_greeks.vega > dec!(0)); // Vega is positive

        let eth_greeks = &response.greeks[1];
        assert!(eth_greeks.delta < dec!(0) && eth_greeks.delta > dec!(-1)); // Delta between -1 and 0
        assert!(eth_greeks.gamma > dec!(0)); // Gamma is positive
        assert!(eth_greeks.theta < dec!(0)); // Theta is negative (time decay)
        assert!(eth_greeks.vega > dec!(0)); // Vega is positive
    }
}
