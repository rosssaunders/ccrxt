use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

/// Request to get margin account information
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMarginAccountRequest {
    /// Asset symbol (optional, if not provided returns all assets)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
}

/// Margin asset information
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginAsset {
    /// Asset symbol
    pub asset: String,
    /// Available balance for trading
    pub free: Decimal,
    /// Locked balance (in orders)
    pub locked: Decimal,
    /// Borrowed amount
    pub borrowed: Decimal,
    /// Interest accrued
    pub interest: Decimal,
    /// Net asset value (free + locked - borrowed - interest)
    pub net_asset: Decimal,
}

/// Margin account information
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginAccountInfo {
    /// Whether borrowing is enabled
    pub borrow_enabled: bool,
    /// Current margin level
    pub margin_level: Decimal,
    /// Total asset value in BTC
    pub total_asset_of_btc: Decimal,
    /// Total liability value in BTC
    pub total_liability_of_btc: Decimal,
    /// Total net asset value in BTC
    pub total_net_asset_of_btc: Decimal,
    /// Whether trading is enabled
    pub trade_enabled: bool,
    /// Whether transfers are enabled
    pub transfer_enabled: bool,
    /// List of margin assets
    pub user_assets: Vec<MarginAsset>,
}

/// Response for margin account information
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMarginAccountResponse {
    /// Success indicator
    pub success: bool,
    /// Margin account data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<MarginAccountInfo>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_get_margin_account_request_serialization() {
        let request = GetMarginAccountRequest {
            asset: Some("USDT".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"asset\":\"USDT\""));
    }

    #[test]
    fn test_empty_request() {
        let request = GetMarginAccountRequest {
            asset: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_get_margin_account_response_deserialization() {
        let json = r#"
        {
            "success": true,
            "data": {
                "borrowEnabled": true,
                "marginLevel": "2.5",
                "totalAssetOfBtc": "0.1",
                "totalLiabilityOfBtc": "0.04",
                "totalNetAssetOfBtc": "0.06",
                "tradeEnabled": true,
                "transferEnabled": true,
                "userAssets": [
                    {
                        "asset": "USDT",
                        "free": "1000.00",
                        "locked": "100.00",
                        "borrowed": "200.00",
                        "interest": "1.50",
                        "netAsset": "898.50"
                    },
                    {
                        "asset": "BTC",
                        "free": "0.01",
                        "locked": "0.001",
                        "borrowed": "0.005",
                        "interest": "0.0001",
                        "netAsset": "0.0049"
                    }
                ]
            }
        }
        "#;

        let response: GetMarginAccountResponse = serde_json::from_str(json).unwrap();
        assert!(response.success);
        
        let data = response.data.unwrap();
        assert!(data.borrow_enabled);
        assert_eq!(data.margin_level, dec!(2.5));
        assert_eq!(data.total_asset_of_btc, dec!(0.1));
        assert_eq!(data.total_liability_of_btc, dec!(0.04));
        assert_eq!(data.total_net_asset_of_btc, dec!(0.06));
        assert!(data.trade_enabled);
        assert!(data.transfer_enabled);
        assert_eq!(data.user_assets.len(), 2);

        let usdt_asset = &data.user_assets[0];
        assert_eq!(usdt_asset.asset, "USDT");
        assert_eq!(usdt_asset.free, dec!(1000.00));
        assert_eq!(usdt_asset.locked, dec!(100.00));
        assert_eq!(usdt_asset.borrowed, dec!(200.00));
        assert_eq!(usdt_asset.interest, dec!(1.50));
        assert_eq!(usdt_asset.net_asset, dec!(898.50));

        let btc_asset = &data.user_assets[1];
        assert_eq!(btc_asset.asset, "BTC");
        assert_eq!(btc_asset.free, dec!(0.01));
        assert_eq!(btc_asset.locked, dec!(0.001));
        assert_eq!(btc_asset.borrowed, dec!(0.005));
        assert_eq!(btc_asset.interest, dec!(0.0001));
        assert_eq!(btc_asset.net_asset, dec!(0.0049));
    }
}
