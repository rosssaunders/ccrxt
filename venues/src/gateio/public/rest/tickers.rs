use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for retrieving tickers
#[derive(Debug, Clone, Serialize, Default)]
pub struct TickersRequest {
    /// Currency pair to query ticker for (if omitted, returns all tickers)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,

    /// Timezone for the response (e.g., "utc0", "utc8")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

/// Ticker information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker {
    /// Currency pair
    pub currency_pair: String,

    /// Last trading price
    pub last: String,

    /// Lowest ask price
    pub lowest_ask: String,

    /// Highest bid price
    pub highest_bid: String,

    /// Change percentage in the last 24h
    pub change_percentage: String,

    /// Change amount in the last 24h
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_utc0: Option<String>,

    /// Change amount in the last 24h in given timezone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_utc8: Option<String>,

    /// Base currency traded volume in the last 24h
    pub base_volume: String,

    /// Quote currency traded volume in the last 24h
    pub quote_volume: String,

    /// Highest price in the last 24h
    pub high_24h: String,

    /// Lowest price in the last 24h
    pub low_24h: String,

    /// ETF net value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etf_net_value: Option<String>,

    /// ETF previous close
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etf_pre_net_value: Option<String>,

    /// ETF rebalance time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etf_pre_timestamp: Option<i64>,

    /// ETF leverage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etf_leverage: Option<String>,
}

impl RestClient {
    /// Get tickers for all or specific currency pairs
    ///
    /// This endpoint returns ticker information including 24h price changes,
    /// volumes, and current bid/ask prices. You can get all tickers or filter
    /// by a specific currency pair and timezone.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#retrieve-ticker-information>
    pub async fn get_tickers(&self, params: TickersRequest) -> crate::gateio::Result<Vec<Ticker>> {
        self.get_with_query("/spot/tickers", Some(&params)).await
    }
}
