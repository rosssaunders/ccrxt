use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderAmendment {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub client_order_id: Option<String>,
    pub price: Option<String>,
    pub quantity: Option<String>,
    pub activation_price: Option<String>,
    pub callback_rate: Option<String>,
    pub working_type: Option<String>,
    pub price_protect: Option<bool>,
    pub new_client_order_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderAmendmentResponse {
    pub order_id: i64,
    pub symbol: String,
    pub status: String,
    pub client_order_id: String,
    pub price: String,
    pub avg_price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cum_qty: String,
    pub cum_quote: String,
    pub time_in_force: String,
    pub type_: String,
    pub reduce_only: bool,
    pub close_position: bool,
    pub side: String,
    pub position_side: String,
    pub stop_price: String,
    pub working_type: String,
    pub price_protect: bool,
    pub orig_type: String,
    pub update_time: i64,
}

impl BinanceCoinMPrivateRest {
    /// Amend an existing order
    /// 
    /// # Arguments
    /// 
    /// * `amendment` - Order amendment details
    /// 
    /// # Returns
    /// 
    /// Updated order information
    pub async fn amend_order(&self, amendment: OrderAmendment) -> BinanceCoinMResult<OrderAmendmentResponse> {
        let mut query = format!("symbol={}", amendment.symbol);

        if let Some(id) = amendment.order_id {
            query.push_str(&format!("&orderId={}", id));
        }
        if let Some(id) = amendment.client_order_id {
            query.push_str(&format!("&origClientOrderId={}", id));
        }
        if let Some(price) = amendment.price {
            query.push_str(&format!("&price={}", price));
        }
        if let Some(qty) = amendment.quantity {
            query.push_str(&format!("&quantity={}", qty));
        }
        if let Some(price) = amendment.activation_price {
            query.push_str(&format!("&activationPrice={}", price));
        }
        if let Some(rate) = amendment.callback_rate {
            query.push_str(&format!("&callbackRate={}", rate));
        }
        if let Some(working_type) = amendment.working_type {
            query.push_str(&format!("&workingType={}", working_type));
        }
        if let Some(protect) = amendment.price_protect {
            query.push_str(&format!("&priceProtect={}", protect));
        }
        if let Some(id) = amendment.new_client_order_id {
            query.push_str(&format!("&newClientOrderId={}", id));
        }

        let timestamp = chrono::Utc::now().timestamp_millis();
        query.push_str(&format!("&timestamp={}", timestamp));

        let signature = self.sign_request(&query);
        query.push_str(&format!("&signature={}", signature));

        let url = format!("{}/dapi/v1/order?{}", self.base_url, query);

        let response = self.client
            .put(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(BinanceCoinMError::from_response(response).await);
        }

        let order: OrderAmendmentResponse = response.json().await?;
        Ok(order)
    }
} 