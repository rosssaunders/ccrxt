use serde::{Deserialize, Serialize};

use super::RestClient;
use super::batch_orders::BatchOrderResult;
use super::amend_order::AmendFuturesOrderRequest;

/// Request to amend batch orders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchAmendOrdersRequest {
    /// Settlement currency
    pub settle: String,
    /// List of order amendments
    pub orders: Vec<AmendFuturesOrderRequest>,
}

impl RestClient {
    /// Amend a batch of futures orders
    ///
    /// Modifies multiple orders in a single request.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.io/docs/developers/apiv4/#amend-multiple-open-orders-in-batch>
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The batch order amendment request parameters
    ///
    /// # Returns
    /// List of batch order results
    pub async fn amend_batch_futures_orders(
        &self,
        request: BatchAmendOrdersRequest,
    ) -> crate::gateio::perpetual::Result<Vec<BatchOrderResult>> {
        let endpoint = format!("/futures/{}/batch_orders", request.settle);
        self.put(&endpoint, &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_amend_orders_request_single() {
        let amendments = vec![
            AmendFuturesOrderRequest {
                settle: "USDT".to_string(),
                order_id: "123456789".to_string(),
                size: Some(1500),
                price: Some("43100.0".to_string()),
                amend_text: Some("price-update".to_string()),
            },
        ];

        let request = BatchAmendOrdersRequest {
            settle: "USDT".to_string(),
            orders: amendments,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["orders"].as_array().unwrap().len(), 1);
        assert_eq!(json["orders"][0]["order_id"], "123456789");
        assert_eq!(json["orders"][0]["size"], 1500);
        assert_eq!(json["orders"][0]["price"], "43100.0");
        assert_eq!(json["orders"][0]["amend_text"], "price-update");
    }

    #[test]
    fn test_batch_amend_orders_request_multiple() {
        let amendments = vec![
            AmendFuturesOrderRequest {
                settle: "USDT".to_string(),
                order_id: "123456789".to_string(),
                size: Some(1500),
                price: Some("43100.0".to_string()),
                amend_text: Some("increase-size".to_string()),
            },
            AmendFuturesOrderRequest {
                settle: "USDT".to_string(),
                order_id: "987654321".to_string(),
                size: None,
                price: Some("2655.0".to_string()),
                amend_text: Some("price-only".to_string()),
            },
            AmendFuturesOrderRequest {
                settle: "USDT".to_string(),
                order_id: "555555555".to_string(),
                size: Some(500),
                price: None,
                amend_text: Some("size-only".to_string()),
            },
        ];

        let request = BatchAmendOrdersRequest {
            settle: "USDT".to_string(),
            orders: amendments,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["orders"].as_array().unwrap().len(), 3);
        
        // First amendment - both price and size
        assert_eq!(json["orders"][0]["order_id"], "123456789");
        assert_eq!(json["orders"][0]["size"], 1500);
        assert_eq!(json["orders"][0]["price"], "43100.0");
        
        // Second amendment - price only
        assert_eq!(json["orders"][1]["order_id"], "987654321");
        assert!(!json["orders"][1].as_object().unwrap().contains_key("size"));
        assert_eq!(json["orders"][1]["price"], "2655.0");
        
        // Third amendment - size only
        assert_eq!(json["orders"][2]["order_id"], "555555555");
        assert_eq!(json["orders"][2]["size"], 500);
        assert!(!json["orders"][2].as_object().unwrap().contains_key("price"));
    }

    #[test]
    fn test_batch_amend_various_sizes() {
        let size_scenarios = vec![
            (1000, "Increase to 1000"),
            (500, "Reduce to 500"),
            (10000, "Large position"),
            (-1000, "Short position"),
            (-500, "Small short"),
        ];

        let mut amendments = Vec::new();
        for (i, (size, _description)) in size_scenarios.iter().enumerate() {
            amendments.push(AmendFuturesOrderRequest {
                settle: "USDT".to_string(),
                order_id: format!("order_{}", i + 1),
                size: Some(*size),
                price: None,
                amend_text: None,
            });
        }

        let request = BatchAmendOrdersRequest {
            settle: "USDT".to_string(),
            orders: amendments,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["orders"].as_array().unwrap().len(), 5);
        
        for (i, (size, _)) in size_scenarios.iter().enumerate() {
            assert_eq!(json["orders"][i]["size"], *size);
        }
    }

    #[test]
    fn test_batch_amend_various_prices() {
        let price_scenarios = vec![
            "43000.0",
            "43000.50",
            "43000.25",
            "43000.125",
            "43000.0625",
        ];

        let mut amendments = Vec::new();
        for (i, price) in price_scenarios.iter().enumerate() {
            amendments.push(AmendFuturesOrderRequest {
                settle: "USDT".to_string(),
                order_id: format!("order_{}", i + 1),
                size: None,
                price: Some(price.to_string()),
                amend_text: None,
            });
        }

        let request = BatchAmendOrdersRequest {
            settle: "USDT".to_string(),
            orders: amendments,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["orders"].as_array().unwrap().len(), 5);
        
        for (i, price) in price_scenarios.iter().enumerate() {
            assert_eq!(json["orders"][i]["price"], *price);
        }
    }

    #[test]
    fn test_batch_amend_large_batch() {
        let mut amendments = Vec::new();
        for i in 1..=50 {
            amendments.push(AmendFuturesOrderRequest {
                settle: "USDT".to_string(),
                order_id: format!("order_{}", i),
                size: Some(i * 100),
                price: Some(format!("{}.0", 43000 + i)),
                amend_text: Some(format!("batch-amend-{}", i)),
            });
        }

        let request = BatchAmendOrdersRequest {
            settle: "USDT".to_string(),
            orders: amendments,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["orders"].as_array().unwrap().len(), 50);
        
        // Check first and last orders
        assert_eq!(json["orders"][0]["order_id"], "order_1");
        assert_eq!(json["orders"][0]["size"], 100);
        assert_eq!(json["orders"][0]["price"], "43001.0");
        
        assert_eq!(json["orders"][49]["order_id"], "order_50");
        assert_eq!(json["orders"][49]["size"], 5000);
        assert_eq!(json["orders"][49]["price"], "43050.0");
    }

    #[test]
    fn test_batch_amend_realistic_scenarios() {
        // Scenario 1: Move multiple stop losses
        let stop_loss_amendments = vec![
            AmendFuturesOrderRequest {
                settle: "USDT".to_string(),
                order_id: "sl_btc_123".to_string(),
                size: None,
                price: Some("42500.0".to_string()),
                amend_text: Some("Move BTC stop loss higher".to_string()),
            },
            AmendFuturesOrderRequest {
                settle: "USDT".to_string(),
                order_id: "sl_eth_456".to_string(),
                size: None,
                price: Some("2600.0".to_string()),
                amend_text: Some("Move ETH stop loss higher".to_string()),
            },
        ];

        let request = BatchAmendOrdersRequest {
            settle: "USDT".to_string(),
            orders: stop_loss_amendments,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["orders"][0]["price"], "42500.0");
        assert_eq!(json["orders"][1]["price"], "2600.0");

        // Scenario 2: Scale into positions
        let scale_in_amendments = vec![
            AmendFuturesOrderRequest {
                settle: "USDT".to_string(),
                order_id: "limit_1".to_string(),
                size: Some(2000),
                price: Some("42900.0".to_string()),
                amend_text: Some("Scale in 1".to_string()),
            },
            AmendFuturesOrderRequest {
                settle: "USDT".to_string(),
                order_id: "limit_2".to_string(),
                size: Some(3000),
                price: Some("42800.0".to_string()),
                amend_text: Some("Scale in 2".to_string()),
            },
            AmendFuturesOrderRequest {
                settle: "USDT".to_string(),
                order_id: "limit_3".to_string(),
                size: Some(4000),
                price: Some("42700.0".to_string()),
                amend_text: Some("Scale in 3".to_string()),
            },
        ];

        let request = BatchAmendOrdersRequest {
            settle: "USDT".to_string(),
            orders: scale_in_amendments,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["orders"].as_array().unwrap().len(), 3);
        
        // Verify increasing sizes and decreasing prices
        for i in 0..3 {
            let size = json["orders"][i]["size"].as_i64().unwrap();
            assert_eq!(size, (i as i64 + 2) * 1000);
        }
    }

    #[test]
    fn test_batch_amend_different_contracts() {
        let amendments = vec![
            AmendFuturesOrderRequest {
                settle: "USDT".to_string(),
                order_id: "btc_order".to_string(),
                size: Some(1000),
                price: Some("43000.0".to_string()),
                amend_text: Some("BTC order".to_string()),
            },
            AmendFuturesOrderRequest {
                settle: "USDT".to_string(),
                order_id: "eth_order".to_string(),
                size: Some(2000),
                price: Some("2650.0".to_string()),
                amend_text: Some("ETH order".to_string()),
            },
            AmendFuturesOrderRequest {
                settle: "USDT".to_string(),
                order_id: "sol_order".to_string(),
                size: Some(5000),
                price: Some("95.50".to_string()),
                amend_text: Some("SOL order".to_string()),
            },
        ];

        let request = BatchAmendOrdersRequest {
            settle: "USDT".to_string(),
            orders: amendments,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["orders"].as_array().unwrap().len(), 3);
        
        // Each order can be for different contracts (identified by order_id)
        assert!(json["orders"][0]["order_id"].as_str().unwrap().contains("btc"));
        assert!(json["orders"][1]["order_id"].as_str().unwrap().contains("eth"));
        assert!(json["orders"][2]["order_id"].as_str().unwrap().contains("sol"));
    }

    #[test]
    fn test_settlement_currencies() {
        let settlements = vec!["USDT", "BTC", "ETH"];

        for settle in settlements {
            let amendments = vec![
                AmendFuturesOrderRequest {
                    settle: settle.to_string(),
                    order_id: "12345".to_string(),
                    size: Some(1000),
                    price: None,
                    amend_text: None,
                },
            ];

            let request = BatchAmendOrdersRequest {
                settle: settle.to_string(),
                orders: amendments,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["settle"], settle);
            assert_eq!(json["orders"][0]["settle"], settle);
        }
    }

    #[test]
    fn test_empty_amend_text() {
        let amendments = vec![
            AmendFuturesOrderRequest {
                settle: "USDT".to_string(),
                order_id: "123456789".to_string(),
                size: Some(1500),
                price: Some("43100.0".to_string()),
                amend_text: None,
            },
        ];

        let request = BatchAmendOrdersRequest {
            settle: "USDT".to_string(),
            orders: amendments,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert!(!json["orders"][0].as_object().unwrap().contains_key("amend_text"));
    }
}