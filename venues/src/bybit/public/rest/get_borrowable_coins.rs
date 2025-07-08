use super::client::RestClient;
use crate::bybit::{EndpointType, RestResult};
use serde::{Deserialize, Serialize};

const BORROWABLE_COINS_ENDPOINT: &str = "/v5/crypto-loan/loanable-data";

#[derive(Debug, Clone, Serialize)]
pub struct GetBorrowableCoinsRequest;

#[derive(Debug, Clone, Deserialize)]
pub struct BorrowableCoinInfo {
    #[serde(rename = "loanCoin")]
    pub loan_coin: String,
    #[serde(rename = "maxLoanAmount")]
    pub max_loan_amount: String,
    #[serde(rename = "hourlyBorrowRate")]
    pub hourly_borrow_rate: String,
    #[serde(rename = "minLoanAmount")]
    pub min_loan_amount: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetBorrowableCoinsData {
    pub list: Vec<BorrowableCoinInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetBorrowableCoinsResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: GetBorrowableCoinsData,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl RestClient {
    pub async fn get_borrowable_coins(&self) -> RestResult<GetBorrowableCoinsResponse> {
        self.send_public_request(
            BORROWABLE_COINS_ENDPOINT,
            None::<&GetBorrowableCoinsRequest>,
            EndpointType::Market,
        )
        .await
    }
}
