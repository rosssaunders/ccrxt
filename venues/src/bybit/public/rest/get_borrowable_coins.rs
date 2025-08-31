use serde::{Deserialize, Serialize};

use crate::bybit::{EndpointType, PublicRestClient, RestResult};

const BORROWABLE_COINS_ENDPOINT: &str = "/v5/crypto-loan/loanable-data";

/// Request parameters for getting borrowable coins.
///
/// This is an empty request as the endpoint doesn't require any parameters.
#[derive(Debug, Clone, Serialize)]
pub struct GetBorrowableCoinsRequest;

/// Information about a borrowable coin.
#[derive(Debug, Clone, Deserialize)]
pub struct BorrowableCoinInfo {
    /// Symbol of the coin available for borrowing (e.g., "USDT", "USDC")
    #[serde(rename = "loanCoin")]
    pub loan_coin: String,

    /// Maximum amount available to borrow
    #[serde(rename = "maxLoanAmount")]
    pub max_loan_amount: String,

    /// Hourly interest rate for borrowing (as a decimal, e.g., "0.0001" = 0.01%)
    #[serde(rename = "hourlyBorrowRate")]
    pub hourly_borrow_rate: String,

    /// Minimum amount required to borrow
    #[serde(rename = "minLoanAmount")]
    pub min_loan_amount: String,
}

/// Container for the list of borrowable coins.
#[derive(Debug, Clone, Deserialize)]
pub struct GetBorrowableCoinsData {
    /// List of all borrowable coins with their details
    pub list: Vec<BorrowableCoinInfo>,
}

/// Response from the get borrowable coins API endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct GetBorrowableCoinsResponse {
    /// Return code (0 indicates success)
    #[serde(rename = "retCode")]
    pub ret_code: i32,

    /// Return message
    #[serde(rename = "retMsg")]
    pub ret_msg: String,

    /// Borrowable coins data
    pub result: GetBorrowableCoinsData,

    /// Extended information (varies by endpoint)
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,

    /// Response timestamp in milliseconds
    pub time: u64,
}

impl PublicRestClient {
    /// Get borrowable coins
    ///
    /// Query the list of coins that are available for crypto loans including their
    /// borrowing rates and limits.
    ///
    /// [docs](https://bybit-exchange.github.io/docs/v5/crypto-loan/loanable-coin)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Returns
    /// A result containing the list of borrowable coins with rates and limits
    pub async fn get_borrowable_coins(&self) -> RestResult<GetBorrowableCoinsResponse> {
        self.send_public_request(
            BORROWABLE_COINS_ENDPOINT,
            None::<&GetBorrowableCoinsRequest>,
            EndpointType::Market,
        )
        .await
    }
}
