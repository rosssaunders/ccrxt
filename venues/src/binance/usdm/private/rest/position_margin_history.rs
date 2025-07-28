use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::borrow::Cow;

use super::UsdmClient;
use crate::binance::usdm::RestResult;
use crate::binance::usdm::enums::*;

/// Endpoint path for position margin change history.
const POSITION_MARGIN_HISTORY_ENDPOINT: &str = "/fapi/v1/positionMargin/history";

/// Type of margin modification for position margin history.
///
/// 1: Add position margin
/// 2: Reduce position margin
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum MarginModificationType {
    /// Add position margin
    Add = 1,

    /// Reduce position margin
    Reduce = 2,
}

/// Request parameters for the position margin change history endpoint.
///
/// All credential fields must be securely stored and passed as `SecretString`.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionMarginHistoryRequest {
    /// Trading symbol (e.g., "BTCUSDT"). Required.
    pub symbol: Cow<'static, str>,

    /// Type of margin modification. Optional. 1: Add, 2: Reduce.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub modification_type: Option<MarginModificationType>,

    /// Start time for filtering (milliseconds since epoch). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time for filtering (milliseconds since epoch). Optional. Defaults to current time if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Maximum number of results to return. Optional. Default: 500.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Optional receive window (milliseconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp (milliseconds since epoch). Required by API, set automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,
}

/// Represents a single position margin change history entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionMarginHistoryEntry {
    /// Trading symbol.
    pub symbol: String,

    /// Type of margin modification (1: Add, 2: Reduce).
    #[serde(rename = "type")]
    pub modification_type: MarginModificationType,

    /// Delta type (e.g., "USER_ADJUST").
    pub delta_type: Option<String>,

    /// Amount of margin changed.
    pub amount: String,

    /// Asset (e.g., "USDT").
    pub asset: String,

    /// Time of modification (milliseconds since epoch).
    pub time: u64,

    /// Position side (e.g., "BOTH", "LONG", "SHORT").
    pub position_side: PositionSide,
}

impl UsdmClient {
    /// Get Position Margin Change History (TRADE)
    ///
    /// Retrieves position margin change history for a given symbol on Binance USDM Futures.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Get-Position-Margin-Change-History
    ///
    /// Rate limit: 1 request per second
    ///
    /// # Arguments
    /// * `params` - The request parameters for position margin history
    ///
    /// # Returns
    /// A vector of position margin change history entries
    pub async fn get_position_margin_history(
        &self,
        params: GetPositionMarginHistoryRequest,
    ) -> RestResult<Vec<PositionMarginHistoryEntry>> {
        self.send_signed_request(
            POSITION_MARGIN_HISTORY_ENDPOINT,
            reqwest::Method::GET,
            params,
            1,
            true,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_margin_history_response_deserialization() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "type": 1,
                "deltaType": "USER_ADJUST",
                "amount": "23.36332311",
                "asset": "USDT",
                "time": 1578047897183,
                "positionSide": "BOTH"
            },
            {
                "symbol": "BTCUSDT",
                "type": 2,
                "deltaType": "USER_ADJUST",
                "amount": "100",
                "asset": "USDT",
                "time": 1578047900425,
                "positionSide": "LONG"
            }
        ]"#;

        let result: Vec<PositionMarginHistoryEntry> = serde_json::from_str(json).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].symbol, "BTCUSDT");
        assert_eq!(result[0].amount, "23.36332311");
        assert_eq!(result[0].asset, "USDT");
        assert_eq!(result[0].position_side, PositionSide::Both);
        assert_eq!(result[0].delta_type.as_deref(), Some("USER_ADJUST"));
        assert!(matches!(
            result[0].modification_type,
            MarginModificationType::Add
        ));
        assert!(matches!(
            result[1].modification_type,
            MarginModificationType::Reduce
        ));
    }

    #[test]
    fn test_get_position_margin_history_request_serialization() {
        let request = GetPositionMarginHistoryRequest {
            symbol: Cow::Borrowed("BTCUSDT"),
            modification_type: Some(MarginModificationType::Add),
            start_time: Some(1578047897183),
            end_time: Some(1578047897184),
            limit: Some(100),
            recv_window: Some(5000),
            timestamp: Some(1578047897000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("type=1"));
        assert!(serialized.contains("startTime=1578047897183"));
        assert!(serialized.contains("endTime=1578047897184"));
        assert!(serialized.contains("limit=100"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1578047897000"));
    }

    #[test]
    fn test_margin_modification_type_serialization() {
        let add_type = MarginModificationType::Add;
        let reduce_type = MarginModificationType::Reduce;

        let add_json = serde_json::to_string(&add_type).unwrap();
        let reduce_json = serde_json::to_string(&reduce_type).unwrap();

        assert_eq!(add_json, "1");
        assert_eq!(reduce_json, "2");
    }
}
