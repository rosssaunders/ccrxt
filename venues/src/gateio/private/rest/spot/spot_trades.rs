use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const MY_TRADES_ENDPOINT: &str = "/spot/my_trades";

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetMyTradesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyTrade {
    pub id: String,
    pub create_time: String,
    pub create_time_ms: String,
    pub currency_pair: String,
    pub order_id: String,
    pub side: String,
    pub role: String,
    pub amount: String,
    pub price: String,
    pub fee: String,
    pub fee_currency: String,
    pub point_fee: String,
    pub gt_fee: String,
    pub gt_fee_deduction: bool,
    pub rebated_fee: String,
    pub rebated_fee_currency: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl RestClient {
    pub async fn spot_get_my_trades(
        &self,
        request: GetMyTradesRequest,
    ) -> RestResult<Vec<MyTrade>> {
        self.get_with_query(MY_TRADES_ENDPOINT, &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_my_trades_request_minimal_serialization() {
        let request = GetMyTradesRequest::default();

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }
}
