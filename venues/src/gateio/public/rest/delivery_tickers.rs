use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for delivery tickers
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeliveryTickersRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name (optional - if not provided, returns all contracts)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,
}

/// Delivery ticker information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryTicker {
    /// Contract name
    pub contract: String,
    
    /// Last trading price
    pub last: String,
    
    /// Recent lowest ask
    pub lowest_ask: String,
    
    /// Recent highest bid
    pub highest_bid: String,
    
    /// Change percentage (24h)
    pub change_percentage: String,
    
    /// Change amount (24h)
    pub change_utc0: String,
    
    /// Change amount (UTC 8)
    pub change_utc8: String,
    
    /// Total trading volume (24h)
    pub total_size: String,
    
    /// Trading volume (24h in quote currency)
    pub volume_24h: String,
    
    /// Trading volume (24h in base currency)
    pub volume_24h_btc: String,
    
    /// Trading volume (24h in quote currency) 
    pub volume_24h_usd: String,
    
    /// Trading volume (24h in base currency)
    pub volume_24h_base: String,
    
    /// Trading volume (24h in quote currency)
    pub volume_24h_quote: String,
    
    /// Trading volume (24h in settlement currency, BTC denominated)
    pub volume_24h_settle: String,
    
    /// Mark price
    pub mark_price: String,
    
    /// Index price
    pub index_price: String,
    
    /// Trading enabled
    pub quanto_base_rate: String,
    
    /// Basis rate
    pub basis_rate: String,
    
    /// Basis value
    pub basis_value: String,
}

impl RestClient {
    /// List delivery tickers
    ///
    /// Retrieves ticker information for delivery contracts.
    /// If contract is not specified, returns tickers for all contracts in the settlement currency.
    pub async fn get_delivery_tickers(&self, params: DeliveryTickersRequest) -> crate::gateio::Result<Vec<DeliveryTicker>> {
        let endpoint = format!("/delivery/{}/tickers", params.settle);
        self.get_with_query(&endpoint, Some(&params)).await
    }
}