use serde::{Deserialize, Serialize};

use crate::bybit::{EndpointType, PublicRestClient, RestResult};

const INS_PRODUCT_INFO_ENDPOINT: &str = "/v5/ins-loan/product-infos";

/// Request parameters for getting institutional loan product info (no parameters required)
#[derive(Debug, Clone, Serialize)]
pub struct GetInsProductInfoRequest;

/// Institutional loan product information
#[derive(Debug, Clone, Deserialize)]
pub struct InsProductInfo {
    /// Product ID
    #[serde(rename = "productId")]
    pub product_id: String,

    /// Loan coin
    #[serde(rename = "loanCoin")]
    pub loan_coin: String,

    /// Loan amount
    #[serde(rename = "loanAmount")]
    pub loan_amount: String,

    /// Borrow rate (hourly rate)
    #[serde(rename = "borrowRate")]
    pub borrow_rate: String,

    /// Loan period (e.g., "7", "14", "30")
    #[serde(rename = "loanPeriod")]
    pub loan_period: String,
}

/// Institutional loan product info data container
#[derive(Debug, Clone, Deserialize)]
pub struct GetInsProductInfoData {
    /// List of institutional loan products
    pub list: Vec<InsProductInfo>,
}

/// Response from the institutional loan product info endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct GetInsProductInfoResponse {
    /// Success/Error code (0: success, 1: error)
    #[serde(rename = "retCode")]
    pub ret_code: i32,

    /// Success/Error message
    #[serde(rename = "retMsg")]
    pub ret_msg: String,

    /// Business data result
    pub result: GetInsProductInfoData,

    /// Extended information
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,

    /// Current timestamp in milliseconds
    pub time: u64,
}

impl PublicRestClient {
    /// Get institutional loan product info
    ///
    /// Query for all available institutional loan products, including loan amount,
    /// borrow rate, and loan period information.
    ///
    /// [docs](https://bybit-exchange.github.io/docs/v5/otc/margin-product-info)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// None - This endpoint does not require any parameters
    ///
    /// # Returns
    /// A result containing the institutional loan product info response with available products or an error
    pub async fn get_ins_product_info(&self) -> RestResult<GetInsProductInfoResponse> {
        self.send_public_request(
            INS_PRODUCT_INFO_ENDPOINT,
            None::<&GetInsProductInfoRequest>,
            EndpointType::Market,
        )
        .await
    }
}
