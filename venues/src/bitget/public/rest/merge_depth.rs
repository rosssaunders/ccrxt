use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

use super::RestClient;
use crate::bitget::{ApiError, PricePrecision, RestResponse};

/// Custom deserializer for order book entries that can be mixed number/string arrays
fn deserialize_order_book_entries<'de, D>(deserializer: D) -> Result<Vec<[String; 2]>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde_json::Value;
    let value = Value::deserialize(deserializer)?;

    if let Value::Array(arr) = value {
        let mut result = Vec::new();
        for item in arr {
            if let Value::Array(entry) = item {
                if entry.len() == 2 {
                    let price = match &entry[0] {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        _ => return Err(serde::de::Error::custom("Invalid price format")),
                    };
                    let quantity = match &entry[1] {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        _ => return Err(serde::de::Error::custom("Invalid quantity format")),
                    };
                    result.push([price, quantity]);
                } else {
                    return Err(serde::de::Error::custom(
                        "Entry must have exactly 2 elements",
                    ));
                }
            } else {
                return Err(serde::de::Error::custom("Entry must be an array"));
            }
        }
        Ok(result)
    } else {
        Err(serde::de::Error::custom("Expected array"))
    }
}

/// Request for getting merge depth
#[derive(Debug, Clone)]
pub struct GetMergeDepthRequest {
    /// Trading pair
    pub symbol: String,
    /// Price precision
    pub precision: Option<PricePrecision>,
    /// Limit number
    pub limit: Option<u32>,
}

impl GetMergeDepthRequest {
    /// Create a new request
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            precision: None,
            limit: None,
        }
    }

    /// Set precision
    pub fn precision(mut self, precision: PricePrecision) -> Self {
        self.precision = Some(precision);
        self
    }

    /// Set limit
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Convert to query parameters
    pub fn to_params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), self.symbol.clone());

        if let Some(precision) = &self.precision {
            params.insert("precision".to_string(), precision.to_string());
        }

        if let Some(limit) = self.limit {
            params.insert("limit".to_string(), limit.to_string());
        }

        params
    }
}

/// Merge depth information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MergeDepth {
    /// Ask depth
    #[serde(deserialize_with = "deserialize_order_book_entries")]
    pub asks: Vec<[String; 2]>,
    /// Bid depth
    #[serde(deserialize_with = "deserialize_order_book_entries")]
    pub bids: Vec<[String; 2]>,
    /// Current gear
    pub precision: String,
    /// Actual precision value
    pub scale: String,
    /// Is max precision
    #[serde(rename = "isMaxPrecision")]
    pub is_max_precision: String,
    /// Matching engine timestamp(ms)
    pub ts: String,
}

impl RestClient {
    /// Get merge depth
    ///
    /// # Arguments
    /// * `request` - The request parameters
    ///
    /// # Returns
    /// * `Result<RestResponse<MergeDepth>, ApiError>` - The merge depth information
    ///
    /// # Example
    /// ```rust
    /// use venues::bitget::public::rest::{RestClient, GetMergeDepthRequest};
    /// use venues::bitget::PricePrecision;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = RestClient::new("https://api.bitget.com", Default::default(), reqwest::Client::new());
    ///
    /// let response = client.get_merge_depth(
    ///     GetMergeDepthRequest::new("BTCUSDT")
    ///         .precision(PricePrecision::Scale1)
    ///         .limit(50)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_merge_depth(
        &self,
        request: GetMergeDepthRequest,
    ) -> Result<RestResponse<MergeDepth>, ApiError> {
        let endpoint = "/api/v2/spot/market/merge-depth";
        let params = Some(request.to_params());
        self.get(endpoint, params).await
    }
}
