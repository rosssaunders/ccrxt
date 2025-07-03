use crate::bitget::enums::ProductType;
use rest::Client;
use serde::{Deserialize, Serialize};

/// Get Merge Market Depth
/// 
/// Frequency limit: 20 times/1s (IP)
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketDepthRequest {
    /// Trading pair
    pub symbol: String,
    /// Product type
    pub product_type: ProductType,
    /// Price accuracy
    /// scale0/scale1/scale2/scale3
    /// 'scale0' is not merged, the default value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<String>,
    /// Fixed gear enumeration value: 1/5/15/50/max
    /// Default gear is 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketDepthData {
    /// The selling price (price, quantity)
    pub asks: Vec<Vec<f64>>,
    /// Buying price (price, quantity)
    pub bids: Vec<Vec<f64>>,
    /// Requested precision
    pub precision: String,
    /// Actual precision value
    pub scale: String,
    /// YES indicates that the current accuracy is the maximum, NO indicates that it is not the maximum accuracy
    pub is_max_precision: String,
    /// Matching engine timestamp(ms)
    pub ts: String,
}

pub async fn market_depth(
    client: &Client,
    params: &MarketDepthRequest,
) -> Result<MarketDepthData, crate::bitget::Errors> {
    let endpoint = "/api/v2/mix/market/merge-depth";
    client.get(endpoint, Some(params)).await.map_err(Into::into)
}
