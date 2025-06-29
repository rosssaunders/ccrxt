use serde::{Deserialize, Serialize};

use crate::bybit::{enums::*, EndpointType, RestResult};

use super::client::RestClient;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInstrumentsInfoRequest {
    pub category: Category,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentInfo {
    pub symbol: String,
    pub contract_type: Option<String>,
    pub status: String,
    pub base_coin: String,
    pub quote_coin: String,
    pub launch_time: String,
    pub delivery_time: Option<String>,
    pub delivery_fee_rate: Option<String>,
    pub price_scale: String,
    pub leverage_filter: LeverageFilter,
    pub price_filter: PriceFilter,
    pub lot_size_filter: LotSizeFilter,
    pub unified_margin_trade: Option<bool>,
    pub funding_interval: Option<i32>,
    pub settle_coin: Option<String>,
    pub copy_trading: Option<String>,
    pub upper_funding_rate: Option<String>,
    pub lower_funding_rate: Option<String>,
    pub is_pre_listing: Option<bool>,
    pub pre_listing_info: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeverageFilter {
    pub min_leverage: String,
    pub max_leverage: String,
    pub leverage_step: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceFilter {
    pub min_price: String,
    pub max_price: String,
    pub tick_size: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LotSizeFilter {
    pub max_order_qty: String,
    pub max_market_order_qty: Option<String>,
    pub min_order_qty: String,
    pub qty_step: String,
    pub post_only_max_order_qty: Option<String>,
    pub min_notional_value: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetInstrumentsInfoData {
    pub category: Category,
    pub list: Vec<InstrumentInfo>,
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetInstrumentsInfoResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: GetInstrumentsInfoData,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl RestClient {
    /// Get instruments info
    ///
    /// Query for the instrument specification of online trading pairs.
    ///
    /// # Arguments
    /// * `request` - The instruments info request parameters
    ///
    /// # Returns
    /// A result containing the instruments info response or an error
    pub async fn get_instruments_info(
        &self,
        request: GetInstrumentsInfoRequest,
    ) -> RestResult<GetInstrumentsInfoResponse> {
        self.send_public_request(
            "/v5/market/instruments-info",
            Some(&request),
            EndpointType::Market,
        )
        .await
    }
}

impl GetInstrumentsInfoRequest {
    /// Create a new instruments info request
    pub fn new(category: Category) -> Self {
        Self {
            category,
            symbol: None,
            base_coin: None,
            limit: None,
            cursor: None,
        }
    }

    /// Filter by symbol
    pub fn symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    /// Filter by base coin (option only)
    pub fn base_coin(mut self, base_coin: String) -> Self {
        self.base_coin = Some(base_coin);
        self
    }

    /// Set limit (1-1000, default: 500)
    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set pagination cursor
    pub fn cursor(mut self, cursor: String) -> Self {
        self.cursor = Some(cursor);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_instruments_info_request_builder() {
        let request = GetInstrumentsInfoRequest::new(Category::Linear)
            .symbol("BTCUSDT".to_string())
            .limit(100);

        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.symbol, Some("BTCUSDT".to_string()));
        assert_eq!(request.limit, Some(100));
        assert!(request.base_coin.is_none());
        assert!(request.cursor.is_none());
    }
}