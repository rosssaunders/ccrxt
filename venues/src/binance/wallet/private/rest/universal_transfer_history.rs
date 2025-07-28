use serde::{Deserialize, Serialize};

use crate::binance::{
    coinm::{RestResult, TransferType, PrivateRestClient as RestClient},
};

const UNIVERSAL_TRANSFER_HISTORY_ENDPOINT: &str = "/sapi/v1/asset/transfer";

/// Request parameters for universal transfer history.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UniversalTransferHistoryRequest {
    /// Transfer type
    #[serde(rename = "type")]
    pub transfer_type: TransferType,

    /// Start time in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Current page number, default 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<u32>,

    /// Page size, default 10, max 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,

    /// From symbol (for isolated margin)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_symbol: Option<String>,

    /// To symbol (for isolated margin)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_symbol: Option<String>,

    /// This parameter cannot be used in combination with other parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Transfer record in the history.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UniversalTransferHistoryRow {
    /// Asset
    pub asset: String,

    /// Amount
    pub amount: String,

    /// Transfer type
    #[serde(rename = "type")]
    pub transfer_type: String,

    /// Status (CONFIRMED, FAILED, PENDING)
    pub status: String,

    /// Transaction id
    pub tran_id: u64,

    /// Timestamp
    pub timestamp: u64,
}

/// Response for universal transfer history.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UniversalTransferHistoryResponse {
    /// Total records
    pub total: u32,

    /// Transfer records
    pub rows: Vec<UniversalTransferHistoryRow>,
}

impl RestClient {
    /// Get universal transfer history on Binance.
    ///
    /// See: <https://binance-docs.github.io/apidocs/spot/en/>
    /// GET /sapi/v1/asset/transfer
    /// Weight: 1
    /// Requires API key and signature.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`UniversalTransferHistoryRequest`])
    ///
    /// # Returns
    /// A [`UniversalTransferHistoryResponse`] object with transfer history.
    pub async fn get_universal_transfer_history(
        &self,
        params: UniversalTransferHistoryRequest,
    ) -> RestResult<UniversalTransferHistoryResponse> {
        let weight = 1;
        self.send_signed_request(
            UNIVERSAL_TRANSFER_HISTORY_ENDPOINT,
            reqwest::Method::GET,
            params,
            weight,
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universal_transfer_history_request_minimal() {
        let request = UniversalTransferHistoryRequest {
            transfer_type: TransferType::MainUmfuture,
            start_time: None,
            end_time: None,
            current: None,
            size: None,
            from_symbol: None,
            to_symbol: None,
            recv_window: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "type=MAIN_UMFUTURE");
    }

    #[test]
    fn test_universal_transfer_history_request_full() {
        let request = UniversalTransferHistoryRequest {
            transfer_type: TransferType::IsolatedmarginMargin,
            start_time: Some(1625097500000),
            end_time: Some(1625097700000),
            current: Some(2),
            size: Some(50),
            from_symbol: Some("BTCUSDT".to_string()),
            to_symbol: None,
            recv_window: Some(5000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("type=ISOLATEDMARGIN_MARGIN"));
        assert!(serialized.contains("startTime=1625097500000"));
        assert!(serialized.contains("endTime=1625097700000"));
        assert!(serialized.contains("current=2"));
        assert!(serialized.contains("size=50"));
        assert!(serialized.contains("fromSymbol=BTCUSDT"));
        assert!(!serialized.contains("toSymbol"));
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_universal_transfer_history_row_deserialization() {
        let json = r#"{
            "asset": "USDT",
            "amount": "100.50",
            "type": "MAIN_UMFUTURE",
            "status": "CONFIRMED",
            "tranId": 13526853623,
            "timestamp": 1625097600000
        }"#;

        let row: UniversalTransferHistoryRow = serde_json::from_str(json).unwrap();
        assert_eq!(row.asset, "USDT");
        assert_eq!(row.amount, "100.50");
        assert_eq!(row.transfer_type, "MAIN_UMFUTURE");
        assert_eq!(row.status, "CONFIRMED");
        assert_eq!(row.tran_id, 13526853623);
        assert_eq!(row.timestamp, 1625097600000);
    }

    #[test]
    fn test_universal_transfer_history_response_deserialization() {
        let json = r#"{
            "total": 3,
            "rows": [
                {
                    "asset": "USDT",
                    "amount": "100.50",
                    "type": "MAIN_UMFUTURE",
                    "status": "CONFIRMED",
                    "tranId": 13526853623,
                    "timestamp": 1625097600000
                },
                {
                    "asset": "BTC",
                    "amount": "0.001",
                    "type": "ISOLATEDMARGIN_MAIN",
                    "status": "CONFIRMED",
                    "tranId": 13526853624,
                    "timestamp": 1625097700000
                },
                {
                    "asset": "ETH",
                    "amount": "0.5",
                    "type": "MAIN_ISOLATEDMARGIN",
                    "status": "PENDING",
                    "tranId": 13526853625,
                    "timestamp": 1625097800000
                }
            ]
        }"#;

        let response: UniversalTransferHistoryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.total, 3);
        assert_eq!(response.rows.len(), 3);

        assert_eq!(response.rows[0].asset, "USDT");
        assert_eq!(response.rows[0].amount, "100.50");
        assert_eq!(response.rows[0].status, "CONFIRMED");

        assert_eq!(response.rows[1].asset, "BTC");
        assert_eq!(response.rows[1].transfer_type, "ISOLATEDMARGIN_MAIN");

        assert_eq!(response.rows[2].asset, "ETH");
        assert_eq!(response.rows[2].status, "PENDING");
    }

    #[test]
    fn test_empty_transfer_history_response() {
        let json = r#"{
            "total": 0,
            "rows": []
        }"#;

        let response: UniversalTransferHistoryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.total, 0);
        assert_eq!(response.rows.len(), 0);
    }
}
