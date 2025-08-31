use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize};

use crate::bitget::{ApiError, PricePrecision, PublicRestClient as RestClient, RestResponse};

/// Endpoint for getting merge depth data
const MERGE_DEPTH_ENDPOINT: &str = "/api/v2/spot/market/merge-depth";

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
                    #[allow(clippy::indexing_slicing)]
                    let price = match &entry[0] {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        _ => return Err(serde::de::Error::custom("Invalid price format")),
                    };
                    #[allow(clippy::indexing_slicing)]
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
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMergeDepthRequest {
    /// Trading pair
    pub symbol: String,
    /// Price precision
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<PricePrecision>,
    /// Limit number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
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
    /// Returns merged orderbook depth for a symbol.
    ///
    /// [docs](https://www.bitget.com/api-doc/spot/market/Get-Merge-Depth)
    ///
    /// Rate limit: see official docs
    ///
    /// # Arguments
    /// * `request` - The request parameters
    ///
    /// # Returns
    /// The merge depth information
    pub async fn get_merge_depth(
        &self,
        request: &GetMergeDepthRequest,
    ) -> Result<RestResponse<MergeDepth>, ApiError> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), request.symbol.clone());

        if let Some(precision) = &request.precision {
            params.insert("precision".to_string(), precision.to_string());
        }

        if let Some(limit) = request.limit {
            params.insert("limit".to_string(), limit.to_string());
        }

        self.get(MERGE_DEPTH_ENDPOINT, Some(params)).await
    }
}
