use crate::bitget::{BitgetRestClient, enums::*, error::BitgetError};
use reqwest::Method;
use rest::BitgetRequest;
use serde::{Deserialize, Serialize};

/// Get Transfer Record
///
/// Get transfer record.
///
/// Frequency limit: 20 times/1s (User ID)

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

impl GetTransferRecordRequest {
    pub fn new(coin: impl Into<String>) -> Self {
        Self {
            coin: coin.into(),
            from_type: None,
            start_time: None,
            end_time: None,
            client_oid: None,
            page_num: None,
            limit: None,
            id_less_than: None,
        }
    }

    pub fn from_type(mut self, from_type: AccountType) -> Self {
        self.from_type = Some(from_type);
        self
    }

    pub fn start_time(mut self, start_time: impl Into<String>) -> Self {
        self.start_time = Some(start_time.into());
        self
    }

    pub fn end_time(mut self, end_time: impl Into<String>) -> Self {
        self.end_time = Some(end_time.into());
        self
    }

    pub fn client_oid(mut self, client_oid: impl Into<String>) -> Self {
        self.client_oid = Some(client_oid.into());
        self
    }

    pub fn page_num(mut self, page_num: impl Into<String>) -> Self {
        self.page_num = Some(page_num.into());
        self
    }

    pub fn limit(mut self, limit: impl Into<String>) -> Self {
        self.limit = Some(limit.into());
        self
    }

    pub fn id_less_than(mut self, id_less_than: impl Into<String>) -> Self {
        self.id_less_than = Some(id_less_than.into());
        self
    }
}

impl BitgetRequest for GetTransferRecordRequest {
    type Response = GetTransferRecordResponse;

    fn path(&self) -> String {
        "/api/v2/spot/account/transferRecords".to_string()
    }

    fn method(&self) -> String {
        "GET".to_string()
    }

    fn need_signature(&self) -> bool {
        true
    }
}

impl BitgetRestClient {
    /// Get Transfer Record
    ///
    /// Get transfer record.
    pub async fn get_transfer_record(
        &self,
        request: GetTransferRecordRequest,
    ) -> Result<GetTransferRecordResponse, BitgetError> {
        self.send_request(&request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_transfer_record_request_serialization() {
        let request = GetTransferRecordRequest::new("USDT")
            .from_type(AccountType::Spot)
            .limit("100");

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
        let _request = GetTransferRecordRequest::new("USDT").from_type(AccountType::Spot);

        // Uncomment the following lines to test with real API credentials:
        // let client = BitgetRestClient::new("api_key", "secret", "passphrase", false);
        // let response = client.get_transfer_record(request).await.unwrap();
        // println!("Response: {:?}", response);
    }
}
