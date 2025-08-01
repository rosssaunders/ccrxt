use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::spot::{EndpointType, RestResult};

/// Endpoint for BingX Spot Order Book Aggregation
const ORDER_BOOK_AGGREGATION_ENDPOINT: &str = "/openApi/spot/v2/market/depth";

/// Aggregation type for order book depth precision
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AggregationType {
    /// Default precision
    #[serde(rename = "step0")]
    Step0,
    /// 10x precision
    #[serde(rename = "step1")]
    Step1,
    /// 100x precision
    #[serde(rename = "step2")]
    Step2,
    /// 1000x precision
    #[serde(rename = "step3")]
    Step3,
    /// 10000x precision
    #[serde(rename = "step4")]
    Step4,
    /// 100000x precision
    #[serde(rename = "step5")]
    Step5,
}

impl Default for AggregationType {
    fn default() -> Self {
        Self::Step0
    }
}

/// Request parameters for the BingX Spot Order Book Aggregation endpoint.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderBookAggregationRequest {
    /// Trading pair, e.g., "BTC_USDT". Required.
    pub symbol: String,

    /// Query depth. Required. (e.g., 20)
    pub depth: i64,

    /// Aggregation type. Required. Controls precision level.
    #[serde(rename = "type")]
    pub aggregation_type: AggregationType,
}

/// Response from the BingX Spot Order Book Aggregation endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderBookAggregationResponse {
    /// Buy depth, where each element is [price, quantity] as strings.
    pub bids: Vec<[String; 2]>,

    /// Sell depth, where each element is [price, quantity] as strings.
    pub asks: Vec<[String; 2]>,

    /// Timestamp in milliseconds since epoch.
    pub ts: i64,
}

impl RestClient {
    /// Order Book aggregation
    ///
    /// Used to query aggregated depth for a trading pair.
    ///
    /// [docs]: https://bingx-api.github.io/docs/#/en-us/spot/market-api.html#Order%20Book%20aggregation
    ///
    /// Rate limit: IP - 100 requests per 10 seconds (Group 1)
    ///
    /// # Arguments
    /// * `request` - The order book aggregation request parameters
    ///
    /// # Returns
    /// Order book aggregation response containing bids, asks, and timestamp
    pub async fn get_order_book_aggregation(
        &self,
        request: &GetOrderBookAggregationRequest,
    ) -> RestResult<GetOrderBookAggregationResponse> {
        self.send_request(
            ORDER_BOOK_AGGREGATION_ENDPOINT,
            Some(request),
            EndpointType::PublicMarket,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_book_aggregation_request_creation() {
        let symbol = "BTC_USDT".to_string();
        let depth = 20;
        let aggregation_type = AggregationType::Step0;
        let request = GetOrderBookAggregationRequest {
            symbol: symbol.clone(),
            depth,
            aggregation_type: aggregation_type.clone(),
        };
        assert_eq!(request.symbol, symbol);
        assert_eq!(request.depth, depth);
        assert_eq!(request.aggregation_type, aggregation_type);
    }

    #[test]
    fn test_order_book_aggregation_request_serialization() {
        let request = GetOrderBookAggregationRequest {
            symbol: "BTC_USDT".to_string(),
            depth: 20,
            aggregation_type: AggregationType::Step0,
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"symbol\":\"BTC_USDT\""));
        assert!(json.contains("\"depth\":20"));
        assert!(json.contains("\"type\":\"step0\""));
    }

    #[test]
    fn test_aggregation_type_serialization() {
        assert_eq!(
            serde_json::to_string(&AggregationType::Step0).unwrap(),
            "\"step0\""
        );
        assert_eq!(
            serde_json::to_string(&AggregationType::Step1).unwrap(),
            "\"step1\""
        );
        assert_eq!(
            serde_json::to_string(&AggregationType::Step5).unwrap(),
            "\"step5\""
        );
    }

    #[test]
    fn test_aggregation_type_deserialization() {
        assert_eq!(
            serde_json::from_str::<AggregationType>("\"step0\"").unwrap(),
            AggregationType::Step0
        );
        assert_eq!(
            serde_json::from_str::<AggregationType>("\"step3\"").unwrap(),
            AggregationType::Step3
        );
        assert_eq!(
            serde_json::from_str::<AggregationType>("\"step5\"").unwrap(),
            AggregationType::Step5
        );
    }

    #[test]
    fn test_aggregation_type_default() {
        assert_eq!(AggregationType::default(), AggregationType::Step0);
    }

    #[test]
    fn test_order_book_aggregation_response_deserialization() {
        let json = r#"{
            "bids": [["45000.0", "1.5"], ["44999.0", "2.0"]],
            "asks": [["45001.0", "1.2"], ["45002.0", "0.8"]],
            "ts": 1640995200000
        }"#;
        let response: GetOrderBookAggregationResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.bids.len(), 2);
        assert_eq!(response.asks.len(), 2);
        assert_eq!(response.ts, 1640995200000);
        let bid0 = response.bids.first().expect("Expected at least one bid");
        assert_eq!(*bid0.first().expect("Missing price in bid0"), "45000.0");
        assert_eq!(*bid0.get(1).expect("Missing amount in bid0"), "1.5");
        let ask0 = response.asks.first().expect("Expected at least one ask");
        assert_eq!(*ask0.first().expect("Missing price in ask0"), "45001.0");
        assert_eq!(*ask0.get(1).expect("Missing amount in ask0"), "1.2");
    }
}
