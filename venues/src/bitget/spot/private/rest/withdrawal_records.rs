use serde::{Deserialize, Serialize};

use super::super::RestClient;
use crate::bitget::spot::RestResult;

/// Endpoint for getting withdrawal records
const WITHDRAWAL_RECORDS_ENDPOINT: &str = "/api/v2/spot/wallet/withdrawal-records";

/// Get Withdrawal Records.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWithdrawalRecordsRequest {
    /// Coin name, e.g. USDT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,

    /// Client customized ID
    #[serde(rename = "clientOid", skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,

    /// The record start time for the query. Unix millisecond timestamp
    #[serde(rename = "startTime")]
    pub start_time: String,

    /// The end time of the record for the query. Unix millisecond timestamp
    #[serde(rename = "endTime")]
    pub end_time: String,

    /// Requests the content on the page before this ID (older data),
    /// the value input should be the orderId of the corresponding interface.
    #[serde(rename = "idLessThan", skip_serializing_if = "Option::is_none")]
    pub id_less_than: Option<String>,

    /// The response orderId
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// Number of entries per page
    /// The default value is 20 and the maximum value is 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWithdrawalRecordsResponse {
    pub code: String,
    pub msg: String,
    #[serde(rename = "requestTime")]
    pub request_time: u64,
    pub data: Vec<WithdrawalRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawalRecord {
    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: String,

    /// TX ID
    /// When dest is on_chain, it's the on chain hash value
    /// If the dest is internal_transfer, it is the trade ID
    #[serde(rename = "tradeId")]
    pub trade_id: String,

    /// Token name
    pub coin: String,

    /// Client customized ID
    #[serde(rename = "clientOid")]
    pub client_oid: String,

    /// Type (Fixed value: withdraw)
    #[serde(rename = "type")]
    pub record_type: String,

    /// Type of withdrawal
    /// on_chain: withdrawal on chain
    /// internal_transfer: internal transfer
    pub dest: String,

    /// Quantity
    pub size: String,

    /// Transaction Fee
    pub fee: String,

    /// Withdrawal status
    /// pending: Pending preliminary examination
    /// fail: Failed
    /// success: Successful
    pub status: String,

    /// Withdrawal Initiators
    /// If dest is on_chain, it's the on chain address
    /// If dest is internal_transfer, it would be the UID, email or the mobile
    #[serde(rename = "fromAddress")]
    pub from_address: String,

    /// Coin receiver address
    /// If dest is on_chain, it's the on chain address
    /// If dest is internal_transfer, it would be the UID, email or the mobile
    #[serde(rename = "toAddress")]
    pub to_address: String,

    /// Withdrawal network
    /// If dest is internal_transfer, please ignore this parameter
    pub chain: String,

    /// Number of confirmed blocks
    pub confirm: String,

    /// Tag
    pub tag: String,

    /// Creation time (ms)
    #[serde(rename = "cTime")]
    pub c_time: String,

    /// Update time (ms)
    #[serde(rename = "uTime")]
    pub u_time: String,
}

impl RestClient {
    /// Get Withdrawal Records.
    /// Frequency limit: 10 times/1s (User ID)
    pub async fn get_withdrawal_records(
        &self,
        request: GetWithdrawalRecordsRequest,
    ) -> RestResult<GetWithdrawalRecordsResponse> {
        self.send_get_signed_request(WITHDRAWAL_RECORDS_ENDPOINT, request,
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
    fn test_get_withdrawal_records_request_serialization() {
        let request = GetWithdrawalRecordsRequest {
            coin: Some("USDT".to_string()),
            client_oid: None,
            start_time: "1659036670000".to_string(),
            end_time: "1659076670000".to_string(),
            id_less_than: None,
            order_id: None,
            limit: Some("20".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        // println!("Serialized request: {}", serialized); // Avoid println! per project rules

        assert!(serialized.contains("\"startTime\":\"1659036670000\""));
        assert!(serialized.contains("\"endTime\":\"1659076670000\""));
        assert!(serialized.contains("\"coin\":\"USDT\""));
    }

    #[test]
    fn test_get_withdrawal_records_response_deserialization() {
        let json = r#"
        {
            "code": "00000",
            "msg": "success",
            "requestTime": 1654507973411,
            "data": [
                {
                    "orderId": "1",
                    "tradeId": "1",
                    "coin": "USDT",
                    "dest": "on_chain",
                    "clientOid": "123",
                    "type": "withdraw",
                    "tag": "",
                    "size": "10.00000000",
                    "fee": "-1.00000000",
                    "status": "success",
                    "toAddress": "TJRyWwFs9wTFGZg3JbrVriFbNfCug5tDeC",
                    "fromAddress": "internal_address",
                    "confirm": "100",
                    "chain": "trc20",
                    "cTime": "1653290769222",
                    "uTime": "1653290769222"
                }
            ]
        }"#;

        let response: GetWithdrawalRecordsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, "00000");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].coin, "USDT");
        assert_eq!(response.data[0].status, "success");
    }

    #[tokio::test]
    async fn test_get_withdrawal_records_endpoint() {
        // This test requires API credentials and should be run manually
        let _request = GetWithdrawalRecordsRequest {
            coin: Some("USDT".to_string()),
            client_oid: None,
            start_time: "1659036670000".to_string(),
            end_time: "1659076670000".to_string(),
            id_less_than: None,
            order_id: None,
            limit: None,
        };

        // Uncomment the following lines to test with real API credentials:
        // let client = BitgetRestClient::new("api_key", "secret", "passphrase", false);
        // let response = client.get_withdrawal_records(request).await.unwrap();
        // println!("Response: {:?}", response);
    }
}
