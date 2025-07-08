//! Quarterly Contract Settlement Price (GET /futures/data/delivery-price)
//!
//! See: https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Delivery-Price
use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::{Errors, RestResult};

/// Request parameters for the Quarterly Contract Settlement Price endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeliveryPriceRequest<'a> {
    /// The pair to query (e.g., "BTCUSDT").
    pub pair: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryPriceResponse {
    pub delivery_time: u64,
    pub delivery_price: f64,
}

impl RestClient {
    /// Quarterly Contract Settlement Price (GET /futures/data/delivery-price)
    ///
    /// [API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Delivery-Price)
    pub async fn delivery_price<'a>(
        &self,
        params: DeliveryPriceRequest<'a>,
    ) -> RestResult<Vec<DeliveryPriceResponse>> {
        let endpoint = "/futures/data/delivery-price";
        let query = serde_urlencoded::to_string(&params)
            .map_err(|e| Errors::Error(format!("Failed to serialize params: {e}")))?;
        let resp = self
            .send_request::<Vec<DeliveryPriceResponse>>(
                endpoint,
                reqwest::Method::GET,
                Some(&query),
                None,
                0,
            )
            .await?;
        Ok(resp)
    }
}
