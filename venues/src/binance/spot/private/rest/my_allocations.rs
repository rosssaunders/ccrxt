use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::{AllocationType, RestResult};

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
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#query-allocations--user_data)
    /// Method: GET /api/v3/myAllocations
    /// Weight: 20
    /// Security: USER_DATA
    pub async fn get_my_allocations(
        &self,
        params: MyAllocationsRequest,
    ) -> RestResult<Vec<MyAllocation>> {
        let query_string = serde_urlencoded::to_string(&params)
            .map_err(|e| crate::binance::spot::Errors::Error(format!("URL encoding error: {e}")))?;

        self.send_request(
            "/api/v3/myAllocations",
            reqwest::Method::GET,
            Some(&query_string),
            None,
            20,
            false,
        )
        .await
    }
}
