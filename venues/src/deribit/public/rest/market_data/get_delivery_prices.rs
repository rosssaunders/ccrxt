//! Implements the /public/get_delivery_prices endpoint for Deribit.
//!
//! Retrieves delivery prices for the given index.

use serde::{Deserialize, Serialize};

use crate::deribit::{
    EndpointType, JsonRpcResult, PublicRestClient, RestResult, enums::CurrencyPair,
};

const DELIVERY_PRICES_ENDPOINT: &str = "public/get_delivery_prices";

/// Request parameters for the get_delivery_prices endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct GetDeliveryPricesRequest {
    /// Index identifier, matches (base) cryptocurrency with quote currency.
    #[serde(rename = "index_name")]
    pub index_name: CurrencyPair,

    /// The offset for pagination, default 0.
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,

    /// Number of requested items, default 10.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
}

/// Represents a single delivery price record.
#[derive(Debug, Clone, Deserialize)]
pub struct DeliveryPriceRecord {
    /// The event date with year, month and day.
    #[serde(rename = "date")]
    pub date: String,

    /// The settlement price for the instrument. Only when state = closed.
    #[serde(rename = "delivery_price")]
    pub delivery_price: f64,
}

/// The result object for get_delivery_prices.
#[derive(Debug, Clone, Deserialize)]
pub struct GetDeliveryPricesResult {
    /// Array of delivery price records.
    #[serde(rename = "data")]
    pub data: Vec<DeliveryPriceRecord>,

    /// Available delivery prices.
    #[serde(rename = "records_total")]
    pub records_total: u32,
}

/// Response for public/get_combo_ids endpoint following Deribit JSON-RPC 2.0 format.
pub type GetDeliveryPricesResponse = JsonRpcResult<GetDeliveryPricesResult>;

impl PublicRestClient {
    /// Calls the /public/get_delivery_prices endpoint.
    ///
    /// Retrieves delivery prices for the given index.
    ///
    /// [docs](https://docs.deribit.com/#public-get_delivery_prices)
    pub async fn get_delivery_prices(
        &self,
        params: GetDeliveryPricesRequest,
    ) -> RestResult<GetDeliveryPricesResponse> {
        self.send_post_request(
            DELIVERY_PRICES_ENDPOINT,
            Some(&params),
            EndpointType::NonMatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;
    use crate::deribit::enums::CurrencyPair;

    #[test]
    fn test_serialize_request() {
        let req = GetDeliveryPricesRequest {
            index_name: CurrencyPair::BtcUsd,
            offset: Some(5),
            count: Some(10),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("btc_usd"));
        assert!(json.contains("offset"));
        assert!(json.contains("count"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "data": [
                    { "date": "2024-06-01", "delivery_price": 65000.0 },
                    { "date": "2024-06-02", "delivery_price": 65500.0 }
                ],
                "records_total": 2
            }
        }"#;
        let resp: GetDeliveryPricesResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 1);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.data.len(), 2);
        assert_eq!(resp.result.records_total, 2);
        assert_eq!(resp.result.data[0].date, "2024-06-01");
        assert!((resp.result.data[1].delivery_price - 65500.0).abs() < 1e-8);
    }
}
