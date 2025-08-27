use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, OrderSide, RestResult, public_client::RestClient};

/// Endpoint URL for getting public multi-leg block trades
const GET_PUBLIC_TRADES_ENDPOINT: &str = "api/v5/rfq/public-trades";

/// Request parameters for getting public block trades
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPublicTradesRequest {
    /// The starting blockTdId to begin with.
    /// Pagination of data to return records newer than the requested blockTdId, not including beginId.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_id: Option<String>,

    /// The last blockTdId to end with.
    /// Pagination of data to return records earlier than the requested blockTdId, not including endId.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_id: Option<String>,

    /// Number of results per request. The maximum is 100 which is also the default value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Trade leg information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeLeg {
    /// Instrument ID (e.g., "BTC-USDT-SWAP")
    #[serde(rename = "instId")]
    pub inst_id: String,

    /// The price the leg executed
    pub px: String,

    /// Trade quantity
    /// For spot trading, the unit is base currency
    /// For FUTURES/SWAP/OPTION, the unit is contract
    pub sz: String,

    /// The direction of the leg from the Takers perspective
    pub side: OrderSide,

    /// Trade ID
    #[serde(rename = "tradeId")]
    pub trade_id: String,
}

/// Public block trade information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicTrade {
    /// Option strategy (e.g., "CALL_CALENDAR_SPREAD")
    pub strategy: String,

    /// The time the trade was executed (Unix timestamp in milliseconds)
    #[serde(rename = "cTime")]
    pub c_time: String,

    /// Block trade ID
    #[serde(rename = "blockTdId")]
    pub block_td_id: String,

    /// Legs of trade
    pub legs: Vec<TradeLeg>,
}

impl RestClient {
    /// Get public multi-leg transactions of block trades
    ///
    /// Retrieve the executed block trades. The data will be updated 15 minutes
    /// after the block trade execution.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#block-trading-rest-api-get-public-multi-leg-transactions-of-block-trades)
    ///
    /// Rate limit: 5 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The public trades request parameters
    ///
    /// # Returns
    /// Response containing public block trade data
    pub async fn get_public_trades(
        &self,
        request: GetPublicTradesRequest,
    ) -> RestResult<PublicTrade> {
        self.send_get_request(
            GET_PUBLIC_TRADES_ENDPOINT,
            Some(&request),
            EndpointType::PublicMarketData,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::okx::response::ApiResponse;

    #[test]
    fn test_get_public_trades_request_serialization() {
        let request = GetPublicTradesRequest {
            begin_id: Some("12345".to_string()),
            end_id: None,
            limit: Some("50".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("beginId").and_then(|v| v.as_str()),
            Some("12345")
        );
        assert_eq!(serialized.get("limit").and_then(|v| v.as_str()), Some("50"));
        assert!(serialized.get("endId").is_none());
    }

    #[test]
    fn test_trade_leg_deserialization() {
        let leg_json = json!({
            "instId": "BTC-USDT-SWAP",
            "px": "50000.0",
            "sz": "10",
            "side": "buy",
            "tradeId": "12345"
        });

        let leg: TradeLeg = serde_json::from_value(leg_json).unwrap();
        assert_eq!(leg.inst_id, "BTC-USDT-SWAP");
        assert_eq!(leg.px, "50000.0");
        assert_eq!(leg.sz, "10");
        assert_eq!(leg.side, OrderSide::Buy);
        assert_eq!(leg.trade_id, "12345");
    }

    #[test]
    fn test_public_trade_deserialization() {
        let trade_json = json!({
            "strategy": "CALL_CALENDAR_SPREAD",
            "cTime": "1597026383085",
            "blockTdId": "block123",
            "legs": [
                {
                    "instId": "BTC-USDT-SWAP",
                    "px": "50000.0",
                    "sz": "10",
                    "side": "buy",
                    "tradeId": "12345"
                }
            ]
        });

        let trade: PublicTrade = serde_json::from_value(trade_json).unwrap();
        assert_eq!(trade.strategy, "CALL_CALENDAR_SPREAD");
        assert_eq!(trade.c_time, "1597026383085");
        assert_eq!(trade.block_td_id, "block123");
        assert_eq!(trade.legs.len(), 1);
        assert_eq!(trade.legs[0].inst_id, "BTC-USDT-SWAP");
    }

    #[test]
    fn test_get_public_trades_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "strategy": "CALL_CALENDAR_SPREAD",
                    "cTime": "1597026383085",
                    "blockTdId": "block123",
                    "legs": [
                        {
                            "instId": "BTC-USDT-SWAP",
                            "px": "50000.0",
                            "sz": "10",
                            "side": "buy",
                            "tradeId": "12345"
                        }
                    ]
                }
            ]
        });

        let response: ApiResponse<PublicTrade> = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].strategy, "CALL_CALENDAR_SPREAD");
        assert_eq!(response.data[0].legs.len(), 1);
    }

    #[test]
    fn test_get_public_trades_request_minimal() {
        let request = GetPublicTradesRequest {
            begin_id: None,
            end_id: None,
            limit: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert!(serialized.get("beginId").is_none());
        assert!(serialized.get("endId").is_none());
        assert!(serialized.get("limit").is_none());
        // Should serialize to empty object
        assert_eq!(serialized, json!({}));
    }

    #[test]
    fn test_public_trade_serialization_roundtrip() {
        let original = PublicTrade {
            strategy: "PUT_SPREAD".to_string(),
            c_time: "1597026383085".to_string(),
            block_td_id: "block456".to_string(),
            legs: vec![TradeLeg {
                inst_id: "ETH-USDT".to_string(),
                px: "3000.0".to_string(),
                sz: "5".to_string(),
                side: OrderSide::Sell,
                trade_id: "67890".to_string(),
            }],
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: PublicTrade = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.strategy, deserialized.strategy);
        assert_eq!(original.c_time, deserialized.c_time);
        assert_eq!(original.block_td_id, deserialized.block_td_id);
        assert_eq!(original.legs.len(), deserialized.legs.len());
        assert_eq!(original.legs[0].inst_id, deserialized.legs[0].inst_id);
    }
}
