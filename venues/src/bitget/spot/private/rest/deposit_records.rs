use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bitget::spot::RestResult;

/// Get Deposit Records
///
/// Frequency limit: 10 times/1s (UID)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDepositRecordsRequest {
    /// Coin name, e.g. USDT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,

    /// The response orderId
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

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

    /// Number of entries per page
    /// The default value is 20 and the maximum value is 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDepositRecordsResponse {
    pub code: String,
    pub msg: String,
    #[serde(rename = "requestTime")]
    pub request_time: u64,
    pub data: Vec<DepositRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositRecord {
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

    /// Type (Fixed value: deposit)
    #[serde(rename = "type")]
    pub record_type: String,

    /// Quantity
    pub size: String,

    /// Deposit status
    /// pending: pending confirmation
    /// fail: failed
    /// success: succeeded
    pub status: String,

    /// Deposit Initiators
    /// If dest is on_chain, it's the on chain address
    /// If dest is internal_transfer, it would be the UID, email or the mobile
    #[serde(rename = "fromAddress")]
    pub from_address: String,

    /// Coin Receiver
    /// If dest is on_chain, it's the on chain address
    /// If dest is internal_transfer, it would be the UID, email or the mobile
    #[serde(rename = "toAddress")]
    pub to_address: String,

    /// Deposit network
    /// If dest is internal_transfer, please ignore this parameter
    pub chain: String,

    /// Deposit Type
    /// on_chain: the on chain deposit
    /// internal_transfer: internal deposit
    pub dest: String,

    /// Creation time, ms
    #[serde(rename = "cTime")]
    pub c_time: String,

    /// Edit time, ms
    #[serde(rename = "uTime")]
    pub u_time: String,
}

impl RestClient {
    /// Get Deposit Records
    ///
    /// Get Deposit Records for a given coin, time range, and optional filters.
    ///
    /// [API Documentation](https://www.bitget.com/api-doc/spot/asset/Get-Deposit-Records)
    ///
    /// Frequency limit: 10 times/1s (UID)
    ///
    /// Returns a `RestResult<GetDepositRecordsResponse>` containing the deposit records or an error.
    pub async fn get_deposit_records(
        &self,
        params: GetDepositRecordsRequest,
    ) -> RestResult<GetDepositRecordsResponse> {
        let endpoint = "/api/v2/spot/wallet/deposit-records";
        self.send_signed_post_request(
            endpoint,
            &params,
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
    fn test_get_deposit_records_request_serialization() {
        let request = GetDepositRecordsRequest {
            start_time: "1659036670000".to_string(),
            end_time: "1659076670000".to_string(),
            coin: Some("USDT".to_string()),
            limit: Some("20".to_string()),
            order_id: None,
            id_less_than: None,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        println!("Serialized request: {}", serialized);

        assert!(serialized.contains("\"startTime\":\"1659036670000\""));
        assert!(serialized.contains("\"endTime\":\"1659076670000\""));
        assert!(serialized.contains("\"coin\":\"USDT\""));
    }

    #[test]
    fn test_get_deposit_records_response_deserialization() {
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
                    "type": "deposit",
                    "size": "10.00000000",
                    "status": "success",
                    "toAddress": "0x51xxx",
                    "dest": "on_chain",
                    "chain": "trc20",
                    "fromAddress": "0x52xxx",
                    "cTime": "1653290769222",
                    "uTime": "1653290769222"
                }
            ]
        }"#;

        let response: GetDepositRecordsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, "00000");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].coin, "USDT");
        assert_eq!(response.data[0].status, "success");
        assert_eq!(response.data[0].record_type, "deposit");
    }

    #[tokio::test]
    async fn test_get_deposit_records_endpoint() {
        // This test requires API credentials and should be run manually
        let _request = GetDepositRecordsRequest {
            start_time: "1659036670000".to_string(),
            end_time: "1659076670000".to_string(),
            coin: Some("USDT".to_string()),
            order_id: None,
            id_less_than: None,
            limit: None,
        };

        // Uncomment the following lines to test with real API credentials:
        // let client = BitgetRestClient::new("api_key", "secret", "passphrase", false);
        // let response = client.get_deposit_records(request).await.unwrap();
        // println!("Response: {:?}", response);
    }
}
