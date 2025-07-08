use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bybit::{EndpointType, RestResult, enums::*};

/// Endpoint URL path for tickers
const ENDPOINT_PATH: &str = "/v5/market/tickers";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTickersRequest {
    pub category: Category,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_date: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerInfo {
    pub symbol: String,
    #[serde(rename = "lastPrice")]
    pub last_price: String,
    #[serde(rename = "indexPrice")]
    pub index_price: Option<String>,
    #[serde(rename = "markPrice")]
    pub mark_price: Option<String>,
    #[serde(rename = "prevPrice24h")]
    pub prev_price_24h: String,
    #[serde(rename = "price24hPcnt")]
    pub price_24h_pcnt: String,
    #[serde(rename = "highPrice24h")]
    pub high_price_24h: String,
    #[serde(rename = "lowPrice24h")]
    pub low_price_24h: String,
    #[serde(rename = "prevPrice1h")]
    pub prev_price_1h: Option<String>,
    #[serde(rename = "openInterest")]
    pub open_interest: Option<String>,
    #[serde(rename = "openInterestValue")]
    pub open_interest_value: Option<String>,
    #[serde(rename = "turnover24h")]
    pub turnover_24h: String,
    #[serde(rename = "volume24h")]
    pub volume_24h: String,
    #[serde(rename = "fundingRate")]
    pub funding_rate: Option<String>,
    #[serde(rename = "nextFundingTime")]
    pub next_funding_time: Option<String>,
    #[serde(rename = "predictedDeliveryPrice")]
    pub predicted_delivery_price: Option<String>,
    #[serde(rename = "basisRate")]
    pub basis_rate: Option<String>,
    pub basis: Option<String>,
    #[serde(rename = "deliveryFeeRate")]
    pub delivery_fee_rate: Option<String>,
    #[serde(rename = "deliveryTime")]
    pub delivery_time: Option<String>,
    #[serde(rename = "ask1Size")]
    pub ask1_size: String,
    #[serde(rename = "bid1Price")]
    pub bid1_price: String,
    #[serde(rename = "ask1Price")]
    pub ask1_price: String,
    #[serde(rename = "bid1Size")]
    pub bid1_size: String,
    #[serde(rename = "preOpenPrice")]
    pub pre_open_price: Option<String>,
    #[serde(rename = "preQty")]
    pub pre_qty: Option<String>,
    #[serde(rename = "curPreListingPhase")]
    pub cur_pre_listing_phase: Option<String>,
    #[serde(rename = "usdIndexPrice")]
    pub usd_index_price: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetTickersData {
    pub category: Category,
    pub list: Vec<TickerInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetTickersResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: GetTickersData,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl RestClient {
    /// Get tickers
    ///
    /// Query for the latest price snapshot, best bid/ask price, and trading volume in the last 24 hours.
    ///
    /// # Arguments
    /// * `request` - The tickers request parameters
    ///
    /// # Returns
    /// A result containing the tickers response or an error
    pub async fn get_tickers(&self, request: GetTickersRequest) -> RestResult<GetTickersResponse> {
        self.send_public_request(ENDPOINT_PATH, Some(&request), EndpointType::Market)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tickers_request_builder() {
        let request = GetTickersRequest {
            category: Category::Linear,
            symbol: Some("BTCUSDT".to_string()),
            base_coin: None,
            exp_date: None,
        };

        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.symbol, Some("BTCUSDT".to_string()));
        assert!(request.base_coin.is_none());
        assert!(request.exp_date.is_none());
    }

    #[test]
    fn test_get_tickers_request_options() {
        let request = GetTickersRequest {
            category: Category::Option,
            symbol: None,
            base_coin: Some("BTC".to_string()),
            exp_date: Some("25DEC22".to_string()),
        };

        assert_eq!(request.category, Category::Option);
        assert_eq!(request.base_coin, Some("BTC".to_string()));
        assert_eq!(request.exp_date, Some("25DEC22".to_string()));
        assert!(request.symbol.is_none());
    }
}
