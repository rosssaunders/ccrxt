use crate::bitget::enums::ProductType;
use rest::Client;
use serde::{Deserialize, Serialize};

/// Get Recent Transactions
/// 
/// Frequency limit: 20 times/1s (IP)
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentTransactionsRequest {
    /// Trading pair
    pub symbol: String,
    /// Product type
    pub product_type: ProductType,
    /// Number of transaction records to return, default 100, maximum 500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionData {
    /// Trading pair
    pub symbol: String,
    /// Trade ID
    pub trade_id: String,
    /// Order side
    pub side: String,
    /// Fill quantity
    pub size: String,
    /// Fill price
    pub price: String,
    /// Fill time, milliseconds format
    pub ts: String,
}

pub async fn recent_transactions(
    client: &Client,
    params: &RecentTransactionsRequest,
) -> Result<Vec<TransactionData>, crate::bitget::Errors> {
    let endpoint = "/api/v2/mix/market/fills";
    client.get(endpoint, Some(params)).await.map_err(Into::into)
}
