use serde::{Deserialize, Serialize};

use super::super::RestClient;
use crate::bitget::spot::{RestResult, enums::*};

/// Endpoint for getting transfer records
const TRANSFER_RECORD_ENDPOINT: &str = "/api/v2/spot/wallet/transfer-records";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTransferRecordRequest {
    /// Token name
    pub coin: String,

    /// Account type
    #[serde(rename = "fromType", skip_serializing_if = "Option::is_none")]
    pub from_type: Option<AccountType>,

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

    /// Requests the content on the page
    /// default: 1, max: 1000
    #[serde(rename = "pageNum", skip_serializing_if = "Option::is_none")]
    pub page_num: Option<String>,

    /// Number of results returned
    /// Default: 100, maximum 500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,

    /// (Deprecated) Requests the content on the page before this ID (older data)
    /// The value input should be the transferId of the corresponding interface
    #[serde(rename = "idLessThan", skip_serializing_if = "Option::is_none")]
    pub id_less_than: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTransferRecordResponse {
    pub code: String,
    pub msg: String,
    #[serde(rename = "requestTime")]
    pub request_time: u64,
    pub data: Vec<TransferRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRecord {
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

    /// Trading pair for the recipient account
    /// Returned when the recipient account is isolated_margin
    #[serde(rename = "toSymbol")]
    pub to_symbol: String,

    /// Sender account type
    #[serde(rename = "fromType")]
    pub from_type: String,

    /// Trading pair for the sending account
    /// Return when the sending account is isolated_margin
    #[serde(rename = "fromSymbol")]
    pub from_symbol: String,

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
}

impl RestClient {
    /// Get Transfer Record
    ///
    /// Get transfer record.
    pub async fn get_transfer_record(
        &self,
        request: GetTransferRecordRequest,
    ) -> RestResult<GetTransferRecordResponse> {
        self.send_signed_get_request(TRANSFER_RECORD_ENDPOINT, Some(&request), 20, false, None)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_transfer_record_request_serialization() {
        let request = GetTransferRecordRequest {
            coin: "USDT".to_string(),
            from_type: Some(AccountType::Spot),
            limit: Some("100".to_string()),
            start_time: None,
            end_time: None,
            client_oid: None,
            page_num: None,
            id_less_than: None,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        println!("Serialized request: {}", serialized);

        assert!(serialized.contains("\"coin\":\"USDT\""));
        assert!(serialized.contains("\"limit\":\"100\""));
    }

    #[test]
    fn test_get_transfer_record_response_deserialization() {
        let json = r#"
        {
            "code": "00000",
            "data": [
                {
                    "coin": "btc",
                    "status": "Successful",
                    "toType": "usdt_futures",
                    "toSymbol": "",
                    "fromType": "spot",
                    "fromSymbol": "BTC/USD",
                    "size": "1000.00000000",
                    "ts": "1631070374488",
                    "clientOid": "1",
                    "transferId": "1"
                }
            ],
            "msg": "success",
            "requestTime": 1631608142260
        }"#;

        let response: GetTransferRecordResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, "00000");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].coin, "btc");
        assert_eq!(response.data[0].status, "Successful");
    }

    #[tokio::test]
    async fn test_get_transfer_record_endpoint() {
        // This test requires API credentials and should be run manually
        let _request = GetTransferRecordRequest {
            coin: "USDT".to_string(),
            from_type: Some(AccountType::Spot),
            start_time: None,
            end_time: None,
            client_oid: None,
            page_num: None,
            limit: None,
            id_less_than: None,
        };

        // Uncomment the following lines to test with real API credentials:
        // let client = BitgetRestClient::new("api_key", "secret", "passphrase", false);
        // let response = client.get_transfer_record(request).await.unwrap();
        // println!("Response: {:?}", response);
    }
}
