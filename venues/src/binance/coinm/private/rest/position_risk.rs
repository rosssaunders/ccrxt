// Position Risk endpoint implementation for GET /dapi/v1/positionRisk
// See: https://binance-docs.github.io/apidocs/delivery/en/>

use serde::{Deserialize, Serialize};

use crate::binance::{
    coinm::{RestResult, private::rest::client::RestClient},
    shared,
};

const POSITION_RISK_ENDPOINT: &str = "/dapi/v1/positionRisk";

/// Request parameters for position risk (GET /dapi/v1/positionRisk).
#[derive(Debug, Clone, Serialize, Default)]
pub struct PositionRiskRequest {
    /// Margin asset. Optional.
    #[serde(rename = "marginAsset", skip_serializing_if = "Option::is_none")]
    pub margin_asset: Option<String>,

    /// Trading pair. Optional.
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,

    /// The value cannot be greater than 60000. Optional.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp (milliseconds since epoch).
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Response for position risk (GET /dapi/v1/positionRisk).
#[derive(Debug, Clone, Deserialize)]
pub struct PositionRisk {
    /// Trading symbol (e.g., "BTCUSD_201225").
    pub symbol: String,
    /// Position amount.
    #[serde(rename = "positionAmt")]
    pub position_amt: String,
    /// Entry price.
    #[serde(rename = "entryPrice")]
    pub entry_price: String,
    /// Break-even price.
    #[serde(rename = "breakEvenPrice")]
    pub break_even_price: String,
    /// Mark price.
    #[serde(rename = "markPrice")]
    pub mark_price: String,
    /// Unrealized profit.
    #[serde(rename = "unRealizedProfit")]
    pub unrealized_profit: String,
    /// Liquidation price.
    #[serde(rename = "liquidationPrice")]
    pub liquidation_price: String,
    /// Leverage.
    pub leverage: String,
    /// Maximum quantity of base asset.
    #[serde(rename = "maxQty")]
    pub max_qty: String,
    /// Margin type (e.g., "cross").
    #[serde(rename = "marginType")]
    pub margin_type: String,
    /// Isolated margin.
    #[serde(rename = "isolatedMargin")]
    pub isolated_margin: String,
    /// Is auto add margin enabled.
    #[serde(rename = "isAutoAddMargin")]
    pub is_auto_add_margin: String,
    /// Position side ("BOTH", "LONG", "SHORT").
    #[serde(rename = "positionSide")]
    pub position_side: String,
    /// Update time (milliseconds since epoch).
    #[serde(rename = "updateTime")]
    pub update_time: u64,
}

impl RestClient {
    /// Get current account position risk information on Binance Coin-M Futures.
    ///
    /// See: <https://binance-docs.github.io/apidocs/delivery/en/>
    /// GET /dapi/v1/positionRisk
    /// Weight: 1
    /// Requires API key and signature.
    pub async fn get_position_risk(
        &self,
        params: PositionRiskRequest,
    ) -> RestResult<Vec<PositionRisk>> {
        let weight = 1;
        shared::send_signed_request(
            self,
            POSITION_RISK_ENDPOINT,
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
    fn test_position_risk_request_serialization() {
        let request = PositionRiskRequest {
            margin_asset: Some("BTC".to_string()),
            pair: Some("BTCUSD".to_string()),
            recv_window: None,
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("marginAsset=BTC"));
        assert!(serialized.contains("pair=BTCUSD"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(!serialized.contains("recvWindow"));
    }

    #[test]
    fn test_position_risk_request_minimal() {
        let request = PositionRiskRequest {
            margin_asset: None,
            pair: None,
            recv_window: None,
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "timestamp=1625097600000");
    }

    #[test]
    fn test_position_risk_request_with_recv_window() {
        let request = PositionRiskRequest {
            margin_asset: None,
            pair: None,
            recv_window: Some(5000),
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1625097600000"));
    }

    #[test]
    fn test_position_risk_response_deserialization() {
        let json = r#"[
            {
                "symbol": "BTCUSD_PERP",
                "positionAmt": "10.0",
                "entryPrice": "45000.0",
                "breakEvenPrice": "45000.0",
                "markPrice": "45100.0",
                "unRealizedProfit": "0.00222",
                "liquidationPrice": "40000.0",
                "leverage": "20",
                "maxQty": "1000.0",
                "marginType": "cross",
                "isolatedMargin": "0.0",
                "isAutoAddMargin": "false",
                "positionSide": "LONG",
                "updateTime": 1625097600000
            }
        ]"#;

        let response: Vec<PositionRisk> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);

        let position = &response[0];
        assert_eq!(position.symbol, "BTCUSD_PERP");
        assert_eq!(position.position_amt, "10.0");
        assert_eq!(position.entry_price, "45000.0");
        assert_eq!(position.break_even_price, "45000.0");
        assert_eq!(position.mark_price, "45100.0");
        assert_eq!(position.unrealized_profit, "0.00222");
        assert_eq!(position.liquidation_price, "40000.0");
        assert_eq!(position.leverage, "20");
        assert_eq!(position.max_qty, "1000.0");
        assert_eq!(position.margin_type, "cross");
        assert_eq!(position.isolated_margin, "0.0");
        assert_eq!(position.is_auto_add_margin, "false");
        assert_eq!(position.position_side, "LONG");
        assert_eq!(position.update_time, 1625097600000);
    }

    #[test]
    fn test_multiple_positions_response() {
        let json = r#"[
            {
                "symbol": "BTCUSD_PERP",
                "positionAmt": "10.0",
                "entryPrice": "45000.0",
                "breakEvenPrice": "45000.0",
                "markPrice": "45100.0",
                "unRealizedProfit": "0.00222",
                "liquidationPrice": "40000.0",
                "leverage": "20",
                "maxQty": "1000.0",
                "marginType": "cross",
                "isolatedMargin": "0.0",
                "isAutoAddMargin": "false",
                "positionSide": "LONG",
                "updateTime": 1625097600000
            },
            {
                "symbol": "ETHUSD_PERP",
                "positionAmt": "-5.0",
                "entryPrice": "3000.0",
                "breakEvenPrice": "3000.0",
                "markPrice": "2990.0",
                "unRealizedProfit": "0.00167",
                "liquidationPrice": "3500.0",
                "leverage": "10",
                "maxQty": "500.0",
                "marginType": "isolated",
                "isolatedMargin": "0.5",
                "isAutoAddMargin": "true",
                "positionSide": "SHORT",
                "updateTime": 1625097700000
            }
        ]"#;

        let response: Vec<PositionRisk> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 2);

        assert_eq!(response[0].symbol, "BTCUSD_PERP");
        assert_eq!(response[0].position_amt, "10.0");
        assert_eq!(response[0].position_side, "LONG");

        assert_eq!(response[1].symbol, "ETHUSD_PERP");
        assert_eq!(response[1].position_amt, "-5.0");
        assert_eq!(response[1].position_side, "SHORT");
        assert_eq!(response[1].margin_type, "isolated");
        assert_eq!(response[1].is_auto_add_margin, "true");
    }

    #[test]
    fn test_empty_position_risk_response() {
        let json = r#"[]"#;
        let response: Vec<PositionRisk> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 0);
    }
}
