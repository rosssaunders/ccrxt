use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, RestResult};

/// REST API endpoint constant
const MOVE_POSITIONS_ENDPOINT: &str = "private/move_positions";

/// Trade data for position move
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovePositionTrade {
    /// Instrument name (e.g., "BTC-PERPETUAL").
    /// This must match an open position in the source subaccount.
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,

    /// Price for trade. If not provided, the average price of the position is used.
    /// Optional.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,

    /// The requested trade size. For perpetual and inverse futures, the amount is in USD units.
    /// For options and linear futures, it is the underlying base currency coin. Amount can't exceed position size.
    #[serde(rename = "amount")]
    pub amount: f64,
}

/// Request parameters for moving positions between subaccounts.
#[derive(Debug, Clone, Serialize)]
pub struct MovePositionsRequest {
    /// The currency symbol (e.g., "BTC"). Optional.
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// ID of source subaccount. Can be found in My Account >> Subaccounts tab.
    #[serde(rename = "source_uid")]
    pub source_uid: i32,

    /// ID of target subaccount. Can be found in My Account >> Subaccounts tab.
    #[serde(rename = "target_uid")]
    pub target_uid: i32,

    /// List of trades for position move. Each entry describes a position to move.
    #[serde(rename = "trades")]
    pub trades: Vec<MovePositionTrade>,
}

/// Trade result data in response to a move positions request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovePositionTradeResult {
    /// Trade amount. For perpetual and inverse futures the amount is in USD units. For options and linear futures it is the underlying base currency coin.
    #[serde(rename = "amount")]
    pub amount: f64,

    /// Direction: buy or sell.
    #[serde(rename = "direction")]
    pub direction: String,

    /// Unique instrument identifier.
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,

    /// Price in base currency.
    #[serde(rename = "price")]
    pub price: f64,

    /// Trade source uid.
    #[serde(rename = "source_uid")]
    pub source_uid: i32,

    /// Trade target uid.
    #[serde(rename = "target_uid")]
    pub target_uid: i32,
}

/// Response for move positions endpoint.
pub type MovePositionsResponse = Vec<MovePositionTradeResult>;

impl RestClient {
    /// Move positions between subaccounts.
    ///
    /// [Deribit API docs](https://docs.deribit.com/#private-move_positions)
    ///
    /// This endpoint allows moving open positions from one subaccount to another.
    /// Requires authentication and appropriate permissions.
    pub async fn move_positions(
        &self,
        params: MovePositionsRequest,
    ) -> RestResult<MovePositionsResponse> {
        self.send_signed_request(
            MOVE_POSITIONS_ENDPOINT,
            &params,
            EndpointType::NonMatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// REST API endpoint constant

    #[test]
    fn test_serialize_move_positions_request() {
        let req = MovePositionsRequest {
            currency: Some("BTC".to_string()),
            source_uid: 123,
            target_uid: 456,
            trades: vec![MovePositionTrade {
                instrument_name: "BTC-PERPETUAL".to_string(),
                price: Some(50000.0),
                amount: 100.0,
            }],
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("BTC-PERPETUAL"));
        assert!(json.contains("50000"));
        assert!(json.contains("100"));
    }

    #[test]
    fn test_deserialize_move_positions_response() {
        let json = r#"[
            {
                "amount": 100.0,
                "direction": "buy",
                "instrument_name": "BTC-PERPETUAL",
                "price": 50000.0,
                "source_uid": 123,
                "target_uid": 456
            }
        ]"#;
        let resp: MovePositionsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.len(), 1);
        assert_eq!(resp[0].instrument_name, "BTC-PERPETUAL");
        assert_eq!(resp[0].direction, "buy");
    }
}
