use super::RestClient;
use crate::binance::usdm::RestResult;

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

const HISTORICAL_TRADES_ENDPOINT: &str = "/fapi/v1/historicalTrades";

/// Request parameters for old trades lookup.
#[derive(Debug, Clone, Serialize, Default)]
pub struct HistoricalTradesRequest {
    /// Trading pair symbol (e.g., "BTCUSDT").
    pub symbol: Cow<'static, str>,

    /// Number of trades to return. Default 100; max 500.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,

    /// TradeId to fetch from. Default gets most recent trades.
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,
}

/// Represents a single historical trade.
#[derive(Debug, Clone, Deserialize)]
pub struct HistoricalTrade {
    /// Trade ID.
    pub id: u64,

    /// Price as a string.
    pub price: String,

    /// Quantity as a string.
    pub qty: String,

    /// Quote quantity as a string.
    #[serde(rename = "quoteQty")]
    pub quote_qty: String,

    /// Trade time (milliseconds since epoch).
    pub time: u64,

    /// True if buyer is the maker.
    #[serde(rename = "isBuyerMaker")]
    pub is_buyer_maker: bool,
}

impl RestClient {
    /// Old Trades Lookup (MARKET_DATA)
    ///
    /// Get older market historical trades.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Old-Trades-Lookup
    ///
    /// Rate limit: 20
    ///
    /// # Arguments
    /// * `params` - The historical trades request parameters
    ///
    /// # Returns
    /// Vector of historical trades
    pub async fn get_historical_trades(
        &self,
        params: HistoricalTradesRequest,
    ) -> RestResult<Vec<HistoricalTrade>> {
        self.send_public_request(
            HISTORICAL_TRADES_ENDPOINT,
            reqwest::Method::GET,
            Some(params),
            20,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_historical_trades_request_default() {
        let request = HistoricalTradesRequest::default();
        assert_eq!(request.symbol, "");
        assert_eq!(request.limit, None);
        assert_eq!(request.from_id, None);
    }

    #[test]
    fn test_historical_trades_request_serialization() {
        let request = HistoricalTradesRequest {
            symbol: "BTCUSDT".into(),
            limit: Some(100),
            from_id: Some(12345),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"symbol\":\"BTCUSDT\""));
        assert!(json.contains("\"limit\":100"));
        assert!(json.contains("\"fromId\":12345"));
    }

    #[test]
    fn test_historical_trades_request_serialization_with_optional_fields() {
        let request = HistoricalTradesRequest {
            symbol: "ETHUSDT".into(),
            limit: None,
            from_id: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"symbol\":\"ETHUSDT\""));
        assert!(!json.contains("\"limit\""));
        assert!(!json.contains("\"fromId\""));
    }

    #[test]
    fn test_historical_trade_deserialization() {
        let json = r#"{
            "id": 28457,
            "price": "4.00000100",
            "qty": "12.00000000",
            "quoteQty": "8000.00",
            "time": 1499865549590,
            "isBuyerMaker": true
        }"#;

        let trade: HistoricalTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, 28457);
        assert_eq!(trade.price, "4.00000100");
        assert_eq!(trade.qty, "12.00000000");
        assert_eq!(trade.quote_qty, "8000.00");
        assert_eq!(trade.time, 1499865549590);
        assert_eq!(trade.is_buyer_maker, true);
    }

    #[test]
    fn test_historical_trade_list_deserialization() {
        let json = r#"[
            {
                "id": 28457,
                "price": "4.00000100",
                "qty": "12.00000000",
                "quoteQty": "8000.00",
                "time": 1499865549590,
                "isBuyerMaker": true
            },
            {
                "id": 28458,
                "price": "4.00000200",
                "qty": "10.00000000",
                "quoteQty": "7500.00",
                "time": 1499865549600,
                "isBuyerMaker": false
            }
        ]"#;

        let trades: Vec<HistoricalTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 2);
        assert_eq!(trades[0].id, 28457);
        assert_eq!(trades[1].id, 28458);
        assert_eq!(trades[0].is_buyer_maker, true);
        assert_eq!(trades[1].is_buyer_maker, false);
    }

    #[test]
    fn test_endpoint_constant() {
        assert_eq!(HISTORICAL_TRADES_ENDPOINT, "/fapi/v1/historicalTrades");
    }
}
