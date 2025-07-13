use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for options candlesticks
#[derive(Debug, Clone, Serialize, Default)]
pub struct OptionsCandlesticksRequest {
    /// Contract name
    pub contract: String,

    /// Start time (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Maximum number of records to return (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Interval time frame (10s, 1m, 5m, 15m, 30m, 1h, 4h, 8h, 1d, 7d, 30d)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
}

/// Options candlestick data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsCandlestick {
    /// Timestamp
    pub t: i64,

    /// Trade volume (unit: Quote currency, unit: underlying corresponding option price)
    pub v: String,

    /// Close price (quote currency, unit: underlying corresponding option price)
    pub c: String,

    /// Highest price (quote currency, unit: underlying corresponding option price)
    pub h: String,

    /// Lowest price (quote currency, unit: underlying corresponding option price)
    pub l: String,

    /// Open price (quote currency, unit: underlying corresponding option price)
    pub o: String,
}

/// Request parameters for underlying candlesticks
#[derive(Debug, Clone, Serialize, Default)]
pub struct UnderlyingCandlesticksRequest {
    /// Underlying asset name
    pub underlying: String,

    /// Start time (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Maximum number of records to return (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Interval time frame (10s, 1m, 5m, 15m, 30m, 1h, 4h, 8h, 1d, 7d, 30d)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
}

/// Underlying mark price candlestick data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnderlyingCandlestick {
    /// Timestamp
    pub t: i64,

    /// Close price (quote currency)
    pub c: String,

    /// Highest price (quote currency)
    pub h: String,

    /// Lowest price (quote currency)
    pub l: String,

    /// Open price (quote currency)
    pub o: String,

    /// Trading volume (unit: Quote currency)
    pub sum: String,
}

impl RestClient {
    /// Get options candlesticks
    ///
    /// Retrieves candlestick data for a specific options contract.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#get-options-candlesticks>
    pub async fn get_options_candlesticks(
        &self,
        params: OptionsCandlesticksRequest,
    ) -> crate::gateio::spotandmargin::Result<Vec<OptionsCandlestick>> {
        self.get_with_query("/options/candlesticks", Some(&params))
            .await
    }

    /// Mark price candlesticks of an underlying
    ///
    /// Retrieves mark price candlestick data for an underlying asset.
    pub async fn get_underlying_candlesticks(
        &self,
        params: UnderlyingCandlesticksRequest,
    ) -> crate::gateio::spotandmargin::Result<Vec<UnderlyingCandlestick>> {
        self.get_with_query("/options/underlying/candlesticks", Some(&params))
            .await
    }
}
