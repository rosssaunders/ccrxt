use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

const GET_SPREAD_TRADES_ENDPOINT: &str = "/api/v5/sprd/trades";

/// Request parameters for getting spread trades
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSpreadTradesRequest {
    /// Spread ID
    #[serde(rename = "sprdId", skip_serializing_if = "Option::is_none")]
    pub sprd_id: Option<String>,

    /// Trade ID
    #[serde(rename = "tradeId", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,

    /// Order ID
    #[serde(rename = "ordId", skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,

    /// Start trade ID the request to begin with
    /// Pagination of data to return records newer than the requested tradeId, not including beginId
    #[serde(rename = "beginId", skip_serializing_if = "Option::is_none")]
    pub begin_id: Option<String>,

    /// End trade ID the request to end with
    /// Pagination of data to return records earlier than the requested tradeId, not including endId
    #[serde(rename = "endId", skip_serializing_if = "Option::is_none")]
    pub end_id: Option<String>,

    /// Filter with a begin timestamp
    /// Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(rename = "begin", skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,

    /// Filter with an end timestamp
    /// Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,

    /// Number of results per request. The maximum is 100. The default is 100
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Spread trade leg information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpreadTradeLeg {
    /// Instrument ID, e.g. BTC-USDT-SWAP
    #[serde(rename = "instId")]
    pub inst_id: String,

    /// The price the leg executed
    #[serde(rename = "px")]
    pub px: String,

    /// The size of each leg
    #[serde(rename = "sz")]
    pub sz: String,

    /// Filled amount of the contract
    /// Only applicable to contracts, return "" for spot
    #[serde(rename = "szCont")]
    pub sz_cont: String,

    /// The direction of the leg. Valid value can be buy or sell.
    #[serde(rename = "side")]
    pub side: String,

    /// Last filled profit and loss, applicable to orders which have a trade and aim to close position
    /// It always is 0 in other conditions
    #[serde(rename = "fillPnl")]
    pub fill_pnl: String,

    /// Fee. Negative number represents the user transaction fee charged by the platform.
    /// Positive number represents rebate.
    #[serde(rename = "fee")]
    pub fee: String,

    /// Fee currency
    #[serde(rename = "feeCcy")]
    pub fee_ccy: String,

    /// Traded ID in the OKX orderbook
    #[serde(rename = "tradeId")]
    pub trade_id: String,
}

/// Response data for getting spread trades
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpreadTradeData {
    /// Spread ID
    #[serde(rename = "sprdId")]
    pub sprd_id: String,

    /// Trade ID
    #[serde(rename = "tradeId")]
    pub trade_id: String,

    /// Order ID
    #[serde(rename = "ordId")]
    pub ord_id: String,

    /// Client Order ID as assigned by the client
    #[serde(rename = "clOrdId")]
    pub cl_ord_id: String,

    /// Order tag
    #[serde(rename = "tag")]
    pub tag: String,

    /// Filled price
    #[serde(rename = "fillPx")]
    pub fill_px: String,

    /// Filled quantity
    #[serde(rename = "fillSz")]
    pub fill_sz: String,

    /// Order side, buy sell
    #[serde(rename = "side")]
    pub side: String,

    /// Trade state. Valid values are filled and rejected
    #[serde(rename = "state")]
    pub state: String,

    /// Liquidity taker or maker, T: taker M: maker
    #[serde(rename = "execType")]
    pub exec_type: String,

    /// Data generation time, Unix timestamp format in milliseconds
    #[serde(rename = "ts")]
    pub ts: String,

    /// Legs of trade
    #[serde(rename = "legs")]
    pub legs: Vec<SpreadTradeLeg>,

    /// Error Code, the default is 0
    #[serde(rename = "code")]
    pub code: String,

    /// Error Message, the default is ""
    #[serde(rename = "msg")]
    pub msg: String,
}

impl RestClient {
    /// Get spread trades
    ///
    /// Retrieve spread trades for the current account
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#spread-trading-rest-api-get-trades)
    pub async fn get_spread_trades(
        &self,
        request: Option<GetSpreadTradesRequest>,
    ) -> RestResult<SpreadTradeData> {
        self.send_get_request(
            GET_SPREAD_TRADES_ENDPOINT,
            request.as_ref(),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_get_spread_trades_request_full() {
        let request = GetSpreadTradesRequest {
            sprd_id: Some("BTC-USDT_BTC-USDT-SWAP".to_string()),
            trade_id: Some("123456789".to_string()),
            ord_id: Some("312269865356374016".to_string()),
            begin_id: Some("123456788".to_string()),
            end_id: Some("123456790".to_string()),
            begin: Some("1597026383085".to_string()),
            end: Some("1597112783085".to_string()),
            limit: Some("50".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GetSpreadTradesRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_get_spread_trades_request_minimal() {
        let request = GetSpreadTradesRequest {
            sprd_id: None,
            trade_id: None,
            ord_id: None,
            begin_id: None,
            end_id: None,
            begin: None,
            end: None,
            limit: None,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert_eq!(serialized, "{}");
    }

    #[test]
    fn test_get_spread_trades_request_none() {
        let request: Option<GetSpreadTradesRequest> = None;
        assert!(request.is_none());
    }

    #[test]
    fn test_spread_trade_leg_deserialization() {
        let json_response = r#"{
            "instId": "BTC-USDT-SWAP",
            "px": "50000",
            "sz": "1",
            "szCont": "100",
            "side": "buy",
            "fillPnl": "0",
            "fee": "-2.5",
            "feeCcy": "USDT",
            "tradeId": "987654321"
        }"#;

        let leg: SpreadTradeLeg = serde_json::from_str(json_response).unwrap();
        assert_eq!(leg.inst_id, "BTC-USDT-SWAP");
        assert_eq!(leg.px, "50000");
        assert_eq!(leg.sz, "1");
        assert_eq!(leg.sz_cont, "100");
        assert_eq!(leg.side, "buy");
        assert_eq!(leg.fee, "-2.5");
        assert_eq!(leg.fee_ccy, "USDT");
    }

    #[test]
    fn test_spread_trade_data_deserialization() {
        let json_response = r#"{
            "sprdId": "BTC-USDT_BTC-USDT-SWAP",
            "tradeId": "123456789",
            "ordId": "312269865356374016",
            "clOrdId": "client123",
            "tag": "",
            "fillPx": "50",
            "fillSz": "1",
            "side": "buy",
            "state": "filled",
            "execType": "T",
            "ts": "1597026383085",
            "legs": [
                {
                    "instId": "BTC-USDT",
                    "px": "50000",
                    "sz": "1",
                    "szCont": "",
                    "side": "buy",
                    "fillPnl": "0",
                    "fee": "-2.5",
                    "feeCcy": "USDT",
                    "tradeId": "987654321"
                },
                {
                    "instId": "BTC-USDT-SWAP",
                    "px": "49950",
                    "sz": "1",
                    "szCont": "100",
                    "side": "sell",
                    "fillPnl": "0",
                    "fee": "-2.5",
                    "feeCcy": "USDT",
                    "tradeId": "987654322"
                }
            ],
            "code": "0",
            "msg": ""
        }"#;

        let trade: SpreadTradeData = serde_json::from_str(json_response).unwrap();
        assert_eq!(trade.sprd_id, "BTC-USDT_BTC-USDT-SWAP");
        assert_eq!(trade.trade_id, "123456789");
        assert_eq!(trade.ord_id, "312269865356374016");
        assert_eq!(trade.state, "filled");
        assert_eq!(trade.exec_type, "T");
        assert_eq!(trade.legs.len(), 2);
        assert_eq!(trade.legs[0].inst_id, "BTC-USDT");
        assert_eq!(trade.legs[1].inst_id, "BTC-USDT-SWAP");
        assert_eq!(trade.code, "0");
    }

    #[test]
    fn test_spread_trade_data_serialization() {
        let leg = SpreadTradeLeg {
            inst_id: "BTC-USDT".to_string(),
            px: "50000".to_string(),
            sz: "1".to_string(),
            sz_cont: "".to_string(),
            side: "buy".to_string(),
            fill_pnl: "0".to_string(),
            fee: "-2.5".to_string(),
            fee_ccy: "USDT".to_string(),
            trade_id: "987654321".to_string(),
        };

        let trade = SpreadTradeData {
            sprd_id: "BTC-USDT_BTC-USDT-SWAP".to_string(),
            trade_id: "123456789".to_string(),
            ord_id: "312269865356374016".to_string(),
            cl_ord_id: "client123".to_string(),
            tag: "".to_string(),
            fill_px: "50".to_string(),
            fill_sz: "1".to_string(),
            side: "buy".to_string(),
            state: "filled".to_string(),
            exec_type: "T".to_string(),
            ts: "1597026383085".to_string(),
            legs: vec![leg],
            code: "0".to_string(),
            msg: "".to_string(),
        };

        let serialized = serde_json::to_string(&trade).unwrap();
        let deserialized: SpreadTradeData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(trade, deserialized);
    }

    #[test]
    fn test_trade_states() {
        let states = vec!["filled", "rejected"];

        for state in states {
            let json = format!(
                r#"{{
                "sprdId": "BTC-USDT_BTC-USDT-SWAP",
                "tradeId": "123456789",
                "ordId": "312269865356374016",
                "clOrdId": "client123",
                "tag": "",
                "fillPx": "50",
                "fillSz": "1",
                "side": "buy",
                "state": "{}",
                "execType": "T",
                "ts": "1597026383085",
                "legs": [],
                "code": "0",
                "msg": ""
            }}"#,
                state
            );

            let trade: SpreadTradeData = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.state, state);
        }
    }

    #[test]
    fn test_exec_types() {
        let exec_types = vec!["T", "M"];

        for exec_type in exec_types {
            let json = format!(
                r#"{{
                "sprdId": "BTC-USDT_BTC-USDT-SWAP",
                "tradeId": "123456789",
                "ordId": "312269865356374016",
                "clOrdId": "client123",
                "tag": "",
                "fillPx": "50",
                "fillSz": "1",
                "side": "buy",
                "state": "filled",
                "execType": "{}",
                "ts": "1597026383085",
                "legs": [],
                "code": "0",
                "msg": ""
            }}"#,
                exec_type
            );

            let trade: SpreadTradeData = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.exec_type, exec_type);
        }
    }
}
