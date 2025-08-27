use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, OrderSide, RestResult, public_client::RestClient};

/// Endpoint URL for getting public single-leg block trades
const GET_PUBLIC_BLOCK_TRADES_ENDPOINT: &str = "api/v5/public/block-trades";

/// Request parameters for getting public single-leg block trades
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPublicBlockTradesRequest {
    /// Instrument ID (e.g., "BTC-USDT")
    #[serde(rename = "instId")]
    pub inst_id: String,
}

/// Public single-leg block trade information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicBlockTrade {
    /// Instrument ID
    #[serde(rename = "instId")]
    pub inst_id: String,

    /// Trade ID
    #[serde(rename = "tradeId")]
    pub trade_id: String,

    /// Trade price
    pub px: String,

    /// Trade quantity
    /// For spot trading, the unit is base currency
    /// For FUTURES/SWAP/OPTION, the unit is contract
    pub sz: String,

    /// Trade side ("buy" or "sell")
    pub side: OrderSide,

    /// Implied volatility (Only applicable to OPTION)
    #[serde(rename = "fillVol", skip_serializing_if = "Option::is_none")]
    pub fill_vol: Option<String>,

    /// Forward price (Only applicable to OPTION)
    #[serde(rename = "fwdPx", skip_serializing_if = "Option::is_none")]
    pub fwd_px: Option<String>,

    /// Index price (Applicable to FUTURES, SWAP, OPTION)
    #[serde(rename = "idxPx", skip_serializing_if = "Option::is_none")]
    pub idx_px: Option<String>,

    /// Mark price (Applicable to FUTURES, SWAP, OPTION)
    #[serde(rename = "markPx", skip_serializing_if = "Option::is_none")]
    pub mark_px: Option<String>,

    /// Trade time (Unix timestamp format in milliseconds)
    pub ts: String,
}

impl RestClient {
    /// Get public single-leg transactions of block trades
    ///
    /// Retrieve the recent block trading transactions of an instrument. Descending order by tradeId.
    /// The data will be updated 15 minutes after the block trade execution.
    /// Up to 500 most recent historical public transaction data can be retrieved.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#block-trading-rest-api-get-public-single-leg-transactions-of-block-trades)
    ///
    /// Rate limit: 20 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The public block trades request parameters
    ///
    /// # Returns
    /// Response containing public single-leg block trade data
    pub async fn get_public_block_trades(
        &self,
        request: GetPublicBlockTradesRequest,
    ) -> RestResult<PublicBlockTrade> {
        self.send_get_request(
            GET_PUBLIC_BLOCK_TRADES_ENDPOINT,
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
    fn test_get_public_block_trades_request_serialization() {
        let request = GetPublicBlockTradesRequest {
            inst_id: "BTC-USDT".to_string(),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instId").and_then(|v| v.as_str()),
            Some("BTC-USDT")
        );
    }

    #[test]
    fn test_public_block_trade_deserialization() {
        let trade_json = json!({
            "instId": "BTC-USDT",
            "tradeId": "12345",
            "px": "50000.0",
            "sz": "1.5",
            "side": "buy",
            "ts": "1597026383085"
        });

        let trade: PublicBlockTrade = serde_json::from_value(trade_json).unwrap();
        assert_eq!(trade.inst_id, "BTC-USDT");
        assert_eq!(trade.trade_id, "12345");
        assert_eq!(trade.px, "50000.0");
        assert_eq!(trade.sz, "1.5");
        assert_eq!(trade.side, OrderSide::Buy);
        assert_eq!(trade.ts, "1597026383085");
        assert!(trade.fill_vol.is_none());
        assert!(trade.fwd_px.is_none());
    }

    #[test]
    fn test_public_block_trade_deserialization_with_option_fields() {
        let trade_json = json!({
            "instId": "BTC-USD-CALL-50000",
            "tradeId": "67890",
            "px": "2000.0",
            "sz": "10",
            "side": "sell",
            "fillVol": "0.65",
            "fwdPx": "51000.0",
            "idxPx": "50500.0",
            "markPx": "50300.0",
            "ts": "1597026383085"
        });

        let trade: PublicBlockTrade = serde_json::from_value(trade_json).unwrap();
        assert_eq!(trade.inst_id, "BTC-USD-CALL-50000");
        assert_eq!(trade.trade_id, "67890");
        assert_eq!(trade.px, "2000.0");
        assert_eq!(trade.sz, "10");
        assert_eq!(trade.side, OrderSide::Sell);
        assert_eq!(trade.fill_vol, Some("0.65".to_string()));
        assert_eq!(trade.fwd_px, Some("51000.0".to_string()));
        assert_eq!(trade.idx_px, Some("50500.0".to_string()));
        assert_eq!(trade.mark_px, Some("50300.0".to_string()));
        assert_eq!(trade.ts, "1597026383085");
    }

    #[test]
    fn test_get_public_block_trades_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instId": "BTC-USDT",
                    "tradeId": "12345",
                    "px": "50000.0",
                    "sz": "1.5",
                    "side": "buy",
                    "ts": "1597026383085"
                },
                {
                    "instId": "BTC-USDT",
                    "tradeId": "12346",
                    "px": "49900.0",
                    "sz": "2.0",
                    "side": "sell",
                    "ts": "1597026383086"
                }
            ]
        });

        let response: ApiResponse<PublicBlockTrade> =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data[0].inst_id, "BTC-USDT");
        assert_eq!(response.data[1].trade_id, "12346");
    }

    #[test]
    fn test_public_block_trade_serialization_roundtrip() {
        let original = PublicBlockTrade {
            inst_id: "ETH-USDT".to_string(),
            trade_id: "98765".to_string(),
            px: "3000.0".to_string(),
            sz: "5.0".to_string(),
            side: OrderSide::Sell,
            fill_vol: None,
            fwd_px: None,
            idx_px: Some("3005.0".to_string()),
            mark_px: Some("3002.0".to_string()),
            ts: "1597026383087".to_string(),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: PublicBlockTrade = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.inst_id, deserialized.inst_id);
        assert_eq!(original.trade_id, deserialized.trade_id);
        assert_eq!(original.px, deserialized.px);
        assert_eq!(original.sz, deserialized.sz);
        assert_eq!(original.side, deserialized.side);
        assert_eq!(original.fill_vol, deserialized.fill_vol);
        assert_eq!(original.fwd_px, deserialized.fwd_px);
        assert_eq!(original.idx_px, deserialized.idx_px);
        assert_eq!(original.mark_px, deserialized.mark_px);
        assert_eq!(original.ts, deserialized.ts);
    }

    #[test]
    fn test_get_public_block_trades_request_roundtrip() {
        let original = GetPublicBlockTradesRequest {
            inst_id: "BTC-USD-SWAP".to_string(),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: GetPublicBlockTradesRequest = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.inst_id, deserialized.inst_id);
    }
}
