use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::spot::RestResult;

use super::client::RestClient;

/// Request parameters for getting account trades
#[derive(Debug, Clone, Serialize)]
pub struct MyTradesRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order ID - if set, it will get trades for this order
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Start time timestamp in ms
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time timestamp in ms
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Trade ID to fetch from
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,

    /// Default 500; max 1000
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Account trade information
#[derive(Debug, Clone, Deserialize)]
pub struct MyTrade {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Trade ID
    #[serde(rename = "id")]
    pub id: u64,

    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Order list ID
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,

    /// Trade price
    #[serde(rename = "price")]
    pub price: Decimal,

    /// Trade quantity
    #[serde(rename = "qty")]
    pub qty: Decimal,

    /// Quote quantity
    #[serde(rename = "quoteQty")]
    pub quote_qty: Decimal,

    /// Commission amount
    #[serde(rename = "commission")]
    pub commission: Decimal,

    /// Commission asset
    #[serde(rename = "commissionAsset")]
    pub commission_asset: String,

    /// Trade execution time
    #[serde(rename = "time")]
    pub time: u64,

    /// Is buyer
    #[serde(rename = "isBuyer")]
    pub is_buyer: bool,

    /// Is maker
    #[serde(rename = "isMaker")]
    pub is_maker: bool,

    /// Is best match
    #[serde(rename = "isBestMatch")]
    pub is_best_match: bool,
}

impl RestClient {
    /// Get trades for a specific account and symbol
    ///
    /// Get trades for a specific account and symbol.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#account-trade-list--user_data)
    /// Method: GET /api/v3/myTrades
    /// Weight: 20 (without orderId), 5 (with orderId)
    /// Security: USER_DATA
    pub async fn get_my_trades(&self, params: MyTradesRequest) -> RestResult<Vec<MyTrade>> {
        let weight = if params.order_id.is_some() { 5 } else { 20 };

        let query_string = serde_urlencoded::to_string(&params)
            .map_err(|e| crate::binance::spot::Errors::Error(format!("URL encoding error: {e}")))?;

        self.send_request(
            "/api/v3/myTrades",
            reqwest::Method::GET,
            Some(&query_string),
            None,
            weight,
            false,
        )
        .await
    }
}
