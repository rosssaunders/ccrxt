use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::{RestResult, enums::IncomeType};

const INCOME_HISTORY_ENDPOINT: &str = "/fapi/v1/income";

/// Request parameters for getting income history
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetIncomeHistoryRequest {
    /// Trading symbol (e.g., "BTCUSDT"). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Income type filter. If not sent, all kinds of flow will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub income_type: Option<IncomeType>,

    /// Timestamp in ms to get funding from INCLUSIVE. If neither startTime nor endTime is sent, the recent 7-day data will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// Timestamp in ms to get funding until INCLUSIVE. If neither startTime nor endTime is sent, the recent 7-day data will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Page number for pagination. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Number of records to return (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Receive window for request validity (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Individual income history entry
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomeHistoryEntry {
    /// Trading symbol (if existing).
    pub symbol: String,

    /// Income type.
    pub income_type: IncomeType,

    /// Income amount.
    pub income: String,

    /// Income asset.
    pub asset: String,

    /// Extra information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,

    /// Time when income was recorded (milliseconds since epoch).
    pub time: u64,

    /// Transaction ID (unique in the same incomeType for a user).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tran_id: Option<u64>,

    /// Trade ID (if existing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
}

/// Response wrapper for income history endpoint
pub type GetIncomeHistoryResponse = Vec<IncomeHistoryEntry>;

impl UsdmClient {
    /// Get Income History (USER_DATA)
    ///
    /// Query income history
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Get-Income-History
    ///
    /// Rate limit: 30
    ///
    /// # Arguments
    /// * `request` - The income history request parameters
    ///
    /// # Returns
    /// A list of income history entries
    pub async fn get_income_history(
        &self,
        request: GetIncomeHistoryRequest,
    ) -> RestResult<GetIncomeHistoryResponse> {
        self.send_get_signed_request(INCOME_HISTORY_ENDPOINT, request, 30, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_income_history_request_serialization_minimal() {
        let request = GetIncomeHistoryRequest::default();
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_get_income_history_request_serialization_with_symbol() {
        let request = GetIncomeHistoryRequest {
            symbol: Some("BTCUSDT".to_string()),
            ..Default::default()
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTCUSDT");
    }

    #[test]
    fn test_get_income_history_request_serialization_full() {
        let request = GetIncomeHistoryRequest {
            symbol: Some("BTCUSDT".to_string()),
            income_type: Some(IncomeType::RealizedPnl),
            start_time: Some(1625097600000),
            end_time: Some(1625184000000),
            page: Some(1),
            limit: Some(100),
            recv_window: Some(5000),
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("incomeType=REALIZED_PNL"));
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625184000000"));
        assert!(serialized.contains("page=1"));
        assert!(serialized.contains("limit=100"));
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_income_history_entry_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "incomeType": "REALIZED_PNL",
            "income": "-1.37500000",
            "asset": "USDT",
            "info": "BTCUSDT",
            "time": 1570608000000,
            "tranId": 9689322392,
            "tradeId": "2059192"
        }"#;

        let entry: IncomeHistoryEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.symbol, "BTCUSDT");
        assert_eq!(entry.income, "-1.37500000");
        assert_eq!(entry.asset, "USDT");
        assert_eq!(entry.time, 1570608000000);
        assert_eq!(entry.tran_id, Some(9689322392));
        assert_eq!(entry.trade_id, Some("2059192".to_string()));
    }

    #[test]
    fn test_income_history_response_deserialization() {
        let json = r#"
        [
            {
                "symbol": "BTCUSDT",
                "incomeType": "REALIZED_PNL",
                "income": "-1.37500000",
                "asset": "USDT",
                "info": "BTCUSDT",
                "time": 1570608000000,
                "tranId": 9689322392,
                "tradeId": "2059192"
            },
            {
                "symbol": "",
                "incomeType": "TRANSFER",
                "income": "-0.37500000",
                "asset": "USDT",
                "info": "TRANSFER",
                "time": 1570608000000,
                "tranId": 9689322393,
                "tradeId": ""
            }
        ]
        "#;

        let response: GetIncomeHistoryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 2);

        // Test first entry
        assert_eq!(response[0].symbol, "BTCUSDT");
        assert_eq!(response[0].income, "-1.37500000");
        assert_eq!(response[0].asset, "USDT");
        assert_eq!(response[0].time, 1570608000000);

        // Test second entry
        assert_eq!(response[1].symbol, "");
        assert_eq!(response[1].income, "-0.37500000");
        assert_eq!(response[1].asset, "USDT");
        assert_eq!(response[1].time, 1570608000000);
    }

    #[test]
    fn test_income_history_response_deserialization_empty() {
        let json = "[]";
        let response: GetIncomeHistoryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 0);
    }

    #[test]
    fn test_income_history_entry_optional_fields() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "incomeType": "COMMISSION",
            "income": "-0.01000000",
            "asset": "USDT",
            "time": 1570636800000
        }"#;

        let entry: IncomeHistoryEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.symbol, "BTCUSDT");
        assert_eq!(entry.income, "-0.01000000");
        assert_eq!(entry.asset, "USDT");
        assert_eq!(entry.time, 1570636800000);
        assert_eq!(entry.info, None);
        assert_eq!(entry.tran_id, None);
        assert_eq!(entry.trade_id, None);
    }
}
