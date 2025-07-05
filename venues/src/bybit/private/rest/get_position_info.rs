use serde::{Deserialize, Serialize};

use crate::bybit::{enums::*, EndpointType, RestResult};

use super::client::RestClient;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionInfoRequest {
    pub category: Category,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionInfo {
    pub position_idx: i32,
    pub risk_id: i32,
    pub risk_limit_value: String,
    pub symbol: String,
    pub side: String,
    pub size: String,
    pub avg_price: String,
    pub position_value: String,
    pub trade_mode: i32,
    pub auto_add_margin: i32,
    pub position_status: String,
    pub leverage: String,
    pub mark_price: String,
    pub liq_price: String,
    pub bust_price: String,
    pub position_im: String,
    pub position_mm: String,
    pub position_balance: String,
    pub take_profit: String,
    pub stop_loss: String,
    pub trailing_stop: String,
    pub session_avg_price: String,
    pub delta: String,
    pub gamma: String,
    pub vega: String,
    pub theta: String,
    pub unrealised_pnl: String,
    pub cur_realised_pnl: String,
    pub cum_realised_pnl: String,
    pub adl_rank_indicator: i32,
    pub created_time: String,
    pub updated_time: String,
    pub seq: i64,
    pub is_reduce_only: bool,
    pub mmr_sys_updated_time: String,
    pub leverage_sys_updated_time: String,
    pub tpsl_mode: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionInfoData {
    pub category: Category,
    pub next_page_cursor: String,
    pub list: Vec<PositionInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetPositionInfoResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: GetPositionInfoData,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl RestClient {
    /// Get position information
    ///
    /// Get position list information for derivatives.
    ///
    /// # Arguments
    /// * `request` - The get position info request parameters
    ///
    /// # Returns
    /// A result containing the position info response or an error
    pub async fn get_position_info(
        &self,
        request: GetPositionInfoRequest,
    ) -> RestResult<GetPositionInfoResponse> {
        self.send_signed_request(
            "/v5/position/list",
            reqwest::Method::GET,
            request,
            EndpointType::Position,
        )
        .await
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_position_info_request_direct_construction() {
        let request = GetPositionInfoRequest {
            category: Category::Linear,
            symbol: Some("BTCUSDT".to_string()),
            limit: Some(50),
            base_coin: None,
            settle_coin: None,
            cursor: None,
        };

        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.symbol, Some("BTCUSDT".to_string()));
        assert_eq!(request.limit, Some(50));
        assert!(request.base_coin.is_none());
        assert!(request.settle_coin.is_none());
    }

    #[test]
    fn test_get_position_info_request_with_settle_coin() {
        let request = GetPositionInfoRequest {
            category: Category::Linear,
            settle_coin: Some("USDT".to_string()),
            limit: Some(10),
            symbol: None,
            base_coin: None,
            cursor: None,
        };

        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.settle_coin, Some("USDT".to_string()));
        assert_eq!(request.limit, Some(10));
        assert!(request.symbol.is_none());
    }

    #[test]
    fn test_get_position_info_request_serialization() {
        let request = GetPositionInfoRequest {
            category: Category::Option,
            base_coin: Some("BTC".to_string()),
            limit: Some(20),
            symbol: None,
            settle_coin: None,
            cursor: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"category\":\"option\""));
        assert!(json.contains("\"baseCoin\":\"BTC\""));
        assert!(json.contains("\"limit\":20"));
        assert!(!json.contains("symbol")); // Should be skipped when None
    }
}