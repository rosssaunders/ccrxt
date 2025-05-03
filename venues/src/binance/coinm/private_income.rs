use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeQuery {
    pub symbol: Option<String>,
    pub income_type: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i32>,
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
    /// Get income history
    /// 
    /// # Arguments
    /// 
    /// * `query` - Query parameters for filtering income history
    /// 
    /// # Returns
    /// 
    /// List of income records matching the query criteria
    pub async fn get_income_history(&self, query: IncomeQuery) -> BinanceCoinMResult<Vec<Income>> {
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

        let url = format!("{}/dapi/v1/income?{}", self.base_url, query_str);

        let response = self.client
            .get(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(BinanceCoinMError::from_response(response).await);
        }

        let income: Vec<Income> = response.json().await?;
        Ok(income)
    }
} 