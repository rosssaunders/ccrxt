//! Get fills endpoint for Coinbase Exchange REST API
//!
//! Get a list of fills (partial or complete matches on orders).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::coinbase::{
    EndpointType, RestResult,
    enums::{Liquidity, OrderSide},
};

use super::{RestClient, get_account_balances::PaginationInfo};

const FILLS_ENDPOINT: &str = "fills";

/// Request to get fills
#[derive(Debug, Clone, Serialize)]
pub struct GetFillsRequest {
    /// Limit to fills on a specific order. Either order_id or product_id is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// Limit to fills on a specific product. Either order_id or product_id is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,

    /// Limit on number of results to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// Used for pagination. Sets start cursor to before id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Used for pagination. Sets end cursor to after id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Market type which the order was filled in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_type: Option<String>,

    /// Search by minimum posted date time and is inclusive of time provided.
    /// Valid formats are either RFC3339, date or date time and must be after Unix Epoch time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateTime<Utc>>,

    /// Search by maximum posted date time and is inclusive of time provided.
    /// Valid formats are either RFC3339, date or date time and must be after Unix Epoch time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateTime<Utc>>,
}

/// Fill information
#[derive(Debug, Clone, Deserialize)]
pub struct Fill {
    /// ID of trade that created the fill
    pub trade_id: u64,

    /// Product ID the order was placed on
    pub product_id: String,

    /// Order ID
    pub order_id: String,

    /// User ID
    pub user_id: String,

    /// Profile ID that placed the order
    pub profile_id: String,

    /// Liquidity type (M = Maker, T = Taker, O = Other)
    pub liquidity: Liquidity,

    /// Price per unit of base currency
    pub price: String,

    /// Amount of base currency to buy/sell
    pub size: String,

    /// Fees paid on current filled amount
    pub fee: String,

    /// Timestamp of fill
    pub created_at: DateTime<Utc>,

    /// Order side
    pub side: OrderSide,

    /// True if funds have been exchanged and settled
    #[serde(default)]
    pub settled: bool,

    /// USD volume of the fill
    #[serde(default)]
    pub usd_volume: String,

    /// Market type which the order was filled in
    #[serde(default)]
    pub market_type: String,

    /// Funding currency which the order was filled in
    #[serde(default)]
    pub funding_currency: String,
}

/// Response from getting fills
pub type GetFillsResponse = Vec<Fill>;

impl RestClient {
    /// Get fills
    ///
    /// Get a list of fills. A fill is a partial or complete match on a specific order.
    /// Fees are recorded in two stages and the fee field indicates the fees charged for this individual fill.
    ///
    /// # Arguments
    /// * `request` - The fills request parameters
    ///
    /// # Returns
    /// A result containing the list of fills and pagination info or an error
    ///
    /// # API Key Permissions
    /// This endpoint requires either the "view" or "trade" permission.
    ///
    /// # Rate Limits
    /// This endpoint has a custom rate limit:
    /// - Requests per second per profile: 10
    /// - Requests per second per profile in bursts: Up to 20
    pub async fn get_fills(
        &self,
        request: &GetFillsRequest,
    ) -> RestResult<(GetFillsResponse, Option<PaginationInfo>)> {
        // Use custom endpoint type for fills due to different rate limits
        self.send_request_with_pagination(
            FILLS_ENDPOINT,
            reqwest::Method::GET,
            Some(request),
            EndpointType::PrivateFills,
        )
        .await
    }
}

impl Default for GetFillsRequest {
    fn default() -> Self {
        Self {
            order_id: None,
            product_id: None,
            limit: Some(100),
            before: None,
            after: None,
            market_type: None,
            start_date: None,
            end_date: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fills_request_serialization() {
        let request = GetFillsRequest {
            order_id: None,
            product_id: Some("BTC-USD".to_string()),
            limit: Some(50),
            before: None,
            after: None,
            market_type: None,
            start_date: None,
            end_date: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("product_id=BTC-USD"));
        assert!(serialized.contains("limit=50"));
        assert!(!serialized.contains("order_id")); // Should be omitted when None
    }

    #[test]
    fn test_fill_deserialization() {
        let json = r#"{
            "trade_id": 74,
            "product_id": "BTC-USD",
            "order_id": "d0c5340b-6d6c-49d9-b567-48c4bfca13d2",
            "user_id": "5844eceecf7e803e259d0365",
            "profile_id": "765d1549-9660-4be2-97d4-fa2d65fa3352",
            "liquidity": "T",
            "price": "50000.00",
            "size": "0.01",
            "fee": "0.25",
            "created_at": "2021-01-01T00:00:00.000Z",
            "side": "buy",
            "settled": true,
            "usd_volume": "500.00",
            "market_type": "spot",
            "funding_currency": "USD"
        }"#;

        let fill: Fill = serde_json::from_str(json).unwrap();
        assert_eq!(fill.trade_id, 74);
        assert_eq!(fill.product_id, "BTC-USD");
        assert_eq!(fill.order_id, "d0c5340b-6d6c-49d9-b567-48c4bfca13d2");
        assert_eq!(fill.liquidity, Liquidity::T);
        assert_eq!(fill.price, "50000.00");
        assert_eq!(fill.size, "0.01");
        assert_eq!(fill.fee, "0.25");
        assert_eq!(fill.side, OrderSide::Buy);
        assert!(fill.settled);
        assert_eq!(fill.usd_volume, "500.00");
    }

    #[test]
    fn test_get_fills_response_deserialization() {
        let json = r#"[{
            "trade_id": 74,
            "product_id": "BTC-USD",
            "order_id": "d0c5340b-6d6c-49d9-b567-48c4bfca13d2",
            "user_id": "5844eceecf7e803e259d0365",
            "profile_id": "765d1549-9660-4be2-97d4-fa2d65fa3352",
            "liquidity": "T",
            "price": "50000.00",
            "size": "0.01",
            "fee": "0.25",
            "created_at": "2021-01-01T00:00:00.000Z",
            "side": "buy",
            "settled": true,
            "usd_volume": "500.00",
            "market_type": "spot",
            "funding_currency": "USD"
        }]"#;

        let fills: GetFillsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(fills.len(), 1);
        assert_eq!(fills[0].trade_id, 74);
        assert_eq!(fills[0].product_id, "BTC-USD");
    }

    #[test]
    fn test_get_fills_request_validation() {
        // Test that either order_id or product_id should be provided
        let request_with_order = GetFillsRequest {
            order_id: Some("d0c5340b-6d6c-49d9-b567-48c4bfca13d2".to_string()),
            product_id: None,
            limit: Some(50),
            before: None,
            after: None,
            market_type: None,
            start_date: None,
            end_date: None,
        };

        let serialized = serde_urlencoded::to_string(&request_with_order).unwrap();
        assert!(serialized.contains("order_id=d0c5340b-6d6c-49d9-b567-48c4bfca13d2"));
        assert!(!serialized.contains("product_id"));

        let request_with_product = GetFillsRequest {
            order_id: None,
            product_id: Some("BTC-USD".to_string()),
            limit: Some(50),
            before: None,
            after: None,
            market_type: None,
            start_date: None,
            end_date: None,
        };

        let serialized = serde_urlencoded::to_string(&request_with_product).unwrap();
        assert!(serialized.contains("product_id=BTC-USD"));
        assert!(!serialized.contains("order_id"));
    }
}
