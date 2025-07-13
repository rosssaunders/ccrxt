//! Trading statistics and trade history functionality
use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for getting personal trading history
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetMyTradesRequest {
    /// Currency pair
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,
    /// Limit the number of records
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Account type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Start timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    /// End timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

/// Personal trade information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyTrade {
    /// Trade ID
    pub id: String,
    /// Trading time
    pub create_time: String,
    /// Trading time in milliseconds
    pub create_time_ms: String,
    /// Currency pair
    pub currency_pair: String,
    /// Order ID
    pub order_id: String,
    /// Trade side
    pub side: String,
    /// Trade role (taker/maker)
    pub role: String,
    /// Trade amount
    pub amount: String,
    /// Trade price
    pub price: String,
    /// Trade fee
    pub fee: String,
    /// Fee currency
    pub fee_currency: String,
    /// Point fee
    pub point_fee: String,
    /// GT fee
    pub gt_fee: String,
    /// Whether GT fee is used
    pub gt_fee_deduction: bool,
    /// Rebated fee
    pub rebated_fee: String,
    /// Rebated fee currency
    pub rebated_fee_currency: String,
    /// Text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// Implementation for the client
impl RestClient {
    /// Get personal trading history
    ///
    /// This endpoint returns your personal trading history.
    /// You can filter by currency pair, time range, and other parameters.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#list-personal-trading-history>
    pub async fn get_my_trades(
        &self,
        request: GetMyTradesRequest,
    ) -> crate::gateio::spot::Result<Vec<MyTrade>> {
        self.get_with_query("/spot/my_trades", &request).await
    }

    /// Get all personal trades for a currency pair
    pub async fn get_my_trades_for_pair(
        &self,
        currency_pair: &str,
        limit: Option<u32>,
    ) -> crate::gateio::spot::Result<Vec<MyTrade>> {
        let request = GetMyTradesRequest {
            currency_pair: Some(currency_pair.to_string()),
            limit,
            ..Default::default()
        };
        self.get_my_trades(request).await
    }

    /// Get trades for a specific order
    pub async fn get_order_trades(
        &self,
        order_id: &str,
        currency_pair: &str,
    ) -> crate::gateio::spot::Result<Vec<MyTrade>> {
        let request = GetMyTradesRequest {
            currency_pair: Some(currency_pair.to_string()),
            order_id: Some(order_id.to_string()),
            ..Default::default()
        };
        self.get_my_trades(request).await
    }

    /// Get trades within a time range
    pub async fn get_my_trades_in_range(
        &self,
        currency_pair: Option<&str>,
        from: i64,
        to: i64,
        limit: Option<u32>,
    ) -> crate::gateio::spot::Result<Vec<MyTrade>> {
        let request = GetMyTradesRequest {
            currency_pair: currency_pair.map(|s| s.to_string()),
            from: Some(from),
            to: Some(to),
            limit,
            ..Default::default()
        };
        self.get_my_trades(request).await
    }

    /// Get recent trades (last 24 hours)
    pub async fn get_recent_my_trades(
        &self,
        currency_pair: Option<&str>,
        limit: Option<u32>,
    ) -> crate::gateio::spot::Result<Vec<MyTrade>> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let yesterday = now - 86400; // 24 hours ago

        self.get_my_trades_in_range(currency_pair, yesterday, now, limit)
            .await
    }
}
