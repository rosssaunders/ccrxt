use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::bitget::{ApiError, RestResponse, SymbolStatus};
use super::RestClient;

/// Request for getting symbol information
#[derive(Debug, Clone, Default)]
pub struct GetSymbolInfoRequest {
    /// Specific symbol to query, if empty returns all symbols
    pub symbol: Option<String>,
}

impl GetSymbolInfoRequest {
    /// Create a new request for all symbols
    pub fn new() -> Self {
        Self::default()
    }

    /// Set specific symbol to query
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }

    /// Convert to query parameters
    pub fn to_params(&self) -> Option<HashMap<String, String>> {
        let mut params = HashMap::new();
        
        if let Some(symbol) = &self.symbol {
            params.insert("symbol".to_string(), symbol.clone());
        }

        if params.is_empty() {
            None
        } else {
            Some(params)
        }
    }
}

/// Symbol information
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolInfo {
    /// Trading pair symbol
    pub symbol: String,
    /// Base coin
    pub base_coin: String,
    /// Quote coin
    pub quote_coin: String,
    /// Minimum order quantity
    pub min_trade_amount: String,
    /// Maximum order quantity
    pub max_trade_amount: String,
    /// Taker fee rate
    pub taker_fee_rate: String,
    /// Maker fee rate
    pub maker_fee_rate: String,
    /// Price precision
    pub price_precision: String,
    /// Quantity precision
    pub quantity_precision: String,
    /// Quote precision
    pub quote_precision: String,
    /// Symbol status
    pub status: SymbolStatus,
    /// Minimum trade amount in USDT
    #[serde(default)]
    pub min_trade_usdt: Option<String>,
    /// Buy limit price ratio
    pub buy_limit_price_ratio: String,
    /// Sell limit price ratio
    pub sell_limit_price_ratio: String,
    /// Area symbol
    pub area_symbol: String,
    /// Order quantity
    pub order_quantity: String,
    /// Open time
    pub open_time: String,
    /// Off time
    pub off_time: String,
}

impl RestClient {
    /// Get symbol information
    /// 
    /// # Arguments
    /// * `request` - The request parameters
    /// 
    /// # Returns
    /// * `Result<RestResponse<Vec<SymbolInfo>>, ApiError>` - The symbol information
    /// 
    /// # Example
    /// ```rust
    /// use venues::bitget::public::rest::{RestClient, GetSymbolInfoRequest};
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = RestClient::new("https://api.bitget.com", Default::default(), reqwest::Client::new());
    /// 
    /// // Get all symbols
    /// let response = client.get_symbol_info(GetSymbolInfoRequest::new()).await?;
    /// 
    /// // Get specific symbol
    /// let response = client.get_symbol_info(GetSymbolInfoRequest::new().symbol("BTCUSDT")).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_symbol_info(&self, request: GetSymbolInfoRequest) -> Result<RestResponse<Vec<SymbolInfo>>, ApiError> {
        let endpoint = "/api/v2/spot/public/symbols";
        let params = request.to_params();
        self.get(endpoint, params).await
    }
}
