use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

/// Request to get batch sub-account assets overview
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchSubAccountAssetsRequest {
    /// Sub-account UIDs (comma-separated, optional - if not provided, returns all)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_uid_list: Option<String>,
    /// Page number (default: 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    /// Page size (default: 10, max: 200)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

/// Sub-account asset summary
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountAssetSummary {
    /// Sub-account UID
    pub sub_uid: String,
    /// Sub-account email
    pub email: String,
    /// Total balance in USDT
    pub total_balance_usdt: Decimal,
    /// Total spot balance in USDT
    pub spot_balance_usdt: Decimal,
    /// Total futures balance in USDT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub futures_balance_usdt: Option<Decimal>,
    /// Total margin balance in USDT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_balance_usdt: Option<Decimal>,
    /// Account status
    pub status: String,
    /// Last update time
    pub update_time: i64,
}

/// Response for batch sub-account assets
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchSubAccountAssetsResponse {
    /// Success indicator
    pub success: bool,
    /// Sub-account asset summaries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<BatchSubAccountAssetsData>,
}

/// Batch sub-account assets data
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchSubAccountAssetsData {
    /// List of sub-account asset summaries
    pub sub_accounts: Vec<SubAccountAssetSummary>,
    /// Total count of sub-accounts
    pub total_count: i32,
    /// Current page
    pub page: i32,
    /// Page size
    pub size: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_batch_sub_account_assets_request_serialization() {
        let request = BatchSubAccountAssetsRequest {
            sub_uid_list: Some("12345,67890".to_string()),
            page: Some(1),
            size: Some(20),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subUidList\":\"12345,67890\""));
        assert!(json.contains("\"page\":1"));
        assert!(json.contains("\"size\":20"));
    }

    #[test]
    fn test_empty_request() {
        let request = BatchSubAccountAssetsRequest {
            sub_uid_list: None,
            page: None,
            size: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_batch_sub_account_assets_response_deserialization() {
        let json = r#"
        {
            "success": true,
            "data": {
                "subAccounts": [
                    {
                        "subUid": "12345",
                        "email": "sub1@example.com",
                        "totalBalanceUsdt": "1000.50",
                        "spotBalanceUsdt": "800.30",
                        "futuresBalanceUsdt": "200.20",
                        "marginBalanceUsdt": "0.00",
                        "status": "NORMAL",
                        "updateTime": 1640995200000
                    },
                    {
                        "subUid": "67890",
                        "email": "sub2@example.com",
                        "totalBalanceUsdt": "500.25",
                        "spotBalanceUsdt": "500.25",
                        "status": "NORMAL",
                        "updateTime": 1640995200000
                    }
                ],
                "totalCount": 2,
                "page": 1,
                "size": 10
            }
        }
        "#;

        let response: BatchSubAccountAssetsResponse = serde_json::from_str(json).unwrap();
        assert!(response.success);
        
        let data = response.data.unwrap();
        assert_eq!(data.sub_accounts.len(), 2);
        assert_eq!(data.total_count, 2);
        assert_eq!(data.page, 1);
        assert_eq!(data.size, 10);

        let first_account = &data.sub_accounts[0];
        assert_eq!(first_account.sub_uid, "12345");
        assert_eq!(first_account.email, "sub1@example.com");
        assert_eq!(first_account.total_balance_usdt, dec!(1000.50));
        assert_eq!(first_account.spot_balance_usdt, dec!(800.30));
        assert_eq!(first_account.futures_balance_usdt, Some(dec!(200.20)));
        assert_eq!(first_account.margin_balance_usdt, Some(dec!(0.00)));
        assert_eq!(first_account.status, "NORMAL");

        let second_account = &data.sub_accounts[1];
        assert_eq!(second_account.sub_uid, "67890");
        assert!(second_account.futures_balance_usdt.is_none());
        assert!(second_account.margin_balance_usdt.is_none());
    }
}
