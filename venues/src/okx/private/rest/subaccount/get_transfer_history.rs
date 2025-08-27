use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::okx::{EndpointType, RestResult};

/// Endpoint URL for getting history of sub-account transfer
const GET_TRANSFER_HISTORY_ENDPOINT: &str = "api/v5/asset/subaccount/bills";

/// Request to get history of sub-account transfer
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransferHistoryRequest {
    /// Currency, such as BTC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,

    /// Transfer type
    /// 0: Transfers from master account to sub-account
    /// 1: Transfers from sub-account to master account
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub transfer_type: Option<String>,

    /// Sub-account name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_acct: Option<String>,

    /// Query the data prior to the requested bill ID creation time (exclude)
    /// Unix timestamp in millisecond format, e.g. 1597026383085
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Query the data after the requested bill ID creation time (exclude)
    /// Unix timestamp in millisecond format, e.g. 1597026383085
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Number of results per request. The maximum is 100. The default is 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Sub-account transfer history information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferHistory {
    /// Bill ID
    pub bill_id: String,

    /// Transfer currency
    pub ccy: String,

    /// Transfer amount
    pub amt: String,

    /// Bill type
    #[serde(rename = "type")]
    pub bill_type: String,

    /// Sub-account name
    pub sub_acct: String,

    /// Bill ID creation time, Unix timestamp in millisecond format
    pub ts: String,
}

impl RestClient {
    /// Get history of sub-account transfer
    ///
    /// This endpoint is only available for master accounts. Transfer records are available
    /// from September 28, 2022 onwards.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#sub-account-rest-api-get-history-of-sub-account-transfer)
    ///
    /// Rate limit: 6 requests per second
    ///
    /// # Arguments
    /// * `request` - The transfer history request parameters
    ///
    /// # Returns
    /// A result containing the transfer history records for the sub-account
    pub async fn get_transfer_history(
        &self,
        request: GetTransferHistoryRequest,
    ) -> RestResult<TransferHistory> {
        self.send_get_request(
            GET_TRANSFER_HISTORY_ENDPOINT,
            request,
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::okx::response::ApiResponse;

    #[test]
    fn test_get_transfer_history_request_serialization() {
        let request = GetTransferHistoryRequest {
            ccy: Some("BTC".to_string()),
            transfer_type: Some("0".to_string()),
            sub_acct: Some("test_sub_001".to_string()),
            after: Some("1597026383085".to_string()),
            before: Some("1597026383086".to_string()),
            limit: Some("50".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("ccy=BTC"));
        assert!(serialized.contains("type=0"));
        assert!(serialized.contains("subAcct=test_sub_001"));
        assert!(serialized.contains("after=1597026383085"));
        assert!(serialized.contains("before=1597026383086"));
        assert!(serialized.contains("limit=50"));
    }

    #[test]
    fn test_get_transfer_history_request_minimal() {
        let request = GetTransferHistoryRequest {
            ccy: None,
            transfer_type: None,
            sub_acct: None,
            after: None,
            before: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_transfer_history_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "billId": "123456789012345678",
                    "ccy": "BTC",
                    "amt": "0.1",
                    "type": "0",
                    "subAcct": "test_sub_001",
                    "ts": "1597026383085"
                }
            ]
        }"#;

        let response: ApiResponse<TransferHistory> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let transfer = &response.data[0];
        assert_eq!(transfer.bill_id, "123456789012345678");
        assert_eq!(transfer.ccy, "BTC");
        assert_eq!(transfer.amt, "0.1");
        assert_eq!(transfer.bill_type, "0");
        assert_eq!(transfer.sub_acct, "test_sub_001");
        assert_eq!(transfer.ts, "1597026383085");
    }

    #[test]
    fn test_transfer_history_deserialization_multiple() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "billId": "123456789012345678",
                    "ccy": "BTC",
                    "amt": "0.1",
                    "type": "0",
                    "subAcct": "test_sub_001",
                    "ts": "1597026383085"
                },
                {
                    "billId": "123456789012345679",
                    "ccy": "ETH",
                    "amt": "2.5",
                    "type": "1",
                    "subAcct": "test_sub_002",
                    "ts": "1597026383086"
                },
                {
                    "billId": "123456789012345680",
                    "ccy": "USDT",
                    "amt": "1000",
                    "type": "0",
                    "subAcct": "test_sub_003",
                    "ts": "1597026383087"
                }
            ]
        }"#;

        let response: ApiResponse<TransferHistory> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 3);

        let btc_transfer = &response.data[0];
        assert_eq!(btc_transfer.bill_id, "123456789012345678");
        assert_eq!(btc_transfer.ccy, "BTC");
        assert_eq!(btc_transfer.amt, "0.1");
        assert_eq!(btc_transfer.bill_type, "0");
        assert_eq!(btc_transfer.sub_acct, "test_sub_001");
        assert_eq!(btc_transfer.ts, "1597026383085");

        let eth_transfer = &response.data[1];
        assert_eq!(eth_transfer.bill_id, "123456789012345679");
        assert_eq!(eth_transfer.ccy, "ETH");
        assert_eq!(eth_transfer.amt, "2.5");
        assert_eq!(eth_transfer.bill_type, "1");
        assert_eq!(eth_transfer.sub_acct, "test_sub_002");
        assert_eq!(eth_transfer.ts, "1597026383086");

        let usdt_transfer = &response.data[2];
        assert_eq!(usdt_transfer.bill_id, "123456789012345680");
        assert_eq!(usdt_transfer.ccy, "USDT");
        assert_eq!(usdt_transfer.amt, "1000");
        assert_eq!(usdt_transfer.bill_type, "0");
        assert_eq!(usdt_transfer.sub_acct, "test_sub_003");
        assert_eq!(usdt_transfer.ts, "1597026383087");
    }

    #[test]
    fn test_transfer_history_deserialization_empty() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": []
        }"#;

        let response: ApiResponse<TransferHistory> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 0);
    }
}
