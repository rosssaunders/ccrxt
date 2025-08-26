use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const MY_TRADES_ENDPOINT: &str = "/spot/my_trades";

/// Request parameters for retrieving personal spot trading history
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetMyTradesRequest {
    /// Trading pair filter (e.g., "BTC_USDT")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,

    /// Maximum number of records to return (default: 100, max: 1000)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Page number for pagination (default: 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Filter by specific order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// Account type filter (e.g., "spot", "margin")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// Start time filter (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time filter (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

/// Personal spot trading record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyTrade {
    /// Trade ID
    pub id: String,

    /// Trade creation time (Unix timestamp in seconds)
    pub create_time: String,

    /// Trade creation time in milliseconds
    pub create_time_ms: String,

    /// Trading pair symbol
    pub currency_pair: String,

    /// Order ID that generated this trade
    pub order_id: String,

    /// Trade side: "buy" or "sell"
    pub side: String,

    /// Trade role: "taker" or "maker"
    pub role: String,

    /// Trade quantity
    pub amount: String,

    /// Trade price
    pub price: String,

    /// Trading fee paid
    pub fee: String,

    /// Currency of the trading fee
    pub fee_currency: String,

    /// Point fee (discount fee using points)
    pub point_fee: String,

    /// GT fee (discount fee using GT tokens)
    pub gt_fee: String,

    /// Whether GT fee deduction is enabled
    pub gt_fee_deduction: bool,

    /// Rebated fee amount
    pub rebated_fee: String,

    /// Currency of the rebated fee
    pub rebated_fee_currency: String,

    /// Custom order text label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl RestClient {
    /// List Personal Trading History
    ///
    /// Retrieve personal spot trading history with comprehensive filtering options.
    /// Supports pagination and time-based filtering for efficient data retrieval.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#list-personal-trading-history)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - Request parameters including filters and pagination options
    ///
    /// # Returns
    /// List of personal trade records with detailed trade information
    pub async fn get_my_trades(
        &self,
        request: Option<GetMyTradesRequest>,
    ) -> RestResult<Vec<MyTrade>> {
        self.send_get_request(MY_TRADES_ENDPOINT, request.as_ref())
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_my_trades_request_minimal_serialization() {
        let request = GetMyTradesRequest::default();

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }
}
