use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::okx::{EndpointType, RestResult};

/// Endpoint URL for getting history of managed sub-account transfer
const GET_MANAGED_TRANSFER_HISTORY_ENDPOINT: &str =
    "api/v5/asset/subaccount/managed-subaccount-bills";

/// Request to get history of managed sub-account transfer
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetManagedTransferHistoryRequest {
    /// Currency, e.g. BTC
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

    /// Sub-account UID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_uid: Option<String>,

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

/// Managed sub-account transfer history information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedTransferHistory {
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

    /// Sub-account UID
    pub sub_uid: String,

    /// Bill ID creation time, Unix timestamp in millisecond format
    pub ts: String,
}

impl RestClient {
    /// Get history of managed sub-account transfer
    ///
    /// Only applicable to the trading team's master account to getting transfer records
    /// of managed sub accounts entrusted to oneself.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#sub-account-rest-api-get-history-of-managed-sub-account-transfer)
    ///
    /// Rate limit: 6 requests per second
    ///
    /// # Arguments
    /// * `request` - The managed transfer history request parameters
    ///
    /// # Returns
    /// A result containing the managed transfer history records
    pub async fn get_managed_transfer_history(
        &self,
        request: GetManagedTransferHistoryRequest,
    ) -> RestResult<ManagedTransferHistory> {
        self.send_get_request(
            GET_MANAGED_TRANSFER_HISTORY_ENDPOINT,
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
    fn test_get_managed_transfer_history_request_serialization() {
        let request = GetManagedTransferHistoryRequest {
            ccy: Some("BTC".to_string()),
            transfer_type: Some("1".to_string()),
            sub_acct: Some("managed_sub_001".to_string()),
            sub_uid: Some("446556018520336384".to_string()),
            after: Some("1597026383085".to_string()),
            before: Some("1597026383086".to_string()),
            limit: Some("25".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("ccy=BTC"));
        assert!(serialized.contains("type=1"));
        assert!(serialized.contains("subAcct=managed_sub_001"));
        assert!(serialized.contains("subUid=446556018520336384"));
        assert!(serialized.contains("after=1597026383085"));
        assert!(serialized.contains("before=1597026383086"));
        assert!(serialized.contains("limit=25"));
    }

    #[test]
    fn test_get_managed_transfer_history_request_minimal() {
        let request = GetManagedTransferHistoryRequest {
            ccy: None,
            transfer_type: None,
            sub_acct: None,
            sub_uid: None,
            after: None,
            before: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_managed_transfer_history_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "billId": "987654321098765432",
                    "ccy": "ETH",
                    "amt": "5.0",
                    "type": "1",
                    "subAcct": "managed_sub_001",
                    "subUid": "446556018520336384",
                    "ts": "1597026383085"
                }
            ]
        }"#;

        let response: ApiResponse<ManagedTransferHistory> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let transfer = &response.data[0];
        assert_eq!(transfer.bill_id, "987654321098765432");
        assert_eq!(transfer.ccy, "ETH");
        assert_eq!(transfer.amt, "5.0");
        assert_eq!(transfer.bill_type, "1");
        assert_eq!(transfer.sub_acct, "managed_sub_001");
        assert_eq!(transfer.sub_uid, "446556018520336384");
        assert_eq!(transfer.ts, "1597026383085");
    }

    #[test]
    fn test_managed_transfer_history_deserialization_multiple() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "billId": "987654321098765432",
                    "ccy": "ETH",
                    "amt": "5.0",
                    "type": "1",
                    "subAcct": "managed_sub_001",
                    "subUid": "446556018520336384",
                    "ts": "1597026383085"
                },
                {
                    "billId": "987654321098765433",
                    "ccy": "BTC",
                    "amt": "0.25",
                    "type": "0",
                    "subAcct": "managed_sub_002",
                    "subUid": "446556018520336385",
                    "ts": "1597026383086"
                },
                {
                    "billId": "987654321098765434",
                    "ccy": "USDT",
                    "amt": "2500",
                    "type": "1",
                    "subAcct": "managed_sub_003",
                    "subUid": "446556018520336386",
                    "ts": "1597026383087"
                }
            ]
        }"#;

        let response: ApiResponse<ManagedTransferHistory> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 3);

        let eth_transfer = &response.data[0];
        assert_eq!(eth_transfer.bill_id, "987654321098765432");
        assert_eq!(eth_transfer.ccy, "ETH");
        assert_eq!(eth_transfer.amt, "5.0");
        assert_eq!(eth_transfer.bill_type, "1");
        assert_eq!(eth_transfer.sub_acct, "managed_sub_001");
        assert_eq!(eth_transfer.sub_uid, "446556018520336384");
        assert_eq!(eth_transfer.ts, "1597026383085");

        let btc_transfer = &response.data[1];
        assert_eq!(btc_transfer.bill_id, "987654321098765433");
        assert_eq!(btc_transfer.ccy, "BTC");
        assert_eq!(btc_transfer.amt, "0.25");
        assert_eq!(btc_transfer.bill_type, "0");
        assert_eq!(btc_transfer.sub_acct, "managed_sub_002");
        assert_eq!(btc_transfer.sub_uid, "446556018520336385");
        assert_eq!(btc_transfer.ts, "1597026383086");

        let usdt_transfer = &response.data[2];
        assert_eq!(usdt_transfer.bill_id, "987654321098765434");
        assert_eq!(usdt_transfer.ccy, "USDT");
        assert_eq!(usdt_transfer.amt, "2500");
        assert_eq!(usdt_transfer.bill_type, "1");
        assert_eq!(usdt_transfer.sub_acct, "managed_sub_003");
        assert_eq!(usdt_transfer.sub_uid, "446556018520336386");
        assert_eq!(usdt_transfer.ts, "1597026383087");
    }

    #[test]
    fn test_managed_transfer_history_deserialization_empty() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": []
        }"#;

        let response: ApiResponse<ManagedTransferHistory> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 0);
    }
}
