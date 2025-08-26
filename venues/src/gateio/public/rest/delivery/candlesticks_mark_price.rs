use super::RestClient;
use crate::gateio::{
    RestResult,
    private::rest::delivery::models::{DeliveryCandlestick, DeliveryCandlesticksRequest},
};

const DELIVERY_MARK_PRICE_CANDLESTICKS_ENDPOINT: &str = "/delivery/{}/mark_price_candlesticks";

impl RestClient {
    /// Get delivery mark price candlesticks
    ///
    /// Retrieves mark price candlestick data for a specific delivery contract.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#get-delivery-mark-price-candlesticks)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `params` - The delivery candlesticks request parameters
    ///
    /// # Returns
    /// List of delivery mark price candlesticks
    pub async fn get_delivery_mark_price_candlesticks(
        &self,
        params: DeliveryCandlesticksRequest,
    ) -> RestResult<Vec<DeliveryCandlestick>> {
        let endpoint = DELIVERY_MARK_PRICE_CANDLESTICKS_ENDPOINT.replace("{}", &params.settle);
        self.get_with_query(&endpoint, Some(&params)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gateio::enums::CandlestickInterval;

    #[test]
    fn test_mark_price_contract() {
        let request = DeliveryCandlesticksRequest {
            settle: "USDT".to_string(),
            contract: "mark_BTC_USDT_20241227".to_string(),
            interval: Some(CandlestickInterval::Minutes1),
            from: None,
            to: None,
            limit: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["contract"], "mark_BTC_USDT_20241227");
        assert!(json["contract"].as_str().unwrap().starts_with("mark_"));
    }
}
