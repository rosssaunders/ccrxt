use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for options tickers
#[derive(Debug, Clone, Serialize, Default)]
pub struct OptionsTickersRequest {
    /// Underlying asset filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,
}

/// Options ticker information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsTicker {
    /// Contract name
    pub name: String,

    /// Last trading price
    pub last: String,

    /// Change percentage (24h)
    pub change_percentage: String,

    /// Total size (24h)
    pub total_size: String,

    /// Lowest ask
    pub lowest_ask: String,

    /// Highest bid
    pub highest_bid: String,

    /// Mark price
    pub mark_price: String,

    /// Mark IV (implied volatility)
    pub mark_iv: String,

    /// Index price
    pub index_price: String,

    /// Bid IV
    pub bid_iv: String,

    /// Ask IV
    pub ask_iv: String,

    /// Position size
    pub position_size: i64,

    /// Delta
    pub delta: String,

    /// Gamma
    pub gamma: String,

    /// Vega
    pub vega: String,

    /// Theta
    pub theta: String,

    /// Rho
    pub rho: String,
}

/// Underlying ticker information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnderlyingTicker {
    /// Trading enabled
    pub trade_enabled: bool,

    /// Index price (quote currency)
    pub index_price: String,

    /// Total put options trades amount in last 24h
    pub trade_put: i64,

    /// Total call options trades amount in last 24h
    pub trade_call: i64,
}

impl RestClient {
    /// List tickers of options contracts
    ///
    /// Retrieves ticker information for options contracts.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#list-tickers-of-options-contracts>
    pub async fn get_options_tickers(
        &self,
        params: OptionsTickersRequest,
    ) -> crate::gateio::options::Result<Vec<OptionsTicker>> {
        self.get_with_query("/options/tickers", Some(&params)).await
    }

    /// Get underlying ticker
    ///
    /// Retrieves ticker information for a specific underlying asset.
    pub async fn get_underlying_ticker(
        &self,
        underlying: &str,
    ) -> crate::gateio::options::Result<UnderlyingTicker> {
        let endpoint = format!("/options/underlying/tickers/{}", underlying);
        self.get(&endpoint).await
    }
}
