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
    pub last: Option<String>,

    /// Change percentage (24h)
    pub change_percentage: Option<String>,

    /// Total size (24h)
    pub total_size: Option<String>,

    /// Lowest ask
    pub lowest_ask: Option<String>,

    /// Highest bid
    pub highest_bid: Option<String>,

    /// Mark price
    pub mark_price: Option<String>,

    /// Mark IV (implied volatility)
    pub mark_iv: Option<String>,

    /// Index price
    pub index_price: Option<String>,

    /// Bid IV
    pub bid_iv: Option<String>,

    /// Ask IV
    pub ask_iv: Option<String>,

    /// Position size
    pub position_size: Option<i64>,

    /// Delta
    pub delta: Option<String>,

    /// Gamma
    pub gamma: Option<String>,

    /// Vega
    pub vega: Option<String>,

    /// Theta
    pub theta: Option<String>,

    /// Rho
    pub rho: Option<String>,
}

/// Underlying ticker information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnderlyingTicker {
    /// Trading enabled
    pub trade_enabled: Option<bool>,

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
