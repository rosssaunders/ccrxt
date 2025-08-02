use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bitget::spot::{RestResult, enums::*};

/// Get MainSub Transfer Record
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetMainSubTransferRecordRequest {
    /// Token name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,

    /// Transfer out type (default: initiator)
    /// initiator: initiator
    /// receiver: receiver
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<TransferRole>,

    /// Sub-account UID
    /// If empty, it only query the records that transfer from main account
    #[serde(rename = "subUid", skip_serializing_if = "Option::is_none")]
    pub sub_uid: Option<String>,

    /// The start time of the billing history
    /// Unix millisecond timestamp, e.g. 1690196141868
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,

    /// The end time of the billing history
    /// Unix millisecond timestamp, e.g. 1690196141868
    /// The interval between startTime and endTime must not exceed 90 days
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,

    /// Order ID customized by user
    #[serde(rename = "clientOid", skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,

    /// Number of results returned
    /// Default: 100, maximum 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,

    /// Requests the content on the page before this ID (older data)
    /// The value input should be the transferId of the corresponding interface
    #[serde(rename = "idLessThan", skip_serializing_if = "Option::is_none")]
    pub id_less_than: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMainSubTransferRecordResponse {
    pub code: String,
    pub msg: String,
    #[serde(rename = "requestTime")]
    pub request_time: u64,
    pub data: Vec<MainSubTransferRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainSubTransferRecord {
    /// Token name
    pub coin: String,

    /// Status of transfer
    /// Successful: Successful
    /// Failed: Failed
    /// Processing: Processing
    pub status: String,

    /// Recipient account type
    #[serde(rename = "toType")]
    pub to_type: String,

    /// Sender account type
    #[serde(rename = "fromType")]
    pub from_type: String,

    /// Quantity
    pub size: String,

    /// Transfer time, Unix millisecond timestamp
    pub ts: String,

    /// Order ID customized by user
    #[serde(rename = "clientOid")]
    pub client_oid: String,

    /// Transfer order ID
    #[serde(rename = "transferId")]
    pub transfer_id: String,

    /// The user ID who initiate the transfer ID
    #[serde(rename = "fromUserId")]
    pub from_user_id: String,

    /// The user ID who receive the transfer
    #[serde(rename = "toUserId")]
    pub to_user_id: String,
}

impl RestClient {
    /// Get MainSub Transfer Record
    ///
    /// Get transfer record for main/sub accounts.
    ///
    /// [docs]: https://www.bitget.com/api-doc/spot/account/Get-Transfer-Records
    ///
    /// Rate limit: 20 req/sec/UID
    ///
    /// Returns a `RestResult<GetMainSubTransferRecordResponse>` containing the transfer records or an error.
    pub async fn get_main_sub_transfer_record(
        &self,
        params: GetMainSubTransferRecordRequest,
    ) -> RestResult<GetMainSubTransferRecordResponse> {
        let endpoint = "/api/v2/spot/account/sub-main-trans-record";
        self.send_post_signed_request(endpoint, params, 20, false, None)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_main_sub_transfer_record_request_serialization() {
        let request = GetMainSubTransferRecordRequest {
            coin: Some("USDT".to_string()),
            role: Some(TransferRole::Initiator),
            limit: Some("100".to_string()),
            ..Default::default()
        };

        let serialized = serde_json::to_string(&request).unwrap();
        println!("Serialized request: {}", serialized);

        assert!(serialized.contains("\"coin\":\"USDT\""));
        assert!(serialized.contains("\"limit\":\"100\""));
    }

    #[test]
    fn test_get_main_sub_transfer_record_response_deserialization() {
        let json = r#"
        {
            "code": "00000",
            "msg": "success",
            "requestTime": 1700556280430,
            "data": [
                {
                    "coin": "USDT",
                    "status": "Successful",
                    "toType": "usdt_futures",
                    "fromType": "spot",
                    "size": "1020.00000000",
                    "ts": "1691476360467",
                    "clientOid": "xxxx",
                    "transferId": "xxxx",
                    "fromUserId": "xxxx",
                    "toUserId": "xxxx"
                }
            ]
        }"#;

        let response: GetMainSubTransferRecordResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, "00000");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].coin, "USDT");
        assert_eq!(response.data[0].status, "Successful");
    }

    #[tokio::test]
    async fn test_get_main_sub_transfer_record_endpoint() {
        // This test requires API credentials and should be run manually
        let _request = GetMainSubTransferRecordRequest {
            coin: Some("USDT".to_string()),
            role: None,
            sub_uid: None,
            start_time: None,
            end_time: None,
            client_oid: None,
            limit: None,
            id_less_than: None,
        };

        // Uncomment the following lines to test with real API credentials:
        // let client = BitgetRestClient::new("api_key", "secret", "passphrase", false);
        // let response = client.get_main_sub_transfer_record(request).await.unwrap();
        // println!("Response: {:?}", response);
    }
}
