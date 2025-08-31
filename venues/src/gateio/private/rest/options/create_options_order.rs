use serde::Serialize;

use super::{RestClient, RestResult, order::OptionsOrder};

const CREATE_OPTIONS_ORDER_ENDPOINT: &str = "/options/orders";

/// Request to create options order
#[derive(Debug, Clone, Serialize)]
pub struct CreateOptionsOrderRequest {
    /// Contract name
    pub contract: String,

    /// Order size
    pub size: String,

    /// Order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Time in force (gtc, ioc, poc, fok)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tif: Option<String>,

    /// Text label for order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Reduce only order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,

    /// Close position order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close: Option<bool>,
}

impl RestClient {
    /// Create an Options Order
    ///
    /// This endpoint creates a new options order.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#create-an-options-order)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The options order creation request parameters
    ///
    /// # Returns
    /// Created options order information
    pub async fn create_options_order(
        &self,
        request: CreateOptionsOrderRequest,
    ) -> RestResult<OptionsOrder> {
        self.post(CREATE_OPTIONS_ORDER_ENDPOINT, &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_options_order_request_minimal() {
        let request = CreateOptionsOrderRequest {
            contract: "BTC-20240101-50000-C".to_string(),
            size: "1".to_string(),
            price: None,
            tif: None,
            text: None,
            reduce_only: None,
            close: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["contract"], "BTC-20240101-50000-C");
        assert_eq!(json["size"], "1");

        // Ensure optional fields are not serialized when None
        assert!(json.get("price").is_none());
        assert!(json.get("tif").is_none());
        assert!(json.get("text").is_none());
        assert!(json.get("reduce_only").is_none());
        assert!(json.get("close").is_none());
    }

    #[test]
    fn test_create_options_order_request_full() {
        let request = CreateOptionsOrderRequest {
            contract: "ETH-20240101-3000-P".to_string(),
            size: "2.5".to_string(),
            price: Some("0.05".to_string()),
            tif: Some("gtc".to_string()),
            text: Some("test order".to_string()),
            reduce_only: Some(true),
            close: Some(false),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["contract"], "ETH-20240101-3000-P");
        assert_eq!(json["size"], "2.5");
        assert_eq!(json["price"], "0.05");
        assert_eq!(json["tif"], "gtc");
        assert_eq!(json["text"], "test order");
        assert!(json["reduce_only"].as_bool().unwrap_or(false));
        assert!(!json["close"].as_bool().unwrap_or(true));
    }

    #[test]
    fn test_tif_values() {
        let tif_values = vec!["gtc", "ioc", "poc", "fok"];

        for tif in tif_values {
            let request = CreateOptionsOrderRequest {
                contract: "BTC-20240101-50000-C".to_string(),
                size: "1".to_string(),
                price: Some("0.1".to_string()),
                tif: Some(tif.to_string()),
                text: None,
                reduce_only: None,
                close: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["tif"], tif);
        }
    }

    #[test]
    fn test_market_order() {
        let request = CreateOptionsOrderRequest {
            contract: "BTC-20240101-50000-C".to_string(),
            size: "0.5".to_string(),
            price: None, // Market order
            tif: Some("ioc".to_string()),
            text: Some("market buy".to_string()),
            reduce_only: Some(false),
            close: Some(false),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["contract"], "BTC-20240101-50000-C");
        assert_eq!(json["size"], "0.5");
        assert!(json.get("price").is_none());
        assert_eq!(json["tif"], "ioc");
        assert_eq!(json["text"], "market buy");
    }

    #[test]
    fn test_options_contract_formats() {
        let contracts = vec![
            "BTC-20240101-50000-C", // Call option
            "BTC-20240101-50000-P", // Put option
            "ETH-20240101-3000-C",
            "ETH-20240101-3000-P",
        ];

        for contract in contracts {
            let request = CreateOptionsOrderRequest {
                contract: contract.to_string(),
                size: "1".to_string(),
                price: Some("0.1".to_string()),
                tif: None,
                text: None,
                reduce_only: None,
                close: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["contract"], contract);
        }
    }

    #[test]
    fn test_reduce_only_order() {
        let request = CreateOptionsOrderRequest {
            contract: "BTC-20240101-50000-C".to_string(),
            size: "1".to_string(),
            price: Some("0.08".to_string()),
            tif: Some("gtc".to_string()),
            text: Some("reduce position".to_string()),
            reduce_only: Some(true),
            close: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert!(json["reduce_only"].as_bool().unwrap_or(false));
    }

    #[test]
    fn test_close_position_order() {
        let request = CreateOptionsOrderRequest {
            contract: "BTC-20240101-50000-P".to_string(),
            size: "2".to_string(),
            price: None,
            tif: Some("ioc".to_string()),
            text: Some("close position".to_string()),
            reduce_only: None,
            close: Some(true),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert!(json["close"].as_bool().unwrap_or(false));
    }

    #[test]
    fn test_various_sizes() {
        let sizes = vec!["0.1", "0.5", "1", "1.5", "2", "5", "10", "100"];

        for size in sizes {
            let request = CreateOptionsOrderRequest {
                contract: "BTC-20240101-50000-C".to_string(),
                size: size.to_string(),
                price: Some("0.1".to_string()),
                tif: None,
                text: None,
                reduce_only: None,
                close: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["size"], size);
        }
    }

    #[test]
    fn test_various_prices() {
        let prices = vec!["0.001", "0.01", "0.05", "0.1", "0.5", "1", "5"];

        for price in prices {
            let request = CreateOptionsOrderRequest {
                contract: "ETH-20240101-3000-C".to_string(),
                size: "1".to_string(),
                price: Some(price.to_string()),
                tif: Some("gtc".to_string()),
                text: None,
                reduce_only: None,
                close: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["price"], price);
        }
    }
}
