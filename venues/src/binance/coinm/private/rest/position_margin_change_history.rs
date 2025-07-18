// Get Position Margin Change History (TRADE) endpoint implementation for GET /dapi/v1/positionMargin/history
// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Get-Position-Margin-Change-History>

use serde::{Deserialize, Serialize};

use crate::binance::{
    coinm::{MarginModificationType, PositionSide, RestResult, private::rest::client::RestClient},
    shared,
};

const POSITION_MARGIN_HISTORY_ENDPOINT: &str = "/dapi/v1/positionMargin/history";

/// Request parameters for getting position margin change history (GET /dapi/v1/positionMargin/history).
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetPositionMarginChangeHistoryRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    pub symbol: String,

    /// Margin modification type: 1 = Add position margin, 2 = Reduce position margin. Optional.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub modification_type: Option<MarginModificationType>,

    /// Start time in milliseconds. Optional.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time in milliseconds. Optional.
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Number of records to return. Default: 50. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Receive window in milliseconds.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds.
    pub timestamp: u64,
}

/// Individual position margin change history entry.
#[derive(Debug, Clone, Deserialize)]
pub struct PositionMarginChangeHistoryEntry {
    /// Margin amount.
    pub amount: String,

    /// Asset name.
    pub asset: String,

    /// Trading symbol.
    pub symbol: String,

    /// Time of the margin change.
    pub time: u64,

    /// Modification type: 1 = Add position margin, 2 = Reduce position margin.
    #[serde(rename = "type")]
    pub modification_type: u32,

    /// Position side.
    #[serde(rename = "positionSide")]
    pub position_side: PositionSide,
}

/// Response for getting position margin change history (GET /dapi/v1/positionMargin/history).
pub type GetPositionMarginChangeHistoryResponse = Vec<PositionMarginChangeHistoryEntry>;

impl RestClient {
    /// Gets position margin change history (TRADE) on Binance Coin-M Futures.
    ///
    /// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Get-Position-Margin-Change-History>
    /// GET /dapi/v1/positionMargin/history
    /// Weight: 1
    /// Requires API key and signature.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`GetPositionMarginChangeHistoryRequest`])
    ///
    /// # Returns
    /// A [`GetPositionMarginChangeHistoryResponse`] - array of position margin change history entries.
    pub async fn get_position_margin_change_history(
        &self,
        params: GetPositionMarginChangeHistoryRequest,
    ) -> RestResult<GetPositionMarginChangeHistoryResponse> {
        let weight = 1;
        shared::send_signed_request(
            self,
            POSITION_MARGIN_HISTORY_ENDPOINT,
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
    fn test_position_margin_change_history_request_serialization() {
        let request = GetPositionMarginChangeHistoryRequest {
            symbol: "BTCUSD_PERP".to_string(),
            modification_type: Some(MarginModificationType::Add),
            start_time: None,
            end_time: None,
            limit: None,
            recv_window: None,
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSD_PERP"));
        assert!(serialized.contains("type=1"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(!serialized.contains("startTime"));
        assert!(!serialized.contains("endTime"));
        assert!(!serialized.contains("limit"));
    }

    #[test]
    fn test_position_margin_change_history_request_full() {
        let request = GetPositionMarginChangeHistoryRequest {
            symbol: "ETHUSD_PERP".to_string(),
            modification_type: Some(MarginModificationType::Reduce),
            start_time: Some(1625097500000),
            end_time: Some(1625097700000),
            limit: Some(100),
            recv_window: Some(5000),
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETHUSD_PERP"));
        assert!(serialized.contains("type=2"));
        assert!(serialized.contains("startTime=1625097500000"));
        assert!(serialized.contains("endTime=1625097700000"));
        assert!(serialized.contains("limit=100"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1625097600000"));
    }

    #[test]
    fn test_position_margin_change_history_request_minimal() {
        let request = GetPositionMarginChangeHistoryRequest {
            symbol: "BTCUSD_PERP".to_string(),
            modification_type: None,
            start_time: None,
            end_time: None,
            limit: None,
            recv_window: None,
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSD_PERP"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(!serialized.contains("type"));
    }

    #[test]
    fn test_position_margin_change_history_entry_deserialization() {
        let json = r#"{
            "amount": "0.001",
            "asset": "BTC",
            "symbol": "BTCUSD_PERP",
            "time": 1625097600000,
            "type": 1,
            "positionSide": "LONG"
        }"#;

        let entry: PositionMarginChangeHistoryEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.amount, "0.001");
        assert_eq!(entry.asset, "BTC");
        assert_eq!(entry.symbol, "BTCUSD_PERP");
        assert_eq!(entry.time, 1625097600000);
        assert_eq!(entry.modification_type, 1);
        assert_eq!(entry.position_side, PositionSide::Long);
    }

    #[test]
    fn test_position_margin_change_history_response_deserialization() {
        let json = r#"[
            {
                "amount": "0.001",
                "asset": "BTC",
                "symbol": "BTCUSD_PERP",
                "time": 1625097600000,
                "type": 1,
                "positionSide": "LONG"
            },
            {
                "amount": "0.0005",
                "asset": "ETH",
                "symbol": "ETHUSD_PERP",
                "time": 1625097700000,
                "type": 2,
                "positionSide": "SHORT"
            }
        ]"#;

        let response: GetPositionMarginChangeHistoryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 2);
        
        let entry1 = &response[0];
        assert_eq!(entry1.amount, "0.001");
        assert_eq!(entry1.asset, "BTC");
        assert_eq!(entry1.symbol, "BTCUSD_PERP");
        assert_eq!(entry1.modification_type, 1);
        assert_eq!(entry1.position_side, PositionSide::Long);
        
        let entry2 = &response[1];
        assert_eq!(entry2.amount, "0.0005");
        assert_eq!(entry2.asset, "ETH");
        assert_eq!(entry2.symbol, "ETHUSD_PERP");
        assert_eq!(entry2.modification_type, 2);
        assert_eq!(entry2.position_side, PositionSide::Short);
    }

    #[test]
    fn test_empty_position_margin_change_history_response() {
        let json = r#"[]"#;
        let response: GetPositionMarginChangeHistoryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 0);
    }
}
