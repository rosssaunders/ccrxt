use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{
    OrderSide, OrderStatus, OrderType, ResponseHeaders, RestResponse, Result,
};

/// Get recent closed orders request
#[derive(Debug, Clone, Serialize)]
pub struct GetRecentClosedOrdersRequest {
    /// Optional symbol to filter orders by contract
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

/// Recent closed order item
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentClosedOrderItem {
    /// Order ID
    pub id: String,
    /// Contract symbol
    pub symbol: String,
    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// Order side
    pub side: OrderSide,
    /// Order amount
    pub size: String,
    /// Order value
    pub value: String,
    /// Deal value
    pub deal_value: String,
    /// Deal size
    pub deal_size: String,
    /// Taker fee rate
    pub taker_fee_rate: String,
    /// Maker fee rate
    pub maker_fee_rate: String,
    /// Taker fixed fee
    pub taker_fixed_fee: String,
    /// Maker fixed fee  
    pub maker_fixed_fee: String,
    /// Order price
    pub price: String,
    /// Stop type
    pub stop: String,
    /// Order time
    pub created_at: i64,
    /// Update time
    pub updated_at: i64,
    /// End time
    pub end_at: Option<i64>,
    /// Order status
    pub status: OrderStatus,
    /// Time in force
    pub time_in_force: String,
    /// Post only flag
    pub post_only: bool,
    /// Hidden flag
    pub hidden: bool,
    /// Iceberg flag
    pub iceberg: bool,
    /// Leverage
    pub leverage: String,
    /// Force hold flag
    pub force_hold: bool,
    /// Close order flag
    pub close_order: bool,
    /// Visible size
    pub visible_size: String,
    /// Client order ID
    pub client_oid: String,
    /// Remark
    pub remark: String,
    /// Tags
    pub tags: String,
    /// Reduce only flag
    pub reduce_only: bool,
    /// Stop price type
    pub stop_price_type: String,
    /// Settlement currency
    pub settle_currency: String,
}

/// Response for getting recent closed orders
pub type GetRecentClosedOrdersResponse = Vec<RecentClosedOrderItem>;

impl super::RestClient {
    /// Get recent closed orders (last 1000 orders in 24 hours)
    ///
    /// <https://www.kucoin.com/docs-new/rest/futures-trading/orders/get-recent-closed-orders>
    pub async fn get_recent_closed_orders(
        &self,
        request: GetRecentClosedOrdersRequest,
    ) -> Result<(RestResponse<GetRecentClosedOrdersResponse>, ResponseHeaders)> {
        const GET_RECENT_CLOSED_ORDERS_ENDPOINT: &str = "/api/v1/recentDoneOrders";

        let params = if let Some(symbol) = request.symbol {
            let mut params = std::collections::HashMap::new();
            params.insert("symbol".to_string(), symbol);
            Some(params)
        } else {
            None
        };

        self.get(GET_RECENT_CLOSED_ORDERS_ENDPOINT, params.as_ref())
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_recent_closed_orders_request_creation() {
        let request = GetRecentClosedOrdersRequest {
            symbol: Some("XBTUSDTM".to_string()),
        };
        assert_eq!(request.symbol, Some("XBTUSDTM".to_string()));
    }

    #[test]
    fn test_get_recent_closed_orders_request_without_symbol() {
        let request = GetRecentClosedOrdersRequest { symbol: None };
        assert!(request.symbol.is_none());
    }

    #[test]
    fn test_recent_closed_order_item_deserialization() {
        let json = r#"{
            "id": "5e8c8c2f1a3b4a001c5d8e31",
            "symbol": "XBTUSDTM",
            "type": "limit",
            "side": "buy",
            "size": "1",
            "value": "50000",
            "dealValue": "49950",
            "dealSize": "1",
            "takerFeeRate": "0.0006",
            "makerFeeRate": "0.0002",
            "takerFixedFee": "0",
            "makerFixedFee": "0",
            "price": "50000",
            "stop": "",
            "createdAt": 1700000000000,
            "updatedAt": 1700000001000,
            "endAt": 1700000002000,
            "status": "done",
            "timeInForce": "GTC",
            "postOnly": false,
            "hidden": false,
            "iceberg": false,
            "leverage": "10",
            "forceHold": false,
            "closeOrder": false,
            "visibleSize": "1",
            "clientOid": "test123",
            "remark": "test order",
            "tags": "",
            "reduceOnly": false,
            "stopPriceType": "",
            "settleCurrency": "USDT"
        }"#;

        let item: RecentClosedOrderItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.id, "5e8c8c2f1a3b4a001c5d8e31");
        assert_eq!(item.symbol, "XBTUSDTM");
        assert_eq!(item.side, OrderSide::Buy);
        assert_eq!(item.status, OrderStatus::Done);
        assert_eq!(item.created_at, 1700000000000);
        assert_eq!(item.end_at, Some(1700000002000));
    }
}
