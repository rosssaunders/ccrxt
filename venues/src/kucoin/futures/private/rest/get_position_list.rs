use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

/// Get position list request
#[derive(Debug, Clone, Serialize)]
pub struct GetPositionListRequest {
    /// Quote currency code (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

/// Position information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionInfo {
    /// Position ID
    pub id: String,
    /// Contract symbol
    pub symbol: String,
    /// Margin mode
    pub margin_mode: String,
    /// Auto deposit status
    pub auto_deposit: bool,
    /// Maintenance margin requirement
    pub maint_margin_req: String,
    /// Risk limit
    pub risk_limit: String,
    /// Real leverage
    pub real_leverage: String,
    /// Cross mode flag
    pub cross_mode: bool,
    /// Deleveraging rank percentile
    pub deleverage_rank: Option<String>,
    /// Open timestamp
    pub open_timestamp: i64,
    /// Current quantity
    pub current_qty: String,
    /// Current cost
    pub current_cost: String,
    /// Current commission
    pub current_comm: String,
    /// Unrealized cost
    pub unrealised_cost: String,
    /// Realized gross PnL
    pub realised_gross_pnl: String,
    /// Realized PnL
    pub realised_pnl: String,
    /// Unrealized PnL
    pub unrealised_pnl: String,
    /// Unrealized percentage PnL
    pub unrealised_pnl_pcnt: String,
    /// Unrealized ROE
    pub unrealised_roe_pcnt: String,
    /// Average entry price
    pub avg_entry_price: String,
    /// Liquidation price
    pub liquidation_price: String,
    /// Bankruptcy price
    pub bankruptcy_price: String,
    /// Settlement currency
    pub settle_currency: String,
    /// Maintain margin
    pub maintain_margin: String,
    /// Risk size
    pub risk_size: String,
}

/// Response for getting position list
pub type GetPositionListResponse = Vec<PositionInfo>;

impl super::RestClient {
    /// Get position list
    ///
    /// <https://www.kucoin.com/docs-new/rest/futures-trading/positions/get-position-list>
    pub async fn get_position_list(
        &self,
        request: GetPositionListRequest,
    ) -> Result<(RestResponse<GetPositionListResponse>, ResponseHeaders)> {
        const GET_POSITION_LIST_ENDPOINT: &str = "/api/v1/positions";

        let params = if let Some(currency) = request.currency {
            let mut params = std::collections::HashMap::new();
            params.insert("currency".to_string(), currency);
            Some(params)
        } else {
            None
        };

        self.get(GET_POSITION_LIST_ENDPOINT, params.as_ref()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_position_list_request_creation() {
        let request = GetPositionListRequest {
            currency: Some("USDT".to_string()),
        };
        assert_eq!(request.currency, Some("USDT".to_string()));
    }

    #[test]
    fn test_get_position_list_request_without_currency() {
        let request = GetPositionListRequest { currency: None };
        assert!(request.currency.is_none());
    }

    #[test]
    fn test_position_info_deserialization() {
        let json = r#"{
            "id": "615ba79f5f7a3a001b9bafde",
            "symbol": "XBTUSDTM",
            "marginMode": "ISOLATED",
            "autoDeposit": false,
            "maintMarginReq": "0.005",
            "riskLimit": "200000",
            "realLeverage": "9.47",
            "crossMode": false,
            "deleverageRank": null,
            "openTimestamp": 1633372575000,
            "currentQty": "10",
            "currentCost": "527.99",
            "currentComm": "0.3168",
            "unrealisedCost": "527.99",
            "realisedGrossPnl": "0",
            "realisedPnl": "-0.3168",
            "unrealisedPnl": "72.01",
            "unrealisedPnlPcnt": "0.1364",
            "unrealisedRoePcnt": "0.1364",
            "avgEntryPrice": "52799",
            "liquidationPrice": "47234.5",
            "bankruptcyPrice": "47079.24",
            "settleCurrency": "USDT",
            "maintainMargin": "3.61215",
            "riskSize": "10"
        }"#;

        let position: PositionInfo = serde_json::from_str(json).unwrap();
        assert_eq!(position.id, "615ba79f5f7a3a001b9bafde");
        assert_eq!(position.symbol, "XBTUSDTM");
        assert_eq!(position.margin_mode, "ISOLATED");
        assert_eq!(position.current_qty, "10");
        assert_eq!(position.avg_entry_price, "52799");
        assert_eq!(position.settle_currency, "USDT");
    }
}
