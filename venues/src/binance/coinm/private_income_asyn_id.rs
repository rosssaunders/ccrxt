use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeAsynIdQuery {
    pub download_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeAsynIdResponse {
    pub status: String,
    pub data: Option<Vec<Income>>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Income {
    pub symbol: String,
    pub income_type: String,
    pub income: String,
    pub asset: String,
    pub info: String,
    pub time: i64,
    pub tran_id: i64,
    pub trade_id: Option<String>,
}

impl BinanceCoinMPrivateRest {
    /// Get income history by download ID
    /// 
    /// # Arguments
    /// 
    /// * `query` - Query parameters containing the download ID
    /// 
    /// # Returns
    /// 
    /// Status and data of the asynchronous income history request
    pub async fn get_income_history_asyn_id(&self, query: IncomeAsynIdQuery) -> BinanceCoinMResult<IncomeAsynIdResponse> {
        let mut query_str = format!("downloadId={}", query.download_id);

        let timestamp = chrono::Utc::now().timestamp_millis();
        query_str.push_str(&format!("&timestamp={}", timestamp));

        let signature = self.sign_request(&query_str);
        query_str.push_str(&format!("&signature={}", signature));

        let url = format!("{}/dapi/v1/income/asyn/id?{}", self.base_url, query_str);

        let response = self.client
            .get(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(BinanceCoinMError::from_response(response).await);
        }

        let result: IncomeAsynIdResponse = response.json().await?;
        Ok(result)
    }
} 