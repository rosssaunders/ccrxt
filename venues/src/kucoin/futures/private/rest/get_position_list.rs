use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

/// Endpoint URL for getting position list
const GET_POSITION_LIST_ENDPOINT: &str = "/api/v1/positions";

/// Request parameters for getting position list.
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetPositionListRequest {
    /// Quote currency code to filter positions by settlement currency.
    /// Optional field - if not provided, returns positions for all currencies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

/// Position information details.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionInfo {
    /// Unique position identifier.
    pub id: String,

    /// Contract symbol (e.g., "XBTUSDTM").
    pub symbol: String,

    /// Margin mode for the position (e.g., "ISOLATED", "CROSS").
    pub margin_mode: String,

    /// Whether automatic margin deposit is enabled.
    pub auto_deposit: bool,

    /// Maintenance margin requirement as a percentage string.
    pub maint_margin_req: String,

    /// Risk limit amount for the position as a string.
    pub risk_limit: String,

    /// Real leverage ratio as a string.
    pub real_leverage: String,

    /// Whether the position is in cross margin mode.
    pub cross_mode: bool,

    /// Current deleveraging rank percentile.
    /// Optional - may not be present if position not subject to deleveraging.
    pub deleverage_rank: Option<String>,

    /// Position opening timestamp (milliseconds since epoch).
    pub open_timestamp: i64,

    /// Current position quantity as a string.
    pub current_qty: String,

    /// Current cost of the position as a string.
    pub current_cost: String,

    /// Current commission paid as a string.
    pub current_comm: String,

    /// Unrealized cost of the position as a string.
    pub unrealised_cost: String,

    /// Realized gross profit and loss as a string.
    pub realised_gross_pnl: String,

    /// Realized profit and loss after fees as a string.
    pub realised_pnl: String,

    /// Unrealized profit and loss as a string.
    pub unrealised_pnl: String,

    /// Unrealized PnL percentage as a string.
    pub unrealised_pnl_pcnt: String,

    /// Unrealized return on equity percentage as a string.
    pub unrealised_roe_pcnt: String,

    /// Average entry price of the position as a string.
    pub avg_entry_price: String,

    /// Liquidation price threshold as a string.
    pub liquidation_price: String,

    /// Bankruptcy price level as a string.
    pub bankruptcy_price: String,

    /// Settlement currency for the position (e.g., "USDT").
    pub settle_currency: String,

    /// Maintenance margin amount as a string.
    pub maintain_margin: String,

    /// Risk size of the position as a string.
    pub risk_size: String,
}

/// Response type for position list endpoint.
pub type GetPositionListResponse = Vec<PositionInfo>;

impl super::RestClient {
    /// Get Position List
    ///
    /// Get the position list for the current user with optional currency filtering.
    ///
    /// [docs]: https://www.kucoin.com/docs-new/rest/futures-trading/positions/get-position-list
    ///
    /// Rate limit: 9
    ///
    /// # Arguments
    /// * `request` - The position list request parameters
    ///
    /// # Returns
    /// A list of position information for the current user
    pub async fn get_position_list(
        &self,
        request: GetPositionListRequest,
    ) -> Result<(RestResponse<GetPositionListResponse>, ResponseHeaders)> {
        self.get(GET_POSITION_LIST_ENDPOINT, Some(&request)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_position_list_request_default() {
        let request = GetPositionListRequest::default();
        assert!(request.currency.is_none());
    }

    #[test]
    fn test_get_position_list_request_with_currency() {
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
    fn test_request_serialization() {
        let request = GetPositionListRequest {
            currency: Some("USDT".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "USDT");
    }

    #[test]
    fn test_request_serialization_no_currency() {
        let request = GetPositionListRequest { currency: None };

        let json = serde_json::to_value(&request).unwrap();
        // currency should not be present when None
        assert!(json.get("currency").is_none());
    }

    #[test]
    fn test_currency_variations() {
        let currencies = ["USDT", "USDC", "BTC", "ETH"];

        for currency in currencies.iter() {
            let request = GetPositionListRequest {
                currency: Some(currency.to_string()),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["currency"], *currency);
        }
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
        assert_eq!(position.auto_deposit, false);
        assert_eq!(position.current_qty, "10");
        assert_eq!(position.avg_entry_price, "52799");
        assert_eq!(position.settle_currency, "USDT");
        assert!(position.deleverage_rank.is_none());
    }

    #[test]
    fn test_position_info_with_deleverage_rank() {
        let json = r#"{
            "id": "615ba79f5f7a3a001b9bafde",
            "symbol": "ETHUSDTM",
            "marginMode": "CROSS",
            "autoDeposit": true,
            "maintMarginReq": "0.003",
            "riskLimit": "500000",
            "realLeverage": "5.25",
            "crossMode": true,
            "deleverageRank": "75",
            "openTimestamp": 1633372575000,
            "currentQty": "50",
            "currentCost": "1500.00",
            "currentComm": "0.75",
            "unrealisedCost": "1500.00",
            "realisedGrossPnl": "25.00",
            "realisedPnl": "24.25",
            "unrealisedPnl": "150.00",
            "unrealisedPnlPcnt": "0.10",
            "unrealisedRoePcnt": "0.10",
            "avgEntryPrice": "3000",
            "liquidationPrice": "2500",
            "bankruptcyPrice": "2450",
            "settleCurrency": "USDT",
            "maintainMargin": "4.50",
            "riskSize": "50"
        }"#;

        let position: PositionInfo = serde_json::from_str(json).unwrap();
        assert_eq!(position.symbol, "ETHUSDTM");
        assert_eq!(position.margin_mode, "CROSS");
        assert_eq!(position.auto_deposit, true);
        assert_eq!(position.cross_mode, true);
        assert_eq!(position.deleverage_rank, Some("75".to_string()));
    }

    #[test]
    fn test_multiple_positions_deserialization() {
        let json = r#"[
            {
                "id": "pos1",
                "symbol": "XBTUSDTM",
                "marginMode": "ISOLATED",
                "autoDeposit": false,
                "maintMarginReq": "0.005",
                "riskLimit": "200000",
                "realLeverage": "10.0",
                "crossMode": false,
                "deleverageRank": null,
                "openTimestamp": 1633372575000,
                "currentQty": "100",
                "currentCost": "5000.00",
                "currentComm": "2.50",
                "unrealisedCost": "5000.00",
                "realisedGrossPnl": "0",
                "realisedPnl": "-2.50",
                "unrealisedPnl": "500.00",
                "unrealisedPnlPcnt": "0.10",
                "unrealisedRoePcnt": "0.10",
                "avgEntryPrice": "50000",
                "liquidationPrice": "45000",
                "bankruptcyPrice": "44500",
                "settleCurrency": "USDT",
                "maintainMargin": "25.00",
                "riskSize": "100"
            },
            {
                "id": "pos2",
                "symbol": "ETHUSDTM",
                "marginMode": "CROSS",
                "autoDeposit": true,
                "maintMarginReq": "0.003",
                "riskLimit": "500000",
                "realLeverage": "5.0",
                "crossMode": true,
                "deleverageRank": "50",
                "openTimestamp": 1633372600000,
                "currentQty": "200",
                "currentCost": "6000.00",
                "currentComm": "3.00",
                "unrealisedCost": "6000.00",
                "realisedGrossPnl": "100.00",
                "realisedPnl": "97.00",
                "unrealisedPnl": "300.00",
                "unrealisedPnlPcnt": "0.05",
                "unrealisedRoePcnt": "0.05",
                "avgEntryPrice": "3000",
                "liquidationPrice": "2700",
                "bankruptcyPrice": "2650",
                "settleCurrency": "USDT",
                "maintainMargin": "18.00",
                "riskSize": "200"
            }
        ]"#;

        let positions: GetPositionListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(positions.len(), 2);

        let btc_position = &positions[0];
        assert_eq!(btc_position.symbol, "XBTUSDTM");
        assert_eq!(btc_position.margin_mode, "ISOLATED");
        assert_eq!(btc_position.cross_mode, false);
        assert!(btc_position.deleverage_rank.is_none());

        let eth_position = &positions[1];
        assert_eq!(eth_position.symbol, "ETHUSDTM");
        assert_eq!(eth_position.margin_mode, "CROSS");
        assert_eq!(eth_position.cross_mode, true);
        assert_eq!(eth_position.deleverage_rank, Some("50".to_string()));
    }

    #[test]
    fn test_field_types() {
        let json = r#"{
            "id": "test_id",
            "symbol": "XBTUSDTM",
            "marginMode": "ISOLATED",
            "autoDeposit": false,
            "maintMarginReq": "0.005",
            "riskLimit": "200000",
            "realLeverage": "10.0",
            "crossMode": false,
            "deleverageRank": "25",
            "openTimestamp": 1633372575000,
            "currentQty": "100",
            "currentCost": "5000.00",
            "currentComm": "2.50",
            "unrealisedCost": "5000.00",
            "realisedGrossPnl": "0",
            "realisedPnl": "-2.50",
            "unrealisedPnl": "500.00",
            "unrealisedPnlPcnt": "0.10",
            "unrealisedRoePcnt": "0.10",
            "avgEntryPrice": "50000",
            "liquidationPrice": "45000",
            "bankruptcyPrice": "44500",
            "settleCurrency": "USDT",
            "maintainMargin": "25.00",
            "riskSize": "100"
        }"#;

        let json_value = serde_json::from_str::<serde_json::Value>(json).unwrap();

        // Verify field types in JSON
        assert!(json_value["id"].is_string());
        assert!(json_value["symbol"].is_string());
        assert!(json_value["autoDeposit"].is_boolean());
        assert!(json_value["crossMode"].is_boolean());
        assert!(json_value["openTimestamp"].is_number());
        assert!(json_value["maintMarginReq"].is_string());
        assert!(json_value["currentQty"].is_string());

        // Verify deserialization works
        let position: PositionInfo = serde_json::from_str(json).unwrap();
        assert_eq!(position.id, "test_id");
        assert_eq!(position.auto_deposit, false);
        assert_eq!(position.cross_mode, false);
        assert_eq!(position.open_timestamp, 1633372575000);
    }

    #[test]
    fn test_camel_case_conversion() {
        let json = r#"{
            "id": "test",
            "symbol": "XBTUSDTM",
            "marginMode": "ISOLATED",
            "autoDeposit": false,
            "maintMarginReq": "0.005",
            "riskLimit": "200000",
            "realLeverage": "10.0",
            "crossMode": false,
            "deleverageRank": null,
            "openTimestamp": 1633372575000,
            "currentQty": "100",
            "currentCost": "5000.00",
            "currentComm": "2.50",
            "unrealisedCost": "5000.00",
            "realisedGrossPnl": "0",
            "realisedPnl": "-2.50",
            "unrealisedPnl": "500.00",
            "unrealisedPnlPcnt": "0.10",
            "unrealisedRoePcnt": "0.10",
            "avgEntryPrice": "50000",
            "liquidationPrice": "45000",
            "bankruptcyPrice": "44500",
            "settleCurrency": "USDT",
            "maintainMargin": "25.00",
            "riskSize": "100"
        }"#;

        let position: PositionInfo = serde_json::from_str(json).unwrap();

        // Verify camelCase fields are properly converted to snake_case
        assert_eq!(position.margin_mode, "ISOLATED");
        assert_eq!(position.auto_deposit, false);
        assert_eq!(position.maint_margin_req, "0.005");
        assert_eq!(position.risk_limit, "200000");
        assert_eq!(position.real_leverage, "10.0");
        assert_eq!(position.cross_mode, false);
        assert_eq!(position.deleverage_rank, None);
        assert_eq!(position.open_timestamp, 1633372575000);
        assert_eq!(position.current_qty, "100");
        assert_eq!(position.current_cost, "5000.00");
        assert_eq!(position.current_comm, "2.50");
        assert_eq!(position.unrealised_cost, "5000.00");
        assert_eq!(position.realised_gross_pnl, "0");
        assert_eq!(position.realised_pnl, "-2.50");
        assert_eq!(position.unrealised_pnl, "500.00");
        assert_eq!(position.unrealised_pnl_pcnt, "0.10");
        assert_eq!(position.unrealised_roe_pcnt, "0.10");
        assert_eq!(position.avg_entry_price, "50000");
        assert_eq!(position.liquidation_price, "45000");
        assert_eq!(position.bankruptcy_price, "44500");
        assert_eq!(position.settle_currency, "USDT");
        assert_eq!(position.maintain_margin, "25.00");
        assert_eq!(position.risk_size, "100");
    }

    #[test]
    fn test_margin_mode_variations() {
        let modes = ["ISOLATED", "CROSS", "FIXED"];

        for mode in modes.iter() {
            let json = format!(
                r#"{{
                "id": "test",
                "symbol": "XBTUSDTM",
                "marginMode": "{}",
                "autoDeposit": false,
                "maintMarginReq": "0.005",
                "riskLimit": "200000",
                "realLeverage": "10.0",
                "crossMode": false,
                "deleverageRank": null,
                "openTimestamp": 1633372575000,
                "currentQty": "100",
                "currentCost": "5000.00",
                "currentComm": "2.50",
                "unrealisedCost": "5000.00",
                "realisedGrossPnl": "0",
                "realisedPnl": "-2.50",
                "unrealisedPnl": "500.00",
                "unrealisedPnlPcnt": "0.10",
                "unrealisedRoePcnt": "0.10",
                "avgEntryPrice": "50000",
                "liquidationPrice": "45000",
                "bankruptcyPrice": "44500",
                "settleCurrency": "USDT",
                "maintainMargin": "25.00",
                "riskSize": "100"
            }}"#,
                mode
            );

            let position: PositionInfo = serde_json::from_str(&json).unwrap();
            assert_eq!(position.margin_mode, *mode);
        }
    }

    #[test]
    fn test_empty_position_list() {
        let json = r#"[]"#;
        let positions: GetPositionListResponse = serde_json::from_str(json).unwrap();
        assert!(positions.is_empty());
    }

    #[test]
    fn test_endpoint_constant() {
        assert_eq!(GET_POSITION_LIST_ENDPOINT, "/api/v1/positions");
    }
}
