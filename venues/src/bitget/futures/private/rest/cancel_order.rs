use crate::bitget::{BitgetRestClient, MarginCoin, ProductType};
use reqwest::Method;
use rest::BitgetRequest;
use serde::{Deserialize, Serialize};

/// Request for cancelling a single order
#[derive(Debug, Clone, Serialize)]
pub struct CancelOrderRequest {
    /// Trading pair
    pub symbol: String,

    /// Product type
    #[serde(rename = "productType")]
    pub product_type: ProductType,

    /// Margin coin (must be capitalized)
    #[serde(rename = "marginCoin", skip_serializing_if = "Option::is_none")]
    pub margin_coin: Option<MarginCoin>,

    /// Order ID (either orderId or clientOid is required)
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// Custom order ID (either orderId or clientOid is required)
    #[serde(rename = "clientOid", skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,
}

/// Response for cancelling an order
#[derive(Debug, Clone, Deserialize)]
pub struct CancelOrderResponse {
    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: String,

    /// Client customized ID
    #[serde(rename = "clientOid")]
    pub client_oid: String,
}

impl CancelOrderRequest {
    /// Create a new cancel order request using order ID
    pub fn by_order_id(
        symbol: impl Into<String>,
        product_type: ProductType,
        order_id: impl Into<String>,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            product_type,
            margin_coin: None,
            order_id: Some(order_id.into()),
            client_oid: None,
        }
    }

    /// Create a new cancel order request using client order ID
    pub fn by_client_oid(
        symbol: impl Into<String>,
        product_type: ProductType,
        client_oid: impl Into<String>,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            product_type,
            margin_coin: None,
            order_id: None,
            client_oid: Some(client_oid.into()),
        }
    }

    /// Set the margin coin
    pub fn margin_coin(mut self, margin_coin: MarginCoin) -> Self {
        self.margin_coin = Some(margin_coin);
        self
    }
}

impl BitgetRequest for CancelOrderRequest {
    type Response = CancelOrderResponse;

    fn path(&self) -> String {
        "/api/v2/mix/order/cancel-order".to_string()
    }

    fn method(&self) -> String {
        "POST".to_string()
    }

    fn need_signature(&self) -> bool {
        true
    }
}

/// Order identifier for batch cancel
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct OrderIdentifier {
    /// Order ID (either orderId or clientOid is required)
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// Custom order ID (either orderId or clientOid is required)
    #[serde(rename = "clientOid", skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,
}

impl OrderIdentifier {
    /// Create order identifier using order ID
    pub fn by_order_id(order_id: impl Into<String>) -> Self {
        Self {
            order_id: Some(order_id.into()),
            client_oid: None,
        }
    }

    /// Create order identifier using client order ID
    pub fn by_client_oid(client_oid: impl Into<String>) -> Self {
        Self {
            order_id: None,
            client_oid: Some(client_oid.into()),
        }
    }
}

/// Request for batch cancelling orders
#[derive(Debug, Clone, Serialize)]
pub struct BatchCancelOrdersRequest {
    /// Order ID list (maximum length: 50)
    #[serde(rename = "orderIdList", skip_serializing_if = "Option::is_none")]
    pub order_id_list: Option<Vec<OrderIdentifier>>,

    /// Trading pair (required when orderIdList is set)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Product type
    #[serde(rename = "productType")]
    pub product_type: ProductType,

    /// Margin coin (must be capitalized)
    #[serde(rename = "marginCoin", skip_serializing_if = "Option::is_none")]
    pub margin_coin: Option<MarginCoin>,
}

/// Response for batch cancelling orders
#[derive(Debug, Clone, Deserialize)]
pub struct BatchCancelOrdersResponse {
    /// List of cancelled order results
    pub data: Vec<CancelOrderResponse>,
}

impl BatchCancelOrdersRequest {
    /// Create a new batch cancel request for specific orders
    pub fn specific_orders(
        symbol: impl Into<String>,
        product_type: ProductType,
        order_identifiers: Vec<OrderIdentifier>,
    ) -> Self {
        Self {
            order_id_list: Some(order_identifiers),
            symbol: Some(symbol.into()),
            product_type,
            margin_coin: None,
        }
    }

    /// Create a new batch cancel request for all orders
    pub fn all_orders(product_type: ProductType) -> Self {
        Self {
            order_id_list: None,
            symbol: None,
            product_type,
            margin_coin: None,
        }
    }

    /// Set the margin coin
    pub fn margin_coin(mut self, margin_coin: MarginCoin) -> Self {
        self.margin_coin = Some(margin_coin);
        self
    }
}

impl BitgetRequest for BatchCancelOrdersRequest {
    type Response = BatchCancelOrdersResponse;

    fn path(&self) -> String {
        "/api/v2/mix/order/batch-cancel-orders".to_string()
    }

    fn method(&self) -> String {
        "POST".to_string()
    }

    fn need_signature(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_order_by_id() {
        let request =
            CancelOrderRequest::by_order_id("BTCUSDT", ProductType::UsdtFutures, "123456");

        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.product_type, ProductType::UsdtFutures);
        assert_eq!(request.order_id, Some("123456".to_string()));
        assert_eq!(request.client_oid, None);
    }

    #[test]
    fn test_cancel_order_by_client_oid() {
        let request =
            CancelOrderRequest::by_client_oid("ETHUSDT", ProductType::UsdtFutures, "client-123");

        assert_eq!(request.symbol, "ETHUSDT");
        assert_eq!(request.order_id, None);
        assert_eq!(request.client_oid, Some("client-123".to_string()));
    }

    #[test]
    fn test_batch_cancel_specific_orders() {
        let identifiers = vec![
            OrderIdentifier::by_order_id("123"),
            OrderIdentifier::by_client_oid("client-456"),
        ];
        let request = BatchCancelOrdersRequest::specific_orders(
            "BTCUSDT",
            ProductType::UsdtFutures,
            identifiers,
        );

        assert_eq!(request.symbol, Some("BTCUSDT".to_string()));
        assert_eq!(request.product_type, ProductType::UsdtFutures);
        assert!(request.order_id_list.is_some());
        assert_eq!(request.order_id_list.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_batch_cancel_all_orders() {
        let request = BatchCancelOrdersRequest::all_orders(ProductType::CoinFutures);

        assert_eq!(request.symbol, None);
        assert_eq!(request.product_type, ProductType::CoinFutures);
        assert_eq!(request.order_id_list, None);
    }
}
