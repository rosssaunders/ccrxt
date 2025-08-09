//! Implements the /public/get_currencies endpoint for Deribit.
//!
//! Retrieves all cryptocurrencies supported by the API.

use serde::Deserialize;

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult, enums::Currency};

const CURRENCIES_ENDPOINT: &str = "public/get_currencies";

/// Represents a withdrawal priority for a currency.
#[derive(Debug, Clone, Deserialize)]
pub struct WithdrawalPriority {
    /// Name of the withdrawal priority.
    #[serde(rename = "name")]
    pub name: String,

    /// Value of the withdrawal priority.
    #[serde(rename = "value")]
    pub value: f64,
}

/// Represents a single currency supported by the API.
#[derive(Debug, Clone, Deserialize)]
pub struct CurrencyInfo {
    /// Simple Moving Average (SMA) of the last 7 days of rewards. Only for yield-generating tokens.
    #[serde(rename = "apr")]
    pub apr: Option<f64>,

    /// The type of the currency.
    #[serde(rename = "coin_type")]
    pub coin_type: String,

    /// The abbreviation of the currency.
    #[serde(rename = "currency")]
    pub currency: Currency,

    /// The full name for the currency.
    #[serde(rename = "currency_long")]
    pub currency_long: String,

    /// Fee precision.
    #[serde(rename = "fee_precision")]
    pub fee_precision: u32,

    /// True if the currency is part of the cross collateral pool.
    #[serde(rename = "in_cross_collateral_pool")]
    pub in_cross_collateral_pool: bool,

    /// Minimum number of block chain confirmations before deposit is accepted.
    #[serde(rename = "min_confirmations")]
    pub min_confirmations: u32,

    /// The minimum transaction fee paid for withdrawals.
    #[serde(rename = "min_withdrawal_fee")]
    pub min_withdrawal_fee: f64,

    /// The total transaction fee paid for withdrawals.
    #[serde(rename = "withdrawal_fee")]
    pub withdrawal_fee: f64,

    /// Withdrawal priorities for the currency.
    #[serde(rename = "withdrawal_priorities")]
    pub withdrawal_priorities: Vec<WithdrawalPriority>,
}

/// Response for public/get_combo_ids endpoint following Deribit JSON-RPC 2.0 format.
pub type GetCurrenciesResponse = JsonRpcResult<Vec<CurrencyInfo>>;

impl RestClient {
    /// Calls the /public/get_currencies endpoint.
    ///
    /// Retrieves all cryptocurrencies supported by the API.
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_currencies)
    pub async fn get_currencies(&self) -> RestResult<GetCurrenciesResponse> {
        self.send_post_request(
            CURRENCIES_ENDPOINT,
            None::<&()>,
            EndpointType::NonMatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 1,
            "jsonrpc": "2.0",
            "result": [
                {
                    "apr": 0.05,
                    "coin_type": "crypto",
                    "currency": "BTC",
                    "currency_long": "Bitcoin",
                    "fee_precision": 8,
                    "in_cross_collateral_pool": true,
                    "min_confirmations": 2,
                    "min_withdrawal_fee": 0.0005,
                    "withdrawal_fee": 0.0007,
                    "withdrawal_priorities": [
                        { "name": "high", "value": 0.0007 }
                    ]
                }
            ]
        }"#;
        let resp: GetCurrenciesResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 1);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.len(), 1);
        assert_eq!(resp.result[0].currency, Currency::BTC);
        assert_eq!(resp.result[0].currency_long, "Bitcoin");
        assert!(resp.result[0].in_cross_collateral_pool);
    }
}
