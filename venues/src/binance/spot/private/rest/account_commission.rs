use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::RestResult;

const GET_ACCOUNT_COMMISSION_ENDPOINT: &str = "/api/v3/account/commission";

/// Request parameters for getting account commission rates
#[derive(Debug, Clone, Serialize)]
pub struct AccountCommissionRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Account commission rates response
#[derive(Debug, Clone, Deserialize)]
pub struct AccountCommissionResponse {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Standard commission rates
    #[serde(rename = "standardCommission")]
    pub standard_commission: CommissionRates,

    /// Tax commission rates
    #[serde(rename = "taxCommission")]
    pub tax_commission: CommissionRates,

    /// Discount information
    #[serde(rename = "discount")]
    pub discount: Discount,
}

/// Commission rates structure
#[derive(Debug, Clone, Deserialize)]
pub struct CommissionRates {
    /// Maker commission rate
    #[serde(rename = "maker")]
    pub maker: Decimal,

    /// Taker commission rate
    #[serde(rename = "taker")]
    pub taker: Decimal,

    /// Buyer commission rate
    #[serde(rename = "buyer")]
    pub buyer: Decimal,

    /// Seller commission rate
    #[serde(rename = "seller")]
    pub seller: Decimal,
}

/// Discount information
#[derive(Debug, Clone, Deserialize)]
pub struct Discount {
    /// Enable buy back for account
    #[serde(rename = "enabledForAccount")]
    pub enabled_for_account: bool,

    /// Enable buy back for symbol
    #[serde(rename = "enabledForSymbol")]
    pub enabled_for_symbol: bool,

    /// Discount asset
    #[serde(rename = "discountAsset")]
    pub discount_asset: String,

    /// Discount rate
    #[serde(rename = "discount")]
    pub discount: Decimal,
}

impl RestClient {
    /// Get current account commission rates
    ///
    /// Get current account commission rates.
    ///
    /// [docs](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#account-commission-rates--user_data)
    /// Method: GET /api/v3/account/commission
    /// Weight: 20
    /// Security: USER_DATA
    pub async fn get_account_commission(
        &self,
        params: AccountCommissionRequest,
    ) -> RestResult<AccountCommissionResponse> {
        self.send_get_signed_request(GET_ACCOUNT_COMMISSION_ENDPOINT, params, 20, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_commission_request_serialization() {
        let request = AccountCommissionRequest {
            symbol: "BTCUSDT".to_string(),
            recv_window: Some(5000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_account_commission_request_minimal() {
        let request = AccountCommissionRequest {
            symbol: "ETHUSDT".to_string(),
            recv_window: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=ETHUSDT");
    }

    #[test]
    fn test_commission_rates_deserialization() {
        let json = r#"{
            "maker": "0.00100000",
            "taker": "0.00100000",
            "buyer": "0.00000000",
            "seller": "0.00000000"
        }"#;

        let rates: CommissionRates = serde_json::from_str(json).unwrap();
        assert_eq!(rates.maker.to_string(), "0.00100000");
        assert_eq!(rates.taker.to_string(), "0.00100000");
        assert_eq!(rates.buyer.to_string(), "0.00000000");
        assert_eq!(rates.seller.to_string(), "0.00000000");
    }

    #[test]
    fn test_discount_deserialization() {
        let json = r#"{
            "enabledForAccount": true,
            "enabledForSymbol": true,
            "discountAsset": "BNB",
            "discount": "0.75000000"
        }"#;

        let discount: Discount = serde_json::from_str(json).unwrap();
        assert!(discount.enabled_for_account);
        assert!(discount.enabled_for_symbol);
        assert_eq!(discount.discount_asset, "BNB");
        assert_eq!(discount.discount.to_string(), "0.75000000");
    }

    #[test]
    fn test_account_commission_response_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "standardCommission": {
                "maker": "0.00100000",
                "taker": "0.00100000",
                "buyer": "0.00000000",
                "seller": "0.00000000"
            },
            "taxCommission": {
                "maker": "0.00000000",
                "taker": "0.00000000",
                "buyer": "0.00000000",
                "seller": "0.00000000"
            },
            "discount": {
                "enabledForAccount": true,
                "enabledForSymbol": true,
                "discountAsset": "BNB",
                "discount": "0.75000000"
            }
        }"#;

        let response: AccountCommissionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.standard_commission.maker.to_string(), "0.00100000");
        assert_eq!(response.standard_commission.taker.to_string(), "0.00100000");
        assert_eq!(response.tax_commission.maker.to_string(), "0.00000000");
        assert!(response.discount.enabled_for_account);
        assert!(response.discount.enabled_for_symbol);
        assert_eq!(response.discount.discount_asset, "BNB");
        assert_eq!(response.discount.discount.to_string(), "0.75000000");
    }

    #[test]
    fn test_account_commission_response_no_discount() {
        let json = r#"{
            "symbol": "ETHUSDT",
            "standardCommission": {
                "maker": "0.00050000",
                "taker": "0.00050000",
                "buyer": "0.00000000",
                "seller": "0.00000000"
            },
            "taxCommission": {
                "maker": "0.00010000",
                "taker": "0.00010000",
                "buyer": "0.00000000",
                "seller": "0.00000000"
            },
            "discount": {
                "enabledForAccount": false,
                "enabledForSymbol": false,
                "discountAsset": "BNB",
                "discount": "0.00000000"
            }
        }"#;

        let response: AccountCommissionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "ETHUSDT");
        assert!(!response.discount.enabled_for_account);
        assert!(!response.discount.enabled_for_symbol);
        assert_eq!(response.discount.discount.to_string(), "0.00000000");
    }

    #[test]
    fn test_commission_rates_with_taxes() {
        let json = r#"{
            "maker": "0.00200000",
            "taker": "0.00300000",
            "buyer": "0.00100000",
            "seller": "0.00150000"
        }"#;

        let rates: CommissionRates = serde_json::from_str(json).unwrap();
        assert_eq!(rates.maker.to_string(), "0.00200000");
        assert_eq!(rates.taker.to_string(), "0.00300000");
        assert_eq!(rates.buyer.to_string(), "0.00100000");
        assert_eq!(rates.seller.to_string(), "0.00150000");
    }

    #[test]
    fn test_discount_partial_enabled() {
        let json = r#"{
            "enabledForAccount": true,
            "enabledForSymbol": false,
            "discountAsset": "BNB",
            "discount": "0.25000000"
        }"#;

        let discount: Discount = serde_json::from_str(json).unwrap();
        assert!(discount.enabled_for_account);
        assert!(!discount.enabled_for_symbol);
        assert_eq!(discount.discount.to_string(), "0.25000000");
    }

    #[test]
    fn test_account_commission_different_discount_asset() {
        let json = r#"{
            "symbol": "ADAUSDT",
            "standardCommission": {
                "maker": "0.00100000",
                "taker": "0.00100000",
                "buyer": "0.00000000",
                "seller": "0.00000000"
            },
            "taxCommission": {
                "maker": "0.00000000",
                "taker": "0.00000000",
                "buyer": "0.00000000",
                "seller": "0.00000000"
            },
            "discount": {
                "enabledForAccount": true,
                "enabledForSymbol": true,
                "discountAsset": "BUSD",
                "discount": "0.50000000"
            }
        }"#;

        let response: AccountCommissionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.discount.discount_asset, "BUSD");
        assert_eq!(response.discount.discount.to_string(), "0.50000000");
    }

    #[test]
    fn test_commission_rates_high_precision() {
        let json = r#"{
            "maker": "0.00012345",
            "taker": "0.00023456",
            "buyer": "0.00001234",
            "seller": "0.00002345"
        }"#;

        let rates: CommissionRates = serde_json::from_str(json).unwrap();
        assert_eq!(rates.maker.to_string(), "0.00012345");
        assert_eq!(rates.taker.to_string(), "0.00023456");
        assert_eq!(rates.buyer.to_string(), "0.00001234");
        assert_eq!(rates.seller.to_string(), "0.00002345");
    }
}
