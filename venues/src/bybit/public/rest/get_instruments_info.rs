use serde::{Deserialize, Serialize};

use crate::bybit::{EndpointType, PublicRestClient, RestResult, enums::*};

const INSTRUMENTS_INFO_ENDPOINT: &str = "/v5/market/instruments-info";

/// Request parameters for getting instruments info
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInstrumentsInfoRequest {
    /// Product type
    pub category: Category,

    /// Symbol name (e.g., "BTCUSDT")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Base coin. For Option only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,

    /// Limit for data size per page. [1, 1000]. Default: 500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Cursor. Use the nextPageCursor token from the response to retrieve the next page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Instrument specification information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentInfo {
    /// Symbol name
    pub symbol: String,

    /// Contract type
    pub contract_type: Option<String>,

    /// Instrument status
    pub status: String,

    /// Base coin
    pub base_coin: String,

    /// Quote coin
    pub quote_coin: String,

    /// Launch timestamp in milliseconds
    pub launch_time: String,

    /// Delivery timestamp in milliseconds. Valid for Inverse Futures
    pub delivery_time: Option<String>,

    /// Delivery fee rate. Valid for Inverse Futures
    pub delivery_fee_rate: Option<String>,

    /// Price scale
    pub price_scale: String,

    /// Leverage filter
    pub leverage_filter: LeverageFilter,

    /// Price filter
    pub price_filter: PriceFilter,

    /// Lot size filter
    pub lot_size_filter: LotSizeFilter,

    /// Whether to support unified margin trade
    pub unified_margin_trade: Option<bool>,

    /// Funding interval (minutes)
    pub funding_interval: Option<i32>,

    /// Settle coin
    pub settle_coin: Option<String>,

    /// Copy trading support ("none", "both", "copyOnly", "normalOnly")
    pub copy_trading: Option<String>,

    /// Upper funding rate
    pub upper_funding_rate: Option<String>,

    /// Lower funding rate
    pub lower_funding_rate: Option<String>,

    /// Whether the contract is in pre-listing phase
    pub is_pre_listing: Option<bool>,

    /// Pre-listing information
    pub pre_listing_info: Option<serde_json::Value>,
}

/// Leverage filter parameters
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeverageFilter {
    /// Minimum leverage
    pub min_leverage: String,

    /// Maximum leverage
    pub max_leverage: String,

    /// Leverage step
    pub leverage_step: String,
}

/// Price filter parameters
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceFilter {
    /// Minimum order price
    pub min_price: String,

    /// Maximum order price
    pub max_price: String,

    /// Tick size
    pub tick_size: String,
}

/// Lot size filter parameters
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LotSizeFilter {
    /// Maximum order quantity
    pub max_order_qty: String,

    /// Maximum market order quantity
    pub max_market_order_qty: Option<String>,

    /// Minimum order quantity
    pub min_order_qty: String,

    /// Order quantity step
    pub qty_step: String,

    /// Maximum order quantity for post-only orders
    pub post_only_max_order_qty: Option<String>,

    /// Minimum notional value
    pub min_notional_value: Option<String>,
}

/// Instruments info data container
#[derive(Debug, Clone, Deserialize)]
pub struct GetInstrumentsInfoData {
    /// Product type
    pub category: Category,

    /// Array of instrument info
    pub list: Vec<InstrumentInfo>,

    /// Cursor for pagination
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: String,
}

/// Response from the instruments info endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct GetInstrumentsInfoResponse {
    /// Success/Error code (0: success, 1: error)
    #[serde(rename = "retCode")]
    pub ret_code: i32,

    /// Success/Error message
    #[serde(rename = "retMsg")]
    pub ret_msg: String,

    /// Business data result
    pub result: GetInstrumentsInfoData,

    /// Extended information
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,

    /// Current timestamp in milliseconds
    pub time: u64,
}

impl PublicRestClient {
    /// Get instruments info
    ///
    /// Query for the instrument specification of online trading pairs.
    ///
    /// [docs](https://bybit-exchange.github.io/docs/v5/market/instrument)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The instruments info request parameters including:
    ///   - `category`: Product type
    ///   - `symbol`: Optional symbol name
    ///   - `base_coin`: Optional base coin (for Option only)
    ///   - `limit`: Optional result limit
    ///   - `cursor`: Optional pagination cursor
    ///
    /// # Returns
    /// A result containing the instruments info response with instrument specifications or an error
    pub async fn get_instruments_info(
        &self,
        request: GetInstrumentsInfoRequest,
    ) -> RestResult<GetInstrumentsInfoResponse> {
        self.send_public_request(
            INSTRUMENTS_INFO_ENDPOINT,
            Some(&request),
            EndpointType::Market,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_instruments_info_request_builder() {
        let request = GetInstrumentsInfoRequest {
            category: Category::Linear,
            symbol: Some("BTCUSDT".to_string()),
            base_coin: None,
            limit: Some(100),
            cursor: None,
        };

        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.symbol, Some("BTCUSDT".to_string()));
        assert_eq!(request.limit, Some(100));
        assert!(request.base_coin.is_none());
        assert!(request.cursor.is_none());
    }
}
