use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeAsynQuery {
    pub symbol: Option<String>,
    pub income_type: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeAsynResponse {
    pub download_id: String,
}

impl BinanceCoinMPrivateRest {
    /// Get income history asynchronously
    /// 
    /// # Arguments
    /// 
    /// * `query` - Query parameters for filtering income history
    /// 
    /// # Returns
    /// 
    /// Download ID for retrieving the results
    pub async fn get_income_history_asyn(&self, query: IncomeAsynQuery) -> BinanceCoinMResult<IncomeAsynResponse> {
        let mut query_str = String::new();

        if let Some(sym) = query.symbol {
            query_str.push_str(&format!("symbol={}", sym));
        }
        if let Some(income_type) = query.income_type {
            if !query_str.is_empty() {
                query_str.push('&');
            }
            query_str.push_str(&format!("incomeType={}", income_type));
        }
        if let Some(time) = query.start_time {
            if !query_str.is_empty() {
                query_str.push('&');
            }
            query_str.push_str(&format!("startTime={}", time));
        }
        if let Some(time) = query.end_time {
            if !query_str.is_empty() {
                query_str.push('&');
            }
            query_str.push_str(&format!("endTime={}", time));
        }
        if let Some(limit) = query.limit {
            if !query_str.is_empty() {
                query_str.push('&');
            }
            query_str.push_str(&format!("limit={}", limit));
        }

        let timestamp = chrono::Utc::now().timestamp_millis();
        if !query_str.is_empty() {
            query_str.push('&');
        }
        query_str.push_str(&format!("timestamp={}", timestamp));

        let signature = self.sign_request(&query_str);
        query_str.push_str(&format!("&signature={}", signature));

        let url = format!("{}/dapi/v1/income/asyn?{}", self.base_url, query_str);

        let response = self.client
            .get(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(BinanceCoinMError::from_response(response).await);
        }

        let result: IncomeAsynResponse = response.json().await?;
        Ok(result)
    }
} 