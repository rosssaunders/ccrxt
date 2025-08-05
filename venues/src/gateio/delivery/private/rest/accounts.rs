use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::gateio::delivery::RestResult;

const DELIVERY_ACCOUNTS_ENDPOINT: &str = "/delivery/{}/accounts";

/// Request parameters for delivery accounts
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeliveryAccountsRequest {
    /// Settlement currency (BTC, USDT, etc.)
    pub settle: String,
}

/// Delivery account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryAccount {
    /// Total balance
    pub total: String,

    /// Unrealized PnL
    pub unrealised_pnl: String,

    /// Position margin
    pub position_margin: String,

    /// Order margin
    pub order_margin: String,

    /// Available balance
    pub available: String,

    /// Point balance
    pub point: String,

    /// Currency
    pub currency: String,

    /// Enable credit
    pub enable_credit: bool,

    /// Positions cross margin
    pub position_cross_margin: String,

    /// Orders cross margin
    pub order_cross_margin: String,

    /// Available cross margin
    pub available_cross_margin: String,

    /// Total cross margin
    pub total_cross_margin: String,
}

impl RestClient {
    /// Get delivery account information
    ///
    /// This endpoint returns delivery account balances and margin information.
    ///
    /// See: <https://www.gate.com/docs/developers/apiv4/#get-futures-account-2>
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `params` - The delivery accounts request parameters
    ///
    /// # Returns
    /// Delivery account information including balances and margins
    pub async fn get_delivery_accounts(
        &self,
        params: DeliveryAccountsRequest,
    ) -> RestResult<DeliveryAccount> {
        let endpoint = DELIVERY_ACCOUNTS_ENDPOINT.replace("{}", &params.settle);
        self.get(&endpoint).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delivery_accounts_endpoint() {
        assert_eq!(DELIVERY_ACCOUNTS_ENDPOINT, "/delivery/{}/accounts");
    }

    #[test]
    fn test_delivery_accounts_request_btc() {
        let request = DeliveryAccountsRequest {
            settle: "BTC".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "BTC");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 1);
    }

    #[test]
    fn test_delivery_accounts_request_usdt() {
        let request = DeliveryAccountsRequest {
            settle: "USDT".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
    }

    #[test]
    fn test_delivery_accounts_request_default() {
        let request = DeliveryAccountsRequest::default();
        assert_eq!(request.settle, "");
    }

    #[test]
    fn test_delivery_account_deserialization() {
        let json = r#"{
            "total": "1000.5",
            "unrealised_pnl": "10.25",
            "position_margin": "100.0",
            "order_margin": "50.0",
            "available": "840.25",
            "point": "0",
            "currency": "BTC",
            "enable_credit": true,
            "position_cross_margin": "75.0",
            "order_cross_margin": "25.0",
            "available_cross_margin": "900.5",
            "total_cross_margin": "1000.5"
        }"#;

        let account: DeliveryAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.total, "1000.5");
        assert_eq!(account.unrealised_pnl, "10.25");
        assert_eq!(account.position_margin, "100.0");
        assert_eq!(account.order_margin, "50.0");
        assert_eq!(account.available, "840.25");
        assert_eq!(account.point, "0");
        assert_eq!(account.currency, "BTC");
        assert_eq!(account.enable_credit, true);
        assert_eq!(account.position_cross_margin, "75.0");
        assert_eq!(account.order_cross_margin, "25.0");
        assert_eq!(account.available_cross_margin, "900.5");
        assert_eq!(account.total_cross_margin, "1000.5");
    }

    #[test]
    fn test_delivery_account_round_trip() {
        let original = DeliveryAccount {
            total: "2500.75".to_string(),
            unrealised_pnl: "-25.5".to_string(),
            position_margin: "200.0".to_string(),
            order_margin: "100.25".to_string(),
            available: "2200.0".to_string(),
            point: "5.0".to_string(),
            currency: "USDT".to_string(),
            enable_credit: false,
            position_cross_margin: "150.0".to_string(),
            order_cross_margin: "50.25".to_string(),
            available_cross_margin: "2300.5".to_string(),
            total_cross_margin: "2500.75".to_string(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: DeliveryAccount = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.total, original.total);
        assert_eq!(deserialized.unrealised_pnl, original.unrealised_pnl);
        assert_eq!(deserialized.position_margin, original.position_margin);
        assert_eq!(deserialized.order_margin, original.order_margin);
        assert_eq!(deserialized.available, original.available);
        assert_eq!(deserialized.point, original.point);
        assert_eq!(deserialized.currency, original.currency);
        assert_eq!(deserialized.enable_credit, original.enable_credit);
        assert_eq!(deserialized.position_cross_margin, original.position_cross_margin);
        assert_eq!(deserialized.order_cross_margin, original.order_cross_margin);
        assert_eq!(deserialized.available_cross_margin, original.available_cross_margin);
        assert_eq!(deserialized.total_cross_margin, original.total_cross_margin);
    }

    #[test]
    fn test_delivery_account_different_currencies() {
        let btc_account = DeliveryAccount {
            total: "10.5".to_string(),
            unrealised_pnl: "0.1".to_string(),
            position_margin: "1.0".to_string(),
            order_margin: "0.5".to_string(),
            available: "8.9".to_string(),
            point: "0".to_string(),
            currency: "BTC".to_string(),
            enable_credit: true,
            position_cross_margin: "0.8".to_string(),
            order_cross_margin: "0.2".to_string(),
            available_cross_margin: "9.5".to_string(),
            total_cross_margin: "10.5".to_string(),
        };

        let usdt_account = DeliveryAccount {
            total: "50000.0".to_string(),
            unrealised_pnl: "100.0".to_string(),
            position_margin: "5000.0".to_string(),
            order_margin: "2500.0".to_string(),
            available: "42400.0".to_string(),
            point: "100.0".to_string(),
            currency: "USDT".to_string(),
            enable_credit: false,
            position_cross_margin: "4000.0".to_string(),
            order_cross_margin: "1000.0".to_string(),
            available_cross_margin: "45000.0".to_string(),
            total_cross_margin: "50000.0".to_string(),
        };

        assert_eq!(btc_account.currency, "BTC");
        assert_eq!(usdt_account.currency, "USDT");
        assert_ne!(btc_account.enable_credit, usdt_account.enable_credit);
    }

    #[test]
    fn test_delivery_account_margin_calculations() {
        let account = DeliveryAccount {
            total: "1000.0".to_string(),
            unrealised_pnl: "50.0".to_string(),
            position_margin: "100.0".to_string(),
            order_margin: "50.0".to_string(),
            available: "900.0".to_string(),
            point: "0".to_string(),
            currency: "USDT".to_string(),
            enable_credit: true,
            position_cross_margin: "80.0".to_string(),
            order_cross_margin: "20.0".to_string(),
            available_cross_margin: "950.0".to_string(),
            total_cross_margin: "1050.0".to_string(),
        };

        // Test that values are consistently formatted as strings
        assert!(account.total.parse::<f64>().is_ok());
        assert!(account.unrealised_pnl.parse::<f64>().is_ok());
        assert!(account.position_margin.parse::<f64>().is_ok());
        assert!(account.order_margin.parse::<f64>().is_ok());
        assert!(account.available.parse::<f64>().is_ok());
    }
}
