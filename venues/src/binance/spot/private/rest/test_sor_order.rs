use serde::Serialize;

use super::{client::RestClient, new_sor_order::SorOrderRequest};
use crate::binance::spot::RestResult;

const TEST_SOR_ORDER_ENDPOINT: &str = "/api/v3/sor/order/test";

impl RestClient {
    /// Test SOR order creation
    ///
    /// Test SOR order creation and signature/recvWindow.
    ///
    /// [docs](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#test-new-sor-order--trade)
    /// Method: POST /api/v3/sor/order/test
    /// Weight: 1 (without computeCommissionRates), 20 (with computeCommissionRates)
    /// Security: TRADE
    pub async fn test_sor_order(
        &self,
        params: SorOrderRequest,
        compute_commission_rates: Option<bool>,
    ) -> RestResult<serde_json::Value> {
        let weight = if compute_commission_rates.unwrap_or(false) {
            20
        } else {
            1
        };

        // Create a new request struct with computeCommissionRates field
        #[derive(Debug, Clone, Serialize)]
        struct TestSorOrderRequest {
            #[serde(flatten)]
            base: SorOrderRequest,
            #[serde(
                rename = "computeCommissionRates",
                skip_serializing_if = "Option::is_none"
            )]
            compute_commission_rates: Option<bool>,
        }

        let test_request = TestSorOrderRequest {
            base: params,
            compute_commission_rates,
        };

        self.send_post_signed_request(TEST_SOR_ORDER_ENDPOINT, test_request, weight, true)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;
    use crate::binance::spot::{OrderSide, OrderType, TimeInForce};

    #[test]
    fn test_test_sor_order_response_without_commission_rates() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 0,
            "orderListId": -1,
            "clientOrderId": "test-sor-111",
            "transactTime": 1684123456793
        }"#;

        let response: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(response["orderId"], 0);
        assert_eq!(response["clientOrderId"], "test-sor-111");
        assert!(response.get("standardCommissionForOrder").is_none());
    }

    #[test]
    fn test_test_sor_order_response_with_commission_rates() {
        let json = r#"{
            "symbol": "ETHUSDT",
            "orderId": 0,
            "orderListId": -1,
            "clientOrderId": "test-sor-222",
            "transactTime": 1684123456794,
            "standardCommissionForOrder": {
                "maker": "0.00000000",
                "taker": "0.00100000"
            },
            "taxCommissionForOrder": {
                "maker": "0.00000000",
                "taker": "0.00000000"
            },
            "discount": {
                "enabledForAccount": true,
                "enabledForSymbol": true,
                "discountAsset": "BNB",
                "discount": "0.25000000"
            }
        }"#;

        let response: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(response["orderId"], 0);
        assert_eq!(response["clientOrderId"], "test-sor-222");

        // Check commission structure
        assert!(response["standardCommissionForOrder"].is_object());
        assert_eq!(
            response["standardCommissionForOrder"]["maker"],
            "0.00000000"
        );
        assert_eq!(
            response["standardCommissionForOrder"]["taker"],
            "0.00100000"
        );

        // Check tax commission
        assert!(response["taxCommissionForOrder"].is_object());
        assert_eq!(response["taxCommissionForOrder"]["maker"], "0.00000000");
        assert_eq!(response["taxCommissionForOrder"]["taker"], "0.00000000");

        // Check discount structure
        assert!(response["discount"].is_object());
        assert_eq!(response["discount"]["enabledForAccount"], true);
        assert_eq!(response["discount"]["enabledForSymbol"], true);
        assert_eq!(response["discount"]["discountAsset"], "BNB");
        assert_eq!(response["discount"]["discount"], "0.25000000");
    }

    #[test]
    fn test_test_sor_order_request_serialization() {
        let base_request = SorOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Market,
            time_in_force: None,
            quantity: Some(dec!(0.001)),
            price: None,
            new_client_order_id: None,
            strategy_id: None,
            strategy_type: None,
            iceberg_qty: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            recv_window: None,
        };

        #[derive(Debug, Clone, Serialize)]
        struct TestSorOrderRequest {
            #[serde(flatten)]
            base: SorOrderRequest,
            #[serde(
                rename = "computeCommissionRates",
                skip_serializing_if = "Option::is_none"
            )]
            compute_commission_rates: Option<bool>,
        }

        // Test without commission rates
        let test_request = TestSorOrderRequest {
            base: base_request.clone(),
            compute_commission_rates: None,
        };

        let json = serde_json::to_value(&test_request).unwrap();
        assert_eq!(json["symbol"], "BTCUSDT");
        assert_eq!(json["side"], "BUY");
        assert_eq!(json["type"], "MARKET");
        assert_eq!(json["quantity"], "0.001");
        assert!(json.get("computeCommissionRates").is_none());

        // Test with commission rates enabled
        let test_request_with_rates = TestSorOrderRequest {
            base: base_request.clone(),
            compute_commission_rates: Some(true),
        };

        let json_with_rates = serde_json::to_value(&test_request_with_rates).unwrap();
        assert_eq!(json_with_rates["symbol"], "BTCUSDT");
        assert_eq!(json_with_rates["computeCommissionRates"], true);

        // Test with commission rates disabled
        let test_request_rates_false = TestSorOrderRequest {
            base: base_request,
            compute_commission_rates: Some(false),
        };

        let json_rates_false = serde_json::to_value(&test_request_rates_false).unwrap();
        assert_eq!(json_rates_false["computeCommissionRates"], false);
    }

    #[test]
    fn test_test_sor_order_error_response() {
        let json = r#"{
            "code": -1102,
            "msg": "Mandatory parameter 'symbol' was not sent, was empty/null, or malformed."
        }"#;

        let response: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(response["code"], -1102);
        assert!(
            response["msg"]
                .as_str()
                .unwrap()
                .contains("Mandatory parameter")
        );
    }

    #[test]
    fn test_test_sor_order_with_limit_order_and_commission() {
        let base_request = SorOrderRequest {
            symbol: "ETHUSDT".to_string(),
            side: OrderSide::Sell,
            order_type: OrderType::Limit,
            time_in_force: Some(TimeInForce::GTC),
            quantity: Some(dec!(0.5)),
            price: Some(dec!(3000.50)),
            new_client_order_id: Some("test-sor-limit".to_string()),
            strategy_id: None,
            strategy_type: None,
            iceberg_qty: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            recv_window: Some(5000),
        };

        #[derive(Debug, Clone, Serialize)]
        struct TestSorOrderRequest {
            #[serde(flatten)]
            base: SorOrderRequest,
            #[serde(
                rename = "computeCommissionRates",
                skip_serializing_if = "Option::is_none"
            )]
            compute_commission_rates: Option<bool>,
        }

        let test_request = TestSorOrderRequest {
            base: base_request,
            compute_commission_rates: Some(true),
        };

        let json = serde_json::to_value(&test_request).unwrap();
        assert_eq!(json["symbol"], "ETHUSDT");
        assert_eq!(json["side"], "SELL");
        assert_eq!(json["type"], "LIMIT");
        assert_eq!(json["timeInForce"], "GTC");
        assert_eq!(json["quantity"], "0.5");
        assert_eq!(json["price"], "3000.50");
        assert_eq!(json["newClientOrderId"], "test-sor-limit");
        assert_eq!(json["computeCommissionRates"], true);
        assert_eq!(json["recvWindow"], 5000);
    }
}
