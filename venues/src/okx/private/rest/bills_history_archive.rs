use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::okx::{EndpointType, RestResult};

const ACCOUNT_BILLS_HISTORY_ARCHIVE_ENDPOINT: &str = "api/v5/account/bills-history-archive";

/// Request to post bills history archive
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostBillsHistoryArchiveRequest {
    /// Year to retrieve, e.g. "2022"
    pub year: String,

    /// Quarter to retrieve. Q1, Q2, Q3, Q4
    pub quarter: String,
}

/// Response from posting bills history archive
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostBillsHistoryArchiveResponse {
    /// Result
    pub result: String,
}

/// Request to get bills history archive
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBillsHistoryArchiveRequest {
    /// Download link
    pub file_id: String,
}

/// Response from getting bills history archive
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBillsHistoryArchiveResponse {
    /// File ID
    pub file_id: String,

    /// File name
    pub file_name: String,

    /// File size
    pub file_size: String,

    /// Download link
    pub download_link: String,

    /// Link generated timestamp
    pub link_gen_ts: String,

    /// Link expiry timestamp
    pub link_exp_ts: String,
}

impl RestClient {
    /// Post bills history archive
    ///
    /// [docs]: https://www.okx.com/docs-v5/en/#trading-account-rest-api-get-bills-details-3-months
    ///
    /// # Arguments
    /// * `request` - The post bills history archive request
    ///
    /// # Returns
    /// A result containing the response or an error
    pub async fn post_bills_history_archive(
        &self,
        request: PostBillsHistoryArchiveRequest,
    ) -> RestResult<PostBillsHistoryArchiveResponse> {
        self.send_post_request(
            ACCOUNT_BILLS_HISTORY_ARCHIVE_ENDPOINT,
            request,
            EndpointType::PrivateAccount,
        )
        .await
    }

    /// Get bills history archive
    ///
    /// # Arguments
    /// * `request` - The get bills history archive request
    ///
    /// # Returns
    /// A result containing the response or an error
    pub async fn get_bills_history_archive(
        &self,
        request: GetBillsHistoryArchiveRequest,
    ) -> RestResult<GetBillsHistoryArchiveResponse> {
        self.send_get_request(
            ACCOUNT_BILLS_HISTORY_ARCHIVE_ENDPOINT,
            request,
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::okx::response::OkxApiResponse;

    #[test]
    fn test_post_bills_history_archive_request_serialization() {
        let request = PostBillsHistoryArchiveRequest {
            year: "2022".to_string(),
            quarter: "Q1".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"year\":\"2022\""));
        assert!(json.contains("\"quarter\":\"Q1\""));
    }

    #[test]
    fn test_get_bills_history_archive_request_serialization() {
        let request = GetBillsHistoryArchiveRequest {
            file_id: "file123".to_string(),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("fileId=file123"));
    }

    #[test]
    fn test_post_bills_history_archive_response_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "result": "true"
                }
            ]
        }"#;

        let response: OkxApiResponse<PostBillsHistoryArchiveResponse> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].result, "true");
    }

    #[test]
    fn test_get_bills_history_archive_response_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "fileId": "file123",
                    "fileName": "bills_2022_Q1.csv",
                    "fileSize": "1024",
                    "downloadLink": "https://example.com/download",
                    "linkGenTs": "1597026383085",
                    "linkExpTs": "1597026383085"
                }
            ]
        }"#;

        let response: OkxApiResponse<GetBillsHistoryArchiveResponse> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let archive = &response.data[0];
        assert_eq!(archive.file_id, "file123");
        assert_eq!(archive.file_name, "bills_2022_Q1.csv");
        assert_eq!(archive.download_link, "https://example.com/download");
    }
}
