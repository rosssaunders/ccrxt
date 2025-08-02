use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::{AllocationType, RestResult};

const GET_MY_ALLOCATIONS_ENDPOINT: &str = "/api/v3/myAllocations";

/// Request parameters for getting account allocations
#[derive(Debug, Clone, Serialize)]
pub struct MyAllocationsRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Start time timestamp in ms
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time timestamp in ms
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Allocation ID to fetch from
    #[serde(rename = "fromAllocationId", skip_serializing_if = "Option::is_none")]
    pub from_allocation_id: Option<u64>,

    /// Default 500; max 1000
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Order ID
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Account allocation information
#[derive(Debug, Clone, Deserialize)]
pub struct MyAllocation {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Allocation ID
    #[serde(rename = "allocationId")]
    pub allocation_id: u64,

    /// Allocation type
    #[serde(rename = "allocationType")]
    pub allocation_type: AllocationType,

    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Order list ID
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,

    /// Allocation price
    #[serde(rename = "price")]
    pub price: Decimal,

    /// Allocation quantity
    #[serde(rename = "qty")]
    pub qty: Decimal,

    /// Quote quantity
    #[serde(rename = "quoteQty")]
    pub quote_qty: Decimal,

    /// Commission amount
    #[serde(rename = "commission")]
    pub commission: Decimal,

    /// Commission asset
    #[serde(rename = "commissionAsset")]
    pub commission_asset: String,

    /// Allocation time
    #[serde(rename = "time")]
    pub time: u64,

    /// Is buyer
    #[serde(rename = "isBuyer")]
    pub is_buyer: bool,

    /// Is maker
    #[serde(rename = "isMaker")]
    pub is_maker: bool,
}

impl RestClient {
    /// Retrieve allocations resulting from SOR order placement
    ///
    /// Retrieve allocations resulting from SOR order placement.
    ///
    /// [docs]: (https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#query-allocations--user_data)
    /// Method: GET /api/v3/myAllocations
    /// Weight: 20
    /// Security: USER_DATA
    pub async fn get_my_allocations(
        &self,
        params: MyAllocationsRequest,
    ) -> RestResult<Vec<MyAllocation>> {
        self.send_get_signed_request(
            GET_MY_ALLOCATIONS_ENDPOINT,
            params,
            20,
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_allocations_request_serialization_minimal() {
        let request = MyAllocationsRequest {
            symbol: "BTCUSDT".to_string(),
            start_time: None,
            end_time: None,
            from_allocation_id: None,
            limit: None,
            order_id: None,
            recv_window: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTCUSDT");
    }

    #[test]
    fn test_my_allocations_request_serialization_with_time_range() {
        let request = MyAllocationsRequest {
            symbol: "ETHUSDT".to_string(),
            start_time: Some(1625184000000),
            end_time: Some(1625270400000),
            from_allocation_id: None,
            limit: None,
            order_id: None,
            recv_window: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETHUSDT"));
        assert!(serialized.contains("startTime=1625184000000"));
        assert!(serialized.contains("endTime=1625270400000"));
    }

    #[test]
    fn test_my_allocations_request_serialization_with_order_id() {
        let request = MyAllocationsRequest {
            symbol: "BNBUSDT".to_string(),
            start_time: None,
            end_time: None,
            from_allocation_id: None,
            limit: None,
            order_id: Some(123456789),
            recv_window: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BNBUSDT"));
        assert!(serialized.contains("orderId=123456789"));
    }

    #[test]
    fn test_my_allocations_request_serialization_with_pagination() {
        let request = MyAllocationsRequest {
            symbol: "ADAUSDT".to_string(),
            start_time: None,
            end_time: None,
            from_allocation_id: Some(987654321),
            limit: Some(500),
            order_id: None,
            recv_window: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ADAUSDT"));
        assert!(serialized.contains("fromAllocationId=987654321"));
        assert!(serialized.contains("limit=500"));
    }

    #[test]
    fn test_my_allocations_request_serialization_full() {
        let request = MyAllocationsRequest {
            symbol: "DOTUSDT".to_string(),
            start_time: Some(1625184000000),
            end_time: Some(1625270400000),
            from_allocation_id: Some(1000000),
            limit: Some(1000),
            order_id: Some(2000000),
            recv_window: Some(5000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=DOTUSDT"));
        assert!(serialized.contains("startTime=1625184000000"));
        assert!(serialized.contains("endTime=1625270400000"));
        assert!(serialized.contains("fromAllocationId=1000000"));
        assert!(serialized.contains("limit=1000"));
        assert!(serialized.contains("orderId=2000000"));
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_my_allocations_request_serialization_with_recv_window() {
        let request = MyAllocationsRequest {
            symbol: "SOLUSDT".to_string(),
            start_time: None,
            end_time: None,
            from_allocation_id: None,
            limit: None,
            order_id: None,
            recv_window: Some(60000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=SOLUSDT"));
        assert!(serialized.contains("recvWindow=60000"));
        assert!(!serialized.contains("startTime"));
        assert!(!serialized.contains("endTime"));
        assert!(!serialized.contains("fromAllocationId"));
        assert!(!serialized.contains("limit"));
        assert!(!serialized.contains("orderId"));
    }

    #[test]
    fn test_my_allocation_deserialization_basic() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "allocationId": 123456789,
            "allocationType": "SOR",
            "orderId": 987654321,
            "orderListId": -1,
            "price": "50000.00000000",
            "qty": "0.10000000",
            "quoteQty": "5000.00000000",
            "commission": "0.00010000",
            "commissionAsset": "BTC",
            "time": 1625184000000,
            "isBuyer": true,
            "isMaker": false
        }"#;

        let allocation: MyAllocation = serde_json::from_str(json).unwrap();
        assert_eq!(allocation.symbol, "BTCUSDT");
        assert_eq!(allocation.allocation_id, 123456789);
        assert_eq!(allocation.order_id, 987654321);
        assert_eq!(allocation.order_list_id, -1);
        assert_eq!(allocation.price.to_string(), "50000.00000000");
        assert_eq!(allocation.qty.to_string(), "0.10000000");
        assert_eq!(allocation.quote_qty.to_string(), "5000.00000000");
        assert_eq!(allocation.commission.to_string(), "0.00010000");
        assert_eq!(allocation.commission_asset, "BTC");
        assert_eq!(allocation.time, 1625184000000);
        assert!(allocation.is_buyer);
        assert!(!allocation.is_maker);
    }

    #[test]
    fn test_my_allocation_deserialization_sell_order() {
        let json = r#"{
            "symbol": "ETHUSDT",
            "allocationId": 555666777,
            "allocationType": "SOR",
            "orderId": 111222333,
            "orderListId": 0,
            "price": "3500.50000000",
            "qty": "1.50000000",
            "quoteQty": "5250.75000000",
            "commission": "5.25075000",
            "commissionAsset": "USDT",
            "time": 1625270400000,
            "isBuyer": false,
            "isMaker": true
        }"#;

        let allocation: MyAllocation = serde_json::from_str(json).unwrap();
        assert_eq!(allocation.symbol, "ETHUSDT");
        assert_eq!(allocation.allocation_id, 555666777);
        assert_eq!(allocation.order_id, 111222333);
        assert_eq!(allocation.order_list_id, 0);
        assert_eq!(allocation.price.to_string(), "3500.50000000");
        assert_eq!(allocation.qty.to_string(), "1.50000000");
        assert_eq!(allocation.quote_qty.to_string(), "5250.75000000");
        assert_eq!(allocation.commission.to_string(), "5.25075000");
        assert_eq!(allocation.commission_asset, "USDT");
        assert_eq!(allocation.time, 1625270400000);
        assert!(!allocation.is_buyer);
        assert!(allocation.is_maker);
    }

    #[test]
    fn test_my_allocation_allocation_type_deserialization() {
        let json = r#"{
            "symbol": "BNBUSDT",
            "allocationId": 999888777,
            "allocationType": "SOR",
            "orderId": 444555666,
            "orderListId": -1,
            "price": "400.00000000",
            "qty": "5.00000000",
            "quoteQty": "2000.00000000",
            "commission": "0.00500000",
            "commissionAsset": "BNB",
            "time": 1625356800000,
            "isBuyer": true,
            "isMaker": true
        }"#;

        let allocation: MyAllocation = serde_json::from_str(json).unwrap();
        // Verify the allocation type is properly deserialized
        match allocation.allocation_type {
            AllocationType::Sor => {} // Expected
        }
    }

    #[test]
    fn test_my_allocation_with_zero_commission() {
        let json = r#"{
            "symbol": "ADAUSDT",
            "allocationId": 333444555,
            "allocationType": "SOR",
            "orderId": 777888999,
            "orderListId": -1,
            "price": "1.20000000",
            "qty": "100.00000000",
            "quoteQty": "120.00000000",
            "commission": "0.00000000",
            "commissionAsset": "ADA",
            "time": 1625443200000,
            "isBuyer": false,
            "isMaker": false
        }"#;

        let allocation: MyAllocation = serde_json::from_str(json).unwrap();
        assert_eq!(allocation.symbol, "ADAUSDT");
        assert_eq!(allocation.commission.to_string(), "0.00000000");
        assert_eq!(allocation.commission_asset, "ADA");
    }

    #[test]
    fn test_my_allocation_with_large_values() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "allocationId": 9999999999,
            "allocationType": "SOR",
            "orderId": 8888888888,
            "orderListId": 1234567890,
            "price": "100000.00000000",
            "qty": "10.00000000",
            "quoteQty": "1000000.00000000",
            "commission": "1.00000000",
            "commissionAsset": "USDT",
            "time": 1625529600000,
            "isBuyer": true,
            "isMaker": false
        }"#;

        let allocation: MyAllocation = serde_json::from_str(json).unwrap();
        assert_eq!(allocation.allocation_id, 9999999999);
        assert_eq!(allocation.order_id, 8888888888);
        assert_eq!(allocation.order_list_id, 1234567890);
        assert_eq!(allocation.price.to_string(), "100000.00000000");
        assert_eq!(allocation.qty.to_string(), "10.00000000");
        assert_eq!(allocation.quote_qty.to_string(), "1000000.00000000");
    }

    #[test]
    fn test_my_allocation_list_deserialization() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "allocationId": 111111111,
                "allocationType": "SOR",
                "orderId": 222222222,
                "orderListId": -1,
                "price": "45000.00000000",
                "qty": "0.50000000",
                "quoteQty": "22500.00000000",
                "commission": "0.00050000",
                "commissionAsset": "BTC",
                "time": 1625616000000,
                "isBuyer": true,
                "isMaker": false
            },
            {
                "symbol": "BTCUSDT",
                "allocationId": 333333333,
                "allocationType": "SOR",
                "orderId": 444444444,
                "orderListId": -1,
                "price": "45100.00000000",
                "qty": "0.30000000",
                "quoteQty": "13530.00000000",
                "commission": "0.00030000",
                "commissionAsset": "BTC",
                "time": 1625616100000,
                "isBuyer": true,
                "isMaker": true
            }
        ]"#;

        let allocations: Vec<MyAllocation> = serde_json::from_str(json).unwrap();
        assert_eq!(allocations.len(), 2);

        assert_eq!(allocations[0].allocation_id, 111111111);
        assert_eq!(allocations[0].price.to_string(), "45000.00000000");
        assert_eq!(allocations[0].qty.to_string(), "0.50000000");
        assert!(!allocations[0].is_maker);

        assert_eq!(allocations[1].allocation_id, 333333333);
        assert_eq!(allocations[1].price.to_string(), "45100.00000000");
        assert_eq!(allocations[1].qty.to_string(), "0.30000000");
        assert!(allocations[1].is_maker);
    }

    #[test]
    fn test_empty_allocation_list_deserialization() {
        let json = r#"[]"#;
        let allocations: Vec<MyAllocation> = serde_json::from_str(json).unwrap();
        assert_eq!(allocations.len(), 0);
    }
}
