use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for futures index constituents
#[derive(Debug, Clone, Serialize, Default)]
pub struct FuturesIndexConstituentsRequest {
    /// Settlement currency
    pub settle: String,
    /// Index name
    pub index: String,
}

/// Index constituent information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexConstituent {
    /// Exchange name
    pub exchange: String,

    /// Trading pair
    pub symbol: Option<String>,

    /// Weight percentage
    pub weight: Option<String>,

    /// Price
    pub price: Option<String>,

    /// Last update time
    pub update_time: Option<i64>,
}

/// Index constituents response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexConstituentsResponse {
    /// Index name
    pub index: String,
    
    /// List of constituents
    pub constituents: Vec<IndexConstituent>,
}

impl RestClient {
    /// Get index constituents
    ///
    /// Retrieves the constituent exchanges and their weights for a specific index.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#get-index-constituents>
    pub async fn get_futures_index_constituents(
        &self,
        params: FuturesIndexConstituentsRequest,
    ) -> crate::gateio::perpetual::Result<IndexConstituentsResponse> {
        let endpoint = format!(
            "/futures/{}/index_constituents/{}",
            params.settle, params.index
        );
        self.get(&endpoint).await
    }
}
