use serde::Serialize;

use super::RestClient;

const DELIVERY_MY_TRADES_ENDPOINT: &str = "/delivery/{}/my_trades";

/// Request parameters for delivery my trades
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeliveryMyTradesRequest {
    /// Settlement currency
    pub settle: String,

    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,

    /// Order ID filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,

    /// Maximum number of records (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// List offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    /// Specify starting point
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,

    /// Count only (returns count instead of trades)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_total: Option<i32>,
}

/// Delivery trade information (reusing the public DeliveryTrade from public module)
pub type DeliveryTrade = crate::gateio::delivery::public::rest::trades::DeliveryTrade;

impl RestClient {
    /// List personal delivery trading history
    ///
    /// Retrieves the user's trading history for delivery contracts.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `params` - The delivery trades request parameters
    ///
    /// # Returns
    /// List of delivery trades
    pub async fn get_delivery_my_trades(
        &self,
        params: DeliveryMyTradesRequest,
    ) -> crate::gateio::delivery::RestResult<Vec<DeliveryTrade>> {
        let endpoint = DELIVERY_MY_TRADES_ENDPOINT.replace("{}", &params.settle);
        self.get_with_query(&endpoint, &params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delivery_my_trades_endpoint() {
        assert_eq!(DELIVERY_MY_TRADES_ENDPOINT, "/delivery/{}/my_trades");
    }

    #[test]
    fn test_my_trades_endpoint_construction() {
        let settle = "BTC";
        let endpoint = DELIVERY_MY_TRADES_ENDPOINT.replace("{}", settle);
        assert_eq!(endpoint, "/delivery/BTC/my_trades");
    }

    #[test]
    fn test_my_trades_request_minimal() {
        let request = DeliveryMyTradesRequest {
            settle: "BTC".to_string(),
            ..Default::default()
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "BTC");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 1);
        assert!(!obj.contains_key("contract"));
        assert!(!obj.contains_key("order"));
        assert!(!obj.contains_key("limit"));
        assert!(!obj.contains_key("offset"));
        assert!(!obj.contains_key("last_id"));
        assert!(!obj.contains_key("count_total"));
    }

    #[test]
    fn test_my_trades_request_full() {
        let request = DeliveryMyTradesRequest {
            settle: "USDT".to_string(),
            contract: Some("BTC_USDT_20240315".to_string()),
            order: Some(12345678),
            limit: Some(50),
            offset: Some(10),
            last_id: Some("last123".to_string()),
            count_total: Some(1),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT_20240315");
        assert_eq!(json["order"], 12345678);
        assert_eq!(json["limit"], 50);
        assert_eq!(json["offset"], 10);
        assert_eq!(json["last_id"], "last123");
        assert_eq!(json["count_total"], 1);

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 7);
    }

    #[test]
    fn test_my_trades_request_with_contract_filter() {
        let request = DeliveryMyTradesRequest {
            settle: "BTC".to_string(),
            contract: Some("ETH_BTC_20240415".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["contract"], "ETH_BTC_20240415");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 2);
    }

    #[test]
    fn test_my_trades_request_with_order_filter() {
        let request = DeliveryMyTradesRequest {
            settle: "USDT".to_string(),
            order: Some(987654321),
            ..Default::default()
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["order"], 987654321);

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 2);
    }

    #[test]
    fn test_my_trades_request_with_pagination() {
        let request = DeliveryMyTradesRequest {
            settle: "USDT".to_string(),
            limit: Some(100),
            offset: Some(25),
            ..Default::default()
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["limit"], 100);
        assert_eq!(json["offset"], 25);

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 3);
    }

    #[test]
    fn test_my_trades_request_with_last_id() {
        let request = DeliveryMyTradesRequest {
            settle: "BTC".to_string(),
            last_id: Some("trade_12345".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["last_id"], "trade_12345");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 2);
    }

    #[test]
    fn test_my_trades_different_settlements() {
        let settlements = vec!["BTC", "USDT", "ETH"];
        for settle in settlements {
            let endpoint = DELIVERY_MY_TRADES_ENDPOINT.replace("{}", settle);
            assert_eq!(endpoint, format!("/delivery/{}/my_trades", settle));
        }
    }
}
