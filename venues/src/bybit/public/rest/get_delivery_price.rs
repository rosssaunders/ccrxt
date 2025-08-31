use serde::{Deserialize, Serialize};

use crate::bybit::{EndpointType, PublicRestClient, RestResult, enums::*};

const DELIVERY_PRICE_ENDPOINT: &str = "/v5/market/delivery-price";

/// Request parameters for getting delivery price information.
#[derive(Debug, Clone, Serialize)]
pub struct GetDeliveryPriceRequest {
    /// Product category (option, linear, inverse)
    pub category: Category,

    /// Symbol name (e.g., "BTCUSDT"). If not provided, returns all symbols.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Base coin (e.g., "BTC", "ETH"). Only applicable for option.
    #[serde(rename = "baseCoin", skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,

    /// Limit for data size per page. Default: 200, Max: 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Cursor for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Delivery price information for a symbol.
#[derive(Debug, Clone, Deserialize)]
pub struct DeliveryPriceInfo {
    /// Symbol name
    pub symbol: String,

    /// The delivery price for the symbol
    #[serde(rename = "deliveryPrice")]
    pub delivery_price: String,

    /// Delivery timestamp in milliseconds
    #[serde(rename = "deliveryTime")]
    pub delivery_time: String,
}

/// Container for delivery price data.
#[derive(Debug, Clone, Deserialize)]
pub struct GetDeliveryPriceData {
    /// Product category
    pub category: Category,

    /// List of delivery price information
    pub list: Vec<DeliveryPriceInfo>,

    /// Cursor for next page (empty string if no more data)
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: String,
}

/// Response from the get delivery price API endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct GetDeliveryPriceResponse {
    /// Return code (0 indicates success)
    #[serde(rename = "retCode")]
    pub ret_code: i32,

    /// Return message
    #[serde(rename = "retMsg")]
    pub ret_msg: String,

    /// Delivery price data
    pub result: GetDeliveryPriceData,

    /// Extended information (varies by endpoint)
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,

    /// Response timestamp in milliseconds
    pub time: u64,
}

impl PublicRestClient {
    /// Get delivery price
    ///
    /// Query the delivery price of futures/options that have been delivered in the past.
    ///
    /// [docs](https://bybit-exchange.github.io/docs/v5/market/delivery-price)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The delivery price request parameters
    ///
    /// # Returns
    /// A result containing delivery price information for the requested symbols
    pub async fn get_delivery_price(
        &self,
        request: GetDeliveryPriceRequest,
    ) -> RestResult<GetDeliveryPriceResponse> {
        self.send_public_request(
            DELIVERY_PRICE_ENDPOINT,
            Some(&request),
            EndpointType::Market,
        )
        .await
    }
}
