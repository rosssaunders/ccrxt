use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::spot::RestResult;

use super::client::RestClient;

/// Request parameters for getting account commission rates
#[derive(Debug, Clone, Serialize)]
pub struct AccountCommissionRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Account commission rates response
#[derive(Debug, Clone, Deserialize)]
pub struct AccountCommissionResponse {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Standard commission rates
    #[serde(rename = "standardCommission")]
    pub standard_commission: CommissionRates,

    /// Tax commission rates
    #[serde(rename = "taxCommission")]
    pub tax_commission: CommissionRates,

    /// Discount information
    #[serde(rename = "discount")]
    pub discount: Discount,
}

/// Commission rates structure
#[derive(Debug, Clone, Deserialize)]
pub struct CommissionRates {
    /// Maker commission rate
    #[serde(rename = "maker")]
    pub maker: Decimal,

    /// Taker commission rate
    #[serde(rename = "taker")]
    pub taker: Decimal,

    /// Buyer commission rate
    #[serde(rename = "buyer")]
    pub buyer: Decimal,

    /// Seller commission rate
    #[serde(rename = "seller")]
    pub seller: Decimal,
}

/// Discount information
#[derive(Debug, Clone, Deserialize)]
pub struct Discount {
    /// Enable buy back for account
    #[serde(rename = "enabledForAccount")]
    pub enabled_for_account: bool,

    /// Enable buy back for symbol
    #[serde(rename = "enabledForSymbol")]
    pub enabled_for_symbol: bool,

    /// Discount asset
    #[serde(rename = "discountAsset")]
    pub discount_asset: String,

    /// Discount rate
    #[serde(rename = "discount")]
    pub discount: Decimal,
}

impl RestClient {
    /// Get current account commission rates
    ///
    /// Get current account commission rates.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#account-commission-rates--user_data)
    /// Method: GET /api/v3/account/commission
    /// Weight: 20
    /// Security: USER_DATA
    pub async fn get_account_commission(
        &self,
        params: AccountCommissionRequest,
    ) -> RestResult<AccountCommissionResponse> {
        let query_string = serde_urlencoded::to_string(&params)
            .map_err(|e| crate::binance::spot::Errors::Error(format!("URL encoding error: {e}")))?;

        self.send_request(
            "/api/v3/account/commission",
            reqwest::Method::GET,
            Some(&query_string),
            None,
            20,
            false,
        )
        .await
    }
}
