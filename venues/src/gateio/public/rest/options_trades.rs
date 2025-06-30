use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for options trades
#[derive(Debug, Clone, Serialize, Default)]
pub struct OptionsTradesRequest {
    /// Contract name
    pub contract: String,
    
    /// Filter trades after this ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,
    
    /// Maximum number of records to return (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Options trade information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsTrade {
    /// Trade ID
    pub id: i64,
    
    /// Trade timestamp
    pub create_time: f64,
    
    /// Contract name
    pub contract: String,
    
    /// Trade size
    pub size: String,
    
    /// Trading price (quote currency)
    pub price: String,
    
    /// Whether internal trade
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_internal: Option<bool>,
}

impl RestClient {
    /// Options trade history
    ///
    /// Retrieves recent trade history for a specific options contract.
    pub async fn get_options_trades(&self, params: OptionsTradesRequest) -> crate::gateio::Result<Vec<OptionsTrade>> {
        self.get_with_query("/options/trades", Some(&params)).await
    }
}