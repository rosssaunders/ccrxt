use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bitget::spot::{Errors, RestResult};

const SUBACCOUNT_DEPOSIT_RECORDS_ENDPOINT: &str = "/api/v2/spot/wallet/subaccount-deposit-records";
/// Request for getting subaccount deposit records
#[derive(Debug, Clone, Serialize)]
pub struct GetSubaccountDepositRecordsRequest {
    /// Sub Account UID
    #[serde(rename = "subUid")]
    pub sub_uid: String,
    /// Coin name, e.g. USDT (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    /// Record start time (Unix millisecond timestamp) (optional)
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// Record end time (Unix millisecond timestamp) (optional)
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// Request content before this ID (older data) (optional)
    #[serde(rename = "idLessThan", skip_serializing_if = "Option::is_none")]
    pub id_less_than: Option<String>,
    /// Number of entries per page (default: 20, max: 100) (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Subaccount deposit record
#[derive(Debug, Clone, Deserialize)]
pub struct SubaccountDepositRecord {
    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: String,
    /// Trade ID (on-chain hash or internal trade ID)
    #[serde(rename = "tradeId")]
    pub trade_id: String,
    /// Token name
    pub coin: String,
    /// Customized order ID
    #[serde(rename = "clientOid")]
    pub client_oid: Option<String>,
    /// Quantity
    pub size: String,
    /// Deposit status (pending, fail, success)
    pub status: String,
    /// Deposit initiator address
    #[serde(rename = "fromAddress")]
    pub from_address: String,
    /// Coin receiver address
    #[serde(rename = "toAddress")]
    pub to_address: String,
    /// Deposit network
    pub chain: String,
    /// Number of confirmed blocks
    pub confirm: Option<String>,
    /// Deposit type (on_chain, internal_transfer)
    pub dest: String,
    /// Tag
    pub tag: String,
    /// Creation time in ms
    #[serde(rename = "cTime")]
    pub c_time: String,
    /// Update time in ms
    #[serde(rename = "uTime")]
    pub u_time: String,
}

/// Response for getting subaccount deposit records
#[derive(Debug, Clone, Deserialize)]
pub struct GetSubaccountDepositRecordsResponse {
    /// List of deposit records
    pub data: Vec<SubaccountDepositRecord>,
}

impl RestClient {
    /// Get Subaccount Deposit Records
    pub async fn get_subaccount_deposit_records(
        &self,
        request: GetSubaccountDepositRecordsRequest,
    ) -> RestResult<GetSubaccountDepositRecordsResponse> {
        self.send_signed_request(
            SUBACCOUNT_DEPOSIT_RECORDS_ENDPOINT,
            reqwest::Method::GET,
            None,
            Some(
                &serde_json::to_string(&request)
                    .map_err(|e| Errors::Error(format!("Serialization error: {e}")))?,
            ),
            10,
            false,
            None,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_builder() {
        let request = GetSubaccountDepositRecordsRequest {
            sub_uid: "12121212".to_string(),
            coin: Some("USDT".to_string()),
            id_less_than: Some("1111120137173336063".to_string()),
            limit: Some("5".to_string()),
            start_time: None,
            end_time: None,
        };

        assert_eq!(request.sub_uid, "12121212");
        assert_eq!(request.coin, Some("USDT".to_string()));
        assert_eq!(
            request.id_less_than,
            Some("1111120137173336063".to_string())
        );
        assert_eq!(request.limit, Some("5".to_string()));
    }

    #[test]
    fn test_request_builder_required_only() {
        let request = GetSubaccountDepositRecordsRequest {
            sub_uid: "12121212".to_string(),
            coin: None,
            start_time: None,
            end_time: None,
            id_less_than: None,
            limit: None,
        };

        assert_eq!(request.sub_uid, "12121212");
        assert_eq!(request.coin, None);
        assert_eq!(request.start_time, None);
        assert_eq!(request.end_time, None);
        assert_eq!(request.id_less_than, None);
        assert_eq!(request.limit, None);
    }
}
