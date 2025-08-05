use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::okx::{EndpointType, InstrumentType, RestResult};

const ACCOUNT_POSITIONS_ENDPOINT: &str = "api/v5/account/positions";

/// Request to get account positions
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionsRequest {
    /// Instrument type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<InstrumentType>,

    /// Instrument ID, e.g. "BTC-USDT-SWAP"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,

    /// Position ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_id: Option<String>,
}

/// Position details
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    /// Instrument type
    pub inst_type: String,

    /// Margin mode
    pub mgn_mode: String,

    /// Position ID
    pub pos_id: String,

    /// Position side
    pub pos_side: String,

    /// Quantity of positions
    pub pos: String,

    /// Base currency
    pub base_ccy: Option<String>,

    /// Quote currency
    pub quote_ccy: Option<String>,

    /// Position currency
    pub pos_ccy: String,

    /// Average cost of position
    pub avg_px: String,

    /// Unrealized profit and loss
    pub upl: String,

    /// Unrealized profit and loss ratio
    pub upl_ratio: String,

    /// Unrealized profit and loss of last mark price
    pub upl_last_px: Option<String>,

    /// Unrealized profit and loss ratio of last mark price
    pub upl_ratio_last_px: Option<String>,

    /// Instrument ID
    pub inst_id: String,

    /// Leverage
    pub lever: String,

    /// Liquidation price
    pub liq_px: Option<String>,

    /// Mark price
    pub mark_px: String,

    /// Initial margin requirement
    pub imr: String,

    /// Margin
    pub margin: String,

    /// Margin ratio
    pub mgn_ratio: String,

    /// Maintenance margin requirement
    pub mmr: String,

    /// Liability
    pub liab: Option<String>,

    /// Liability currency
    pub liab_ccy: Option<String>,

    /// Interest
    pub interest: String,

    /// Last trade ID
    pub trade_id: String,

    /// Options value
    pub opt_val: Option<String>,

    /// Pending close order quantity
    pub pending_close_ord_liab_val: Option<String>,

    /// Notional value of currency (option)
    pub notional_ccy: Option<String>,

    /// Notional value of USD
    pub notional_usd: String,

    /// Auto-decrease line
    pub adl: String,

    /// Currency
    pub ccy: String,

    /// Last price
    pub last: String,

    /// Index price
    pub idx_px: Option<String>,

    /// USD price
    pub usd_px: Option<String>,

    /// TWAP (time-weighted average price)
    pub b_twap: Option<String>,

    /// TWAP (time-weighted average price) ratio
    pub b_twap_ratio: Option<String>,

    /// Mark price when the position was opened
    pub mark_px_open: Option<String>,

    /// Position creation time
    pub c_time: String,

    /// Position update time
    pub u_time: String,

    /// Realized profit and loss
    pub realized_pnl: String,

    /// P&L ratio
    pub pnl_ratio: String,

    /// Fee
    pub fee: String,

    /// Funding fee
    pub funding_fee: String,

    /// Liquidation fee
    pub liq_penalty: String,

    /// Close order algorithm
    pub close_order_algo: Vec<CloseOrderAlgo>,

    /// Whether the position has TP/SL orders
    pub biz_ref_id: Option<String>,

    /// Whether the position has TP/SL orders
    pub biz_ref_type: Option<String>,
}

/// Close order algorithm details
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseOrderAlgo {
    /// Algorithm order ID
    pub algo_id: String,

    /// Stop loss price
    pub sl_trigger_px: Option<String>,

    /// Stop loss order price
    pub sl_ord_px: Option<String>,

    /// Take profit price
    pub tp_trigger_px: Option<String>,

    /// Take profit order price
    pub tp_ord_px: Option<String>,

    /// Close position type
    pub close_pos_type: Option<String>,
}

impl RestClient {
    /// Get positions
    ///
    /// Retrieve information on your current positions. When there are no positions, an empty array will be returned.
    ///
    /// [docs]: https://www.okx.com/docs-v5/en/#rest-api-account-rest-api-get-positions
    ///
    /// Rate limit: 10 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The get positions request
    ///
    /// # Returns
    /// A result containing the positions or an error
    pub async fn get_positions(&self, request: &GetPositionsRequest) -> RestResult<Position> {
        self.send_get_request(
            ACCOUNT_POSITIONS_ENDPOINT,
            Some(request),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::okx::response::OkxApiResponse;

    #[test]
    fn test_get_positions_request_serialization() {
        let request = GetPositionsRequest {
            inst_type: Some(InstrumentType::Swap),
            inst_id: Some("BTC-USDT-SWAP".to_string()),
            pos_id: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("instType=SWAP"));
        assert!(serialized.contains("instId=BTC-USDT-SWAP"));
    }

    #[test]
    fn test_get_positions_minimal_request() {
        let request = GetPositionsRequest {
            inst_type: None,
            inst_id: None,
            pos_id: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_position_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instType": "SWAP",
                    "mgnMode": "isolated",
                    "posId": "1111111111",
                    "posSide": "long",
                    "pos": "1",
                    "baseCcy": "",
                    "quoteCcy": "",
                    "posCcy": "BTC",
                    "avgPx": "50000",
                    "upl": "1000",
                    "uplRatio": "0.02",
                    "uplLastPx": "",
                    "uplRatioLastPx": "",
                    "instId": "BTC-USDT-SWAP",
                    "lever": "10",
                    "liqPx": "45000",
                    "markPx": "51000",
                    "imr": "5000",
                    "margin": "5000",
                    "mgnRatio": "0.1",
                    "mmr": "500",
                    "liab": "",
                    "liabCcy": "",
                    "interest": "0",
                    "tradeId": "123456",
                    "optVal": "",
                    "pendingCloseOrdLiabVal": "",
                    "notionalCcy": "",
                    "notionalUsd": "51000",
                    "adl": "1",
                    "ccy": "USDT",
                    "last": "51000",
                    "idxPx": "50900",
                    "usdPx": "",
                    "bTwap": "",
                    "bTwapRatio": "",
                    "markPxOpen": "",
                    "cTime": "1597026383085",
                    "uTime": "1597026383085",
                    "realizedPnl": "0",
                    "pnlRatio": "0.02",
                    "fee": "-10",
                    "fundingFee": "-5",
                    "liqPenalty": "0",
                    "closeOrderAlgo": [],
                    "bizRefId": "",
                    "bizRefType": ""
                }
            ]
        }"#;

        let response: OkxApiResponse<Position> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let position = &response.data[0];
        assert_eq!(position.inst_id, "BTC-USDT-SWAP");
        assert_eq!(position.pos_side, "long");
        assert_eq!(position.pos, "1");
        assert_eq!(position.avg_px, "50000");
        assert_eq!(position.upl, "1000");
        assert_eq!(position.lever, "10");
    }
}
