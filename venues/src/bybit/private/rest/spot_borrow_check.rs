use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bybit::{EndpointType, RestResult, enums::*};

#[derive(Debug, Clone, Serialize)]
pub struct SpotBorrowCheckRequest {
    pub category: Category,
    pub symbol: String,
    pub side: Side,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotBorrowCheckData {
    pub symbol: String,
    pub side: Side,
    pub max_trade_qty: String,
    pub max_trade_amount: String,
    pub spot_max_trade_qty: String,
    pub spot_max_trade_amount: String,
    pub borrow_coin: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SpotBorrowCheckResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: SpotBorrowCheckData,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl RestClient {
    /// Spot borrow check
    ///
    /// Query available balance for Spot trading and Margin trading.
    ///
    /// # Arguments
    /// * `request` - The spot borrow check request parameters
    ///
    /// # Returns
    /// A result containing the spot borrow check response or an error
    pub async fn spot_borrow_check(
        &self,
        request: SpotBorrowCheckRequest,
    ) -> RestResult<SpotBorrowCheckResponse> {
        self.send_signed_request(
            "/v5/order/spot-borrow-check",
            reqwest::Method::GET,
            request,
            EndpointType::Trade,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spot_borrow_check_request() {
        let request = SpotBorrowCheckRequest {
            category: Category::Spot,
            symbol: "BTCUSDT".to_string(),
            side: Side::Buy,
        };

        assert_eq!(request.category, Category::Spot);
        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.side, Side::Buy);
    }

    #[test]
    fn test_spot_borrow_check_request_serialization() {
        let request = SpotBorrowCheckRequest {
            category: Category::Spot,
            symbol: "ETHUSDT".to_string(),
            side: Side::Sell,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"category\":\"spot\""));
        assert!(json.contains("\"symbol\":\"ETHUSDT\""));
        assert!(json.contains("\"side\":\"Sell\""));
    }
}
